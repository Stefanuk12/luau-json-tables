// Dependencies
use serde::{Deserialize, Serialize, ser::SerializeStruct};
use super::SpecialWrapper;

/// A Vector3 object wrapper.
#[derive(Serialize, Deserialize)]
struct Vector3Data {
    x: f64,
    y: f64,
    z: f64
}

/// A Vector3 object.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Vector3(pub f64, pub f64, pub f64);
impl Serialize for Vector3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut state = serializer.serialize_struct("Vector3", 3)?;
        state.serialize_field("type", "Vector3")?;
        state.serialize_field("data", &Vector3Data {
            x: self.0,
            y: self.1,
            z: self.2
        })?;
        state.end()
    }
}
impl<'de> Deserialize<'de> for Vector3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let wrapper = SpecialWrapper::<Vector3Data>::deserialize(deserializer)?;
        Ok(Vector3(wrapper.data.x, wrapper.data.y, wrapper.data.z))
    }
}
impl std::fmt::Display for Vector3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}, {}]", self.0, self.1, self.2)
    }
}