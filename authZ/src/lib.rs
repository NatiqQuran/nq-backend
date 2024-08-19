pub mod middleware;

use std::net::SocketAddr;

use actix_web::{
    http::{header::HeaderMap, Uri},
    ResponseError,
};
use async_trait::async_trait;

#[async_trait]
/// Model Permission
/// Returns the attr of the Model
pub trait ModelPermission<T, A>: Sync + Send
where
    T: Sized,
{
    /// Returns the value of the attr
    ///
    /// if its exists
    async fn get_attr(&self, name: T) -> Option<A>
    where
        A: Sized;
}

#[async_trait]
pub trait CheckPermission {
    /// Check if the permissions are valid
    ///
    /// This args will passed from the Middleware
    ///
    /// The Err will be sent as response when permission denied
    async fn check(
        &self,
        req_addr: SocketAddr,
        headers: HeaderMap,
        uri: Uri,
        account_id: Option<u32>,
        path: ParsedPath,
        method: String,
    ) -> Result<(), Box<dyn ResponseError>>;
}

#[async_trait]
pub trait GetModel<T, A>
where
    T: Sized,
    A: Sized,
{
    /// Get model
    ///
    /// Name defines wich model we should check
    /// for attrs
    ///
    /// TODO: maybe Resource Id is not always Int(u32) ?
    async fn get_model(
        &self,
        resource_name: &str,
        resource_id: u32,
    ) -> Box<dyn ModelPermission<T, A>>;
}

#[derive(Default, Debug, Clone)]
/// This is the Natiq Way of reading URLs
///
/// Overall format: `/{controller}/{action}/{id}`
///
/// Each of these can be Optional and None, For example this is a valid,
/// Url: `/{controller}/{id}` Example: `/product/1`
///
/// Note that there is no action, ***Every part of natiq standard Url can be Optional***
pub struct ParsedPath {
    /// Path Controller
    ///
    /// example: `/organization`
    /// format: `/{controller}`
    pub controller: Option<String>,

    /// Path Action
    ///
    /// example: `/organization/add`
    /// format: `/{_}/{action}/{_}`
    pub action: Option<String>,

    /// Path Id
    ///
    /// example: `/organization/10`
    /// example: `/organization/edit/10`
    /// format: `/{_}/{_}/{id}`
    pub id: Option<String>,
}

impl<'a> From<&'a str> for ParsedPath {
    /// value must start with '/'
    ///
    /// `/controller/action/id`
    ///
    /// `/controller/id`
    ///
    /// `/controller`
    fn from(value: &'a str) -> Self {
        let mut splited = value
            .split('/')
            .skip_while(|s| s.is_empty())
            .map(|c| c.to_string());

        if splited.clone().count() >= 3 {
            Self {
                controller: splited.next(),
                action: splited.next(),
                id: splited.next(),
            }
        } else {
            Self {
                controller: splited.next(),
                action: None,
                id: splited.next(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsed_path_from_str() {
        let parsed_path = ParsedPath::from("/controller/action/id");

        assert_eq!(parsed_path.controller, Some("controller".to_string()));
        assert_eq!(parsed_path.action, Some("action".to_string()));
        assert_eq!(parsed_path.id, Some("id".to_string()));
    }

    #[test]
    fn test_parsed_path_from_str_with_no_action() {
        let parsed_path = ParsedPath::from("/controller/id");

        assert_eq!(parsed_path.controller, Some("controller".to_string()));
        assert_eq!(parsed_path.action, None);
        assert_eq!(parsed_path.id, Some("id".to_string()));
    }

    #[test]
    fn test_parsed_path_from_str_with_no_id_and_action() {
        let parsed_path = ParsedPath::from("/controller");

        assert_eq!(parsed_path.controller, Some("controller".to_string()));
        assert_eq!(parsed_path.action, None);
        assert_eq!(parsed_path.id, None);
    }

    #[test]
    fn test_empty_url() {
        let parsed_path = ParsedPath::from("/");

        assert_eq!(parsed_path.controller, None);
        assert_eq!(parsed_path.action, None);
        assert_eq!(parsed_path.id, None);
    }
}
