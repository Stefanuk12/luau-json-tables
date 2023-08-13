// Dependencies
use serde::{Deserialize, Serialize, ser::SerializeStruct};
use super::SpecialWrapper;

/// A Vector2 object wrapper.
#[derive(Serialize, Deserialize)]
struct Vector2Data {
    x: f64,
    y: f64
}

/// A Vector2 object.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Vector2(pub f64, pub f64);
impl Serialize for Vector2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut state = serializer.serialize_struct("Vector2", 3)?;
        state.serialize_field("type", "Vector2")?;
        state.serialize_field("data", &Vector2Data {
            x: self.0,
            y: self.1
        })?;
        state.end()
    }
}
impl<'de> Deserialize<'de> for Vector2 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let wrapper = SpecialWrapper::<Vector2Data>::deserialize(deserializer)?;
        Ok(Vector2(wrapper.data.x, wrapper.data.y))
    }
}
impl std::fmt::Display for Vector2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.0, self.1)
    }
}