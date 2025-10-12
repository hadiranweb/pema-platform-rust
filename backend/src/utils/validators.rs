use validator::ValidationError;
use uuid::Uuid;

pub fn is_uuid(value: &Uuid) -> Result<(), ValidationError> {
    if value.is_nil() {
        return Err(ValidationError::new("invalid_uuid"));
    }
    Ok(())
}

