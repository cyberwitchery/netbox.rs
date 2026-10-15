//! query parameter builder for filtering api requests

use serde::Serialize;
use std::collections::HashMap;

/// builder for constructing query parameters for netbox api requests
///
/// netbox supports various filtering options through query parameters.
/// this builder makes it easy to construct complex filters.
///
/// # Example
///
/// ```
/// use netbox::QueryBuilder;
///
/// let query = QueryBuilder::new()
///     .filter("site", "dc1")
///     .filter("status", "active")
///     .limit(50)
///     .build();
/// ```
#[derive(Debug, Default, Clone)]
pub struct QueryBuilder {
    params: HashMap<String, Vec<String>>,
}

impl Serialize for QueryBuilder {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut pairs: Vec<(&str, &str)> = Vec::new();
        for (key, values) in &self.params {
            for value in values {
                pairs.push((key.as_str(), value.as_str()));
            }
        }
        pairs.serialize(serializer)
    }
}

impl QueryBuilder {
    /// create a new empty query builder
    pub fn new() -> Self {
        Self::default()
    }

    /// add a filter parameter
    ///
    /// multiple values for the same key are allowed (netbox interprets them as OR).
    ///
    /// # Example
    ///
    /// ```
    /// use netbox::QueryBuilder;
    ///
    /// let query = QueryBuilder::new()
    ///     .filter("site", "dc1")
    ///     .filter("site", "dc2");
    /// ```
    pub fn filter(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        let key = key.into();
        let value = value.into();
        self.params.entry(key).or_default().push(value);
        self
    }

    /// add multiple filters from an iterator
    pub fn filters<I, K, V>(mut self, filters: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        for (key, value) in filters {
            self = self.filter(key, value);
        }
        self
    }

    /// set the limit (page size)
    ///
    /// default: netbox's default (usually 50)
    pub fn limit(self, limit: usize) -> Self {
        self.filter("limit", limit.to_string())
    }

    /// set the offset for pagination
    pub fn offset(self, offset: usize) -> Self {
        self.filter("offset", offset.to_string())
    }

    /// set ordering field
    ///
    /// # Example
    ///
    /// ```
    /// use netbox::QueryBuilder;
    ///
    /// let query = QueryBuilder::new().order_by("name");
    ///
    /// let query = QueryBuilder::new().order_by("-name");
    /// ```
    pub fn order_by(self, field: impl Into<String>) -> Self {
        self.filter("ordering", field)
    }

    /// search query (if the endpoint supports it)
    pub fn search(self, query: impl Into<String>) -> Self {
        self.filter("q", query)
    }

    /// filter by id
    pub fn id(self, id: impl Into<String>) -> Self {
        self.filter("id", id)
    }

    /// filter by multiple IDs
    pub fn ids<I, V>(mut self, ids: I) -> Self
    where
        I: IntoIterator<Item = V>,
        V: Into<String>,
    {
        for id in ids {
            self = self.filter("id", id);
        }
        self
    }

    /// filter by tag
    pub fn tag(self, tag: impl Into<String>) -> Self {
        self.filter("tag", tag)
    }

    /// add a custom parameter
    ///
    /// this is useful for filters not covered by the builder methods.
    pub fn param(self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.filter(key, value)
    }

    /// build the query parameters
    ///
    /// this returns self, but the method makes the api more explicit.
    pub fn build(self) -> Self {
        self
    }

    /// check if the query is empty
    pub fn is_empty(&self) -> bool {
        self.params.is_empty()
    }
}

/// helper function to build query parameters from filters
#[allow(dead_code)]
pub fn filters<I, K, V>(filters: I) -> QueryBuilder
where
    I: IntoIterator<Item = (K, V)>,
    K: Into<String>,
    V: Into<String>,
{
    QueryBuilder::new().filters(filters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_params() {
        type Params<'a> = Vec<(&'a str, Vec<&'a str>)>;
        // (builder, expected params)
        let cases: Vec<(QueryBuilder, Params)> = vec![
            (QueryBuilder::new(), vec![]),
            (
                QueryBuilder::new().filter("site", "dc1"),
                vec![("site", vec!["dc1"])],
            ),
            (
                QueryBuilder::new()
                    .filter("site", "dc1")
                    .filter("site", "dc2"),
                vec![("site", vec!["dc1", "dc2"])],
            ),
            (
                QueryBuilder::new().limit(100).offset(50),
                vec![("limit", vec!["100"]), ("offset", vec!["50"])],
            ),
            (
                QueryBuilder::new().order_by("-name"),
                vec![("ordering", vec!["-name"])],
            ),
            (
                QueryBuilder::new().search("test device"),
                vec![("q", vec!["test device"])],
            ),
            (
                QueryBuilder::new().ids(vec!["1", "2", "3"]),
                vec![("id", vec!["1", "2", "3"])],
            ),
        ];
        for (query, expected) in cases {
            let expected: HashMap<String, Vec<String>> = expected
                .into_iter()
                .map(|(key, values)| {
                    (
                        key.to_string(),
                        values.into_iter().map(str::to_string).collect(),
                    )
                })
                .collect();
            assert_eq!(query.is_empty(), expected.is_empty());
            assert_eq!(query.params, expected);
        }
    }

    #[test]
    fn test_serialization() {
        let query = QueryBuilder::new()
            .filter("site", "dc1")
            .filter("status", "active")
            .limit(50);

        let json = serde_json::to_value(&query).unwrap();
        let pairs = json
            .as_array()
            .expect("query serialization should be a list of pairs");
        assert!(
            pairs
                .iter()
                .any(|pair| pair == &serde_json::json!(["site", "dc1"]))
        );
        assert!(
            pairs
                .iter()
                .any(|pair| pair == &serde_json::json!(["status", "active"]))
        );
        assert!(
            pairs
                .iter()
                .any(|pair| pair == &serde_json::json!(["limit", "50"]))
        );
    }

    #[test]
    fn test_filters_helper() {
        let query = filters([("site", "dc1"), ("status", "active")]);
        assert_eq!(query.params.len(), 2);
    }
}
