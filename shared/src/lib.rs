use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(feature = "apimodels")]
pub mod apimodels;

#[cfg(feature = "persistencemodels")]
pub mod persistencemodels;

// /// Individual person in space model
// #[derive(Debug, Serialize, Deserialize)]
// pub struct PersonModel {
//     pub name: String,
//     pub craft: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct PeopleInSpaceModel {
//     #[serde(alias = "updatedTime")]
//     pub update_time: String,
//     pub people: Vec<PersonModel>,
// }
