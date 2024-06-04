use validator::Validate;

use crate::error::RouterError;

/// Validate the value and return actix error
pub fn validate<T>(data: &T) -> Result<(), RouterError>
where
    T: Validate,
{
    let validation = data.validate();

    match validation {
        Err(_error_detail) => {
            // TODO: This error in response is so ugly.
            Err(RouterError::from_predefined("VALIDATION_ERROR"))
        }

        Ok(()) => Ok(()),
    }
}
