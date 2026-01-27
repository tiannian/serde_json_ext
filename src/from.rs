// Deserialization functions with configuration

use crate::Config;
use crate::de::Deserializer;
use serde::Deserialize;
use std::io::Read;

/// Deserializes a value from a JSON string with the given configuration.
///
/// # Example
///
/// ```
/// use serde_json_helper::{from_str, Config};
///
/// #[derive(serde::Deserialize)]
/// struct TestStruct {
///     data: Vec<u8>,
/// }
///
/// let config = Config::default().set_bytes_hex().enable_hex_prefix();
/// let json = r#"{"data":"0x010203"}"#;
/// let value: TestStruct = from_str(json, &config).unwrap();
/// ```
pub fn from_str<'de, T>(s: &'de str, config: &Config) -> serde_json::Result<T>
where
    T: Deserialize<'de>,
{
    let deserializer = serde_json::Deserializer::from_str(s);
    let wrapper = Deserializer::with_config(deserializer, config.clone());
    T::deserialize(wrapper)
}

/// Deserializes a value from a JSON byte slice with the given configuration.
///
/// # Example
///
/// ```
/// use serde_json_helper::{from_slice, Config};
///
/// #[derive(serde::Deserialize)]
/// struct TestStruct {
///     data: Vec<u8>,
/// }
///
/// let config = Config::default().set_bytes_hex().enable_hex_prefix();
/// let json = br#"{"data":"0x010203"}"#;
/// let value: TestStruct = from_slice(json, &config).unwrap();
/// ```
pub fn from_slice<'de, T>(v: &'de [u8], config: &Config) -> serde_json::Result<T>
where
    T: Deserialize<'de>,
{
    let deserializer = serde_json::Deserializer::from_slice(v);
    let wrapper = Deserializer::with_config(deserializer, config.clone());
    T::deserialize(wrapper)
}

/// Deserializes a value from a JSON reader with the given configuration.
///
/// # Example
///
/// ```
/// use serde_json_helper::{from_reader, Config};
/// use std::io::Cursor;
///
/// #[derive(serde::Deserialize)]
/// struct TestStruct {
///     data: Vec<u8>,
/// }
///
/// let config = Config::default().set_bytes_hex().enable_hex_prefix();
/// let json = r#"{"data":"0x010203"}"#;
/// let reader = Cursor::new(json.as_bytes());
/// let value: TestStruct = from_reader(reader, &config).unwrap();
/// ```
pub fn from_reader<R, T>(rdr: R, config: &Config) -> serde_json::Result<T>
where
    R: Read,
    T: for<'de> Deserialize<'de>,
{
    let deserializer = serde_json::Deserializer::from_reader(rdr);
    let wrapper = Deserializer::with_config(deserializer, config.clone());
    T::deserialize(wrapper)
}

/// Deserializes a value from a `serde_json::Value` with the given configuration.
///
/// # Example
///
/// ```
/// use serde_json_helper::{from_value, Config};
/// use serde_json::json;
///
/// #[derive(serde::Deserialize)]
/// struct TestStruct {
///     data: Vec<u8>,
/// }
///
/// let config = Config::default().set_bytes_hex().enable_hex_prefix();
/// let value = json!({"data": "0x010203"});
/// let result: TestStruct = from_value(value, &config).unwrap();
/// ```
pub fn from_value<T>(value: serde_json::Value, config: &Config) -> serde_json::Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    // Convert Value to string first, then deserialize with our custom deserializer
    let json_str = serde_json::to_string(&value)?;
    from_str(&json_str, config)
}
