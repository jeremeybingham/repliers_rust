//! Listings search endpoint implementation
//!
//! POST https://api.repliers.io/listings

use crate::models::{ListingSearchRequest, ListingSearchResponse};
use crate::{RepliersClient, RepliersError};

impl RepliersClient {
    /// Search for listings based on various criteria
    ///
    /// # Arguments
    ///
    /// * `request` - Search parameters including filters for city, price, bedrooms, etc.
    ///
    /// # Returns
    ///
    /// Returns a paginated response containing matching listings.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails or the response cannot be parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use repliers_beta::{RepliersClient, ListingSearchRequest};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RepliersClient::new("api_key".to_string());
    ///
    /// let request = ListingSearchRequest {
    ///     city: Some("Toronto".to_string()),
    ///     min_price: Some(500000.0),
    ///     max_price: Some(1000000.0),
    ///     ..Default::default()
    /// };
    ///
    /// let results = client.search_listings(request).await?;
    /// println!("Found {} listings", results.count);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_listings(
        &self,
        request: ListingSearchRequest,
    ) -> Result<ListingSearchResponse, RepliersError> {
        let url = format!("{}/listings", self.base_url());

        let response = self
            .post_request(&url)
            .json(&request)
            .send()
            .await?;

        let response = Self::check_response(response).await?;
        let search_response = response.json::<ListingSearchResponse>().await?;

        Ok(search_response)
    }
}
