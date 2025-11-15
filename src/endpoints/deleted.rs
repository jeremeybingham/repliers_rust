//! Deleted listings endpoint
//!
//! GET https://api.repliers.io/listings/deleted

use crate::models::{DeletedListingsQuery, DeletedListingsResponse};
use crate::{RepliersClient, RepliersError};

impl RepliersClient {
    /// Get listings that have been deleted from the MLS
    ///
    /// Useful for data synchronization and keeping local databases up to date.
    ///
    /// # Arguments
    ///
    /// * `query` - Query parameters including date ranges and pagination
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use repliers_beta::{RepliersClient, DeletedListingsQuery};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RepliersClient::new("api_key".to_string());
    ///
    /// let query = DeletedListingsQuery {
    ///     min_updated_on: Some("2024-01-01".to_string()),
    ///     max_updated_on: Some("2024-12-31".to_string()),
    ///     page: Some(1),
    ///     results_per_page: Some(100),
    ///     ..Default::default()
    /// };
    ///
    /// let deleted = client.get_deleted_listings(query).await?;
    /// println!("Found {} deleted listings", deleted.count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_deleted_listings(
        &self,
        query: DeletedListingsQuery,
    ) -> Result<DeletedListingsResponse, RepliersError> {
        let url = format!("{}/listings/deleted", self.base_url());

        let response = self
            .get_request(&url)
            .query(&query)
            .send()
            .await?;

        let response = Self::check_response(response).await?;
        let deleted_response = response.json::<DeletedListingsResponse>().await?;

        Ok(deleted_response)
    }
}
