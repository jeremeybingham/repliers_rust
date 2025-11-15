//! Listing data models

use serde::{Deserialize, Serialize};

/// Status of a listing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ListingStatus {
    Active,
    Sold,
    Leased,
    Expired,
    Cancelled,
    Suspended,
}

/// A property listing from the MLS
///
/// Note: This is a minimal struct containing only the MLS number. In practice, the Repliers API
/// returns comprehensive listing data that varies by MLS board. Additional fields can include:
/// - address: Full property address structure
/// - city, state, postal_code: Location details
/// - list_price, sold_price: Pricing information
/// - bedrooms, bathrooms: Property details
/// - square_footage: Living area size
/// - property_type, property_sub_type: Type categorization
/// - status: Active, Sold, Expired, etc.
/// - list_date, sold_date: Important dates
/// - description, remarks: Property descriptions
/// - images: Photo URLs and metadata
/// - agent, office: Listing agent information
/// - features: Property features (pool, garage, etc.)
///
/// For flexibility, most endpoints return `serde_json::Value` to handle varying field availability.
/// A fully typed struct would need extensive `Option<T>` fields to handle all MLS board variations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    /// MLS number (unique identifier)
    #[serde(rename = "mlsNumber")]
    pub mls_number: String,
}
