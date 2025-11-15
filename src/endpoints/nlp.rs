//! AI-powered natural language search endpoint
//!
//! POST https://api.repliers.io/nlp

use crate::models::{NLPSearchRequest, NLPSearchResponse};
use crate::{RepliersClient, RepliersError};

impl RepliersClient {
    /// Search for listings using natural language prompts
    ///
    /// Converts conversational queries into structured API parameters.
    ///
    /// # Arguments
    ///
    /// * `prompt` - Natural language search query (e.g., "find me properties in New York")
    /// * `board_id` - Optional board ID for multi-MLS accounts
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use repliers_beta::RepliersClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = RepliersClient::new("api_key".to_string());
    ///
    /// let result = client.ai_search_listings(
    ///     "find me 3 bedroom condos in Toronto under $800k",
    ///     None,
    /// ).await?;
    ///
    /// println!("Converted to URL: {}", result.url);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn ai_search_listings(
        &self,
        prompt: &str,
        board_id: Option<&str>,
    ) -> Result<NLPSearchResponse, RepliersError> {
        let url = format!("{}/nlp", self.base_url());

        let request = NLPSearchRequest {
            prompt: prompt.to_string(),
            board_id: board_id.map(|s| s.to_string()),
        };

        let response = self
            .post_request(&url)
            .json(&request)
            .send()
            .await?;

        let response = Self::check_response(response).await?;
        let nlp_response = response.json::<NLPSearchResponse>().await?;

        Ok(nlp_response)
    }
}
