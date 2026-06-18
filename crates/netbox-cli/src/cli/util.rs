use reqwest::Method;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt;
use std::fs;
use std::path::PathBuf;

use crate::{ApiClient, GraphqlInput, JsonInput, JsonInputOptional};

#[derive(Debug)]
pub(crate) struct RequestError {
    method: Method,
    path: String,
    source: Box<dyn std::error::Error + 'static>,
}

impl RequestError {
    pub(crate) fn new(
        method: Method,
        path: impl Into<String>,
        source: Box<dyn std::error::Error + 'static>,
    ) -> Self {
        Self {
            method,
            path: path.into(),
            source,
        }
    }
}

impl fmt::Display for RequestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(message) = format_netbox_error(&self.method, &self.path, self.source.as_ref()) {
            return write!(f, "{message}");
        }
        write!(
            f,
            "request failed: {} {}: {}",
            self.method.as_str(),
            self.path,
            self.source
        )
    }
}

impl std::error::Error for RequestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&*self.source)
    }
}

pub(crate) fn normalize_api_path(path: &str) -> String {
    let trimmed = path.trim_start_matches('/');
    match trimmed.strip_prefix("api/") {
        Some(stripped) => stripped.to_string(),
        None => trimmed.to_string(),
    }
}

pub(crate) fn append_query(
    path: &str,
    query: &[String],
) -> Result<String, Box<dyn std::error::Error>> {
    let pairs = parse_query_pairs(query)?;
    if pairs.is_empty() {
        return Ok(path.to_string());
    }

    let query_string = serde_urlencoded::to_string(pairs)?;
    let separator = if path.contains('?') { "&" } else { "?" };
    Ok(format!("{}{}{}", path, separator, query_string))
}

pub(crate) fn parse_query_pairs(
    query: &[String],
) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
    let mut pairs = Vec::with_capacity(query.len());
    for item in query {
        let mut parts = item.splitn(2, '=');
        let key = parts.next().unwrap_or_default();
        let value = parts.next();
        if key.is_empty() || value.is_none() {
            return Err(format!("Invalid query parameter: {}", item).into());
        }
        pairs.push((key.to_string(), value.unwrap().to_string()));
    }
    Ok(pairs)
}

pub(crate) fn load_json<T>(input: &JsonInput) -> Result<T, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let content = if let Some(json) = &input.json {
        json.clone()
    } else if let Some(path) = &input.file {
        fs::read_to_string(path)?
    } else {
        return Err("Provide --json or --file".into());
    };

    Ok(serde_json::from_str(&content)?)
}

pub(crate) fn load_json_optional<T>(
    input: &JsonInputOptional,
) -> Result<Option<T>, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let content = if let Some(json) = &input.json {
        Some(json.clone())
    } else if let Some(path) = &input.file {
        Some(fs::read_to_string(path)?)
    } else {
        None
    };

    match content {
        Some(content) => Ok(Some(serde_json::from_str(&content)?)),
        None => Ok(None),
    }
}

pub(crate) fn load_graphql_query(
    input: &GraphqlInput,
) -> Result<String, Box<dyn std::error::Error>> {
    if let Some(query) = &input.query {
        return Ok(query.clone());
    }
    if let Some(path) = &input.query_file {
        return Ok(fs::read_to_string(path)?);
    }
    Err("Provide --query or --query-file".into())
}

pub(crate) fn load_graphql_vars(
    input: &GraphqlInput,
) -> Result<Option<Value>, Box<dyn std::error::Error>> {
    match &input.vars {
        Some(vars) => Ok(Some(serde_json::from_str(vars)?)),
        None => Ok(None),
    }
}

pub(crate) fn build_schema_path(
    format: Option<&str>,
    lang: Option<&str>,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut query = Vec::new();
    if let Some(format) = format {
        query.push(format!("format={}", format));
    }
    if let Some(lang) = lang {
        query.push(format!("lang={}", lang));
    }
    append_query("schema/", &query)
}

pub(crate) async fn request_raw_with_context(
    client: &impl ApiClient,
    method: Method,
    path: &str,
    body: Option<&Value>,
) -> Result<Value, Box<dyn std::error::Error>> {
    client
        .request_raw(method.clone(), path, body)
        .await
        .map_err(|err| wrap_request_error(method, path, err))
}

pub(crate) fn wrap_request_error(
    method: Method,
    path: &str,
    err: Box<dyn std::error::Error + 'static>,
) -> Box<dyn std::error::Error> {
    Box::new(RequestError::new(method, path, err))
}

pub(crate) fn format_netbox_error(
    method: &Method,
    path: &str,
    err: &(dyn std::error::Error + 'static),
) -> Option<String> {
    let netbox_err = err.downcast_ref::<netbox::Error>()?;
    let netbox::Error::ApiError {
        status,
        message,
        body,
    } = netbox_err
    else {
        return None;
    };

    let mut detail = format!("status {}", status);
    if let Some(request_id) = extract_request_id(body) {
        detail.push_str(&format!(", request_id {request_id}"));
    }
    let mut summary = format!("request failed: {} {} ({detail})", method.as_str(), path);
    if !message.is_empty() {
        summary.push_str(": ");
        summary.push_str(message);
    }
    Some(summary)
}

fn extract_request_id(body: &str) -> Option<String> {
    let value: Value = serde_json::from_str(body).ok()?;
    for key in ["request_id", "requestId", "request-id"] {
        if let Some(Value::String(id)) = value.get(key) {
            return Some(id.clone());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};
    use std::env;

    #[test]
    fn load_json_from_inline() {
        let input = JsonInput {
            json: Some(r#"{"name":"carrier","slug":"carrier"}"#.to_string()),
            file: None,
        };
        let value: Value = load_json(&input).unwrap();
        assert_eq!(value["name"], "carrier");
        assert_eq!(value["slug"], "carrier");
    }

    #[test]
    fn load_json_from_file() {
        let mut path = env::temp_dir();
        path.push("netbox-cli-test.json");
        fs::write(&path, r#"{"name":"carrier","slug":"carrier"}"#).unwrap();

        let input = JsonInput {
            json: None,
            file: Some(path.clone()),
        };
        let value: Value = load_json(&input).unwrap();
        assert_eq!(value["name"], "carrier");
        assert_eq!(value["slug"], "carrier");

        let _ = fs::remove_file(path);
    }

    #[test]
    fn load_json_requires_input() {
        let input = JsonInput {
            json: None,
            file: None,
        };
        let result: Result<Value, _> = load_json(&input);
        assert!(result.is_err());
    }

    #[test]
    fn load_json_rejects_invalid_json() {
        let input = JsonInput {
            json: Some("{invalid}".to_string()),
            file: None,
        };
        let result: Result<Value, _> = load_json(&input);
        assert!(result.is_err());
    }

    #[test]
    fn load_json_optional_handles_none() {
        let input = JsonInputOptional {
            json: None,
            file: None,
        };
        let value: Option<Value> = load_json_optional(&input).unwrap();
        assert!(value.is_none());
    }

    #[test]
    fn load_json_optional_rejects_invalid_json() {
        let input = JsonInputOptional {
            json: Some("{invalid}".to_string()),
            file: None,
        };
        let result: Result<Option<Value>, _> = load_json_optional(&input);
        assert!(result.is_err());
    }

    #[test]
    fn load_graphql_query_prefers_inline() {
        let input = GraphqlInput {
            query: Some("{ devices { name } }".to_string()),
            query_file: None,
            vars: None,
        };
        let query = load_graphql_query(&input).unwrap();
        assert_eq!(query, "{ devices { name } }");
    }

    #[test]
    fn load_graphql_query_reads_file() {
        let mut path = env::temp_dir();
        path.push("netbox-cli-graphql.graphql");
        fs::write(&path, "{ devices { name } }").unwrap();

        let input = GraphqlInput {
            query: None,
            query_file: Some(path.clone()),
            vars: None,
        };
        let query = load_graphql_query(&input).unwrap();
        assert_eq!(query, "{ devices { name } }");

        let _ = fs::remove_file(path);
    }

    #[test]
    fn load_graphql_vars_parses_json() {
        let input = GraphqlInput {
            query: Some("{ devices { name } }".to_string()),
            query_file: None,
            vars: Some(r#"{"limit":5}"#.to_string()),
        };
        let vars = load_graphql_vars(&input).unwrap().unwrap();
        assert_eq!(vars["limit"], 5);
    }

    #[test]
    fn append_query_encodes_pairs() {
        let path = "dcim/devices/";
        let query = vec!["name=leaf 1".to_string(), "limit=5".to_string()];
        let full = append_query(path, &query).unwrap();
        assert_eq!(full, "dcim/devices/?name=leaf+1&limit=5");
    }

    #[test]
    fn append_query_rejects_missing_value() {
        let path = "dcim/devices/";
        let query = vec!["name".to_string()];
        let result = append_query(path, &query);
        assert!(result.is_err());
    }

    #[test]
    fn append_query_appends_when_query_present() {
        let path = "dcim/devices/?name=leaf-1";
        let query = vec!["limit=5".to_string()];
        let full = append_query(path, &query).unwrap();
        assert_eq!(full, "dcim/devices/?name=leaf-1&limit=5");
    }

    #[test]
    fn parse_query_pairs_rejects_empty_key() {
        let query = vec!["=value".to_string()];
        let result = parse_query_pairs(&query);
        assert!(result.is_err());
    }

    #[test]
    fn normalize_api_path_strips_prefix() {
        assert_eq!(normalize_api_path("api/dcim/devices/"), "dcim/devices/");
        assert_eq!(normalize_api_path("/api/dcim/devices/"), "dcim/devices/");
        assert_eq!(normalize_api_path("dcim/devices/"), "dcim/devices/");
        assert_eq!(normalize_api_path("/dcim/devices/"), "dcim/devices/");
    }

    #[test]
    fn build_schema_path_includes_query() {
        let path = build_schema_path(Some("json"), Some("en")).unwrap();
        assert_eq!(path, "schema/?format=json&lang=en");
    }

    #[test]
    fn format_netbox_error_includes_status_path_and_request_id() {
        let body = r#"{"request_id":"req-123","detail":"bad"}"#.to_string();
        let err = netbox::Error::ApiError {
            status: 400,
            message: "bad".to_string(),
            body,
        };
        let wrapped = RequestError::new(Method::POST, "dcim/devices/", Box::new(err));
        let message = wrapped.to_string();
        assert!(message.contains("POST"));
        assert!(message.contains("dcim/devices/"));
        assert!(message.contains("status 400"));
        assert!(message.contains("request_id req-123"));
        assert!(message.contains("bad"));
    }
}
