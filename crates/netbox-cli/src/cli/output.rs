use comfy_table::{Cell, ContentArrangement, Table};
use reqwest::Method;
use serde_json::{Value, to_string_pretty};
use terminal_size::{Width, terminal_size};

use crate::OutputConfig;

pub(crate) fn print_output(
    value: &Value,
    output: &OutputConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    let formatted = format_output(value, output)?;
    println!("{formatted}");
    Ok(())
}

pub(crate) fn format_output(
    value: &Value,
    output: &OutputConfig,
) -> Result<String, Box<dyn std::error::Error>> {
    let selected = match output.select.as_deref() {
        Some(path) => select_value(value, path),
        None => value.clone(),
    };

    match output.format {
        crate::OutputFormat::Json => Ok(to_string_pretty(&selected)?),
        crate::OutputFormat::Yaml => Ok(serde_yaml::to_string(&selected)?),
        crate::OutputFormat::Table => Ok(format_table(
            &selected,
            output.columns.as_deref(),
            output.max_columns,
        )),
    }
}

pub(crate) fn print_dry_run(
    method: Method,
    path: &str,
    query: Option<&[String]>,
    body: Option<&Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    let full_path = match query {
        Some(query) => super::util::append_query(path, query)?,
        None => path.to_string(),
    };
    let payload = dry_run_payload(method, &full_path, body);
    println!("{}", to_string_pretty(&payload)?);
    Ok(())
}

fn dry_run_payload(method: Method, path: &str, body: Option<&Value>) -> Value {
    serde_json::json!({
        "method": method.as_str(),
        "path": path,
        "body": body,
    })
}

pub(crate) fn format_table(
    value: &Value,
    columns: Option<&[String]>,
    max_columns: usize,
) -> String {
    let width = terminal_width().unwrap_or(120).min(u16::MAX as usize) as u16;
    if let Value::Object(map) = value
        && let Some(Value::Array(items)) = map.get("results")
    {
        let summary = format_table_summary(map);
        let table = table_from_items(items, width, columns, max_columns);
        return if summary.is_empty() {
            table
        } else {
            format!("{summary}\n{table}")
        };
    }

    match value {
        Value::Array(items) => table_from_items(items, width, columns, max_columns),
        Value::Object(map) => {
            let mut table = base_table(width);
            let headers: Vec<String> = if let Some(cols) = columns {
                cols.to_vec()
            } else {
                map.keys().take(max_columns).cloned().collect()
            };
            table.set_header(headers.iter().map(Cell::new));
            let row = headers
                .iter()
                .map(|key| Cell::new(value_to_cell(map.get(key))))
                .collect::<Vec<_>>();
            table.add_row(row);
            table.to_string()
        }
        _ => {
            let mut table = base_table(width);
            table.set_header(vec![Cell::new("value")]);
            table.add_row(vec![Cell::new(value_to_cell(Some(value)))]);
            table.to_string()
        }
    }
}

fn terminal_width() -> Option<usize> {
    terminal_size().map(|(Width(width), _)| width as usize)
}

fn value_to_cell(value: Option<&Value>) -> String {
    match value {
        Some(Value::Null) | None => "".to_string(),
        Some(Value::String(value)) => value.clone(),
        Some(Value::Number(value)) => value.to_string(),
        Some(Value::Bool(value)) => value.to_string(),
        Some(Value::Array(items)) => format!("[{}]", items.len()),
        Some(Value::Object(map)) => extract_display(map)
            .or_else(|| {
                map.get("id")
                    .and_then(Value::as_i64)
                    .map(|id| id.to_string())
            })
            .unwrap_or_else(|| compact_json(&Value::Object(map.clone()))),
    }
}

fn base_table(width: u16) -> Table {
    let mut table = Table::new();
    table
        .load_preset(comfy_table::presets::ASCII_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_width(width);
    table
}

fn table_from_items(
    items: &[Value],
    width: u16,
    columns: Option<&[String]>,
    max_columns: usize,
) -> String {
    let mut table = base_table(width);
    if items.is_empty() {
        let headers = columns
            .filter(|cols| !cols.is_empty())
            .map(|cols| cols.to_vec())
            .unwrap_or_else(|| vec!["value".to_string()]);
        table.set_header(headers.iter().map(Cell::new));
        return table.to_string();
    }

    if let Some(Value::Object(first)) = items.first() {
        let headers = if let Some(cols) = columns {
            cols.to_vec()
        } else {
            infer_columns(items, first, max_columns)
        };
        table.set_header(headers.iter().map(Cell::new));
        for item in items {
            if let Value::Object(map) = item {
                let row = headers
                    .iter()
                    .map(|key| Cell::new(value_to_cell(map.get(key))))
                    .collect::<Vec<_>>();
                table.add_row(row);
            } else {
                table.add_row(vec![Cell::new(value_to_cell(Some(item)))]);
            }
        }
    } else {
        // scalars have no fields to select, so explicit columns are ignored here
        // rather than used to drop the rows.
        table.set_header(vec![Cell::new("value")]);
        for item in items {
            table.add_row(vec![Cell::new(value_to_cell(Some(item)))]);
        }
    }
    table.to_string()
}

fn infer_columns(
    items: &[Value],
    first: &serde_json::Map<String, Value>,
    max_columns: usize,
) -> Vec<String> {
    let preferred = [
        "id",
        "name",
        "display",
        "slug",
        "status",
        "site",
        "role",
        "device_type",
        "manufacturer",
        "model",
        "url",
    ];

    let mut columns = Vec::new();
    for key in preferred {
        if first.contains_key(key) && columns.len() < max_columns {
            columns.push(key.to_string());
        }
    }

    if columns.is_empty() {
        columns = first.keys().take(max_columns).cloned().collect();
    }

    if columns.len() < max_columns {
        let mut additional = first
            .keys()
            .filter(|key| !columns.iter().any(|col| col == *key))
            .take(max_columns - columns.len())
            .cloned()
            .collect::<Vec<_>>();
        columns.append(&mut additional);
    }

    if columns.len() > max_columns {
        columns.truncate(max_columns);
    }

    if columns.len() > 1 && items.iter().any(|item| matches!(item, Value::Object(_))) {
        columns
    } else {
        vec!["value".to_string()]
    }
}

fn format_table_summary(map: &serde_json::Map<String, Value>) -> String {
    let count = map
        .get("count")
        .and_then(Value::as_i64)
        .map(|v| v.to_string());
    let next = map
        .get("next")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let previous = map
        .get("previous")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();

    let mut parts = Vec::new();
    if let Some(count) = count {
        parts.push(format!("count: {count}"));
    }
    if !next.is_empty() {
        parts.push(format!("next: {next}"));
    }
    if !previous.is_empty() {
        parts.push(format!("previous: {previous}"));
    }
    parts.join(" | ")
}

fn extract_display(map: &serde_json::Map<String, Value>) -> Option<String> {
    for key in ["display", "name", "label", "value", "slug"] {
        if let Some(Value::String(value)) = map.get(key) {
            return Some(value.clone());
        }
    }
    None
}

fn compact_json(value: &Value) -> String {
    let raw = serde_json::to_string(value).unwrap_or_else(|_| "<invalid>".to_string());
    if raw.len() > 120 {
        let end = raw.floor_char_boundary(117);
        format!("{}...", &raw[..end])
    } else {
        raw
    }
}

fn select_value(value: &Value, path: &str) -> Value {
    let segments: Vec<&str> = path.split('.').filter(|seg| !seg.is_empty()).collect();
    select_value_segments(value, &segments)
}

fn select_value_segments(value: &Value, segments: &[&str]) -> Value {
    if segments.is_empty() {
        return value.clone();
    }

    match value {
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|item| select_value_segments(item, segments))
                .collect(),
        ),
        Value::Object(map) => map
            .get(segments[0])
            .map(|next| select_value_segments(next, &segments[1..]))
            .unwrap_or(Value::Null),
        _ => Value::Null,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn format_table_handles_objects() {
        let value = json!({"name": "leaf-1", "status": "active"});
        let table = format_table(&value, None, 6);
        assert!(table.contains("name"));
        assert!(table.contains("leaf-1"));
    }

    #[test]
    fn dry_run_payload_includes_path_and_body() {
        let payload = dry_run_payload(
            Method::POST,
            "dcim/devices/",
            Some(&json!({"name":"leaf-1"})),
        );
        assert_eq!(payload["method"], "POST");
        assert_eq!(payload["path"], "dcim/devices/");
        assert_eq!(payload["body"]["name"], "leaf-1");
    }

    #[test]
    fn format_table_flattens_results() {
        let value = json!({
            "count": 2,
            "next": null,
            "previous": null,
            "results": [
                {"id": 1, "name": "alpha"},
                {"id": 2, "name": "beta"}
            ]
        });
        let table = format_table(&value, None, 6);
        assert!(table.contains("count: 2"));
        assert!(table.contains("alpha"));
        assert!(table.contains("beta"));
    }

    #[test]
    fn format_table_respects_explicit_columns() {
        let value = json!({
            "results": [
                {"id": 1, "name": "alpha", "status": "active", "extra": "ignored"},
                {"id": 2, "name": "beta", "status": "planned", "extra": "also ignored"}
            ]
        });
        let columns = vec!["name".to_string(), "status".to_string()];
        let table = format_table(&value, Some(&columns), 6);
        assert!(table.contains("name"));
        assert!(table.contains("status"));
        assert!(table.contains("alpha"));
        assert!(table.contains("active"));
        assert!(!table.contains("extra"));
        assert!(!table.contains("ignored"));
    }

    #[test]
    fn format_table_respects_explicit_columns_with_empty_results() {
        let value = json!({
            "count": 0,
            "next": null,
            "previous": null,
            "results": []
        });
        let columns = vec!["id".to_string(), "name".to_string(), "slug".to_string()];
        let table = format_table(&value, Some(&columns), 6);
        assert!(table.contains("id"));
        assert!(table.contains("name"));
        assert!(table.contains("slug"));
        assert!(!table.contains("value"));
    }

    #[test]
    fn format_table_keeps_non_object_rows_with_explicit_columns() {
        let value = json!(["alpha", "beta", 42]);
        let columns = vec!["name".to_string()];
        let table = format_table(&value, Some(&columns), 6);
        assert!(table.contains("value"));
        assert!(!table.contains("name"));
        assert!(table.contains("alpha"));
        assert!(table.contains("beta"));
        assert!(table.contains("42"));
    }

    #[test]
    fn format_table_keeps_non_object_rows_when_first_item_is_scalar() {
        let value = json!(["bare", {"name": "alpha"}]);
        let columns = vec!["name".to_string()];
        let table = format_table(&value, Some(&columns), 6);
        assert!(table.contains("bare"));
        assert!(table.contains("alpha"));
    }

    #[test]
    fn format_table_keeps_non_object_rows_when_first_item_is_object() {
        let value = json!([{"name": "alpha", "status": "active"}, "bare"]);
        let columns = vec!["name".to_string()];
        let table = format_table(&value, Some(&columns), 6);
        assert!(table.contains("name"));
        assert!(table.contains("alpha"));
        assert!(table.contains("bare"));
        assert!(!table.contains("active"));
    }

    #[test]
    fn format_table_renders_non_object_items_without_columns() {
        let value = json!(["alpha", "beta"]);
        let table = format_table(&value, None, 6);
        assert!(table.contains("value"));
        assert!(table.contains("alpha"));
        assert!(table.contains("beta"));
    }

    #[test]
    fn format_table_renders_empty_array_without_columns() {
        let value = json!([]);
        let table = format_table(&value, None, 6);
        assert!(table.contains("value"));
    }

    #[test]
    fn format_table_respects_max_columns() {
        let value = json!({
            "results": [
                {"a": 1, "b": 2, "c": 3, "d": 4, "e": 5}
            ]
        });
        let table = format_table(&value, None, 2);
        let header_line = table.lines().nth(1).unwrap_or("");
        let column_count = header_line
            .split('|')
            .filter(|s| !s.trim().is_empty())
            .count();
        assert_eq!(column_count, 2);
    }

    #[test]
    fn select_value_handles_arrays() {
        let value = json!({
            "results": [
                {"name": "a"},
                {"name": "b"}
            ]
        });
        let selected = select_value(&value, "results.name");
        assert_eq!(selected, json!(["a", "b"]));
    }
}
