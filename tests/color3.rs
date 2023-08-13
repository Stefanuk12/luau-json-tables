// Dependencies
use luau_json_tables::Color3;

/// Entrypoint.
fn main() -> Result<(), serde_json::Error> {
    // Construct the initial object
    let c = Color3(255, 255, 255);

    // Attempt to serialize and deserialize the object
    let c_str = serde_json::to_string(&c)?;
    assert_eq!(serde_json::from_str::<Color3>(&c_str)?, c);
    Ok(())
}