//! Search request and response models

use serde::{Deserialize, Serialize};

/// Request parameters for listing search
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ListingSearchRequest {
    /// City name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Listing status filter (e.g., ["Active", "Sold"])
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Vec<String>>,

    /// Minimum listing price
    #[serde(skip_serializing_if = "Option::is_none", rename = "minPrice")]
    pub min_price: Option<f64>,

    /// Maximum listing price
    #[serde(skip_serializing_if = "Option::is_none", rename = "maxPrice")]
    pub max_price: Option<f64>,

    /// Number of bedrooms
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bedrooms: Option<u32>,

    /// Property types (e.g., ["Condo", "Detached"])
    #[serde(skip_serializing_if = "Option::is_none", rename = "propertyType")]
    pub property_type: Option<Vec<String>>,

    /// Page number for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,

    /// Number of results per page
    #[serde(skip_serializing_if = "Option::is_none", rename = "resultsPerPage")]
    pub results_per_page: Option<u32>,

    // Note: The Repliers API supports many additional search parameters that can be added:
    // - bathrooms, min_bathrooms, max_bathrooms: Bathroom count filters
    // - min_bedrooms, max_bedrooms: More granular bedroom filtering
    // - min_sqft, max_sqft: Square footage range
    // - area: Geographic area filter
    // - neighborhood: Neighborhood-specific search
    // - property_sub_type: More specific property categorization
    // - listing_date, days_on_market: Time-based filters
    // - features: Specific property features (pool, garage, etc.)
    //
    // These can be added as needed based on use case requirements.
    // Refer to https://docs.repliers.io/reference/search-listings for complete list.
}

/// Response from listing search
#[derive(Debug, Clone, Deserialize)]
pub struct ListingSearchResponse {
    /// Array of listing results
    ///
    /// Currently uses `serde_json::Value` for flexibility, as listing structures
    /// can vary based on MLS board and available data. A fully typed `Listing`
    /// struct could be implemented for stricter type safety, but would need to
    /// handle optional fields for varying data availability.
    pub listings: Vec<serde_json::Value>,

    /// Current page number
    pub page: u32,

    /// Total number of pages
    #[serde(rename = "numPages")]
    pub num_pages: u32,

    /// Number of results per page
    #[serde(rename = "pageSize")]
    pub page_size: u32,

    /// Total count of results
    pub count: u32,
}

/// Request parameters for similar listings search
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct SimilarListingsRequest {
    /// MLS number of the reference listing
    pub mls_number: String,

    /// Board ID for multi-MLS accounts
    #[serde(skip_serializing_if = "Option::is_none", rename = "boardId")]
    pub board_id: Option<String>,

    /// Search radius in kilometers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,

    /// List price range variance (e.g., ±$50,000)
    #[serde(skip_serializing_if = "Option::is_none", rename = "listPriceRange")]
    pub list_price_range: Option<f64>,

    /// Fields to return in response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,

    /// Sort order
    #[serde(skip_serializing_if = "Option::is_none", rename = "sortBy")]
    pub sort_by: Option<String>,
}

/// Response from similar listings search
#[derive(Debug, Clone, Deserialize)]
pub struct SimilarListingsResponse {
    /// Similar listings found
    pub similar: Vec<serde_json::Value>,

    /// Current page
    pub page: u32,

    /// Total number of pages
    #[serde(rename = "numPages")]
    pub num_pages: u32,

    /// Number of results per page
    #[serde(rename = "pageSize")]
    pub page_size: u32,

    /// Total count of similar listings
    pub count: u32,
}

impl ListingSearchRequest {
    /// Creates a new builder for constructing a search request
    pub fn builder() -> ListingSearchRequestBuilder {
        ListingSearchRequestBuilder::default()
    }
}

/// Builder for constructing a ListingSearchRequest
#[derive(Debug, Default)]
pub struct ListingSearchRequestBuilder {
    city: Option<String>,
    status: Option<Vec<String>>,
    min_price: Option<f64>,
    max_price: Option<f64>,
    bedrooms: Option<u32>,
    property_type: Option<Vec<String>>,
    page: Option<u32>,
    results_per_page: Option<u32>,
}

impl ListingSearchRequestBuilder {
    /// Sets the city filter
    pub fn city(mut self, city: impl Into<String>) -> Self {
        self.city = Some(city.into());
        self
    }

    /// Sets the status filter (e.g., ["Active", "Sold"])
    pub fn status(mut self, status: Vec<String>) -> Self {
        self.status = Some(status);
        self
    }

    /// Adds a single status to the filter
    pub fn add_status(mut self, status: impl Into<String>) -> Self {
        self.status.get_or_insert_with(Vec::new).push(status.into());
        self
    }

    /// Sets the minimum price filter
    pub fn min_price(mut self, min_price: f64) -> Self {
        self.min_price = Some(min_price);
        self
    }

    /// Sets the maximum price filter
    pub fn max_price(mut self, max_price: f64) -> Self {
        self.max_price = Some(max_price);
        self
    }

    /// Sets a price range filter
    pub fn price_range(mut self, min: f64, max: f64) -> Self {
        self.min_price = Some(min);
        self.max_price = Some(max);
        self
    }

    /// Sets the number of bedrooms filter
    pub fn bedrooms(mut self, bedrooms: u32) -> Self {
        self.bedrooms = Some(bedrooms);
        self
    }

    /// Sets the property type filter (e.g., ["Condo", "Detached"])
    pub fn property_type(mut self, property_type: Vec<String>) -> Self {
        self.property_type = Some(property_type);
        self
    }

    /// Adds a single property type to the filter
    pub fn add_property_type(mut self, property_type: impl Into<String>) -> Self {
        self.property_type
            .get_or_insert_with(Vec::new)
            .push(property_type.into());
        self
    }

    /// Sets the page number for pagination
    pub fn page(mut self, page: u32) -> Self {
        self.page = Some(page);
        self
    }

    /// Sets the number of results per page
    pub fn results_per_page(mut self, results_per_page: u32) -> Self {
        self.results_per_page = Some(results_per_page);
        self
    }

    /// Builds the ListingSearchRequest
    pub fn build(self) -> ListingSearchRequest {
        ListingSearchRequest {
            city: self.city,
            status: self.status,
            min_price: self.min_price,
            max_price: self.max_price,
            bedrooms: self.bedrooms,
            property_type: self.property_type,
            page: self.page,
            results_per_page: self.results_per_page,
        }
    }
}
