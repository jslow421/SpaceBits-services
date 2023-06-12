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

// Upcoming Launches
#[derive(Debug, Serialize, Deserialize)]
pub struct UpcomingLaunches {
    pub valid_auth: bool,
    pub count: u32,
    pub limit: u32,
    pub total: u64,
    pub last_page: u64,
    pub result: Vec<UpcomingLaunchesLaunch>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpcomingLaunchesLaunch {
    pub id: u64,
    pub cospar_id: Option<String>,
    pub sort_date: String,
    pub name: String,
    pub provider: UpcomingLaunchProvider,
    pub vehicle: UpcomingLaunchVehicle,
    pub pad: Option<UpcomingLaunchPad>,
    pub missions: Vec<UpcomingLaunchMission>,
    pub mission_description: String,
    pub launch_description: String,
    pub win_open: Option<String>,
    pub t0: Option<String>,
    pub win_close: Option<String>,
    pub date_str: String,
    pub tags: Vec<UpcomingLaunchTag>,
    pub slug: String,
    pub weather_summary: Option<String>,
    pub weather_temp: i32,
    pub weather_condition: Option<String>,
    pub weather_wind_mph: Option<i32>,
    pub weather_icon: Option<String>,
    pub weather_updated: Option<String>,
    #[serde(alias = "quick_text")]
    pub quick_text: Option<String>,
    pub suborbital: bool,
    pub modified: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpcomingLaunchProvider {
    pub id: u64,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpcomingLaunchVehicle {
    pub id: u64,
    pub name: String,
    pub company_id: u64,
    pub slug: String,
    pub pad: Option<UpcomingLaunchPad>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpcomingLaunchPad {
    pub id: u64,
    pub name: String,
    pub location: UpcomingLaunchPadLocation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpcomingLaunchPadLocation {
    pub id: u64,
    pub name: String,
    pub state: String,
    #[serde(alias = "statename")]
    pub state_name: String,
    pub country: String,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpcomingLaunchMission {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpcomingLaunchTag {
    pub id: u64,
    pub text: String,
}
