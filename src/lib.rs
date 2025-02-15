use std::collections::HashMap;

use serde::{de::Visitor, Deserialize, Serialize};

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum AttendingField {
    Single(Vec<AttendingValue>),
    Table(HashMap<String, Vec<AttendingValue>>),
}

impl<'de> Deserialize<'de> for AttendingField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(AttendingFieldVisitor)
    }
}

struct AttendingFieldVisitor;

impl<'de> Visitor<'de> for AttendingFieldVisitor {
    type Value = AttendingField;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a list of AttendingValue or a map of strings (names of sub-attendees) to list of AttendingValue")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut v = Vec::new();
        while let Some(elem) = seq.next_element()? {
            v.push(elem);
        }
        Ok(AttendingField::Single(v))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut m = HashMap::new();
        while let Some((name, attending)) = map.next_entry()? {
            m.insert(name, attending);
        }
        Ok(AttendingField::Table(m))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AttendingValue {
    Afternoon,
    Dinner,
    Hike,
}
