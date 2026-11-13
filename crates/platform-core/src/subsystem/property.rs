#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyId{
    namespace: String,
    name: String,
}
fn is_valid(s: &str) -> bool{
    !s.is_empty()
        && s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        && !s.starts_with('-')
        && !s.ends_with('-') && !s.contains("--")
}
impl PropertyId{
    pub fn new(namespace: String, name: String) -> Result<Self, PropertyIdError>{
        if !is_valid(&namespace){
            return Err(PropertyIdError::InvalidNamespace);
        }
        if !is_valid(&name){
            return Err(PropertyIdError::InvalidName);
        }
        Ok(Self { namespace, name })
    }
}
impl std::fmt::Display for PropertyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.namespace, self.name)
    }
}
impl serde::Serialize for PropertyId{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer
    {
        serializer.collect_str(self)
    }
}
impl <'de> serde::Deserialize <'de> for PropertyId{
    fn deserialize< D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de>{
        let s = String::deserialize(deserializer)?;
        let parts = s.split_once('.');
        match parts{
            Some((namespace, name)) => PropertyId::new(namespace.to_string(), name.to_string())
                .map_err(serde::de::Error::custom),
            None => Err(serde::de::Error::custom("Invalid property ID format")),
        }

    }
}

impl TryFrom<String> for PropertyId{
    type Error = PropertyIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts = value.split_once('.');
        match parts{
            Some((namespace, name)) => PropertyId::new(namespace.to_string(), name.to_string()),
            None => Err(PropertyIdError::InvalidFormat),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyIdError{
    InvalidNamespace,
    InvalidName,
    InvalidFormat,
}
impl std::fmt::Display for PropertyIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PropertyIdError::InvalidNamespace => write!(f, "Invalid property namespace"),
            PropertyIdError::InvalidName => write!(f, "Invalid property name"),
            PropertyIdError::InvalidFormat => write!(f, "Invalid property ID format"),
        }
    }
}
impl std::error::Error for PropertyIdError {}




pub enum PropertyValue{
    Number(f64),
    Integer(i64),
    Boolean(bool),
    String(String),
    Range{min: f64, max: f64},
    Enumeration(String)
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_valid_serialization(){
        let id = PropertyId::new("platform".to_string(), "read".to_string()).unwrap();
        let json = serde_json::to_string(&id).unwrap();

        assert_eq!(json, "\"platform.read\"");
    }

    #[test]
    fn test_invalid_serialization_input(){
        let result = PropertyId::new("Platform".to_string(), "read".to_string());

        assert_eq!(result, Err(PropertyIdError::InvalidNamespace));
    }

    #[test]
    fn test_valid_deserialization(){
        let id = serde_json::from_str::<PropertyId>("\"platform.read\"").unwrap();
        let expected = PropertyId::new("platform".to_string(), "read".to_string()).unwrap();

        assert_eq!(id, expected);
    }

    #[test]
    fn test_invalid_deserialization(){
        let result = serde_json::from_str::<PropertyId>("\"Platform.read\"");

        assert!(result.is_err());
    }

}