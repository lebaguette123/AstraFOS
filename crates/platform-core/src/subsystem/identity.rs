//SubsystemInstanceId Section
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(transparent)]
pub struct SubsystemInstanceId{
    id: String,
}

fn is_valid_id(id: &str) -> bool{
    !id.is_empty()
        && id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && !id.starts_with('-')
        && !id.ends_with('-')
        && !id.contains("--")
}
impl SubsystemInstanceId{
    pub fn new(id: String) -> Result<Self, SubsystemInstanceIdError>{
        if !is_valid_id(&id){
            return Err(SubsystemInstanceIdError::InvalidFormat);
        }
        Ok(SubsystemInstanceId{id})
    }
}

impl std::fmt::Display for SubsystemInstanceId{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        f.write_str(&self.id)
    }
}

impl TryFrom<String> for SubsystemInstanceId{
    type Error = SubsystemInstanceIdError;

    fn try_from(value: String) -> Result<Self, Self::Error>{
        SubsystemInstanceId::new(value)
    }
}

impl<'de> serde::Deserialize<'de> for SubsystemInstanceId{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de>{
        let s = String::deserialize(deserializer)?;
        SubsystemInstanceId::new(s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubsystemInstanceIdError{
    InvalidFormat,
}
impl std::error::Error for SubsystemInstanceIdError{}

impl std::fmt::Display for SubsystemInstanceIdError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        match self{
            SubsystemInstanceIdError::InvalidFormat => f.write_str("Invalid subsystem ID format"),
        }
    }
}


//SubsystemTypeId Section
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(transparent)]
pub struct SubsystemTypeId{
    id: String,
}
impl SubsystemTypeId{
    pub fn new(id: String) -> Result<Self, SubsystemTypeIdError>{
        if !is_valid_id(&id){
            return Err(SubsystemTypeIdError::InvalidFormat);
        }
        Ok(SubsystemTypeId{id})
    }
}

impl std::fmt::Display for SubsystemTypeId{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        f.write_str(&self.id)
    }
}

impl TryFrom<String> for SubsystemTypeId{
    type Error = SubsystemTypeIdError;

    fn try_from(value: String) -> Result<Self, Self::Error>{
        SubsystemTypeId::new(value)
    }
}

impl <'de> serde::Deserialize<'de> for SubsystemTypeId{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de>{
        let s = String::deserialize(deserializer)?;
        SubsystemTypeId::new(s).map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubsystemTypeIdError{
    InvalidFormat,
}
impl std::error::Error for SubsystemTypeIdError{}

impl std::fmt::Display for SubsystemTypeIdError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result{
        match self{
            SubsystemTypeIdError::InvalidFormat => f.write_str("Invalid subsystem type ID format"),
        }
    }
}


//Test section
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_valid_serialization_instance(){
        let id = SubsystemInstanceId::new("valid-id-123".to_string()).unwrap();
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"valid-id-123\"");
    }

    #[test]
    fn test_valid_deserialization_instance(){
        let json = serde_json::from_str::<SubsystemInstanceId>("\"valid-id-123\"").unwrap();
        let expected = SubsystemInstanceId::new("valid-id-123".to_string()).unwrap();
        assert_eq!(json, expected);
    }

    #[test]
    fn test_invalid_deserialization_instance(){
        let result = serde_json::from_str::<SubsystemInstanceId>("\"Invalid-ID\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_serialization_type(){
        let id = SubsystemTypeId::new("valid-type-456".to_string()).unwrap();
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"valid-type-456\"");
    }

    #[test]
    fn test_valid_deserialization_type(){
        let json = serde_json::from_str::<SubsystemTypeId>("\"valid-type-456\"").unwrap();
        let expected = SubsystemTypeId::new("valid-type-456".to_string()).unwrap();
        assert_eq!(json, expected);
    }

    #[test]
    fn test_invalid_deserialization_type(){
        let result = serde_json::from_str::<SubsystemTypeId>("\"Invalid-Type\"");
        assert!(result.is_err());
    }
}



