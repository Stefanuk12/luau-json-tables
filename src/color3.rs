// Dependencies
use serde::{Deserialize, Serialize, ser::SerializeStruct};
use super::SpecialWrapper;

/// A Color3 object wrapper.
#[derive(Serialize, Deserialize)]
struct Color3Data {
    r: u8,
    g: u8,
    b: u8
}

/// A Color3 object.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Color3(pub u8, pub u8, pub u8);
impl Serialize for Color3 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        let mut state = serializer.serialize_struct("Color3", 3)?;
        state.serialize_field("type", "Color3")?;
        state.serialize_field("data", &Color3Data {
            r: self.0,
            g: self.1,
            b: self.2
        })?;
        state.end()
    }
}
impl<'de> Deserialize<'de> for Color3 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let wrapper = SpecialWrapper::<Color3Data>::deserialize(deserializer)?;
        Ok(Color3(wrapper.data.r, wrapper.data.g, wrapper.data.b))
    }
}