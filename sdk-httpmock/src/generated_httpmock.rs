// The contents of this file are generated; do not modify them.

pub mod operations {
    //! [`When`](httpmock::When) and [`Then`](httpmock::Then)
    //! wrappers for each operation. Each can be converted to
    //! its inner type with a call to `into_inner()`. This can
    //! be used to explicitly deviate from permitted values.
    use oxide_api::*;
    pub struct DeviceAuthRequestWhen(httpmock::When);
    impl DeviceAuthRequestWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/device/auth$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::DeviceAuthRequest) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct DeviceAuthRequestThen(httpmock::Then);
    impl DeviceAuthRequestThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn default_response(self, status: u16, value: serde_json::Value) -> Self {
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body(value),
            )
        }
    }

    pub struct DeviceAuthConfirmWhen(httpmock::When);
    impl DeviceAuthConfirmWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/device/confirm$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::DeviceAuthVerify) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct DeviceAuthConfirmThen(httpmock::Then);
    impl DeviceAuthConfirmThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct DeviceAccessTokenWhen(httpmock::When);
    impl DeviceAccessTokenWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/device/token$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::DeviceAccessTokenRequest) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct DeviceAccessTokenThen(httpmock::Then);
    impl DeviceAccessTokenThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn default_response(self, status: u16, value: serde_json::Value) -> Self {
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body(value),
            )
        }
    }

    pub struct LoginSpoofWhen(httpmock::When);
    impl LoginSpoofWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/login$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::SpoofLoginBody) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct LoginSpoofThen(httpmock::Then);
    impl LoginSpoofThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct LoginSamlBeginWhen(httpmock::When);
    impl LoginSamlBeginWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/login/[^/]*/saml/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn silo_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/login/{}/saml/.*$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn provider_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/login/.*/saml/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct LoginSamlBeginThen(httpmock::Then);
    impl LoginSamlBeginThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn found(self) -> Self {
            Self(self.0.status(302u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn success(self, status: u16, value: serde_json::Value) -> Self {
            assert_eq!(status / 100u16, 2u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body(value),
            )
        }
    }

    pub struct LoginSamlWhen(httpmock::When);
    impl LoginSamlWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/login/[^/]*/saml/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn silo_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/login/{}/saml/.*$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn provider_name(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!("^/login/.*/saml/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: serde_json::Value) -> Self {
            Self(self.0.json_body(value))
        }
    }

    pub struct LoginSamlThen(httpmock::Then);
    impl LoginSamlThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn see_other(self) -> Self {
            Self(self.0.status(303u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn success(self, status: u16, value: serde_json::Value) -> Self {
            assert_eq!(status / 100u16, 2u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body(value),
            )
        }
    }

    pub struct CertificateListWhen(httpmock::When);
    impl CertificateListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/certificates$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct CertificateListThen(httpmock::Then);
    impl CertificateListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::CertificateResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct CertificateCreateWhen(httpmock::When);
    impl CertificateCreateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/certificates$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::CertificateCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct CertificateCreateThen(httpmock::Then);
    impl CertificateCreateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::Certificate) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct CertificateViewWhen(httpmock::When);
    impl CertificateViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/certificates/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn certificate(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/certificates/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct CertificateViewThen(httpmock::Then);
    impl CertificateViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Certificate) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct CertificateDeleteWhen(httpmock::When);
    impl CertificateDeleteWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/certificates/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn certificate(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/certificates/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct CertificateDeleteThen(httpmock::Then);
    impl CertificateDeleteThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct DiskListWhen(httpmock::When);
    impl DiskListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/disks$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct DiskListThen(httpmock::Then);
    impl DiskListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::DiskResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct DiskCreateWhen(httpmock::When);
    impl DiskCreateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/disks$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("project", value.to_string()))
        }

        pub fn body(self, value: &types::DiskCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct DiskCreateThen(httpmock::Then);
    impl DiskCreateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::Disk) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct DiskViewWhen(httpmock::When);
    impl DiskViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/disks/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn disk(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/disks/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }
    }

    pub struct DiskViewThen(httpmock::Then);
    impl DiskViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Disk) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct DiskDeleteWhen(httpmock::When);
    impl DiskDeleteWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/disks/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn disk(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/disks/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }
    }

    pub struct DiskDeleteThen(httpmock::Then);
    impl DiskDeleteThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct DiskBulkWriteImportWhen(httpmock::When);
    impl DiskBulkWriteImportWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/disks/[^/]*/bulk-write$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn disk(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/disks/{}/bulk-write$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn body(self, value: &types::ImportBlocksBulkWrite) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct DiskBulkWriteImportThen(httpmock::Then);
    impl DiskBulkWriteImportThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct DiskBulkWriteImportStartWhen(httpmock::When);
    impl DiskBulkWriteImportStartWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/disks/[^/]*/bulk-write-start$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn disk(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/disks/{}/bulk-write-start$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }
    }

    pub struct DiskBulkWriteImportStartThen(httpmock::Then);
    impl DiskBulkWriteImportStartThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct DiskBulkWriteImportStopWhen(httpmock::When);
    impl DiskBulkWriteImportStopWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/disks/[^/]*/bulk-write-stop$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn disk(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/disks/{}/bulk-write-stop$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }
    }

    pub struct DiskBulkWriteImportStopThen(httpmock::Then);
    impl DiskBulkWriteImportStopThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct DiskFinalizeImportWhen(httpmock::When);
    impl DiskFinalizeImportWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/disks/[^/]*/finalize$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn disk(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/disks/{}/finalize$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn body(self, value: &types::FinalizeDisk) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct DiskFinalizeImportThen(httpmock::Then);
    impl DiskFinalizeImportThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct DiskImportBlocksFromUrlWhen(httpmock::When);
    impl DiskImportBlocksFromUrlWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/disks/[^/]*/import$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn disk(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/disks/{}/import$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn body(self, value: &types::ImportBlocksFromUrl) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct DiskImportBlocksFromUrlThen(httpmock::Then);
    impl DiskImportBlocksFromUrlThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct DiskMetricsListWhen(httpmock::When);
    impl DiskMetricsListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/disks/[^/]*/metrics/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn disk(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/disks/{}/metrics/.*$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn metric(self, value: types::DiskMetricName) -> Self {
            let re = regex::Regex::new(&format!("^/v1/disks/.*/metrics/{}$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn end_time<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a chrono::DateTime<chrono::offset::Utc>>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("end_time", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "end_time"))
                        .is_none()
                }))
            }
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn start_time<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a chrono::DateTime<chrono::offset::Utc>>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("start_time", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "start_time"))
                        .is_none()
                }))
            }
        }
    }

    pub struct DiskMetricsListThen(httpmock::Then);
    impl DiskMetricsListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::MeasurementResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct GroupListWhen(httpmock::When);
    impl GroupListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/groups$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct GroupListThen(httpmock::Then);
    impl GroupListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::GroupResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct GroupViewWhen(httpmock::When);
    impl GroupViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/groups/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn group_id(self, value: &uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/v1/groups/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct GroupViewThen(httpmock::Then);
    impl GroupViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Group) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ImageListWhen(httpmock::When);
    impl ImageListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/images$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn include_silo_images<T>(self, value: T) -> Self
        where
            T: Into<Option<bool>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("include_silo_images", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "include_silo_images"))
                        .is_none()
                }))
            }
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct ImageListThen(httpmock::Then);
    impl ImageListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::ImageResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ImageCreateWhen(httpmock::When);
    impl ImageCreateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/images$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn body(self, value: &types::ImageCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct ImageCreateThen(httpmock::Then);
    impl ImageCreateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::Image) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ImageViewWhen(httpmock::When);
    impl ImageViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/images/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn image(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/images/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }
    }

    pub struct ImageViewThen(httpmock::Then);
    impl ImageViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Image) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ImageDeleteWhen(httpmock::When);
    impl ImageDeleteWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/images/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn image(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/images/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }
    }

    pub struct ImageDeleteThen(httpmock::Then);
    impl ImageDeleteThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ImageDemoteWhen(httpmock::When);
    impl ImageDemoteWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/images/[^/]*/demote$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn image(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/images/{}/demote$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("project", value.to_string()))
        }
    }

    pub struct ImageDemoteThen(httpmock::Then);
    impl ImageDemoteThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn accepted(self, value: &types::Image) -> Self {
            Self(
                self.0
                    .status(202u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ImagePromoteWhen(httpmock::When);
    impl ImagePromoteWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/images/[^/]*/promote$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn image(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/images/{}/promote$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }
    }

    pub struct ImagePromoteThen(httpmock::Then);
    impl ImagePromoteThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn accepted(self, value: &types::Image) -> Self {
            Self(
                self.0
                    .status(202u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceListWhen(httpmock::When);
    impl InstanceListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/instances$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct InstanceListThen(httpmock::Then);
    impl InstanceListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::InstanceResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceCreateWhen(httpmock::When);
    impl InstanceCreateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/instances$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("project", value.to_string()))
        }

        pub fn body(self, value: &types::InstanceCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceCreateThen(httpmock::Then);
    impl InstanceCreateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::Instance) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceViewWhen(httpmock::When);
    impl InstanceViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/instances/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }
    }

    pub struct InstanceViewThen(httpmock::Then);
    impl InstanceViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Instance) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceDeleteWhen(httpmock::When);
    impl InstanceDeleteWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/instances/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }
    }

    pub struct InstanceDeleteThen(httpmock::Then);
    impl InstanceDeleteThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceDiskListWhen(httpmock::When);
    impl InstanceDiskListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*/disks$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/instances/{}/disks$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct InstanceDiskListThen(httpmock::Then);
    impl InstanceDiskListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::DiskResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceDiskAttachWhen(httpmock::When);
    impl InstanceDiskAttachWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*/disks/attach$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/instances/{}/disks/attach$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn body(self, value: &types::DiskPath) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceDiskAttachThen(httpmock::Then);
    impl InstanceDiskAttachThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn accepted(self, value: &types::Disk) -> Self {
            Self(
                self.0
                    .status(202u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceDiskDetachWhen(httpmock::When);
    impl InstanceDiskDetachWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*/disks/detach$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/instances/{}/disks/detach$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn body(self, value: &types::DiskPath) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceDiskDetachThen(httpmock::Then);
    impl InstanceDiskDetachThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn accepted(self, value: &types::Disk) -> Self {
            Self(
                self.0
                    .status(202u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceExternalIpListWhen(httpmock::When);
    impl InstanceExternalIpListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*/external-ips$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/instances/{}/external-ips$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }
    }

    pub struct InstanceExternalIpListThen(httpmock::Then);
    impl InstanceExternalIpListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::ExternalIpResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceMigrateWhen(httpmock::When);
    impl InstanceMigrateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*/migrate$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/instances/{}/migrate$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn body(self, value: &types::InstanceMigrate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceMigrateThen(httpmock::Then);
    impl InstanceMigrateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Instance) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceRebootWhen(httpmock::When);
    impl InstanceRebootWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*/reboot$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/instances/{}/reboot$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }
    }

    pub struct InstanceRebootThen(httpmock::Then);
    impl InstanceRebootThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn accepted(self, value: &types::Instance) -> Self {
            Self(
                self.0
                    .status(202u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceSerialConsoleWhen(httpmock::When);
    impl InstanceSerialConsoleWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner.method(httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/v1/instances/[^/]*/serial-console$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/instances/{}/serial-console$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn from_start<T>(self, value: T) -> Self
        where
            T: Into<Option<u64>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("from_start", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "from_start"))
                        .is_none()
                }))
            }
        }

        pub fn max_bytes<T>(self, value: T) -> Self
        where
            T: Into<Option<u64>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("max_bytes", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "max_bytes"))
                        .is_none()
                }))
            }
        }

        pub fn most_recent<T>(self, value: T) -> Self
        where
            T: Into<Option<u64>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("most_recent", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "most_recent"))
                        .is_none()
                }))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }
    }

    pub struct InstanceSerialConsoleThen(httpmock::Then);
    impl InstanceSerialConsoleThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::InstanceSerialConsoleData) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceSerialConsoleStreamWhen(httpmock::When);
    impl InstanceSerialConsoleStreamWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(inner.method(httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/instances/[^/]*/serial-console/stream$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/instances/{}/serial-console/stream$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn most_recent<T>(self, value: T) -> Self
        where
            T: Into<Option<u64>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("most_recent", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "most_recent"))
                        .is_none()
                }))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }
    }

    pub struct InstanceSerialConsoleStreamThen(httpmock::Then);
    impl InstanceSerialConsoleStreamThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn default_response(self, status: u16) -> Self {
            Self(self.0.status(status))
        }

        pub fn switching_protocols(self) -> Self {
            Self(self.0.status(101u16))
        }
    }

    pub struct InstanceStartWhen(httpmock::When);
    impl InstanceStartWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*/start$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/instances/{}/start$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }
    }

    pub struct InstanceStartThen(httpmock::Then);
    impl InstanceStartThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn accepted(self, value: &types::Instance) -> Self {
            Self(
                self.0
                    .status(202u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceStopWhen(httpmock::When);
    impl InstanceStopWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*/stop$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/instances/{}/stop$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }
    }

    pub struct InstanceStopThen(httpmock::Then);
    impl InstanceStopThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn accepted(self, value: &types::Instance) -> Self {
            Self(
                self.0
                    .status(202u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct LoginLocalWhen(httpmock::When);
    impl LoginLocalWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/login/[^/]*/local$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn silo_name(self, value: &types::Name) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/login/{}/local$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::UsernamePasswordCredentials) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct LoginLocalThen(httpmock::Then);
    impl LoginLocalThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct LogoutWhen(httpmock::When);
    impl LogoutWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/logout$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }
    }

    pub struct LogoutThen(httpmock::Then);
    impl LogoutThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct CurrentUserViewWhen(httpmock::When);
    impl CurrentUserViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/me$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }
    }

    pub struct CurrentUserViewThen(httpmock::Then);
    impl CurrentUserViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::CurrentUser) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct CurrentUserGroupsWhen(httpmock::When);
    impl CurrentUserGroupsWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/me/groups$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct CurrentUserGroupsThen(httpmock::Then);
    impl CurrentUserGroupsThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::GroupResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct CurrentUserSshKeyListWhen(httpmock::When);
    impl CurrentUserSshKeyListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/me/ssh-keys$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct CurrentUserSshKeyListThen(httpmock::Then);
    impl CurrentUserSshKeyListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SshKeyResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct CurrentUserSshKeyCreateWhen(httpmock::When);
    impl CurrentUserSshKeyCreateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/me/ssh-keys$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::SshKeyCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct CurrentUserSshKeyCreateThen(httpmock::Then);
    impl CurrentUserSshKeyCreateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::SshKey) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct CurrentUserSshKeyViewWhen(httpmock::When);
    impl CurrentUserSshKeyViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/me/ssh-keys/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn ssh_key(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/me/ssh-keys/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct CurrentUserSshKeyViewThen(httpmock::Then);
    impl CurrentUserSshKeyViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SshKey) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct CurrentUserSshKeyDeleteWhen(httpmock::When);
    impl CurrentUserSshKeyDeleteWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/me/ssh-keys/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn ssh_key(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/me/ssh-keys/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct CurrentUserSshKeyDeleteThen(httpmock::Then);
    impl CurrentUserSshKeyDeleteThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceNetworkInterfaceListWhen(httpmock::When);
    impl InstanceNetworkInterfaceListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/network-interfaces$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn instance<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("instance", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "instance"))
                        .is_none()
                }))
            }
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct InstanceNetworkInterfaceListThen(httpmock::Then);
    impl InstanceNetworkInterfaceListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::InstanceNetworkInterfaceResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceNetworkInterfaceCreateWhen(httpmock::When);
    impl InstanceNetworkInterfaceCreateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/network-interfaces$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("instance", value.to_string()))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn body(self, value: &types::InstanceNetworkInterfaceCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceNetworkInterfaceCreateThen(httpmock::Then);
    impl InstanceNetworkInterfaceCreateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::InstanceNetworkInterface) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceNetworkInterfaceViewWhen(httpmock::When);
    impl InstanceNetworkInterfaceViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/network-interfaces/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn interface(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/network-interfaces/{}$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("instance", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "instance"))
                        .is_none()
                }))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }
    }

    pub struct InstanceNetworkInterfaceViewThen(httpmock::Then);
    impl InstanceNetworkInterfaceViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::InstanceNetworkInterface) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceNetworkInterfaceUpdateWhen(httpmock::When);
    impl InstanceNetworkInterfaceUpdateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/network-interfaces/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn interface(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/network-interfaces/{}$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("instance", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "instance"))
                        .is_none()
                }))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn body(self, value: &types::InstanceNetworkInterfaceUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceNetworkInterfaceUpdateThen(httpmock::Then);
    impl InstanceNetworkInterfaceUpdateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::InstanceNetworkInterface) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct InstanceNetworkInterfaceDeleteWhen(httpmock::When);
    impl InstanceNetworkInterfaceDeleteWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/network-interfaces/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn interface(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/network-interfaces/{}$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("instance", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "instance"))
                        .is_none()
                }))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }
    }

    pub struct InstanceNetworkInterfaceDeleteThen(httpmock::Then);
    impl InstanceNetworkInterfaceDeleteThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct PolicyViewWhen(httpmock::When);
    impl PolicyViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }
    }

    pub struct PolicyViewThen(httpmock::Then);
    impl PolicyViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SiloRolePolicy) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct PolicyUpdateWhen(httpmock::When);
    impl PolicyUpdateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::SiloRolePolicy) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct PolicyUpdateThen(httpmock::Then);
    impl PolicyUpdateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SiloRolePolicy) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ProjectListWhen(httpmock::When);
    impl ProjectListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/projects$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct ProjectListThen(httpmock::Then);
    impl ProjectListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::ProjectResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ProjectCreateWhen(httpmock::When);
    impl ProjectCreateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/projects$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::ProjectCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct ProjectCreateThen(httpmock::Then);
    impl ProjectCreateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::Project) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ProjectViewWhen(httpmock::When);
    impl ProjectViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/projects/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/projects/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct ProjectViewThen(httpmock::Then);
    impl ProjectViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Project) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ProjectUpdateWhen(httpmock::When);
    impl ProjectUpdateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/projects/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/projects/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::ProjectUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct ProjectUpdateThen(httpmock::Then);
    impl ProjectUpdateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Project) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ProjectDeleteWhen(httpmock::When);
    impl ProjectDeleteWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/projects/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/projects/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct ProjectDeleteThen(httpmock::Then);
    impl ProjectDeleteThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ProjectPolicyViewWhen(httpmock::When);
    impl ProjectPolicyViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/projects/[^/]*/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/projects/{}/policy$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct ProjectPolicyViewThen(httpmock::Then);
    impl ProjectPolicyViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::ProjectRolePolicy) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct ProjectPolicyUpdateWhen(httpmock::When);
    impl ProjectPolicyUpdateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/projects/[^/]*/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/projects/{}/policy$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::ProjectRolePolicy) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct ProjectPolicyUpdateThen(httpmock::Then);
    impl ProjectPolicyUpdateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::ProjectRolePolicy) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SnapshotListWhen(httpmock::When);
    impl SnapshotListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/snapshots$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct SnapshotListThen(httpmock::Then);
    impl SnapshotListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SnapshotResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SnapshotCreateWhen(httpmock::When);
    impl SnapshotCreateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/snapshots$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("project", value.to_string()))
        }

        pub fn body(self, value: &types::SnapshotCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SnapshotCreateThen(httpmock::Then);
    impl SnapshotCreateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::Snapshot) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SnapshotViewWhen(httpmock::When);
    impl SnapshotViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/snapshots/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn snapshot(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/snapshots/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }
    }

    pub struct SnapshotViewThen(httpmock::Then);
    impl SnapshotViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Snapshot) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SnapshotDeleteWhen(httpmock::When);
    impl SnapshotDeleteWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/snapshots/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn snapshot(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/snapshots/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }
    }

    pub struct SnapshotDeleteThen(httpmock::Then);
    impl SnapshotDeleteThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct PhysicalDiskListWhen(httpmock::When);
    impl PhysicalDiskListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/hardware/disks$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct PhysicalDiskListThen(httpmock::Then);
    impl PhysicalDiskListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::PhysicalDiskResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct RackListWhen(httpmock::When);
    impl RackListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/hardware/racks$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct RackListThen(httpmock::Then);
    impl RackListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::RackResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct RackViewWhen(httpmock::When);
    impl RackViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/hardware/racks/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn rack_id(self, value: &uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/hardware/racks/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct RackViewThen(httpmock::Then);
    impl RackViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Rack) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SledListWhen(httpmock::When);
    impl SledListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/hardware/sleds$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct SledListThen(httpmock::Then);
    impl SledListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SledResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SledViewWhen(httpmock::When);
    impl SledViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/hardware/sleds/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn sled_id(self, value: &uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/hardware/sleds/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SledViewThen(httpmock::Then);
    impl SledViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Sled) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SledPhysicalDiskListWhen(httpmock::When);
    impl SledPhysicalDiskListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(inner.method(httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/hardware/sleds/[^/]*/disks$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn sled_id(self, value: &uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/hardware/sleds/{}/disks$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct SledPhysicalDiskListThen(httpmock::Then);
    impl SledPhysicalDiskListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::PhysicalDiskResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SledInstanceListWhen(httpmock::When);
    impl SledInstanceListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(inner.method(httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/hardware/sleds/[^/]*/instances$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn sled_id(self, value: &uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/hardware/sleds/{}/instances$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct SledInstanceListThen(httpmock::Then);
    impl SledInstanceListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SledInstanceResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct NetworkingSwitchPortListWhen(httpmock::When);
    impl NetworkingSwitchPortListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/hardware/switch-port$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }

        pub fn switch_port_id<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a uuid::Uuid>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("switch_port_id", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "switch_port_id"))
                        .is_none()
                }))
            }
        }
    }

    pub struct NetworkingSwitchPortListThen(httpmock::Then);
    impl NetworkingSwitchPortListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SwitchPortResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct NetworkingSwitchPortApplySettingsWhen(httpmock::When);
    impl NetworkingSwitchPortApplySettingsWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(inner.method(httpmock::Method::POST).path_matches(
                regex::Regex::new("^/v1/system/hardware/switch-port/[^/]*/settings$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn port(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/hardware/switch-port/{}/settings$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn rack_id(self, value: &uuid::Uuid) -> Self {
            Self(self.0.query_param("rack_id", value.to_string()))
        }

        pub fn switch_location(self, value: &types::Name) -> Self {
            Self(self.0.query_param("switch_location", value.to_string()))
        }

        pub fn body(self, value: &types::SwitchPortApplySettings) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct NetworkingSwitchPortApplySettingsThen(httpmock::Then);
    impl NetworkingSwitchPortApplySettingsThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct NetworkingSwitchPortClearSettingsWhen(httpmock::When);
    impl NetworkingSwitchPortClearSettingsWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(inner.method(httpmock::Method::DELETE).path_matches(
                regex::Regex::new("^/v1/system/hardware/switch-port/[^/]*/settings$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn port(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/hardware/switch-port/{}/settings$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn rack_id(self, value: &uuid::Uuid) -> Self {
            Self(self.0.query_param("rack_id", value.to_string()))
        }

        pub fn switch_location(self, value: &types::Name) -> Self {
            Self(self.0.query_param("switch_location", value.to_string()))
        }
    }

    pub struct NetworkingSwitchPortClearSettingsThen(httpmock::Then);
    impl NetworkingSwitchPortClearSettingsThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SwitchListWhen(httpmock::When);
    impl SwitchListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/hardware/switches$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct SwitchListThen(httpmock::Then);
    impl SwitchListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SwitchResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SwitchViewWhen(httpmock::When);
    impl SwitchViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner.method(httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/v1/system/hardware/switches/[^/]*$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn switch_id(self, value: &uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/hardware/switches/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SwitchViewThen(httpmock::Then);
    impl SwitchViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Switch) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SiloIdentityProviderListWhen(httpmock::When);
    impl SiloIdentityProviderListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/identity-providers$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn silo<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("silo", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "silo"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct SiloIdentityProviderListThen(httpmock::Then);
    impl SiloIdentityProviderListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::IdentityProviderResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct LocalIdpUserCreateWhen(httpmock::When);
    impl LocalIdpUserCreateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(inner.method(httpmock::Method::POST).path_matches(
                regex::Regex::new("^/v1/system/identity-providers/local/users$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn silo(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("silo", value.to_string()))
        }

        pub fn body(self, value: &types::UserCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct LocalIdpUserCreateThen(httpmock::Then);
    impl LocalIdpUserCreateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::User) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct LocalIdpUserDeleteWhen(httpmock::When);
    impl LocalIdpUserDeleteWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(inner.method(httpmock::Method::DELETE).path_matches(
                regex::Regex::new("^/v1/system/identity-providers/local/users/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn user_id(self, value: &uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/identity-providers/local/users/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn silo(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("silo", value.to_string()))
        }
    }

    pub struct LocalIdpUserDeleteThen(httpmock::Then);
    impl LocalIdpUserDeleteThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct LocalIdpUserSetPasswordWhen(httpmock::When);
    impl LocalIdpUserSetPasswordWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner.method(httpmock::Method::POST).path_matches(
                    regex::Regex::new(
                        "^/v1/system/identity-providers/local/users/[^/]*/set-password$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn user_id(self, value: &uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/identity-providers/local/users/{}/set-password$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn silo(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("silo", value.to_string()))
        }

        pub fn body(self, value: &types::UserPassword) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct LocalIdpUserSetPasswordThen(httpmock::Then);
    impl LocalIdpUserSetPasswordThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SamlIdentityProviderCreateWhen(httpmock::When);
    impl SamlIdentityProviderCreateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner.method(httpmock::Method::POST).path_matches(
                    regex::Regex::new("^/v1/system/identity-providers/saml$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn silo(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("silo", value.to_string()))
        }

        pub fn body(self, value: &types::SamlIdentityProviderCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SamlIdentityProviderCreateThen(httpmock::Then);
    impl SamlIdentityProviderCreateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::SamlIdentityProvider) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SamlIdentityProviderViewWhen(httpmock::When);
    impl SamlIdentityProviderViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(inner.method(httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/identity-providers/saml/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn provider(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/identity-providers/saml/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn silo(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("silo", value.to_string()))
        }
    }

    pub struct SamlIdentityProviderViewThen(httpmock::Then);
    impl SamlIdentityProviderViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SamlIdentityProvider) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolListWhen(httpmock::When);
    impl IpPoolListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/ip-pools$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct IpPoolListThen(httpmock::Then);
    impl IpPoolListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::IpPoolResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolCreateWhen(httpmock::When);
    impl IpPoolCreateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/system/ip-pools$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::IpPoolCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct IpPoolCreateThen(httpmock::Then);
    impl IpPoolCreateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::IpPool) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolViewWhen(httpmock::When);
    impl IpPoolViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/ip-pools/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn pool(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/system/ip-pools/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct IpPoolViewThen(httpmock::Then);
    impl IpPoolViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::IpPool) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolUpdateWhen(httpmock::When);
    impl IpPoolUpdateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/system/ip-pools/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn pool(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/system/ip-pools/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::IpPoolUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct IpPoolUpdateThen(httpmock::Then);
    impl IpPoolUpdateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::IpPool) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolDeleteWhen(httpmock::When);
    impl IpPoolDeleteWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/system/ip-pools/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn pool(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/system/ip-pools/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct IpPoolDeleteThen(httpmock::Then);
    impl IpPoolDeleteThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolRangeListWhen(httpmock::When);
    impl IpPoolRangeListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/ip-pools/[^/]*/ranges$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn pool(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/ip-pools/{}/ranges$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }
    }

    pub struct IpPoolRangeListThen(httpmock::Then);
    impl IpPoolRangeListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::IpPoolRangeResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolRangeAddWhen(httpmock::When);
    impl IpPoolRangeAddWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner.method(httpmock::Method::POST).path_matches(
                    regex::Regex::new("^/v1/system/ip-pools/[^/]*/ranges/add$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn pool(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/ip-pools/{}/ranges/add$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::IpRange) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct IpPoolRangeAddThen(httpmock::Then);
    impl IpPoolRangeAddThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::IpPoolRange) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolRangeRemoveWhen(httpmock::When);
    impl IpPoolRangeRemoveWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(inner.method(httpmock::Method::POST).path_matches(
                regex::Regex::new("^/v1/system/ip-pools/[^/]*/ranges/remove$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn pool(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/ip-pools/{}/ranges/remove$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::IpRange) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct IpPoolRangeRemoveThen(httpmock::Then);
    impl IpPoolRangeRemoveThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolServiceViewWhen(httpmock::When);
    impl IpPoolServiceViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/ip-pools-service$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }
    }

    pub struct IpPoolServiceViewThen(httpmock::Then);
    impl IpPoolServiceViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::IpPool) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolServiceRangeListWhen(httpmock::When);
    impl IpPoolServiceRangeListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner.method(httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/v1/system/ip-pools-service/ranges$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }
    }

    pub struct IpPoolServiceRangeListThen(httpmock::Then);
    impl IpPoolServiceRangeListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::IpPoolRangeResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolServiceRangeAddWhen(httpmock::When);
    impl IpPoolServiceRangeAddWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(inner.method(httpmock::Method::POST).path_matches(
                regex::Regex::new("^/v1/system/ip-pools-service/ranges/add$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::IpRange) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct IpPoolServiceRangeAddThen(httpmock::Then);
    impl IpPoolServiceRangeAddThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::IpPoolRange) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct IpPoolServiceRangeRemoveWhen(httpmock::When);
    impl IpPoolServiceRangeRemoveWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(inner.method(httpmock::Method::POST).path_matches(
                regex::Regex::new("^/v1/system/ip-pools-service/ranges/remove$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::IpRange) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct IpPoolServiceRangeRemoveThen(httpmock::Then);
    impl IpPoolServiceRangeRemoveThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemMetricWhen(httpmock::When);
    impl SystemMetricWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/metrics/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn metric_name(self, value: types::SystemMetricName) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/system/metrics/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn end_time<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a chrono::DateTime<chrono::offset::Utc>>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("end_time", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "end_time"))
                        .is_none()
                }))
            }
        }

        pub fn id(self, value: &uuid::Uuid) -> Self {
            Self(self.0.query_param("id", value.to_string()))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn start_time<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a chrono::DateTime<chrono::offset::Utc>>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("start_time", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "start_time"))
                        .is_none()
                }))
            }
        }
    }

    pub struct SystemMetricThen(httpmock::Then);
    impl SystemMetricThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::MeasurementResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct NetworkingAddressLotListWhen(httpmock::When);
    impl NetworkingAddressLotListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner.method(httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/v1/system/networking/address-lot$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct NetworkingAddressLotListThen(httpmock::Then);
    impl NetworkingAddressLotListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::AddressLotResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct NetworkingAddressLotCreateWhen(httpmock::When);
    impl NetworkingAddressLotCreateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner.method(httpmock::Method::POST).path_matches(
                    regex::Regex::new("^/v1/system/networking/address-lot$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::AddressLotCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct NetworkingAddressLotCreateThen(httpmock::Then);
    impl NetworkingAddressLotCreateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::AddressLotCreateResponse) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct NetworkingAddressLotDeleteWhen(httpmock::When);
    impl NetworkingAddressLotDeleteWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(inner.method(httpmock::Method::DELETE).path_matches(
                regex::Regex::new("^/v1/system/networking/address-lot/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn address_lot(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/networking/address-lot/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct NetworkingAddressLotDeleteThen(httpmock::Then);
    impl NetworkingAddressLotDeleteThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct NetworkingAddressLotBlockListWhen(httpmock::When);
    impl NetworkingAddressLotBlockListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(inner.method(httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/networking/address-lot/[^/]*/blocks$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn address_lot(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/networking/address-lot/{}/blocks$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct NetworkingAddressLotBlockListThen(httpmock::Then);
    impl NetworkingAddressLotBlockListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::AddressLotBlockResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct NetworkingLoopbackAddressListWhen(httpmock::When);
    impl NetworkingLoopbackAddressListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(inner.method(httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/networking/loopback-address$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct NetworkingLoopbackAddressListThen(httpmock::Then);
    impl NetworkingLoopbackAddressListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::LoopbackAddressResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct NetworkingLoopbackAddressCreateWhen(httpmock::When);
    impl NetworkingLoopbackAddressCreateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(inner.method(httpmock::Method::POST).path_matches(
                regex::Regex::new("^/v1/system/networking/loopback-address$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::LoopbackAddressCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct NetworkingLoopbackAddressCreateThen(httpmock::Then);
    impl NetworkingLoopbackAddressCreateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::LoopbackAddress) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct NetworkingLoopbackAddressDeleteWhen(httpmock::When);
    impl NetworkingLoopbackAddressDeleteWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner.method(httpmock::Method::DELETE).path_matches(
                    regex::Regex::new(
                        "^/v1/system/networking/loopback-address/[^/]*/[^/]*/[^/]*/[^/]*$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn rack_id(self, value: &uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/networking/loopback-address/{}/.*/.*/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn switch_location(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/networking/loopback-address/.*/{}/.*/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn address(self, value: &std::net::IpAddr) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/networking/loopback-address/.*/.*/{}/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn subnet_mask(self, value: u8) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/networking/loopback-address/.*/.*/.*/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct NetworkingLoopbackAddressDeleteThen(httpmock::Then);
    impl NetworkingLoopbackAddressDeleteThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct NetworkingSwitchPortSettingsListWhen(httpmock::When);
    impl NetworkingSwitchPortSettingsListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(inner.method(httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/networking/switch-port-settings$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn port_settings<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("port_settings", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "port_settings"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct NetworkingSwitchPortSettingsListThen(httpmock::Then);
    impl NetworkingSwitchPortSettingsListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SwitchPortSettingsResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct NetworkingSwitchPortSettingsCreateWhen(httpmock::When);
    impl NetworkingSwitchPortSettingsCreateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(inner.method(httpmock::Method::POST).path_matches(
                regex::Regex::new("^/v1/system/networking/switch-port-settings$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::SwitchPortSettingsCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct NetworkingSwitchPortSettingsCreateThen(httpmock::Then);
    impl NetworkingSwitchPortSettingsCreateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::SwitchPortSettingsView) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct NetworkingSwitchPortSettingsDeleteWhen(httpmock::When);
    impl NetworkingSwitchPortSettingsDeleteWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(inner.method(httpmock::Method::DELETE).path_matches(
                regex::Regex::new("^/v1/system/networking/switch-port-settings$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn port_settings<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("port_settings", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "port_settings"))
                        .is_none()
                }))
            }
        }
    }

    pub struct NetworkingSwitchPortSettingsDeleteThen(httpmock::Then);
    impl NetworkingSwitchPortSettingsDeleteThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct NetworkingSwitchPortSettingsViewWhen(httpmock::When);
    impl NetworkingSwitchPortSettingsViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(inner.method(httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/networking/switch-port-settings/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn port(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/networking/switch-port-settings/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct NetworkingSwitchPortSettingsViewThen(httpmock::Then);
    impl NetworkingSwitchPortSettingsViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SwitchPortSettingsView) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemPolicyViewWhen(httpmock::When);
    impl SystemPolicyViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }
    }

    pub struct SystemPolicyViewThen(httpmock::Then);
    impl SystemPolicyViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::FleetRolePolicy) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemPolicyUpdateWhen(httpmock::When);
    impl SystemPolicyUpdateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/system/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::FleetRolePolicy) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SystemPolicyUpdateThen(httpmock::Then);
    impl SystemPolicyUpdateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::FleetRolePolicy) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct RoleListWhen(httpmock::When);
    impl RoleListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/roles$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }
    }

    pub struct RoleListThen(httpmock::Then);
    impl RoleListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::RoleResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct RoleViewWhen(httpmock::When);
    impl RoleViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/roles/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn role_name(self, value: &str) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/system/roles/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct RoleViewThen(httpmock::Then);
    impl RoleViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Role) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SiloListWhen(httpmock::When);
    impl SiloListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/silos$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct SiloListThen(httpmock::Then);
    impl SiloListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SiloResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SiloCreateWhen(httpmock::When);
    impl SiloCreateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/system/silos$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::SiloCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SiloCreateThen(httpmock::Then);
    impl SiloCreateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::Silo) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SiloViewWhen(httpmock::When);
    impl SiloViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/silos/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn silo(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/system/silos/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SiloViewThen(httpmock::Then);
    impl SiloViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Silo) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SiloDeleteWhen(httpmock::When);
    impl SiloDeleteWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/system/silos/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn silo(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/system/silos/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SiloDeleteThen(httpmock::Then);
    impl SiloDeleteThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SiloPolicyViewWhen(httpmock::When);
    impl SiloPolicyViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/silos/[^/]*/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn silo(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/system/silos/{}/policy$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SiloPolicyViewThen(httpmock::Then);
    impl SiloPolicyViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SiloRolePolicy) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SiloPolicyUpdateWhen(httpmock::When);
    impl SiloPolicyUpdateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/system/silos/[^/]*/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn silo(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/system/silos/{}/policy$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::SiloRolePolicy) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SiloPolicyUpdateThen(httpmock::Then);
    impl SiloPolicyUpdateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SiloRolePolicy) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemComponentVersionListWhen(httpmock::When);
    impl SystemComponentVersionListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/update/components$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct SystemComponentVersionListThen(httpmock::Then);
    impl SystemComponentVersionListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::UpdateableComponentResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct UpdateDeploymentsListWhen(httpmock::When);
    impl UpdateDeploymentsListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/update/deployments$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct UpdateDeploymentsListThen(httpmock::Then);
    impl UpdateDeploymentsListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::UpdateDeploymentResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct UpdateDeploymentViewWhen(httpmock::When);
    impl UpdateDeploymentViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner.method(httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/v1/system/update/deployments/[^/]*$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn id(self, value: &uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/update/deployments/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct UpdateDeploymentViewThen(httpmock::Then);
    impl UpdateDeploymentViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::UpdateDeployment) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemUpdateRefreshWhen(httpmock::When);
    impl SystemUpdateRefreshWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/system/update/refresh$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }
    }

    pub struct SystemUpdateRefreshThen(httpmock::Then);
    impl SystemUpdateRefreshThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemUpdateStartWhen(httpmock::When);
    impl SystemUpdateStartWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/system/update/start$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::SystemUpdateStart) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SystemUpdateStartThen(httpmock::Then);
    impl SystemUpdateStartThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn accepted(self, value: &types::UpdateDeployment) -> Self {
            Self(
                self.0
                    .status(202u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemUpdateStopWhen(httpmock::When);
    impl SystemUpdateStopWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/system/update/stop$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }
    }

    pub struct SystemUpdateStopThen(httpmock::Then);
    impl SystemUpdateStopThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemUpdateListWhen(httpmock::When);
    impl SystemUpdateListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/update/updates$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct SystemUpdateListThen(httpmock::Then);
    impl SystemUpdateListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SystemUpdateResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemUpdateViewWhen(httpmock::When);
    impl SystemUpdateViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/update/updates/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn version(self, value: &types::SemverVersion) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/update/updates/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SystemUpdateViewThen(httpmock::Then);
    impl SystemUpdateViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SystemUpdate) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemUpdateComponentsListWhen(httpmock::When);
    impl SystemUpdateComponentsListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(inner.method(httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/update/updates/[^/]*/components$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn version(self, value: &types::SemverVersion) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/update/updates/{}/components$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SystemUpdateComponentsListThen(httpmock::Then);
    impl SystemUpdateComponentsListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::ComponentUpdateResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SystemVersionWhen(httpmock::When);
    impl SystemVersionWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/update/version$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }
    }

    pub struct SystemVersionThen(httpmock::Then);
    impl SystemVersionThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SystemVersion) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SiloUserListWhen(httpmock::When);
    impl SiloUserListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/users$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn silo<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("silo", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "silo"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct SiloUserListThen(httpmock::Then);
    impl SiloUserListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::UserResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct SiloUserViewWhen(httpmock::When);
    impl SiloUserViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/users/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn user_id(self, value: &uuid::Uuid) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/system/users/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn silo(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("silo", value.to_string()))
        }
    }

    pub struct SiloUserViewThen(httpmock::Then);
    impl SiloUserViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::User) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct UserBuiltinListWhen(httpmock::When);
    impl UserBuiltinListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/users-builtin$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct UserBuiltinListThen(httpmock::Then);
    impl UserBuiltinListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::UserBuiltinResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct UserBuiltinViewWhen(httpmock::When);
    impl UserBuiltinViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/users-builtin/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn user(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/system/users-builtin/{}$", value.to_string()))
                    .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct UserBuiltinViewThen(httpmock::Then);
    impl UserBuiltinViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::UserBuiltin) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct UserListWhen(httpmock::When);
    impl UserListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/users$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn group<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a uuid::Uuid>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("group", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "group"))
                        .is_none()
                }))
            }
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::IdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct UserListThen(httpmock::Then);
    impl UserListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::UserResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcFirewallRulesViewWhen(httpmock::When);
    impl VpcFirewallRulesViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/vpc-firewall-rules$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn vpc(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("vpc", value.to_string()))
        }
    }

    pub struct VpcFirewallRulesViewThen(httpmock::Then);
    impl VpcFirewallRulesViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::VpcFirewallRules) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcFirewallRulesUpdateWhen(httpmock::When);
    impl VpcFirewallRulesUpdateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/vpc-firewall-rules$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn vpc(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("vpc", value.to_string()))
        }

        pub fn body(self, value: &types::VpcFirewallRuleUpdateParams) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct VpcFirewallRulesUpdateThen(httpmock::Then);
    impl VpcFirewallRulesUpdateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::VpcFirewallRules) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcRouterRouteListWhen(httpmock::When);
    impl VpcRouterRouteListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/vpc-router-routes$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn router<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("router", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "router"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }

        pub fn vpc<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("vpc", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "vpc"))
                        .is_none()
                }))
            }
        }
    }

    pub struct VpcRouterRouteListThen(httpmock::Then);
    impl VpcRouterRouteListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::RouterRouteResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcRouterRouteCreateWhen(httpmock::When);
    impl VpcRouterRouteCreateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/vpc-router-routes$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn router(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("router", value.to_string()))
        }

        pub fn vpc<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("vpc", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "vpc"))
                        .is_none()
                }))
            }
        }

        pub fn body(self, value: &types::RouterRouteCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct VpcRouterRouteCreateThen(httpmock::Then);
    impl VpcRouterRouteCreateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::RouterRoute) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcRouterRouteViewWhen(httpmock::When);
    impl VpcRouterRouteViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/vpc-router-routes/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn route(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/vpc-router-routes/{}$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn router(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("router", value.to_string()))
        }

        pub fn vpc<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("vpc", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "vpc"))
                        .is_none()
                }))
            }
        }
    }

    pub struct VpcRouterRouteViewThen(httpmock::Then);
    impl VpcRouterRouteViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::RouterRoute) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcRouterRouteUpdateWhen(httpmock::When);
    impl VpcRouterRouteUpdateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/vpc-router-routes/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn route(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/vpc-router-routes/{}$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn router<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("router", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "router"))
                        .is_none()
                }))
            }
        }

        pub fn vpc<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("vpc", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "vpc"))
                        .is_none()
                }))
            }
        }

        pub fn body(self, value: &types::RouterRouteUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct VpcRouterRouteUpdateThen(httpmock::Then);
    impl VpcRouterRouteUpdateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::RouterRoute) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcRouterRouteDeleteWhen(httpmock::When);
    impl VpcRouterRouteDeleteWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/vpc-router-routes/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn route(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/vpc-router-routes/{}$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn router<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("router", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "router"))
                        .is_none()
                }))
            }
        }

        pub fn vpc<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("vpc", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "vpc"))
                        .is_none()
                }))
            }
        }
    }

    pub struct VpcRouterRouteDeleteThen(httpmock::Then);
    impl VpcRouterRouteDeleteThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcRouterListWhen(httpmock::When);
    impl VpcRouterListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/vpc-routers$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }

        pub fn vpc<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("vpc", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "vpc"))
                        .is_none()
                }))
            }
        }
    }

    pub struct VpcRouterListThen(httpmock::Then);
    impl VpcRouterListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::VpcRouterResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcRouterCreateWhen(httpmock::When);
    impl VpcRouterCreateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/vpc-routers$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn vpc(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("vpc", value.to_string()))
        }

        pub fn body(self, value: &types::VpcRouterCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct VpcRouterCreateThen(httpmock::Then);
    impl VpcRouterCreateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::VpcRouter) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcRouterViewWhen(httpmock::When);
    impl VpcRouterViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/vpc-routers/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn router(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/vpc-routers/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn vpc<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("vpc", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "vpc"))
                        .is_none()
                }))
            }
        }
    }

    pub struct VpcRouterViewThen(httpmock::Then);
    impl VpcRouterViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::VpcRouter) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcRouterUpdateWhen(httpmock::When);
    impl VpcRouterUpdateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/vpc-routers/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn router(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/vpc-routers/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn vpc<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("vpc", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "vpc"))
                        .is_none()
                }))
            }
        }

        pub fn body(self, value: &types::VpcRouterUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct VpcRouterUpdateThen(httpmock::Then);
    impl VpcRouterUpdateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::VpcRouter) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcRouterDeleteWhen(httpmock::When);
    impl VpcRouterDeleteWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/vpc-routers/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn router(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/vpc-routers/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn vpc<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("vpc", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "vpc"))
                        .is_none()
                }))
            }
        }
    }

    pub struct VpcRouterDeleteThen(httpmock::Then);
    impl VpcRouterDeleteThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcSubnetListWhen(httpmock::When);
    impl VpcSubnetListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/vpc-subnets$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }

        pub fn vpc<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("vpc", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "vpc"))
                        .is_none()
                }))
            }
        }
    }

    pub struct VpcSubnetListThen(httpmock::Then);
    impl VpcSubnetListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::VpcSubnetResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcSubnetCreateWhen(httpmock::When);
    impl VpcSubnetCreateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/vpc-subnets$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn vpc(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("vpc", value.to_string()))
        }

        pub fn body(self, value: &types::VpcSubnetCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct VpcSubnetCreateThen(httpmock::Then);
    impl VpcSubnetCreateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::VpcSubnet) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcSubnetViewWhen(httpmock::When);
    impl VpcSubnetViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/vpc-subnets/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn subnet(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/vpc-subnets/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn vpc<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("vpc", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "vpc"))
                        .is_none()
                }))
            }
        }
    }

    pub struct VpcSubnetViewThen(httpmock::Then);
    impl VpcSubnetViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::VpcSubnet) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcSubnetUpdateWhen(httpmock::When);
    impl VpcSubnetUpdateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/vpc-subnets/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn subnet(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/vpc-subnets/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn vpc<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("vpc", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "vpc"))
                        .is_none()
                }))
            }
        }

        pub fn body(self, value: &types::VpcSubnetUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct VpcSubnetUpdateThen(httpmock::Then);
    impl VpcSubnetUpdateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::VpcSubnet) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcSubnetDeleteWhen(httpmock::When);
    impl VpcSubnetDeleteWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/vpc-subnets/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn subnet(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/vpc-subnets/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn vpc<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("vpc", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "vpc"))
                        .is_none()
                }))
            }
        }
    }

    pub struct VpcSubnetDeleteThen(httpmock::Then);
    impl VpcSubnetDeleteThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcSubnetListNetworkInterfacesWhen(httpmock::When);
    impl VpcSubnetListNetworkInterfacesWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(inner.method(httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/vpc-subnets/[^/]*/network-interfaces$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn subnet(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/vpc-subnets/{}/network-interfaces$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }

        pub fn vpc<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("vpc", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "vpc"))
                        .is_none()
                }))
            }
        }
    }

    pub struct VpcSubnetListNetworkInterfacesThen(httpmock::Then);
    impl VpcSubnetListNetworkInterfacesThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::InstanceNetworkInterfaceResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcListWhen(httpmock::When);
    impl VpcListWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/vpcs$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<std::num::NonZeroU32>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("limit", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "limit"))
                        .is_none()
                }))
            }
        }

        pub fn page_token<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("page_token", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "page_token"))
                        .is_none()
                }))
            }
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::NameOrIdSortMode>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("sort_by", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "sort_by"))
                        .is_none()
                }))
            }
        }
    }

    pub struct VpcListThen(httpmock::Then);
    impl VpcListThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::VpcResultsPage) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcCreateWhen(httpmock::When);
    impl VpcCreateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/vpcs$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("project", value.to_string()))
        }

        pub fn body(self, value: &types::VpcCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct VpcCreateThen(httpmock::Then);
    impl VpcCreateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::Vpc) -> Self {
            Self(
                self.0
                    .status(201u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcViewWhen(httpmock::When);
    impl VpcViewWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/vpcs/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn vpc(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/vpcs/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }
    }

    pub struct VpcViewThen(httpmock::Then);
    impl VpcViewThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Vpc) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcUpdateWhen(httpmock::When);
    impl VpcUpdateWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/vpcs/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn vpc(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/vpcs/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }

        pub fn body(self, value: &types::VpcUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct VpcUpdateThen(httpmock::Then);
    impl VpcUpdateThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Vpc) -> Self {
            Self(
                self.0
                    .status(200u16)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }

    pub struct VpcDeleteWhen(httpmock::When);
    impl VpcDeleteWhen {
        pub fn new(inner: httpmock::When) -> Self {
            Self(
                inner
                    .method(httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/vpcs/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> httpmock::When {
            self.0
        }

        pub fn vpc(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/vpcs/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("project", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "project"))
                        .is_none()
                }))
            }
        }
    }

    pub struct VpcDeleteThen(httpmock::Then);
    impl VpcDeleteThen {
        pub fn new(inner: httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> httpmock::Then {
            self.0
        }

        pub fn no_content(self) -> Self {
            Self(self.0.status(204u16))
        }

        pub fn client_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 4u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }

        pub fn server_error(self, status: u16, value: &types::Error) -> Self {
            assert_eq!(status / 100u16, 5u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body_obj(value),
            )
        }
    }
}

/// An extension trait for [`MockServer`](httpmock::MockServer) that
/// adds a method for each operation. These are the equivalent of
/// type-checked [`mock()`](httpmock::MockServer::mock) calls.
pub trait MockServerExt {
    fn device_auth_request<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DeviceAuthRequestWhen, operations::DeviceAuthRequestThen);
    fn device_auth_confirm<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DeviceAuthConfirmWhen, operations::DeviceAuthConfirmThen);
    fn device_access_token<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DeviceAccessTokenWhen, operations::DeviceAccessTokenThen);
    fn login_spoof<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::LoginSpoofWhen, operations::LoginSpoofThen);
    fn login_saml_begin<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::LoginSamlBeginWhen, operations::LoginSamlBeginThen);
    fn login_saml<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::LoginSamlWhen, operations::LoginSamlThen);
    fn certificate_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CertificateListWhen, operations::CertificateListThen);
    fn certificate_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CertificateCreateWhen, operations::CertificateCreateThen);
    fn certificate_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CertificateViewWhen, operations::CertificateViewThen);
    fn certificate_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CertificateDeleteWhen, operations::CertificateDeleteThen);
    fn disk_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DiskListWhen, operations::DiskListThen);
    fn disk_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DiskCreateWhen, operations::DiskCreateThen);
    fn disk_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DiskViewWhen, operations::DiskViewThen);
    fn disk_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DiskDeleteWhen, operations::DiskDeleteThen);
    fn disk_bulk_write_import<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DiskBulkWriteImportWhen, operations::DiskBulkWriteImportThen);
    fn disk_bulk_write_import_start<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::DiskBulkWriteImportStartWhen,
            operations::DiskBulkWriteImportStartThen,
        );
    fn disk_bulk_write_import_stop<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DiskBulkWriteImportStopWhen, operations::DiskBulkWriteImportStopThen);
    fn disk_finalize_import<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DiskFinalizeImportWhen, operations::DiskFinalizeImportThen);
    fn disk_import_blocks_from_url<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DiskImportBlocksFromUrlWhen, operations::DiskImportBlocksFromUrlThen);
    fn disk_metrics_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DiskMetricsListWhen, operations::DiskMetricsListThen);
    fn group_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::GroupListWhen, operations::GroupListThen);
    fn group_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::GroupViewWhen, operations::GroupViewThen);
    fn image_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ImageListWhen, operations::ImageListThen);
    fn image_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ImageCreateWhen, operations::ImageCreateThen);
    fn image_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ImageViewWhen, operations::ImageViewThen);
    fn image_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ImageDeleteWhen, operations::ImageDeleteThen);
    fn image_demote<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ImageDemoteWhen, operations::ImageDemoteThen);
    fn image_promote<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ImagePromoteWhen, operations::ImagePromoteThen);
    fn instance_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceListWhen, operations::InstanceListThen);
    fn instance_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceCreateWhen, operations::InstanceCreateThen);
    fn instance_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceViewWhen, operations::InstanceViewThen);
    fn instance_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceDeleteWhen, operations::InstanceDeleteThen);
    fn instance_disk_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceDiskListWhen, operations::InstanceDiskListThen);
    fn instance_disk_attach<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceDiskAttachWhen, operations::InstanceDiskAttachThen);
    fn instance_disk_detach<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceDiskDetachWhen, operations::InstanceDiskDetachThen);
    fn instance_external_ip_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceExternalIpListWhen, operations::InstanceExternalIpListThen);
    fn instance_migrate<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceMigrateWhen, operations::InstanceMigrateThen);
    fn instance_reboot<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceRebootWhen, operations::InstanceRebootThen);
    fn instance_serial_console<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceSerialConsoleWhen, operations::InstanceSerialConsoleThen);
    fn instance_serial_console_stream<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceSerialConsoleStreamWhen,
            operations::InstanceSerialConsoleStreamThen,
        );
    fn instance_start<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceStartWhen, operations::InstanceStartThen);
    fn instance_stop<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceStopWhen, operations::InstanceStopThen);
    fn login_local<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::LoginLocalWhen, operations::LoginLocalThen);
    fn logout<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::LogoutWhen, operations::LogoutThen);
    fn current_user_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CurrentUserViewWhen, operations::CurrentUserViewThen);
    fn current_user_groups<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CurrentUserGroupsWhen, operations::CurrentUserGroupsThen);
    fn current_user_ssh_key_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CurrentUserSshKeyListWhen, operations::CurrentUserSshKeyListThen);
    fn current_user_ssh_key_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CurrentUserSshKeyCreateWhen, operations::CurrentUserSshKeyCreateThen);
    fn current_user_ssh_key_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CurrentUserSshKeyViewWhen, operations::CurrentUserSshKeyViewThen);
    fn current_user_ssh_key_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CurrentUserSshKeyDeleteWhen, operations::CurrentUserSshKeyDeleteThen);
    fn instance_network_interface_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceListWhen,
            operations::InstanceNetworkInterfaceListThen,
        );
    fn instance_network_interface_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceCreateWhen,
            operations::InstanceNetworkInterfaceCreateThen,
        );
    fn instance_network_interface_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceViewWhen,
            operations::InstanceNetworkInterfaceViewThen,
        );
    fn instance_network_interface_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceUpdateWhen,
            operations::InstanceNetworkInterfaceUpdateThen,
        );
    fn instance_network_interface_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceDeleteWhen,
            operations::InstanceNetworkInterfaceDeleteThen,
        );
    fn policy_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::PolicyViewWhen, operations::PolicyViewThen);
    fn policy_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::PolicyUpdateWhen, operations::PolicyUpdateThen);
    fn project_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ProjectListWhen, operations::ProjectListThen);
    fn project_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ProjectCreateWhen, operations::ProjectCreateThen);
    fn project_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ProjectViewWhen, operations::ProjectViewThen);
    fn project_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ProjectUpdateWhen, operations::ProjectUpdateThen);
    fn project_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ProjectDeleteWhen, operations::ProjectDeleteThen);
    fn project_policy_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ProjectPolicyViewWhen, operations::ProjectPolicyViewThen);
    fn project_policy_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ProjectPolicyUpdateWhen, operations::ProjectPolicyUpdateThen);
    fn snapshot_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SnapshotListWhen, operations::SnapshotListThen);
    fn snapshot_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SnapshotCreateWhen, operations::SnapshotCreateThen);
    fn snapshot_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SnapshotViewWhen, operations::SnapshotViewThen);
    fn snapshot_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SnapshotDeleteWhen, operations::SnapshotDeleteThen);
    fn physical_disk_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::PhysicalDiskListWhen, operations::PhysicalDiskListThen);
    fn rack_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::RackListWhen, operations::RackListThen);
    fn rack_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::RackViewWhen, operations::RackViewThen);
    fn sled_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SledListWhen, operations::SledListThen);
    fn sled_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SledViewWhen, operations::SledViewThen);
    fn sled_physical_disk_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SledPhysicalDiskListWhen, operations::SledPhysicalDiskListThen);
    fn sled_instance_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SledInstanceListWhen, operations::SledInstanceListThen);
    fn networking_switch_port_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortListWhen,
            operations::NetworkingSwitchPortListThen,
        );
    fn networking_switch_port_apply_settings<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortApplySettingsWhen,
            operations::NetworkingSwitchPortApplySettingsThen,
        );
    fn networking_switch_port_clear_settings<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortClearSettingsWhen,
            operations::NetworkingSwitchPortClearSettingsThen,
        );
    fn switch_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SwitchListWhen, operations::SwitchListThen);
    fn switch_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SwitchViewWhen, operations::SwitchViewThen);
    fn silo_identity_provider_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::SiloIdentityProviderListWhen,
            operations::SiloIdentityProviderListThen,
        );
    fn local_idp_user_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::LocalIdpUserCreateWhen, operations::LocalIdpUserCreateThen);
    fn local_idp_user_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::LocalIdpUserDeleteWhen, operations::LocalIdpUserDeleteThen);
    fn local_idp_user_set_password<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::LocalIdpUserSetPasswordWhen, operations::LocalIdpUserSetPasswordThen);
    fn saml_identity_provider_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::SamlIdentityProviderCreateWhen,
            operations::SamlIdentityProviderCreateThen,
        );
    fn saml_identity_provider_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::SamlIdentityProviderViewWhen,
            operations::SamlIdentityProviderViewThen,
        );
    fn ip_pool_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::IpPoolListWhen, operations::IpPoolListThen);
    fn ip_pool_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::IpPoolCreateWhen, operations::IpPoolCreateThen);
    fn ip_pool_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::IpPoolViewWhen, operations::IpPoolViewThen);
    fn ip_pool_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::IpPoolUpdateWhen, operations::IpPoolUpdateThen);
    fn ip_pool_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::IpPoolDeleteWhen, operations::IpPoolDeleteThen);
    fn ip_pool_range_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::IpPoolRangeListWhen, operations::IpPoolRangeListThen);
    fn ip_pool_range_add<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::IpPoolRangeAddWhen, operations::IpPoolRangeAddThen);
    fn ip_pool_range_remove<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::IpPoolRangeRemoveWhen, operations::IpPoolRangeRemoveThen);
    fn ip_pool_service_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::IpPoolServiceViewWhen, operations::IpPoolServiceViewThen);
    fn ip_pool_service_range_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::IpPoolServiceRangeListWhen, operations::IpPoolServiceRangeListThen);
    fn ip_pool_service_range_add<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::IpPoolServiceRangeAddWhen, operations::IpPoolServiceRangeAddThen);
    fn ip_pool_service_range_remove<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::IpPoolServiceRangeRemoveWhen,
            operations::IpPoolServiceRangeRemoveThen,
        );
    fn system_metric<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SystemMetricWhen, operations::SystemMetricThen);
    fn networking_address_lot_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingAddressLotListWhen,
            operations::NetworkingAddressLotListThen,
        );
    fn networking_address_lot_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingAddressLotCreateWhen,
            operations::NetworkingAddressLotCreateThen,
        );
    fn networking_address_lot_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingAddressLotDeleteWhen,
            operations::NetworkingAddressLotDeleteThen,
        );
    fn networking_address_lot_block_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingAddressLotBlockListWhen,
            operations::NetworkingAddressLotBlockListThen,
        );
    fn networking_loopback_address_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingLoopbackAddressListWhen,
            operations::NetworkingLoopbackAddressListThen,
        );
    fn networking_loopback_address_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingLoopbackAddressCreateWhen,
            operations::NetworkingLoopbackAddressCreateThen,
        );
    fn networking_loopback_address_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingLoopbackAddressDeleteWhen,
            operations::NetworkingLoopbackAddressDeleteThen,
        );
    fn networking_switch_port_settings_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortSettingsListWhen,
            operations::NetworkingSwitchPortSettingsListThen,
        );
    fn networking_switch_port_settings_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortSettingsCreateWhen,
            operations::NetworkingSwitchPortSettingsCreateThen,
        );
    fn networking_switch_port_settings_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortSettingsDeleteWhen,
            operations::NetworkingSwitchPortSettingsDeleteThen,
        );
    fn networking_switch_port_settings_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortSettingsViewWhen,
            operations::NetworkingSwitchPortSettingsViewThen,
        );
    fn system_policy_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SystemPolicyViewWhen, operations::SystemPolicyViewThen);
    fn system_policy_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SystemPolicyUpdateWhen, operations::SystemPolicyUpdateThen);
    fn role_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::RoleListWhen, operations::RoleListThen);
    fn role_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::RoleViewWhen, operations::RoleViewThen);
    fn silo_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SiloListWhen, operations::SiloListThen);
    fn silo_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SiloCreateWhen, operations::SiloCreateThen);
    fn silo_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SiloViewWhen, operations::SiloViewThen);
    fn silo_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SiloDeleteWhen, operations::SiloDeleteThen);
    fn silo_policy_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SiloPolicyViewWhen, operations::SiloPolicyViewThen);
    fn silo_policy_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SiloPolicyUpdateWhen, operations::SiloPolicyUpdateThen);
    fn system_component_version_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::SystemComponentVersionListWhen,
            operations::SystemComponentVersionListThen,
        );
    fn update_deployments_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::UpdateDeploymentsListWhen, operations::UpdateDeploymentsListThen);
    fn update_deployment_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::UpdateDeploymentViewWhen, operations::UpdateDeploymentViewThen);
    fn system_update_refresh<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SystemUpdateRefreshWhen, operations::SystemUpdateRefreshThen);
    fn system_update_start<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SystemUpdateStartWhen, operations::SystemUpdateStartThen);
    fn system_update_stop<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SystemUpdateStopWhen, operations::SystemUpdateStopThen);
    fn system_update_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SystemUpdateListWhen, operations::SystemUpdateListThen);
    fn system_update_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SystemUpdateViewWhen, operations::SystemUpdateViewThen);
    fn system_update_components_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::SystemUpdateComponentsListWhen,
            operations::SystemUpdateComponentsListThen,
        );
    fn system_version<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SystemVersionWhen, operations::SystemVersionThen);
    fn silo_user_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SiloUserListWhen, operations::SiloUserListThen);
    fn silo_user_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SiloUserViewWhen, operations::SiloUserViewThen);
    fn user_builtin_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::UserBuiltinListWhen, operations::UserBuiltinListThen);
    fn user_builtin_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::UserBuiltinViewWhen, operations::UserBuiltinViewThen);
    fn user_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::UserListWhen, operations::UserListThen);
    fn vpc_firewall_rules_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcFirewallRulesViewWhen, operations::VpcFirewallRulesViewThen);
    fn vpc_firewall_rules_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcFirewallRulesUpdateWhen, operations::VpcFirewallRulesUpdateThen);
    fn vpc_router_route_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterRouteListWhen, operations::VpcRouterRouteListThen);
    fn vpc_router_route_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterRouteCreateWhen, operations::VpcRouterRouteCreateThen);
    fn vpc_router_route_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterRouteViewWhen, operations::VpcRouterRouteViewThen);
    fn vpc_router_route_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterRouteUpdateWhen, operations::VpcRouterRouteUpdateThen);
    fn vpc_router_route_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterRouteDeleteWhen, operations::VpcRouterRouteDeleteThen);
    fn vpc_router_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterListWhen, operations::VpcRouterListThen);
    fn vpc_router_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterCreateWhen, operations::VpcRouterCreateThen);
    fn vpc_router_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterViewWhen, operations::VpcRouterViewThen);
    fn vpc_router_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterUpdateWhen, operations::VpcRouterUpdateThen);
    fn vpc_router_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterDeleteWhen, operations::VpcRouterDeleteThen);
    fn vpc_subnet_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcSubnetListWhen, operations::VpcSubnetListThen);
    fn vpc_subnet_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcSubnetCreateWhen, operations::VpcSubnetCreateThen);
    fn vpc_subnet_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcSubnetViewWhen, operations::VpcSubnetViewThen);
    fn vpc_subnet_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcSubnetUpdateWhen, operations::VpcSubnetUpdateThen);
    fn vpc_subnet_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcSubnetDeleteWhen, operations::VpcSubnetDeleteThen);
    fn vpc_subnet_list_network_interfaces<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::VpcSubnetListNetworkInterfacesWhen,
            operations::VpcSubnetListNetworkInterfacesThen,
        );
    fn vpc_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcListWhen, operations::VpcListThen);
    fn vpc_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcCreateWhen, operations::VpcCreateThen);
    fn vpc_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcViewWhen, operations::VpcViewThen);
    fn vpc_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcUpdateWhen, operations::VpcUpdateThen);
    fn vpc_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcDeleteWhen, operations::VpcDeleteThen);
}

impl MockServerExt for httpmock::MockServer {
    fn device_auth_request<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DeviceAuthRequestWhen, operations::DeviceAuthRequestThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DeviceAuthRequestWhen::new(when),
                operations::DeviceAuthRequestThen::new(then),
            )
        })
    }

    fn device_auth_confirm<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DeviceAuthConfirmWhen, operations::DeviceAuthConfirmThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DeviceAuthConfirmWhen::new(when),
                operations::DeviceAuthConfirmThen::new(then),
            )
        })
    }

    fn device_access_token<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DeviceAccessTokenWhen, operations::DeviceAccessTokenThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DeviceAccessTokenWhen::new(when),
                operations::DeviceAccessTokenThen::new(then),
            )
        })
    }

    fn login_spoof<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::LoginSpoofWhen, operations::LoginSpoofThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::LoginSpoofWhen::new(when),
                operations::LoginSpoofThen::new(then),
            )
        })
    }

    fn login_saml_begin<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::LoginSamlBeginWhen, operations::LoginSamlBeginThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::LoginSamlBeginWhen::new(when),
                operations::LoginSamlBeginThen::new(then),
            )
        })
    }

    fn login_saml<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::LoginSamlWhen, operations::LoginSamlThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::LoginSamlWhen::new(when),
                operations::LoginSamlThen::new(then),
            )
        })
    }

    fn certificate_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CertificateListWhen, operations::CertificateListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::CertificateListWhen::new(when),
                operations::CertificateListThen::new(then),
            )
        })
    }

    fn certificate_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CertificateCreateWhen, operations::CertificateCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::CertificateCreateWhen::new(when),
                operations::CertificateCreateThen::new(then),
            )
        })
    }

    fn certificate_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CertificateViewWhen, operations::CertificateViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::CertificateViewWhen::new(when),
                operations::CertificateViewThen::new(then),
            )
        })
    }

    fn certificate_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CertificateDeleteWhen, operations::CertificateDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::CertificateDeleteWhen::new(when),
                operations::CertificateDeleteThen::new(then),
            )
        })
    }

    fn disk_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DiskListWhen, operations::DiskListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DiskListWhen::new(when),
                operations::DiskListThen::new(then),
            )
        })
    }

    fn disk_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DiskCreateWhen, operations::DiskCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DiskCreateWhen::new(when),
                operations::DiskCreateThen::new(then),
            )
        })
    }

    fn disk_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DiskViewWhen, operations::DiskViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DiskViewWhen::new(when),
                operations::DiskViewThen::new(then),
            )
        })
    }

    fn disk_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DiskDeleteWhen, operations::DiskDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DiskDeleteWhen::new(when),
                operations::DiskDeleteThen::new(then),
            )
        })
    }

    fn disk_bulk_write_import<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DiskBulkWriteImportWhen, operations::DiskBulkWriteImportThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DiskBulkWriteImportWhen::new(when),
                operations::DiskBulkWriteImportThen::new(then),
            )
        })
    }

    fn disk_bulk_write_import_start<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::DiskBulkWriteImportStartWhen,
            operations::DiskBulkWriteImportStartThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DiskBulkWriteImportStartWhen::new(when),
                operations::DiskBulkWriteImportStartThen::new(then),
            )
        })
    }

    fn disk_bulk_write_import_stop<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DiskBulkWriteImportStopWhen, operations::DiskBulkWriteImportStopThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DiskBulkWriteImportStopWhen::new(when),
                operations::DiskBulkWriteImportStopThen::new(then),
            )
        })
    }

    fn disk_finalize_import<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DiskFinalizeImportWhen, operations::DiskFinalizeImportThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DiskFinalizeImportWhen::new(when),
                operations::DiskFinalizeImportThen::new(then),
            )
        })
    }

    fn disk_import_blocks_from_url<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DiskImportBlocksFromUrlWhen, operations::DiskImportBlocksFromUrlThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DiskImportBlocksFromUrlWhen::new(when),
                operations::DiskImportBlocksFromUrlThen::new(then),
            )
        })
    }

    fn disk_metrics_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::DiskMetricsListWhen, operations::DiskMetricsListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::DiskMetricsListWhen::new(when),
                operations::DiskMetricsListThen::new(then),
            )
        })
    }

    fn group_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::GroupListWhen, operations::GroupListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::GroupListWhen::new(when),
                operations::GroupListThen::new(then),
            )
        })
    }

    fn group_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::GroupViewWhen, operations::GroupViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::GroupViewWhen::new(when),
                operations::GroupViewThen::new(then),
            )
        })
    }

    fn image_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ImageListWhen, operations::ImageListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ImageListWhen::new(when),
                operations::ImageListThen::new(then),
            )
        })
    }

    fn image_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ImageCreateWhen, operations::ImageCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ImageCreateWhen::new(when),
                operations::ImageCreateThen::new(then),
            )
        })
    }

    fn image_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ImageViewWhen, operations::ImageViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ImageViewWhen::new(when),
                operations::ImageViewThen::new(then),
            )
        })
    }

    fn image_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ImageDeleteWhen, operations::ImageDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ImageDeleteWhen::new(when),
                operations::ImageDeleteThen::new(then),
            )
        })
    }

    fn image_demote<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ImageDemoteWhen, operations::ImageDemoteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ImageDemoteWhen::new(when),
                operations::ImageDemoteThen::new(then),
            )
        })
    }

    fn image_promote<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ImagePromoteWhen, operations::ImagePromoteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ImagePromoteWhen::new(when),
                operations::ImagePromoteThen::new(then),
            )
        })
    }

    fn instance_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceListWhen, operations::InstanceListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceListWhen::new(when),
                operations::InstanceListThen::new(then),
            )
        })
    }

    fn instance_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceCreateWhen, operations::InstanceCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceCreateWhen::new(when),
                operations::InstanceCreateThen::new(then),
            )
        })
    }

    fn instance_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceViewWhen, operations::InstanceViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceViewWhen::new(when),
                operations::InstanceViewThen::new(then),
            )
        })
    }

    fn instance_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceDeleteWhen, operations::InstanceDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceDeleteWhen::new(when),
                operations::InstanceDeleteThen::new(then),
            )
        })
    }

    fn instance_disk_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceDiskListWhen, operations::InstanceDiskListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceDiskListWhen::new(when),
                operations::InstanceDiskListThen::new(then),
            )
        })
    }

    fn instance_disk_attach<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceDiskAttachWhen, operations::InstanceDiskAttachThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceDiskAttachWhen::new(when),
                operations::InstanceDiskAttachThen::new(then),
            )
        })
    }

    fn instance_disk_detach<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceDiskDetachWhen, operations::InstanceDiskDetachThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceDiskDetachWhen::new(when),
                operations::InstanceDiskDetachThen::new(then),
            )
        })
    }

    fn instance_external_ip_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceExternalIpListWhen, operations::InstanceExternalIpListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceExternalIpListWhen::new(when),
                operations::InstanceExternalIpListThen::new(then),
            )
        })
    }

    fn instance_migrate<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceMigrateWhen, operations::InstanceMigrateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceMigrateWhen::new(when),
                operations::InstanceMigrateThen::new(then),
            )
        })
    }

    fn instance_reboot<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceRebootWhen, operations::InstanceRebootThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceRebootWhen::new(when),
                operations::InstanceRebootThen::new(then),
            )
        })
    }

    fn instance_serial_console<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceSerialConsoleWhen, operations::InstanceSerialConsoleThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceSerialConsoleWhen::new(when),
                operations::InstanceSerialConsoleThen::new(then),
            )
        })
    }

    fn instance_serial_console_stream<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceSerialConsoleStreamWhen,
            operations::InstanceSerialConsoleStreamThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceSerialConsoleStreamWhen::new(when),
                operations::InstanceSerialConsoleStreamThen::new(then),
            )
        })
    }

    fn instance_start<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceStartWhen, operations::InstanceStartThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceStartWhen::new(when),
                operations::InstanceStartThen::new(then),
            )
        })
    }

    fn instance_stop<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::InstanceStopWhen, operations::InstanceStopThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceStopWhen::new(when),
                operations::InstanceStopThen::new(then),
            )
        })
    }

    fn login_local<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::LoginLocalWhen, operations::LoginLocalThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::LoginLocalWhen::new(when),
                operations::LoginLocalThen::new(then),
            )
        })
    }

    fn logout<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::LogoutWhen, operations::LogoutThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::LogoutWhen::new(when),
                operations::LogoutThen::new(then),
            )
        })
    }

    fn current_user_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CurrentUserViewWhen, operations::CurrentUserViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::CurrentUserViewWhen::new(when),
                operations::CurrentUserViewThen::new(then),
            )
        })
    }

    fn current_user_groups<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CurrentUserGroupsWhen, operations::CurrentUserGroupsThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::CurrentUserGroupsWhen::new(when),
                operations::CurrentUserGroupsThen::new(then),
            )
        })
    }

    fn current_user_ssh_key_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CurrentUserSshKeyListWhen, operations::CurrentUserSshKeyListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::CurrentUserSshKeyListWhen::new(when),
                operations::CurrentUserSshKeyListThen::new(then),
            )
        })
    }

    fn current_user_ssh_key_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CurrentUserSshKeyCreateWhen, operations::CurrentUserSshKeyCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::CurrentUserSshKeyCreateWhen::new(when),
                operations::CurrentUserSshKeyCreateThen::new(then),
            )
        })
    }

    fn current_user_ssh_key_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CurrentUserSshKeyViewWhen, operations::CurrentUserSshKeyViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::CurrentUserSshKeyViewWhen::new(when),
                operations::CurrentUserSshKeyViewThen::new(then),
            )
        })
    }

    fn current_user_ssh_key_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::CurrentUserSshKeyDeleteWhen, operations::CurrentUserSshKeyDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::CurrentUserSshKeyDeleteWhen::new(when),
                operations::CurrentUserSshKeyDeleteThen::new(then),
            )
        })
    }

    fn instance_network_interface_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceListWhen,
            operations::InstanceNetworkInterfaceListThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceNetworkInterfaceListWhen::new(when),
                operations::InstanceNetworkInterfaceListThen::new(then),
            )
        })
    }

    fn instance_network_interface_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceCreateWhen,
            operations::InstanceNetworkInterfaceCreateThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceNetworkInterfaceCreateWhen::new(when),
                operations::InstanceNetworkInterfaceCreateThen::new(then),
            )
        })
    }

    fn instance_network_interface_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceViewWhen,
            operations::InstanceNetworkInterfaceViewThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceNetworkInterfaceViewWhen::new(when),
                operations::InstanceNetworkInterfaceViewThen::new(then),
            )
        })
    }

    fn instance_network_interface_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceUpdateWhen,
            operations::InstanceNetworkInterfaceUpdateThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceNetworkInterfaceUpdateWhen::new(when),
                operations::InstanceNetworkInterfaceUpdateThen::new(then),
            )
        })
    }

    fn instance_network_interface_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceDeleteWhen,
            operations::InstanceNetworkInterfaceDeleteThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceNetworkInterfaceDeleteWhen::new(when),
                operations::InstanceNetworkInterfaceDeleteThen::new(then),
            )
        })
    }

    fn policy_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::PolicyViewWhen, operations::PolicyViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::PolicyViewWhen::new(when),
                operations::PolicyViewThen::new(then),
            )
        })
    }

    fn policy_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::PolicyUpdateWhen, operations::PolicyUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::PolicyUpdateWhen::new(when),
                operations::PolicyUpdateThen::new(then),
            )
        })
    }

    fn project_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ProjectListWhen, operations::ProjectListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectListWhen::new(when),
                operations::ProjectListThen::new(then),
            )
        })
    }

    fn project_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ProjectCreateWhen, operations::ProjectCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectCreateWhen::new(when),
                operations::ProjectCreateThen::new(then),
            )
        })
    }

    fn project_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ProjectViewWhen, operations::ProjectViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectViewWhen::new(when),
                operations::ProjectViewThen::new(then),
            )
        })
    }

    fn project_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ProjectUpdateWhen, operations::ProjectUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectUpdateWhen::new(when),
                operations::ProjectUpdateThen::new(then),
            )
        })
    }

    fn project_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ProjectDeleteWhen, operations::ProjectDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectDeleteWhen::new(when),
                operations::ProjectDeleteThen::new(then),
            )
        })
    }

    fn project_policy_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ProjectPolicyViewWhen, operations::ProjectPolicyViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectPolicyViewWhen::new(when),
                operations::ProjectPolicyViewThen::new(then),
            )
        })
    }

    fn project_policy_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::ProjectPolicyUpdateWhen, operations::ProjectPolicyUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectPolicyUpdateWhen::new(when),
                operations::ProjectPolicyUpdateThen::new(then),
            )
        })
    }

    fn snapshot_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SnapshotListWhen, operations::SnapshotListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SnapshotListWhen::new(when),
                operations::SnapshotListThen::new(then),
            )
        })
    }

    fn snapshot_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SnapshotCreateWhen, operations::SnapshotCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SnapshotCreateWhen::new(when),
                operations::SnapshotCreateThen::new(then),
            )
        })
    }

    fn snapshot_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SnapshotViewWhen, operations::SnapshotViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SnapshotViewWhen::new(when),
                operations::SnapshotViewThen::new(then),
            )
        })
    }

    fn snapshot_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SnapshotDeleteWhen, operations::SnapshotDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SnapshotDeleteWhen::new(when),
                operations::SnapshotDeleteThen::new(then),
            )
        })
    }

    fn physical_disk_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::PhysicalDiskListWhen, operations::PhysicalDiskListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::PhysicalDiskListWhen::new(when),
                operations::PhysicalDiskListThen::new(then),
            )
        })
    }

    fn rack_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::RackListWhen, operations::RackListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::RackListWhen::new(when),
                operations::RackListThen::new(then),
            )
        })
    }

    fn rack_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::RackViewWhen, operations::RackViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::RackViewWhen::new(when),
                operations::RackViewThen::new(then),
            )
        })
    }

    fn sled_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SledListWhen, operations::SledListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SledListWhen::new(when),
                operations::SledListThen::new(then),
            )
        })
    }

    fn sled_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SledViewWhen, operations::SledViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SledViewWhen::new(when),
                operations::SledViewThen::new(then),
            )
        })
    }

    fn sled_physical_disk_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SledPhysicalDiskListWhen, operations::SledPhysicalDiskListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SledPhysicalDiskListWhen::new(when),
                operations::SledPhysicalDiskListThen::new(then),
            )
        })
    }

    fn sled_instance_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SledInstanceListWhen, operations::SledInstanceListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SledInstanceListWhen::new(when),
                operations::SledInstanceListThen::new(then),
            )
        })
    }

    fn networking_switch_port_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortListWhen,
            operations::NetworkingSwitchPortListThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingSwitchPortListWhen::new(when),
                operations::NetworkingSwitchPortListThen::new(then),
            )
        })
    }

    fn networking_switch_port_apply_settings<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortApplySettingsWhen,
            operations::NetworkingSwitchPortApplySettingsThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingSwitchPortApplySettingsWhen::new(when),
                operations::NetworkingSwitchPortApplySettingsThen::new(then),
            )
        })
    }

    fn networking_switch_port_clear_settings<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortClearSettingsWhen,
            operations::NetworkingSwitchPortClearSettingsThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingSwitchPortClearSettingsWhen::new(when),
                operations::NetworkingSwitchPortClearSettingsThen::new(then),
            )
        })
    }

    fn switch_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SwitchListWhen, operations::SwitchListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SwitchListWhen::new(when),
                operations::SwitchListThen::new(then),
            )
        })
    }

    fn switch_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SwitchViewWhen, operations::SwitchViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SwitchViewWhen::new(when),
                operations::SwitchViewThen::new(then),
            )
        })
    }

    fn silo_identity_provider_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::SiloIdentityProviderListWhen,
            operations::SiloIdentityProviderListThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloIdentityProviderListWhen::new(when),
                operations::SiloIdentityProviderListThen::new(then),
            )
        })
    }

    fn local_idp_user_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::LocalIdpUserCreateWhen, operations::LocalIdpUserCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::LocalIdpUserCreateWhen::new(when),
                operations::LocalIdpUserCreateThen::new(then),
            )
        })
    }

    fn local_idp_user_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::LocalIdpUserDeleteWhen, operations::LocalIdpUserDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::LocalIdpUserDeleteWhen::new(when),
                operations::LocalIdpUserDeleteThen::new(then),
            )
        })
    }

    fn local_idp_user_set_password<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::LocalIdpUserSetPasswordWhen, operations::LocalIdpUserSetPasswordThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::LocalIdpUserSetPasswordWhen::new(when),
                operations::LocalIdpUserSetPasswordThen::new(then),
            )
        })
    }

    fn saml_identity_provider_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::SamlIdentityProviderCreateWhen,
            operations::SamlIdentityProviderCreateThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SamlIdentityProviderCreateWhen::new(when),
                operations::SamlIdentityProviderCreateThen::new(then),
            )
        })
    }

    fn saml_identity_provider_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::SamlIdentityProviderViewWhen,
            operations::SamlIdentityProviderViewThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SamlIdentityProviderViewWhen::new(when),
                operations::SamlIdentityProviderViewThen::new(then),
            )
        })
    }

    fn ip_pool_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::IpPoolListWhen, operations::IpPoolListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolListWhen::new(when),
                operations::IpPoolListThen::new(then),
            )
        })
    }

    fn ip_pool_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::IpPoolCreateWhen, operations::IpPoolCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolCreateWhen::new(when),
                operations::IpPoolCreateThen::new(then),
            )
        })
    }

    fn ip_pool_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::IpPoolViewWhen, operations::IpPoolViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolViewWhen::new(when),
                operations::IpPoolViewThen::new(then),
            )
        })
    }

    fn ip_pool_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::IpPoolUpdateWhen, operations::IpPoolUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolUpdateWhen::new(when),
                operations::IpPoolUpdateThen::new(then),
            )
        })
    }

    fn ip_pool_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::IpPoolDeleteWhen, operations::IpPoolDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolDeleteWhen::new(when),
                operations::IpPoolDeleteThen::new(then),
            )
        })
    }

    fn ip_pool_range_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::IpPoolRangeListWhen, operations::IpPoolRangeListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolRangeListWhen::new(when),
                operations::IpPoolRangeListThen::new(then),
            )
        })
    }

    fn ip_pool_range_add<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::IpPoolRangeAddWhen, operations::IpPoolRangeAddThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolRangeAddWhen::new(when),
                operations::IpPoolRangeAddThen::new(then),
            )
        })
    }

    fn ip_pool_range_remove<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::IpPoolRangeRemoveWhen, operations::IpPoolRangeRemoveThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolRangeRemoveWhen::new(when),
                operations::IpPoolRangeRemoveThen::new(then),
            )
        })
    }

    fn ip_pool_service_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::IpPoolServiceViewWhen, operations::IpPoolServiceViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolServiceViewWhen::new(when),
                operations::IpPoolServiceViewThen::new(then),
            )
        })
    }

    fn ip_pool_service_range_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::IpPoolServiceRangeListWhen, operations::IpPoolServiceRangeListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolServiceRangeListWhen::new(when),
                operations::IpPoolServiceRangeListThen::new(then),
            )
        })
    }

    fn ip_pool_service_range_add<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::IpPoolServiceRangeAddWhen, operations::IpPoolServiceRangeAddThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolServiceRangeAddWhen::new(when),
                operations::IpPoolServiceRangeAddThen::new(then),
            )
        })
    }

    fn ip_pool_service_range_remove<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::IpPoolServiceRangeRemoveWhen,
            operations::IpPoolServiceRangeRemoveThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolServiceRangeRemoveWhen::new(when),
                operations::IpPoolServiceRangeRemoveThen::new(then),
            )
        })
    }

    fn system_metric<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SystemMetricWhen, operations::SystemMetricThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemMetricWhen::new(when),
                operations::SystemMetricThen::new(then),
            )
        })
    }

    fn networking_address_lot_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingAddressLotListWhen,
            operations::NetworkingAddressLotListThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingAddressLotListWhen::new(when),
                operations::NetworkingAddressLotListThen::new(then),
            )
        })
    }

    fn networking_address_lot_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingAddressLotCreateWhen,
            operations::NetworkingAddressLotCreateThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingAddressLotCreateWhen::new(when),
                operations::NetworkingAddressLotCreateThen::new(then),
            )
        })
    }

    fn networking_address_lot_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingAddressLotDeleteWhen,
            operations::NetworkingAddressLotDeleteThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingAddressLotDeleteWhen::new(when),
                operations::NetworkingAddressLotDeleteThen::new(then),
            )
        })
    }

    fn networking_address_lot_block_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingAddressLotBlockListWhen,
            operations::NetworkingAddressLotBlockListThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingAddressLotBlockListWhen::new(when),
                operations::NetworkingAddressLotBlockListThen::new(then),
            )
        })
    }

    fn networking_loopback_address_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingLoopbackAddressListWhen,
            operations::NetworkingLoopbackAddressListThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingLoopbackAddressListWhen::new(when),
                operations::NetworkingLoopbackAddressListThen::new(then),
            )
        })
    }

    fn networking_loopback_address_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingLoopbackAddressCreateWhen,
            operations::NetworkingLoopbackAddressCreateThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingLoopbackAddressCreateWhen::new(when),
                operations::NetworkingLoopbackAddressCreateThen::new(then),
            )
        })
    }

    fn networking_loopback_address_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingLoopbackAddressDeleteWhen,
            operations::NetworkingLoopbackAddressDeleteThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingLoopbackAddressDeleteWhen::new(when),
                operations::NetworkingLoopbackAddressDeleteThen::new(then),
            )
        })
    }

    fn networking_switch_port_settings_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortSettingsListWhen,
            operations::NetworkingSwitchPortSettingsListThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingSwitchPortSettingsListWhen::new(when),
                operations::NetworkingSwitchPortSettingsListThen::new(then),
            )
        })
    }

    fn networking_switch_port_settings_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortSettingsCreateWhen,
            operations::NetworkingSwitchPortSettingsCreateThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingSwitchPortSettingsCreateWhen::new(when),
                operations::NetworkingSwitchPortSettingsCreateThen::new(then),
            )
        })
    }

    fn networking_switch_port_settings_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortSettingsDeleteWhen,
            operations::NetworkingSwitchPortSettingsDeleteThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingSwitchPortSettingsDeleteWhen::new(when),
                operations::NetworkingSwitchPortSettingsDeleteThen::new(then),
            )
        })
    }

    fn networking_switch_port_settings_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortSettingsViewWhen,
            operations::NetworkingSwitchPortSettingsViewThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingSwitchPortSettingsViewWhen::new(when),
                operations::NetworkingSwitchPortSettingsViewThen::new(then),
            )
        })
    }

    fn system_policy_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SystemPolicyViewWhen, operations::SystemPolicyViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemPolicyViewWhen::new(when),
                operations::SystemPolicyViewThen::new(then),
            )
        })
    }

    fn system_policy_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SystemPolicyUpdateWhen, operations::SystemPolicyUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemPolicyUpdateWhen::new(when),
                operations::SystemPolicyUpdateThen::new(then),
            )
        })
    }

    fn role_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::RoleListWhen, operations::RoleListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::RoleListWhen::new(when),
                operations::RoleListThen::new(then),
            )
        })
    }

    fn role_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::RoleViewWhen, operations::RoleViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::RoleViewWhen::new(when),
                operations::RoleViewThen::new(then),
            )
        })
    }

    fn silo_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SiloListWhen, operations::SiloListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloListWhen::new(when),
                operations::SiloListThen::new(then),
            )
        })
    }

    fn silo_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SiloCreateWhen, operations::SiloCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloCreateWhen::new(when),
                operations::SiloCreateThen::new(then),
            )
        })
    }

    fn silo_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SiloViewWhen, operations::SiloViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloViewWhen::new(when),
                operations::SiloViewThen::new(then),
            )
        })
    }

    fn silo_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SiloDeleteWhen, operations::SiloDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloDeleteWhen::new(when),
                operations::SiloDeleteThen::new(then),
            )
        })
    }

    fn silo_policy_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SiloPolicyViewWhen, operations::SiloPolicyViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloPolicyViewWhen::new(when),
                operations::SiloPolicyViewThen::new(then),
            )
        })
    }

    fn silo_policy_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SiloPolicyUpdateWhen, operations::SiloPolicyUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloPolicyUpdateWhen::new(when),
                operations::SiloPolicyUpdateThen::new(then),
            )
        })
    }

    fn system_component_version_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::SystemComponentVersionListWhen,
            operations::SystemComponentVersionListThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemComponentVersionListWhen::new(when),
                operations::SystemComponentVersionListThen::new(then),
            )
        })
    }

    fn update_deployments_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::UpdateDeploymentsListWhen, operations::UpdateDeploymentsListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::UpdateDeploymentsListWhen::new(when),
                operations::UpdateDeploymentsListThen::new(then),
            )
        })
    }

    fn update_deployment_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::UpdateDeploymentViewWhen, operations::UpdateDeploymentViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::UpdateDeploymentViewWhen::new(when),
                operations::UpdateDeploymentViewThen::new(then),
            )
        })
    }

    fn system_update_refresh<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SystemUpdateRefreshWhen, operations::SystemUpdateRefreshThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemUpdateRefreshWhen::new(when),
                operations::SystemUpdateRefreshThen::new(then),
            )
        })
    }

    fn system_update_start<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SystemUpdateStartWhen, operations::SystemUpdateStartThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemUpdateStartWhen::new(when),
                operations::SystemUpdateStartThen::new(then),
            )
        })
    }

    fn system_update_stop<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SystemUpdateStopWhen, operations::SystemUpdateStopThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemUpdateStopWhen::new(when),
                operations::SystemUpdateStopThen::new(then),
            )
        })
    }

    fn system_update_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SystemUpdateListWhen, operations::SystemUpdateListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemUpdateListWhen::new(when),
                operations::SystemUpdateListThen::new(then),
            )
        })
    }

    fn system_update_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SystemUpdateViewWhen, operations::SystemUpdateViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemUpdateViewWhen::new(when),
                operations::SystemUpdateViewThen::new(then),
            )
        })
    }

    fn system_update_components_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::SystemUpdateComponentsListWhen,
            operations::SystemUpdateComponentsListThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemUpdateComponentsListWhen::new(when),
                operations::SystemUpdateComponentsListThen::new(then),
            )
        })
    }

    fn system_version<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SystemVersionWhen, operations::SystemVersionThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemVersionWhen::new(when),
                operations::SystemVersionThen::new(then),
            )
        })
    }

    fn silo_user_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SiloUserListWhen, operations::SiloUserListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloUserListWhen::new(when),
                operations::SiloUserListThen::new(then),
            )
        })
    }

    fn silo_user_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::SiloUserViewWhen, operations::SiloUserViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloUserViewWhen::new(when),
                operations::SiloUserViewThen::new(then),
            )
        })
    }

    fn user_builtin_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::UserBuiltinListWhen, operations::UserBuiltinListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::UserBuiltinListWhen::new(when),
                operations::UserBuiltinListThen::new(then),
            )
        })
    }

    fn user_builtin_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::UserBuiltinViewWhen, operations::UserBuiltinViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::UserBuiltinViewWhen::new(when),
                operations::UserBuiltinViewThen::new(then),
            )
        })
    }

    fn user_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::UserListWhen, operations::UserListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::UserListWhen::new(when),
                operations::UserListThen::new(then),
            )
        })
    }

    fn vpc_firewall_rules_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcFirewallRulesViewWhen, operations::VpcFirewallRulesViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcFirewallRulesViewWhen::new(when),
                operations::VpcFirewallRulesViewThen::new(then),
            )
        })
    }

    fn vpc_firewall_rules_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcFirewallRulesUpdateWhen, operations::VpcFirewallRulesUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcFirewallRulesUpdateWhen::new(when),
                operations::VpcFirewallRulesUpdateThen::new(then),
            )
        })
    }

    fn vpc_router_route_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterRouteListWhen, operations::VpcRouterRouteListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcRouterRouteListWhen::new(when),
                operations::VpcRouterRouteListThen::new(then),
            )
        })
    }

    fn vpc_router_route_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterRouteCreateWhen, operations::VpcRouterRouteCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcRouterRouteCreateWhen::new(when),
                operations::VpcRouterRouteCreateThen::new(then),
            )
        })
    }

    fn vpc_router_route_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterRouteViewWhen, operations::VpcRouterRouteViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcRouterRouteViewWhen::new(when),
                operations::VpcRouterRouteViewThen::new(then),
            )
        })
    }

    fn vpc_router_route_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterRouteUpdateWhen, operations::VpcRouterRouteUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcRouterRouteUpdateWhen::new(when),
                operations::VpcRouterRouteUpdateThen::new(then),
            )
        })
    }

    fn vpc_router_route_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterRouteDeleteWhen, operations::VpcRouterRouteDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcRouterRouteDeleteWhen::new(when),
                operations::VpcRouterRouteDeleteThen::new(then),
            )
        })
    }

    fn vpc_router_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterListWhen, operations::VpcRouterListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcRouterListWhen::new(when),
                operations::VpcRouterListThen::new(then),
            )
        })
    }

    fn vpc_router_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterCreateWhen, operations::VpcRouterCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcRouterCreateWhen::new(when),
                operations::VpcRouterCreateThen::new(then),
            )
        })
    }

    fn vpc_router_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterViewWhen, operations::VpcRouterViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcRouterViewWhen::new(when),
                operations::VpcRouterViewThen::new(then),
            )
        })
    }

    fn vpc_router_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterUpdateWhen, operations::VpcRouterUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcRouterUpdateWhen::new(when),
                operations::VpcRouterUpdateThen::new(then),
            )
        })
    }

    fn vpc_router_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterDeleteWhen, operations::VpcRouterDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcRouterDeleteWhen::new(when),
                operations::VpcRouterDeleteThen::new(then),
            )
        })
    }

    fn vpc_subnet_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcSubnetListWhen, operations::VpcSubnetListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcSubnetListWhen::new(when),
                operations::VpcSubnetListThen::new(then),
            )
        })
    }

    fn vpc_subnet_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcSubnetCreateWhen, operations::VpcSubnetCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcSubnetCreateWhen::new(when),
                operations::VpcSubnetCreateThen::new(then),
            )
        })
    }

    fn vpc_subnet_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcSubnetViewWhen, operations::VpcSubnetViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcSubnetViewWhen::new(when),
                operations::VpcSubnetViewThen::new(then),
            )
        })
    }

    fn vpc_subnet_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcSubnetUpdateWhen, operations::VpcSubnetUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcSubnetUpdateWhen::new(when),
                operations::VpcSubnetUpdateThen::new(then),
            )
        })
    }

    fn vpc_subnet_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcSubnetDeleteWhen, operations::VpcSubnetDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcSubnetDeleteWhen::new(when),
                operations::VpcSubnetDeleteThen::new(then),
            )
        })
    }

    fn vpc_subnet_list_network_interfaces<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(
            operations::VpcSubnetListNetworkInterfacesWhen,
            operations::VpcSubnetListNetworkInterfacesThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcSubnetListNetworkInterfacesWhen::new(when),
                operations::VpcSubnetListNetworkInterfacesThen::new(then),
            )
        })
    }

    fn vpc_list<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcListWhen, operations::VpcListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcListWhen::new(when),
                operations::VpcListThen::new(then),
            )
        })
    }

    fn vpc_create<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcCreateWhen, operations::VpcCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcCreateWhen::new(when),
                operations::VpcCreateThen::new(then),
            )
        })
    }

    fn vpc_view<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcViewWhen, operations::VpcViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcViewWhen::new(when),
                operations::VpcViewThen::new(then),
            )
        })
    }

    fn vpc_update<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcUpdateWhen, operations::VpcUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcUpdateWhen::new(when),
                operations::VpcUpdateThen::new(then),
            )
        })
    }

    fn vpc_delete<F>(&self, config_fn: F) -> httpmock::Mock
    where
        F: FnOnce(operations::VpcDeleteWhen, operations::VpcDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::VpcDeleteWhen::new(when),
                operations::VpcDeleteThen::new(then),
            )
        })
    }
}
