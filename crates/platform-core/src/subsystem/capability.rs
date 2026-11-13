#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityId{
    namespace: String,
    name: String,
}
fn is_valid(s: &str) -> bool{
    !s.is_empty()
        && s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        && !s.starts_with('-')
        && !s.ends_with('-') && !s.contains("--")
}
impl CapabilityId{
    pub fn new(namespace: String, name: String) -> Result<Self, CapabilityIdError>{
        if !is_valid(&namespace){
            return Err(CapabilityIdError::InvalidNamespace);
        }
        if !is_valid(&name){
            return Err(CapabilityIdError::InvalidName);
        }
        Ok(Self { namespace, name })
    }
}
impl std::fmt::Display for CapabilityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.namespace, self.name)
    }
}
impl serde::Serialize for CapabilityId{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer
    {
        serializer.collect_str(self)
    }
}
impl <'de> serde::Deserialize <'de> for CapabilityId{
    fn deserialize< D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de>{
        let s = String::deserialize(deserializer)?;
        let parts = s.split_once('.');
        match parts{
            Some((namespace, name)) => CapabilityId::new(namespace.to_string(), name.to_string())
                .map_err(serde::de::Error::custom),
            None => Err(serde::de::Error::custom("Invalid capability ID format")),
        }

    }
}

impl TryFrom<String> for CapabilityId{
    type Error = CapabilityIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts = value.split_once('.');
        match parts{
            Some((namespace, name)) => CapabilityId::new(namespace.to_string(), name.to_string()),
            None => Err(CapabilityIdError::InvalidFormat),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityIdError{
    InvalidNamespace,
    InvalidName,
    InvalidFormat,
}
impl std::fmt::Display for CapabilityIdError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CapabilityIdError::InvalidNamespace => write!(f, "Invalid capability namespace"),
            CapabilityIdError::InvalidName => write!(f, "Invalid capability name"),
            CapabilityIdError::InvalidFormat => write!(f, "Invalid capability ID format"),
        }
    }
}
impl std::error::Error for CapabilityIdError {}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_valid_serialization(){
        let id = CapabilityId::new("platform".to_string(), "read".to_string()).unwrap();
        let json = serde_json::to_string(&id).unwrap();

        assert_eq!(json, "\"platform.read\"");
    }

    #[test]
    fn test_invalid_serialization_input(){
        let result = CapabilityId::new("Platform".to_string(), "read".to_string());

        assert_eq!(result, Err(CapabilityIdError::InvalidNamespace));
    }

    #[test]
    fn test_valid_deserialization(){
        let id = serde_json::from_str::<CapabilityId>("\"platform.read\"").unwrap();
        let expected = CapabilityId::new("platform".to_string(), "read".to_string()).unwrap();

        assert_eq!(id, expected);
    }

    #[test]
    fn test_invalid_deserialization(){
        let result = serde_json::from_str::<CapabilityId>("\"Platform.read\"");

        assert!(result.is_err());
    }

}