//! Address history endpoint
//!
//! GET https://api.repliers.io/listings/history

use crate::models::{AddressHistoryQuery, AddressHistoryResponse};
use crate::{RepliersClient, RepliersError};

impl RepliersClient {
    /// Get complete MLS listing history for a specific address
    ///
    /// # Arguments
    ///
    /// * `query` - Address query parameters including street number, name, and location
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use repliers_beta::{RepliersClient, AddressHistoryQuery};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RepliersClient::new("api_key".to_string());
    ///
    /// let query = AddressHistoryQuery {
    ///     street_number: "2031".to_string(),
    ///     street_name: "N. Mt. Juliet Road".to_string(),
    ///     city: Some("Mt Juliet".to_string()),
    ///     state: Some("TN".to_string()),
    ///     zip: None,
    ///     board_id: None,
    /// };
    ///
    /// let history = client.get_address_history(query).await?;
    ///
    /// for entry in history.history {
    ///     println!("Listed: {:?} at ${:?}", entry.list_date, entry.list_price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_address_history(
        &self,
        query: AddressHistoryQuery,
    ) -> Result<AddressHistoryResponse, RepliersError> {
        let url = format!("{}/listings/history", self.base_url());

        let response = self
            .get_request(&url)
            .query(&query)
            .send()
            .await?;

        let response = Self::check_response(response).await?;
        let history_response = response.json::<AddressHistoryResponse>().await?;

        Ok(history_response)
    }
}
