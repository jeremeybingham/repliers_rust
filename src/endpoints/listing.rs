//! Single listing retrieval endpoint
//!
//! GET https://api.repliers.io/listings/{mlsNumber}

use crate::{RepliersClient, RepliersError};

impl RepliersClient {
    /// Get detailed information for a single listing
    ///
    /// Returns expanded view including comparables and history.
    ///
    /// # Arguments
    ///
    /// * `mls_number` - The MLS number of the listing
    /// * `board_id` - Optional board ID (required only for multi-MLS accounts)
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use repliers_beta::RepliersClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RepliersClient::new("api_key".to_string());
    ///
    /// let listing = client.get_listing("N12345678", None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_listing(
        &self,
        mls_number: &str,
        board_id: Option<&str>,
    ) -> Result<serde_json::Value, RepliersError> {
        let url = format!("{}/listings/{}", self.base_url(), mls_number);

        let mut request = self.get_request(&url);

        if let Some(bid) = board_id {
            request = request.query(&[("boardId", bid)]);
        }

        let response = request.send().await?;
        let response = Self::check_response(response).await?;
        let listing_response = response.json::<serde_json::Value>().await?;

        Ok(listing_response)
    }
}
