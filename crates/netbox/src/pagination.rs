//! Pagination support for NetBox API list endpoints

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::fmt;

/// A paginated response from the NetBox API
///
/// NetBox list endpoints return results in this format:
/// ```json
/// {
///   "count": 100,
///   "next": "https://netbox.example.com/api/dcim/devices/?offset=50",
///   "previous": "https://netbox.example.com/api/dcim/devices/?offset=0",
///   "results": [...]
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T> {
    /// Total number of results available
    pub count: usize,

    /// URL of the next page, if any
    pub next: Option<String>,

    /// URL of the previous page, if any
    pub previous: Option<String>,

    /// Results for this page
    pub results: Vec<T>,
}

impl<T> Page<T> {
    /// Check if there is a next page
    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }

    /// Check if there is a previous page
    pub fn has_previous(&self) -> bool {
        self.previous.is_some()
    }

    /// Check if this is the last page
    pub fn is_last(&self) -> bool {
        !self.has_next()
    }

    /// Get the number of results in this page
    pub fn len(&self) -> usize {
        self.results.len()
    }

    /// Check if this page is empty
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }
}

impl<T> fmt::Display for Page<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Page with {} results (total: {})",
            self.results.len(),
            self.count
        )
    }
}

/// Iterator for paginated API results
///
/// This allows iterating through all pages of results automatically.
///
/// # Example
///
/// ```no_run
/// use netbox::{Client, ClientConfig};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// # let config = ClientConfig::new("https://netbox.example.com", "token");
/// # let client = Client::new(config)?;
/// // let mut paginator = client.dcim().devices().paginate();
/// //
/// // while let Some(page) = paginator.next_page().await? {
/// //     for device in page.results {
/// //         println!("{:?}", device);
/// //     }
/// // }
/// # Ok(())
/// # }
/// ```
pub struct Paginator<T> {
    client: crate::Client,
    next_url: Option<String>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> Paginator<T>
where
    T: serde::de::DeserializeOwned,
{
    /// Create a new paginator starting from a given URL
    pub(crate) fn new(client: crate::Client, initial_path: String) -> Self {
        Self {
            client,
            next_url: Some(initial_path),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Fetch the next page of results
    ///
    /// Returns `Ok(None)` when there are no more pages.
    pub async fn next_page(&mut self) -> Result<Option<Page<T>>> {
        match self.next_url.take() {
            Some(url) => {
                let page: Page<T> = self.client.get(&url).await?;
                self.next_url = page.next.clone();
                Ok(Some(page))
            }
            None => Ok(None),
        }
    }

    /// Collect all results from all pages into a single vector
    ///
    /// **Warning**: This will fetch all pages, which could be slow and memory-intensive
    /// for large result sets.
    pub async fn collect_all(mut self) -> Result<Vec<T>> {
        let mut all_results = Vec::new();

        while let Some(page) = self.next_page().await? {
            all_results.extend(page.results);
        }

        Ok(all_results)
    }

    /// Limit the number of pages to fetch
    pub fn limit_pages(self, max_pages: usize) -> LimitedPaginator<T> {
        LimitedPaginator {
            paginator: self,
            max_pages,
            current_page: 0,
        }
    }
}

#[cfg(test)]
impl<T> Paginator<T> {
    pub(crate) fn next_url(&self) -> Option<&str> {
        self.next_url.as_deref()
    }
}

/// A paginator that limits the number of pages fetched
pub struct LimitedPaginator<T> {
    paginator: Paginator<T>,
    max_pages: usize,
    current_page: usize,
}

impl<T> LimitedPaginator<T>
where
    T: serde::de::DeserializeOwned,
{
    /// Fetch the next page, respecting the page limit
    pub async fn next_page(&mut self) -> Result<Option<Page<T>>> {
        if self.current_page >= self.max_pages {
            return Ok(None);
        }

        self.current_page += 1;
        self.paginator.next_page().await
    }

    /// Collect all results up to the page limit
    pub async fn collect_all(mut self) -> Result<Vec<T>> {
        let mut all_results = Vec::new();

        while let Some(page) = self.next_page().await? {
            all_results.extend(page.results);
        }

        Ok(all_results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ClientConfig;
    use httpmock::Method::GET;
    use httpmock::MockServer;

    #[test]
    fn test_page_helpers() {
        let page: Page<String> = Page {
            count: 100,
            next: Some("https://example.com/next".to_string()),
            previous: None,
            results: vec!["item1".to_string(), "item2".to_string()],
        };

        assert_eq!(page.len(), 2);
        assert!(!page.is_empty());
        assert!(page.has_next());
        assert!(!page.has_previous());
        assert!(!page.is_last());
    }

    #[test]
    fn test_page_previous_and_last() {
        let page: Page<String> = Page {
            count: 10,
            next: None,
            previous: Some("https://example.com/prev".to_string()),
            results: vec!["item1".to_string()],
        };

        assert!(page.has_previous());
        assert!(page.is_last());
    }

    #[test]
    fn test_page_display() {
        let page: Page<String> = Page {
            count: 100,
            next: None,
            previous: None,
            results: vec!["item1".to_string()],
        };

        let display = format!("{}", page);
        assert!(display.contains("1 results"));
        assert!(display.contains("total: 100"));
    }

    #[test]
    fn test_empty_page() {
        let page: Page<String> = Page {
            count: 0,
            next: None,
            previous: None,
            results: vec![],
        };

        assert_eq!(page.len(), 0);
        assert!(page.is_empty());
        assert!(page.is_last());
    }

    #[tokio::test]
    async fn paginator_fetches_multiple_pages() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "token").with_max_retries(0);
        let client = crate::Client::new(config).unwrap();

        let first = server.mock(|when, then| {
            when.method(GET)
                .path("/api/dcim/devices/")
                .query_param("offset", "0");
            then.status(200).json_body(serde_json::json!({
                "count": 2,
                "next": "dcim/devices/?offset=1",
                "previous": null,
                "results": [1]
            }));
        });

        let second = server.mock(|when, then| {
            when.method(GET)
                .path("/api/dcim/devices/")
                .query_param("offset", "1");
            then.status(200).json_body(serde_json::json!({
                "count": 2,
                "next": null,
                "previous": "dcim/devices/?offset=0",
                "results": [2]
            }));
        });

        let mut paginator: Paginator<i32> =
            Paginator::new(client, "dcim/devices/?offset=0".to_string());

        let page1 = paginator.next_page().await.unwrap().unwrap();
        assert_eq!(page1.results, vec![1]);
        assert_eq!(paginator.next_url(), Some("dcim/devices/?offset=1"));

        let page2 = paginator.next_page().await.unwrap().unwrap();
        assert_eq!(page2.results, vec![2]);
        assert_eq!(paginator.next_url(), None);

        assert!(paginator.next_page().await.unwrap().is_none());
        first.assert();
        second.assert();
    }

    #[tokio::test]
    async fn paginator_collects_all_results() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "token").with_max_retries(0);
        let client = crate::Client::new(config).unwrap();

        server.mock(|when, then| {
            when.method(GET)
                .path("/api/dcim/devices/")
                .query_param("offset", "0");
            then.status(200).json_body(serde_json::json!({
                "count": 3,
                "next": "dcim/devices/?offset=2",
                "previous": null,
                "results": [1, 2]
            }));
        });

        server.mock(|when, then| {
            when.method(GET)
                .path("/api/dcim/devices/")
                .query_param("offset", "2");
            then.status(200).json_body(serde_json::json!({
                "count": 3,
                "next": null,
                "previous": "dcim/devices/?offset=0",
                "results": [3]
            }));
        });

        let paginator: Paginator<i32> =
            Paginator::new(client, "dcim/devices/?offset=0".to_string());
        let results = paginator.collect_all().await.unwrap();
        assert_eq!(results, vec![1, 2, 3]);
    }

    #[tokio::test]
    async fn limited_paginator_stops_at_limit() {
        let server = MockServer::start();
        let config = ClientConfig::new(server.base_url(), "token").with_max_retries(0);
        let client = crate::Client::new(config).unwrap();

        let first = server.mock(|when, then| {
            when.method(GET)
                .path("/api/dcim/devices/")
                .query_param("offset", "0");
            then.status(200).json_body(serde_json::json!({
                "count": 2,
                "next": "dcim/devices/?offset=1",
                "previous": null,
                "results": [1]
            }));
        });

        let second = server.mock(|when, then| {
            when.method(GET)
                .path("/api/dcim/devices/")
                .query_param("offset", "1");
            then.status(200).json_body(serde_json::json!({
                "count": 2,
                "next": null,
                "previous": "dcim/devices/?offset=0",
                "results": [2]
            }));
        });

        let paginator: Paginator<i32> =
            Paginator::new(client, "dcim/devices/?offset=0".to_string());
        let mut limited = paginator.limit_pages(1);
        let page = limited.next_page().await.unwrap().unwrap();
        assert_eq!(page.results, vec![1]);
        assert!(limited.next_page().await.unwrap().is_none());
        assert_eq!(second.hits(), 0);
        first.assert();
    }
}
