// Dependencies
use luau_json_tables::Vector3;

/// Entrypoint.
fn main() -> Result<(), serde_json::Error> {
    // Construct the initial object
    let c = Vector3(69.0, 420.0, 1337.0);

    // Attempt to serialize and deserialize the object
    let c_str = serde_json::to_string(&c)?;
    assert_eq!(serde_json::from_str::<Vector3>(&c_str)?, c);
    Ok(())
}