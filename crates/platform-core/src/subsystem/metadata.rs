use crate::subsystem::{SubsystemInstanceId, SubsystemTypeId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubsystemMetadata{
    instance_id: SubsystemInstanceId,
    type_id: SubsystemTypeId,
    name: String,
    description: String,
    implementation_version: semver::Version,
    interface_version: semver::Version,
}
impl SubsystemMetadata{
    pub fn new(instance_id: SubsystemInstanceId,
               type_id: SubsystemTypeId,
               name: String,
               description: String,
               implementation_version: semver::Version,
               interface_version: semver::Version) -> Result<Self, SubsystemMetadataError> {

        if name.trim().is_empty() {
            return Err(SubsystemMetadataError::InvalidName);
        }
        Ok(
            Self {
                instance_id,
                type_id,
                name,
                description,
                implementation_version,
                interface_version,
            }
        )
    }

    pub fn instance_id(&self) -> &SubsystemInstanceId {
        &self.instance_id
    }
    pub fn type_id(&self) -> &SubsystemTypeId {
        &self.type_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn description(&self) -> &str {
        &self.description
    }
    pub fn implementation_version(&self) -> &semver::Version {
        &self.implementation_version
    }
    pub fn interface_version(&self) -> &semver::Version {
        &self.interface_version
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubsystemMetadataError{
    InvalidName,
}
impl std::fmt::Display for SubsystemMetadataError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubsystemMetadataError::InvalidName => write!(f, "Subsystem name cannot be empty"),
        }
    }
}

impl std::error::Error for SubsystemMetadataError{}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_valid_metadata_creation(){
        let instance_id =  SubsystemInstanceId::new("valid-instance-id".to_string()).unwrap();
        let type_id = SubsystemTypeId::new("valid-type-id".to_string()).unwrap();
        let name = "Valid Subsystem".to_string();
        let description = "A valid subsystem for testing".to_string();
        let implementation_version = semver::Version::parse("1.0.0").unwrap();
        let interface_version = semver::Version::parse("1.0.0").unwrap();

        let metadata = SubsystemMetadata::new(instance_id, type_id, name, description, implementation_version, interface_version);
        assert!(metadata.is_ok());
    }

    #[test]
    fn reject_empty_name(){
        let instance_id =  SubsystemInstanceId::new("valid-instance-id".to_string()).unwrap();
        let type_id = SubsystemTypeId::new("valid-type-id".to_string()).unwrap();
        let name = "".to_string();
        let description = "A valid subsystem for testing".to_string();
        let implementation_version = semver::Version::parse("1.0.0").unwrap();
        let interface_version = semver::Version::parse("1.0.0").unwrap();

        let metadata = SubsystemMetadata::new(instance_id, type_id, name, description, implementation_version, interface_version);
        assert!(metadata.is_err());
        assert_eq!(metadata.unwrap_err(), SubsystemMetadataError::InvalidName);
    }

    #[test]
    fn reject_whitespace_name(){
        let instance_id =  SubsystemInstanceId::new("valid-instance-id".to_string()).unwrap();
        let type_id = SubsystemTypeId::new("valid-type-id".to_string()).unwrap();
        let name = "   ".to_string();
        let description = "A valid subsystem for testing".to_string();
        let implementation_version = semver::Version::parse("1.0.0").unwrap();
        let interface_version = semver::Version::parse("1.0.0").unwrap();

        let metadata = SubsystemMetadata::new(instance_id, type_id, name, description, implementation_version, interface_version);
        assert!(metadata.is_err());
        assert_eq!(metadata.unwrap_err(), SubsystemMetadataError::InvalidName);
    }
}