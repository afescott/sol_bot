use serde::de::Error as SerdeError;
use serde::{Deserialize, Deserializer};
use serde_json::{Number, Value};

pub(crate) fn deserialize_str_to_number<'de, D>(deserializer: D) -> Result<Number, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Value = Deserialize::deserialize(deserializer)?;
    match v {
        Value::String(s) => s.parse::<Number>().map_err(SerdeError::custom),
        Value::Number(n) => Ok(n),
        _ => Err(SerdeError::custom("Expected a string or number")),
    }
}
