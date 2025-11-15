//! Similar listings endpoint
//!
//! GET https://api.repliers.io/listings/{mlsNumber}/similar

use crate::{models::search::SimilarListingsRequest, models::search::SimilarListingsResponse, RepliersClient, RepliersError};

impl RepliersClient {
    /// Find listings similar to a given property
    ///
    /// # Arguments
    ///
    /// * `request` - Similar listings search parameters
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use repliers_beta::{RepliersClient, models::search::SimilarListingsRequest};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RepliersClient::new("api_key".to_string());
    ///
    /// let request = SimilarListingsRequest {
    ///     mls_number: "N12345678".to_string(),
    ///     radius: Some(5.0),
    ///     list_price_range: Some(50000.0),
    ///     ..Default::default()
    /// };
    ///
    /// let similar = client.get_similar_listings(request).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_similar_listings(
        &self,
        request: SimilarListingsRequest,
    ) -> Result<SimilarListingsResponse, RepliersError> {
        let url = format!("{}/listings/{}/similar", self.base_url(), request.mls_number);

        let mut http_request = self.get_request(&url);

        // Build query parameters
        let mut params = Vec::new();

        if let Some(bid) = &request.board_id {
            params.push(("boardId", bid.clone()));
        }
        if let Some(r) = request.radius {
            params.push(("radius", r.to_string()));
        }
        if let Some(lpr) = request.list_price_range {
            params.push(("listPriceRange", lpr.to_string()));
        }
        if let Some(f) = &request.fields {
            params.push(("fields", f.clone()));
        }
        if let Some(s) = &request.sort_by {
            params.push(("sortBy", s.clone()));
        }

        if !params.is_empty() {
            http_request = http_request.query(&params);
        }

        let response = http_request.send().await?;
        let response = Self::check_response(response).await?;
        let similar_response = response.json::<SimilarListingsResponse>().await?;

        Ok(similar_response)
    }
}
