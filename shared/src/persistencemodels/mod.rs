use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct NearEarthObjectApiResponse {
    pub data: NearEarthObjectModel,
    pub updated_date_time: String,
}

/// Models for storing NEO data in the database
/// Used because the API response is not directly compatible with the database
/// (the API response is a HashMap, and the database is a Vec)
#[derive(Debug, Serialize, Deserialize)]
pub struct NearEarthObjectModel {
    pub links: ApiLinks,
    pub element_count: i32,
    pub updated_date_time: String,
    pub near_earth_objects: Vec<NearEarthObject>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearEarthObject {
    pub id: String,
    pub neo_reference_id: String,
    pub name: String,
    pub nasa_jpl_url: String,
    pub absolute_magnitude_h: f64,
    pub estimated_diameter: EstimatedDiameter,
    pub is_potentially_hazardous_asteroid: bool,
    pub close_approach_data: Vec<CloseApproachData>,
    pub is_sentry_object: bool,
    pub links: ApiLinks,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiLinks {
    pub next: Option<String>,
    pub prev: Option<String>,
    #[serde(alias = "self")]
    pub this: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstimatedDiameter {
    pub kilometers: EstimatedDiameterValues,
    pub meters: EstimatedDiameterValues,
    pub miles: EstimatedDiameterValues,
    pub feet: EstimatedDiameterValues,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstimatedDiameterValues {
    pub estimated_diameter_min: f64,
    pub estimated_diameter_max: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloseApproachData {
    pub close_approach_date: String,
    pub epoch_date_close_approach: i64,
    pub relative_velocity: RelativeVelocity,
    pub miss_distance: MissDistance,
    pub orbiting_body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelativeVelocity {
    pub kilometers_per_second: String,
    pub kilometers_per_hour: String,
    pub miles_per_hour: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MissDistance {
    pub astronomical: String,
    pub lunar: String,
    pub kilometers: String,
    pub miles: String,
}
