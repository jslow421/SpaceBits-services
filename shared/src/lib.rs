use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    pub links: ApiResponseLink,
    pub element_count: i32,
    pub near_earth_objects: HashMap<String, Vec<NearEarthObject>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearEarthObject {
    id: String,
    neo_reference_id: String,
    name: String,
    nasa_jpl_url: String,
    absolute_magnitude_h: f64,
    estimated_diameter: EstimatedDiameter,
    is_potentially_hazardous_asteroid: bool,
    close_approach_data: Vec<CloseApproachData>,
    is_sentry_object: bool,
    links: ApiResponseLink,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NearEarthLink {
    #[serde(alias = "self")]
    this: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponseLink {
    next: Option<String>,
    prev: Option<String>,
    #[serde(alias = "self")]
    this: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstimatedDiameter {
    kilometers: EstimatedDiameterValues,
    meters: EstimatedDiameterValues,
    miles: EstimatedDiameterValues,
    feet: EstimatedDiameterValues,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EstimatedDiameterValues {
    estimated_diameter_min: f64,
    estimated_diameter_max: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CloseApproachData {
    close_approach_date: String,
    epoch_date_close_approach: i64,
    relative_velocity: RelativeVelocity,
    miss_distance: MissDistance,
    orbiting_body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelativeVelocity {
    kilometers_per_second: String,
    kilometers_per_hour: String,
    miles_per_hour: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MissDistance {
    astronomical: String,
    lunar: String,
    kilometers: String,
    miles: String,
}
