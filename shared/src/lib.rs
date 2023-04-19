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

/// Models for storing NEO data in the database
/// Used because the API response is not directly compatible with the database
/// (the API response is a HashMap, and the database is a Vec)
#[derive(Debug, Serialize, Deserialize)]
pub struct NearEarthObjectModel {
    pub links: ApiResponseLink,
    pub element_count: i32,
    pub near_earth_objects: Vec<NearEarthObject>,
}

/// Database model for NEO data
#[derive(Debug, Serialize, Deserialize)]
pub struct StoredNearEarthObjectModel {
    pub updated_date_time: String,
    pub data: ApiResponse,
}

/// Individual person in space model
#[derive(Debug, Serialize, Deserialize)]
pub struct PersonModel {
    pub name: String,
    pub craft: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PeopleInSpaceModel {
    #[serde(alias = "updatedTime")]
    pub update_time: String,
    pub people: Vec<PersonModel>,
}
