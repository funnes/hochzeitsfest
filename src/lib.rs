use std::collections::HashMap;

use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};

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
    Single(AttendingValueList),
    Table(HashMap<String, AttendingValueList>),
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

    fn visit_seq<A>(self, seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        AttendingValueListVisitor
            .visit_seq(seq)
            .map(AttendingField::Single)
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

/// Wrapper type to provide a custom Deserialize implementation that prevents
/// duplicate AttendingValue
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AttendingValueList(Vec<AttendingValue>);

impl<'de> Deserialize<'de> for AttendingValueList {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(AttendingValueListVisitor)
    }
}

struct AttendingValueListVisitor;

impl<'de> Visitor<'de> for AttendingValueListVisitor {
    type Value = AttendingValueList;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a list of AttendingValue")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut v = Vec::new();
        while let Some(elem) = seq.next_element()? {
            if v.contains(&elem) {
                return Err(de::Error::custom("duplicate attending value"));
            }
            v.push(elem);
        }
        Ok(AttendingValueList(v))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AttendingValue {
    Afternoon,
    Dinner,
    Hike,
}
