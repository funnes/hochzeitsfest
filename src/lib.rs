use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GuestList {
    pub guests: Vec<Guest>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Guest {
    pub name: String,
    pub is_invited_by_paper: bool,
    pub is_invited_to_hike: bool,
    pub attending: Option<AttendingField>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AttendingField {
    Single(Vec<AttendingValue>),
    Table(HashMap<String, AttendingField>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AttendingValue {
    Afternoon,
    Dinner,
    Hike,
}
