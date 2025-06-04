// The contents of this file are generated; do not modify them.

pub mod operations {
    //! [`When`](::httpmock::When) and [`Then`](::httpmock::Then)
    //! wrappers for each operation. Each can be converted to
    //! its inner type with a call to `into_inner()`. This can
    //! be used to explicitly deviate from permitted values.
    use oxide::*;
    pub struct DeviceAuthRequestWhen(::httpmock::When);
    impl DeviceAuthRequestWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/device/auth$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::DeviceAuthRequest) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct DeviceAuthRequestThen(::httpmock::Then);
    impl DeviceAuthRequestThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn default_response(self, status: u16, value: ::serde_json::Value) -> Self {
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body(value),
            )
        }
    }

    pub struct DeviceAuthConfirmWhen(::httpmock::When);
    impl DeviceAuthConfirmWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/device/confirm$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::DeviceAuthVerify) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct DeviceAuthConfirmThen(::httpmock::Then);
    impl DeviceAuthConfirmThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct DeviceAccessTokenWhen(::httpmock::When);
    impl DeviceAccessTokenWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/device/token$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::DeviceAccessTokenRequest) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct DeviceAccessTokenThen(::httpmock::Then);
    impl DeviceAccessTokenThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn default_response(self, status: u16, value: ::serde_json::Value) -> Self {
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body(value),
            )
        }
    }

    pub struct ProbeListWhen(::httpmock::When);
    impl ProbeListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/experimental/v1/probes$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct ProbeListThen(::httpmock::Then);
    impl ProbeListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::ProbeInfoResultsPage) -> Self {
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

    pub struct ProbeCreateWhen(::httpmock::When);
    impl ProbeCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/experimental/v1/probes$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("project", value.to_string()))
        }

        pub fn body(self, value: &types::ProbeCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct ProbeCreateThen(::httpmock::Then);
    impl ProbeCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::Probe) -> Self {
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

    pub struct ProbeViewWhen(::httpmock::When);
    impl ProbeViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/experimental/v1/probes/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn probe(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/experimental/v1/probes/{}$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("project", value.to_string()))
        }
    }

    pub struct ProbeViewThen(::httpmock::Then);
    impl ProbeViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::ProbeInfo) -> Self {
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

    pub struct ProbeDeleteWhen(::httpmock::When);
    impl ProbeDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/experimental/v1/probes/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn probe(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/experimental/v1/probes/{}$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("project", value.to_string()))
        }
    }

    pub struct ProbeDeleteThen(::httpmock::Then);
    impl ProbeDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SupportBundleListWhen(::httpmock::When);
    impl SupportBundleListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/experimental/v1/system/support-bundles$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct SupportBundleListThen(::httpmock::Then);
    impl SupportBundleListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SupportBundleInfoResultsPage) -> Self {
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

    pub struct SupportBundleCreateWhen(::httpmock::When);
    impl SupportBundleCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::POST).path_matches(
                regex::Regex::new("^/experimental/v1/system/support-bundles$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct SupportBundleCreateThen(::httpmock::Then);
    impl SupportBundleCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::SupportBundleInfo) -> Self {
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

    pub struct SupportBundleViewWhen(::httpmock::When);
    impl SupportBundleViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/experimental/v1/system/support-bundles/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn bundle_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/experimental/v1/system/support-bundles/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SupportBundleViewThen(::httpmock::Then);
    impl SupportBundleViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SupportBundleInfo) -> Self {
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

    pub struct SupportBundleDeleteWhen(::httpmock::When);
    impl SupportBundleDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::DELETE).path_matches(
                regex::Regex::new("^/experimental/v1/system/support-bundles/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn bundle_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/experimental/v1/system/support-bundles/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SupportBundleDeleteThen(::httpmock::Then);
    impl SupportBundleDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SupportBundleDownloadWhen(::httpmock::When);
    impl SupportBundleDownloadWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/experimental/v1/system/support-bundles/[^/]*/download$")
                        .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn bundle_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/experimental/v1/system/support-bundles/{}/download$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn range<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.header("range", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.headers
                        .as_ref()
                        .and_then(|hs| hs.iter().find(|(key, _)| key == "range"))
                        .is_none()
                }))
            }
        }
    }

    pub struct SupportBundleDownloadThen(::httpmock::Then);
    impl SupportBundleDownloadThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn default_response(self, status: u16, value: ::serde_json::Value) -> Self {
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body(value),
            )
        }
    }

    pub struct SupportBundleHeadWhen(::httpmock::When);
    impl SupportBundleHeadWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::HEAD).path_matches(
                    regex::Regex::new("^/experimental/v1/system/support-bundles/[^/]*/download$")
                        .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn bundle_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/experimental/v1/system/support-bundles/{}/download$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn range<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.header("range", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.headers
                        .as_ref()
                        .and_then(|hs| hs.iter().find(|(key, _)| key == "range"))
                        .is_none()
                }))
            }
        }
    }

    pub struct SupportBundleHeadThen(::httpmock::Then);
    impl SupportBundleHeadThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn default_response(self, status: u16, value: ::serde_json::Value) -> Self {
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body(value),
            )
        }
    }

    pub struct SupportBundleDownloadFileWhen(::httpmock::When);
    impl SupportBundleDownloadFileWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new(
                        "^/experimental/v1/system/support-bundles/[^/]*/download/[^/]*$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn bundle_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/experimental/v1/system/support-bundles/{}/download/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn file(self, value: &str) -> Self {
            let re = regex::Regex::new(&format!(
                "^/experimental/v1/system/support-bundles/.*/download/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn range<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.header("range", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.headers
                        .as_ref()
                        .and_then(|hs| hs.iter().find(|(key, _)| key == "range"))
                        .is_none()
                }))
            }
        }
    }

    pub struct SupportBundleDownloadFileThen(::httpmock::Then);
    impl SupportBundleDownloadFileThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn default_response(self, status: u16, value: ::serde_json::Value) -> Self {
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body(value),
            )
        }
    }

    pub struct SupportBundleHeadFileWhen(::httpmock::When);
    impl SupportBundleHeadFileWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::HEAD).path_matches(
                    regex::Regex::new(
                        "^/experimental/v1/system/support-bundles/[^/]*/download/[^/]*$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn bundle_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/experimental/v1/system/support-bundles/{}/download/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn file(self, value: &str) -> Self {
            let re = regex::Regex::new(&format!(
                "^/experimental/v1/system/support-bundles/.*/download/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn range<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.header("range", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.headers
                        .as_ref()
                        .and_then(|hs| hs.iter().find(|(key, _)| key == "range"))
                        .is_none()
                }))
            }
        }
    }

    pub struct SupportBundleHeadFileThen(::httpmock::Then);
    impl SupportBundleHeadFileThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn default_response(self, status: u16, value: ::serde_json::Value) -> Self {
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body(value),
            )
        }
    }

    pub struct SupportBundleIndexWhen(::httpmock::When);
    impl SupportBundleIndexWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/experimental/v1/system/support-bundles/[^/]*/index$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn bundle_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/experimental/v1/system/support-bundles/{}/index$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn range<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a str>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.header("range", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.headers
                        .as_ref()
                        .and_then(|hs| hs.iter().find(|(key, _)| key == "range"))
                        .is_none()
                }))
            }
        }
    }

    pub struct SupportBundleIndexThen(::httpmock::Then);
    impl SupportBundleIndexThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn default_response(self, status: u16, value: ::serde_json::Value) -> Self {
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body(value),
            )
        }
    }

    pub struct LoginSamlWhen(::httpmock::When);
    impl LoginSamlWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/login/[^/]*/saml/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

        pub fn body(self, value: ::serde_json::Value) -> Self {
            Self(self.0.json_body(value))
        }
    }

    pub struct LoginSamlThen(::httpmock::Then);
    impl LoginSamlThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

        pub fn success(self, status: u16, value: ::serde_json::Value) -> Self {
            assert_eq!(status / 100u16, 2u16);
            Self(
                self.0
                    .status(status)
                    .header("content-type", "application/json")
                    .json_body(value),
            )
        }
    }

    pub struct AffinityGroupListWhen(::httpmock::When);
    impl AffinityGroupListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/affinity-groups$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct AffinityGroupListThen(::httpmock::Then);
    impl AffinityGroupListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::AffinityGroupResultsPage) -> Self {
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

    pub struct AffinityGroupCreateWhen(::httpmock::When);
    impl AffinityGroupCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/affinity-groups$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("project", value.to_string()))
        }

        pub fn body(self, value: &types::AffinityGroupCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct AffinityGroupCreateThen(::httpmock::Then);
    impl AffinityGroupCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::AffinityGroup) -> Self {
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

    pub struct AffinityGroupViewWhen(::httpmock::When);
    impl AffinityGroupViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/affinity-groups/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn affinity_group(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/affinity-groups/{}$", value.to_string())).unwrap();
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

    pub struct AffinityGroupViewThen(::httpmock::Then);
    impl AffinityGroupViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::AffinityGroup) -> Self {
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

    pub struct AffinityGroupUpdateWhen(::httpmock::When);
    impl AffinityGroupUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/affinity-groups/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn affinity_group(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/affinity-groups/{}$", value.to_string())).unwrap();
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

        pub fn body(self, value: &types::AffinityGroupUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct AffinityGroupUpdateThen(::httpmock::Then);
    impl AffinityGroupUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::AffinityGroup) -> Self {
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

    pub struct AffinityGroupDeleteWhen(::httpmock::When);
    impl AffinityGroupDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/affinity-groups/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn affinity_group(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/affinity-groups/{}$", value.to_string())).unwrap();
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

    pub struct AffinityGroupDeleteThen(::httpmock::Then);
    impl AffinityGroupDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct AffinityGroupMemberListWhen(::httpmock::When);
    impl AffinityGroupMemberListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/v1/affinity-groups/[^/]*/members$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn affinity_group(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/affinity-groups/{}/members$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct AffinityGroupMemberListThen(::httpmock::Then);
    impl AffinityGroupMemberListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::AffinityGroupMemberResultsPage) -> Self {
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

    pub struct AffinityGroupMemberInstanceViewWhen(::httpmock::When);
    impl AffinityGroupMemberInstanceViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/affinity-groups/[^/]*/members/instance/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn affinity_group(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/affinity-groups/{}/members/instance/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/affinity-groups/.*/members/instance/{}$",
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

    pub struct AffinityGroupMemberInstanceViewThen(::httpmock::Then);
    impl AffinityGroupMemberInstanceViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::AffinityGroupMember) -> Self {
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

    pub struct AffinityGroupMemberInstanceAddWhen(::httpmock::When);
    impl AffinityGroupMemberInstanceAddWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::POST).path_matches(
                regex::Regex::new("^/v1/affinity-groups/[^/]*/members/instance/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn affinity_group(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/affinity-groups/{}/members/instance/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/affinity-groups/.*/members/instance/{}$",
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

    pub struct AffinityGroupMemberInstanceAddThen(::httpmock::Then);
    impl AffinityGroupMemberInstanceAddThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::AffinityGroupMember) -> Self {
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

    pub struct AffinityGroupMemberInstanceDeleteWhen(::httpmock::When);
    impl AffinityGroupMemberInstanceDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::DELETE).path_matches(
                regex::Regex::new("^/v1/affinity-groups/[^/]*/members/instance/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn affinity_group(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/affinity-groups/{}/members/instance/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/affinity-groups/.*/members/instance/{}$",
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

    pub struct AffinityGroupMemberInstanceDeleteThen(::httpmock::Then);
    impl AffinityGroupMemberInstanceDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct AlertClassListWhen(::httpmock::When);
    impl AlertClassListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/alert-classes$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn filter<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::AlertSubscription>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("filter", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "filter"))
                        .is_none()
                }))
            }
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct AlertClassListThen(::httpmock::Then);
    impl AlertClassListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::AlertClassResultsPage) -> Self {
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

    pub struct AlertReceiverListWhen(::httpmock::When);
    impl AlertReceiverListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/alert-receivers$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct AlertReceiverListThen(::httpmock::Then);
    impl AlertReceiverListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::AlertReceiverResultsPage) -> Self {
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

    pub struct AlertReceiverViewWhen(::httpmock::When);
    impl AlertReceiverViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/alert-receivers/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn receiver(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/alert-receivers/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct AlertReceiverViewThen(::httpmock::Then);
    impl AlertReceiverViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::AlertReceiver) -> Self {
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

    pub struct AlertReceiverDeleteWhen(::httpmock::When);
    impl AlertReceiverDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/alert-receivers/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn receiver(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/alert-receivers/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct AlertReceiverDeleteThen(::httpmock::Then);
    impl AlertReceiverDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct AlertDeliveryListWhen(::httpmock::When);
    impl AlertDeliveryListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/v1/alert-receivers/[^/]*/deliveries$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn receiver(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/alert-receivers/{}/deliveries$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn delivered<T>(self, value: T) -> Self
        where
            T: Into<Option<bool>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("delivered", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "delivered"))
                        .is_none()
                }))
            }
        }

        pub fn failed<T>(self, value: T) -> Self
        where
            T: Into<Option<bool>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("failed", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "failed"))
                        .is_none()
                }))
            }
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

        pub fn pending<T>(self, value: T) -> Self
        where
            T: Into<Option<bool>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("pending", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "pending"))
                        .is_none()
                }))
            }
        }

        pub fn sort_by<T>(self, value: T) -> Self
        where
            T: Into<Option<types::TimeAndIdSortMode>>,
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

    pub struct AlertDeliveryListThen(::httpmock::Then);
    impl AlertDeliveryListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::AlertDeliveryResultsPage) -> Self {
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

    pub struct AlertReceiverProbeWhen(::httpmock::When);
    impl AlertReceiverProbeWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/alert-receivers/[^/]*/probe$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn receiver(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/alert-receivers/{}/probe$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn resend<T>(self, value: T) -> Self
        where
            T: Into<Option<bool>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("resend", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "resend"))
                        .is_none()
                }))
            }
        }
    }

    pub struct AlertReceiverProbeThen(::httpmock::Then);
    impl AlertReceiverProbeThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::AlertProbeResult) -> Self {
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

    pub struct AlertReceiverSubscriptionAddWhen(::httpmock::When);
    impl AlertReceiverSubscriptionAddWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::POST).path_matches(
                regex::Regex::new("^/v1/alert-receivers/[^/]*/subscriptions$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn receiver(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/alert-receivers/{}/subscriptions$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::AlertSubscriptionCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct AlertReceiverSubscriptionAddThen(::httpmock::Then);
    impl AlertReceiverSubscriptionAddThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::AlertSubscriptionCreated) -> Self {
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

    pub struct AlertReceiverSubscriptionRemoveWhen(::httpmock::When);
    impl AlertReceiverSubscriptionRemoveWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::DELETE).path_matches(
                regex::Regex::new("^/v1/alert-receivers/[^/]*/subscriptions/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn receiver(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/alert-receivers/{}/subscriptions/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn subscription(self, value: &types::AlertSubscription) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/alert-receivers/.*/subscriptions/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct AlertReceiverSubscriptionRemoveThen(::httpmock::Then);
    impl AlertReceiverSubscriptionRemoveThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct AlertDeliveryResendWhen(::httpmock::When);
    impl AlertDeliveryResendWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/alerts/[^/]*/resend$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn alert_id(self, value: &::uuid::Uuid) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/alerts/{}/resend$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn receiver(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("receiver", value.to_string()))
        }
    }

    pub struct AlertDeliveryResendThen(::httpmock::Then);
    impl AlertDeliveryResendThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::AlertDeliveryId) -> Self {
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

    pub struct AntiAffinityGroupListWhen(::httpmock::When);
    impl AntiAffinityGroupListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/anti-affinity-groups$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct AntiAffinityGroupListThen(::httpmock::Then);
    impl AntiAffinityGroupListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::AntiAffinityGroupResultsPage) -> Self {
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

    pub struct AntiAffinityGroupCreateWhen(::httpmock::When);
    impl AntiAffinityGroupCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/anti-affinity-groups$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("project", value.to_string()))
        }

        pub fn body(self, value: &types::AntiAffinityGroupCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct AntiAffinityGroupCreateThen(::httpmock::Then);
    impl AntiAffinityGroupCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::AntiAffinityGroup) -> Self {
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

    pub struct AntiAffinityGroupViewWhen(::httpmock::When);
    impl AntiAffinityGroupViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/anti-affinity-groups/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn anti_affinity_group(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/anti-affinity-groups/{}$", value.to_string()))
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

    pub struct AntiAffinityGroupViewThen(::httpmock::Then);
    impl AntiAffinityGroupViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::AntiAffinityGroup) -> Self {
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

    pub struct AntiAffinityGroupUpdateWhen(::httpmock::When);
    impl AntiAffinityGroupUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/anti-affinity-groups/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn anti_affinity_group(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/anti-affinity-groups/{}$", value.to_string()))
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

        pub fn body(self, value: &types::AntiAffinityGroupUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct AntiAffinityGroupUpdateThen(::httpmock::Then);
    impl AntiAffinityGroupUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::AntiAffinityGroup) -> Self {
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

    pub struct AntiAffinityGroupDeleteWhen(::httpmock::When);
    impl AntiAffinityGroupDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/anti-affinity-groups/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn anti_affinity_group(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/anti-affinity-groups/{}$", value.to_string()))
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

    pub struct AntiAffinityGroupDeleteThen(::httpmock::Then);
    impl AntiAffinityGroupDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct AntiAffinityGroupMemberListWhen(::httpmock::When);
    impl AntiAffinityGroupMemberListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/anti-affinity-groups/[^/]*/members$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn anti_affinity_group(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/anti-affinity-groups/{}/members$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct AntiAffinityGroupMemberListThen(::httpmock::Then);
    impl AntiAffinityGroupMemberListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::AntiAffinityGroupMemberResultsPage) -> Self {
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

    pub struct AntiAffinityGroupMemberInstanceViewWhen(::httpmock::When);
    impl AntiAffinityGroupMemberInstanceViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/v1/anti-affinity-groups/[^/]*/members/instance/[^/]*$")
                        .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn anti_affinity_group(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/anti-affinity-groups/{}/members/instance/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/anti-affinity-groups/.*/members/instance/{}$",
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

    pub struct AntiAffinityGroupMemberInstanceViewThen(::httpmock::Then);
    impl AntiAffinityGroupMemberInstanceViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::AntiAffinityGroupMember) -> Self {
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

    pub struct AntiAffinityGroupMemberInstanceAddWhen(::httpmock::When);
    impl AntiAffinityGroupMemberInstanceAddWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::POST).path_matches(
                    regex::Regex::new("^/v1/anti-affinity-groups/[^/]*/members/instance/[^/]*$")
                        .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn anti_affinity_group(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/anti-affinity-groups/{}/members/instance/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/anti-affinity-groups/.*/members/instance/{}$",
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

    pub struct AntiAffinityGroupMemberInstanceAddThen(::httpmock::Then);
    impl AntiAffinityGroupMemberInstanceAddThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::AntiAffinityGroupMember) -> Self {
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

    pub struct AntiAffinityGroupMemberInstanceDeleteWhen(::httpmock::When);
    impl AntiAffinityGroupMemberInstanceDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::DELETE).path_matches(
                    regex::Regex::new("^/v1/anti-affinity-groups/[^/]*/members/instance/[^/]*$")
                        .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn anti_affinity_group(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/anti-affinity-groups/{}/members/instance/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/anti-affinity-groups/.*/members/instance/{}$",
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

    pub struct AntiAffinityGroupMemberInstanceDeleteThen(::httpmock::Then);
    impl AntiAffinityGroupMemberInstanceDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct AuthSettingsViewWhen(::httpmock::When);
    impl AuthSettingsViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/auth-settings$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct AuthSettingsViewThen(::httpmock::Then);
    impl AuthSettingsViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SiloAuthSettings) -> Self {
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

    pub struct AuthSettingsUpdateWhen(::httpmock::When);
    impl AuthSettingsUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/auth-settings$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::SiloAuthSettingsUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct AuthSettingsUpdateThen(::httpmock::Then);
    impl AuthSettingsUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SiloAuthSettings) -> Self {
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

    pub struct CertificateListWhen(::httpmock::When);
    impl CertificateListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/certificates$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct CertificateListThen(::httpmock::Then);
    impl CertificateListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct CertificateCreateWhen(::httpmock::When);
    impl CertificateCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/certificates$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::CertificateCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct CertificateCreateThen(::httpmock::Then);
    impl CertificateCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct CertificateViewWhen(::httpmock::When);
    impl CertificateViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/certificates/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn certificate(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/certificates/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct CertificateViewThen(::httpmock::Then);
    impl CertificateViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct CertificateDeleteWhen(::httpmock::When);
    impl CertificateDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/certificates/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn certificate(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/certificates/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct CertificateDeleteThen(::httpmock::Then);
    impl CertificateDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct DiskListWhen(::httpmock::When);
    impl DiskListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/disks$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct DiskListThen(::httpmock::Then);
    impl DiskListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct DiskCreateWhen(::httpmock::When);
    impl DiskCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/disks$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("project", value.to_string()))
        }

        pub fn body(self, value: &types::DiskCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct DiskCreateThen(::httpmock::Then);
    impl DiskCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct DiskViewWhen(::httpmock::When);
    impl DiskViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/disks/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct DiskViewThen(::httpmock::Then);
    impl DiskViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct DiskDeleteWhen(::httpmock::When);
    impl DiskDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/disks/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct DiskDeleteThen(::httpmock::Then);
    impl DiskDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct DiskBulkWriteImportWhen(::httpmock::When);
    impl DiskBulkWriteImportWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/disks/[^/]*/bulk-write$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct DiskBulkWriteImportThen(::httpmock::Then);
    impl DiskBulkWriteImportThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct DiskBulkWriteImportStartWhen(::httpmock::When);
    impl DiskBulkWriteImportStartWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/disks/[^/]*/bulk-write-start$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct DiskBulkWriteImportStartThen(::httpmock::Then);
    impl DiskBulkWriteImportStartThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct DiskBulkWriteImportStopWhen(::httpmock::When);
    impl DiskBulkWriteImportStopWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/disks/[^/]*/bulk-write-stop$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct DiskBulkWriteImportStopThen(::httpmock::Then);
    impl DiskBulkWriteImportStopThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct DiskFinalizeImportWhen(::httpmock::When);
    impl DiskFinalizeImportWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/disks/[^/]*/finalize$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct DiskFinalizeImportThen(::httpmock::Then);
    impl DiskFinalizeImportThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct DiskMetricsListWhen(::httpmock::When);
    impl DiskMetricsListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/disks/[^/]*/metrics/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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
            T: Into<Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>>,
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
            T: Into<Option<::std::num::NonZeroU32>>,
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

        pub fn order<T>(self, value: T) -> Self
        where
            T: Into<Option<types::PaginationOrder>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("order", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "order"))
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
            T: Into<Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>>,
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

    pub struct DiskMetricsListThen(::httpmock::Then);
    impl DiskMetricsListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct FloatingIpListWhen(::httpmock::When);
    impl FloatingIpListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/floating-ips$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct FloatingIpListThen(::httpmock::Then);
    impl FloatingIpListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::FloatingIpResultsPage) -> Self {
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

    pub struct FloatingIpCreateWhen(::httpmock::When);
    impl FloatingIpCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/floating-ips$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("project", value.to_string()))
        }

        pub fn body(self, value: &types::FloatingIpCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct FloatingIpCreateThen(::httpmock::Then);
    impl FloatingIpCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::FloatingIp) -> Self {
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

    pub struct FloatingIpViewWhen(::httpmock::When);
    impl FloatingIpViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/floating-ips/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn floating_ip(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/floating-ips/{}$", value.to_string())).unwrap();
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

    pub struct FloatingIpViewThen(::httpmock::Then);
    impl FloatingIpViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::FloatingIp) -> Self {
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

    pub struct FloatingIpUpdateWhen(::httpmock::When);
    impl FloatingIpUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/floating-ips/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn floating_ip(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/floating-ips/{}$", value.to_string())).unwrap();
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

        pub fn body(self, value: &types::FloatingIpUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct FloatingIpUpdateThen(::httpmock::Then);
    impl FloatingIpUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::FloatingIp) -> Self {
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

    pub struct FloatingIpDeleteWhen(::httpmock::When);
    impl FloatingIpDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/floating-ips/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn floating_ip(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/floating-ips/{}$", value.to_string())).unwrap();
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

    pub struct FloatingIpDeleteThen(::httpmock::Then);
    impl FloatingIpDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct FloatingIpAttachWhen(::httpmock::When);
    impl FloatingIpAttachWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/floating-ips/[^/]*/attach$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn floating_ip(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/floating-ips/{}/attach$", value.to_string()))
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

        pub fn body(self, value: &types::FloatingIpAttach) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct FloatingIpAttachThen(::httpmock::Then);
    impl FloatingIpAttachThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn accepted(self, value: &types::FloatingIp) -> Self {
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

    pub struct FloatingIpDetachWhen(::httpmock::When);
    impl FloatingIpDetachWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/floating-ips/[^/]*/detach$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn floating_ip(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/floating-ips/{}/detach$", value.to_string()))
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

    pub struct FloatingIpDetachThen(::httpmock::Then);
    impl FloatingIpDetachThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn accepted(self, value: &types::FloatingIp) -> Self {
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

    pub struct GroupListWhen(::httpmock::When);
    impl GroupListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/groups$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct GroupListThen(::httpmock::Then);
    impl GroupListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct GroupViewWhen(::httpmock::When);
    impl GroupViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/groups/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn group_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!("^/v1/groups/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct GroupViewThen(::httpmock::Then);
    impl GroupViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct ImageListWhen(::httpmock::When);
    impl ImageListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/images$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct ImageListThen(::httpmock::Then);
    impl ImageListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct ImageCreateWhen(::httpmock::When);
    impl ImageCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/images$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct ImageCreateThen(::httpmock::Then);
    impl ImageCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct ImageViewWhen(::httpmock::When);
    impl ImageViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/images/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct ImageViewThen(::httpmock::Then);
    impl ImageViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct ImageDeleteWhen(::httpmock::When);
    impl ImageDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/images/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct ImageDeleteThen(::httpmock::Then);
    impl ImageDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct ImageDemoteWhen(::httpmock::When);
    impl ImageDemoteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/images/[^/]*/demote$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct ImageDemoteThen(::httpmock::Then);
    impl ImageDemoteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct ImagePromoteWhen(::httpmock::When);
    impl ImagePromoteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/images/[^/]*/promote$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct ImagePromoteThen(::httpmock::Then);
    impl ImagePromoteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InstanceListWhen(::httpmock::When);
    impl InstanceListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/instances$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct InstanceListThen(::httpmock::Then);
    impl InstanceListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InstanceCreateWhen(::httpmock::When);
    impl InstanceCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/instances$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("project", value.to_string()))
        }

        pub fn body(self, value: &types::InstanceCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceCreateThen(::httpmock::Then);
    impl InstanceCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InstanceViewWhen(::httpmock::When);
    impl InstanceViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct InstanceViewThen(::httpmock::Then);
    impl InstanceViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InstanceUpdateWhen(::httpmock::When);
    impl InstanceUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

        pub fn body(self, value: &types::InstanceUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceUpdateThen(::httpmock::Then);
    impl InstanceUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InstanceDeleteWhen(::httpmock::When);
    impl InstanceDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct InstanceDeleteThen(::httpmock::Then);
    impl InstanceDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InstanceAffinityGroupListWhen(::httpmock::When);
    impl InstanceAffinityGroupListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/v1/instances/[^/]*/affinity-groups$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/instances/{}/affinity-groups$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct InstanceAffinityGroupListThen(::httpmock::Then);
    impl InstanceAffinityGroupListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::AffinityGroupResultsPage) -> Self {
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

    pub struct InstanceAntiAffinityGroupListWhen(::httpmock::When);
    impl InstanceAntiAffinityGroupListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/instances/[^/]*/anti-affinity-groups$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/instances/{}/anti-affinity-groups$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct InstanceAntiAffinityGroupListThen(::httpmock::Then);
    impl InstanceAntiAffinityGroupListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::AntiAffinityGroupResultsPage) -> Self {
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

    pub struct InstanceDiskListWhen(::httpmock::When);
    impl InstanceDiskListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*/disks$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/instances/{}/disks$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct InstanceDiskListThen(::httpmock::Then);
    impl InstanceDiskListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InstanceDiskAttachWhen(::httpmock::When);
    impl InstanceDiskAttachWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*/disks/attach$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct InstanceDiskAttachThen(::httpmock::Then);
    impl InstanceDiskAttachThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InstanceDiskDetachWhen(::httpmock::When);
    impl InstanceDiskDetachWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*/disks/detach$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct InstanceDiskDetachThen(::httpmock::Then);
    impl InstanceDiskDetachThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InstanceExternalIpListWhen(::httpmock::When);
    impl InstanceExternalIpListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*/external-ips$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct InstanceExternalIpListThen(::httpmock::Then);
    impl InstanceExternalIpListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InstanceEphemeralIpAttachWhen(::httpmock::When);
    impl InstanceEphemeralIpAttachWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::POST).path_matches(
                regex::Regex::new("^/v1/instances/[^/]*/external-ips/ephemeral$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/instances/{}/external-ips/ephemeral$",
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

        pub fn body(self, value: &types::EphemeralIpCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InstanceEphemeralIpAttachThen(::httpmock::Then);
    impl InstanceEphemeralIpAttachThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn accepted(self, value: &types::ExternalIp) -> Self {
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

    pub struct InstanceEphemeralIpDetachWhen(::httpmock::When);
    impl InstanceEphemeralIpDetachWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::DELETE).path_matches(
                regex::Regex::new("^/v1/instances/[^/]*/external-ips/ephemeral$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/instances/{}/external-ips/ephemeral$",
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

    pub struct InstanceEphemeralIpDetachThen(::httpmock::Then);
    impl InstanceEphemeralIpDetachThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InstanceRebootWhen(::httpmock::When);
    impl InstanceRebootWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*/reboot$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct InstanceRebootThen(::httpmock::Then);
    impl InstanceRebootThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InstanceSerialConsoleWhen(::httpmock::When);
    impl InstanceSerialConsoleWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/v1/instances/[^/]*/serial-console$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct InstanceSerialConsoleThen(::httpmock::Then);
    impl InstanceSerialConsoleThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InstanceSerialConsoleStreamWhen(::httpmock::When);
    impl InstanceSerialConsoleStreamWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/instances/[^/]*/serial-console/stream$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct InstanceSerialConsoleStreamThen(::httpmock::Then);
    impl InstanceSerialConsoleStreamThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn default_response(self, status: u16) -> Self {
            Self(self.0.status(status))
        }

        pub fn switching_protocols(self) -> Self {
            Self(self.0.status(101u16))
        }
    }

    pub struct InstanceSshPublicKeyListWhen(::httpmock::When);
    impl InstanceSshPublicKeyListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/v1/instances/[^/]*/ssh-public-keys$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn instance(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/instances/{}/ssh-public-keys$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct InstanceSshPublicKeyListThen(::httpmock::Then);
    impl InstanceSshPublicKeyListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InstanceStartWhen(::httpmock::When);
    impl InstanceStartWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*/start$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct InstanceStartThen(::httpmock::Then);
    impl InstanceStartThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InstanceStopWhen(::httpmock::When);
    impl InstanceStopWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/instances/[^/]*/stop$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct InstanceStopThen(::httpmock::Then);
    impl InstanceStopThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InternetGatewayIpAddressListWhen(::httpmock::When);
    impl InternetGatewayIpAddressListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/v1/internet-gateway-ip-addresses$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn gateway<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("gateway", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "gateway"))
                        .is_none()
                }))
            }
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct InternetGatewayIpAddressListThen(::httpmock::Then);
    impl InternetGatewayIpAddressListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::InternetGatewayIpAddressResultsPage) -> Self {
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

    pub struct InternetGatewayIpAddressCreateWhen(::httpmock::When);
    impl InternetGatewayIpAddressCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::POST).path_matches(
                    regex::Regex::new("^/v1/internet-gateway-ip-addresses$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn gateway(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("gateway", value.to_string()))
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

        pub fn body(self, value: &types::InternetGatewayIpAddressCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InternetGatewayIpAddressCreateThen(::httpmock::Then);
    impl InternetGatewayIpAddressCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::InternetGatewayIpAddress) -> Self {
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

    pub struct InternetGatewayIpAddressDeleteWhen(::httpmock::When);
    impl InternetGatewayIpAddressDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::DELETE).path_matches(
                regex::Regex::new("^/v1/internet-gateway-ip-addresses/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn address(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/internet-gateway-ip-addresses/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn cascade<T>(self, value: T) -> Self
        where
            T: Into<Option<bool>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("cascade", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "cascade"))
                        .is_none()
                }))
            }
        }

        pub fn gateway<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("gateway", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "gateway"))
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

    pub struct InternetGatewayIpAddressDeleteThen(::httpmock::Then);
    impl InternetGatewayIpAddressDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InternetGatewayIpPoolListWhen(::httpmock::When);
    impl InternetGatewayIpPoolListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/internet-gateway-ip-pools$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn gateway<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("gateway", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "gateway"))
                        .is_none()
                }))
            }
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct InternetGatewayIpPoolListThen(::httpmock::Then);
    impl InternetGatewayIpPoolListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::InternetGatewayIpPoolResultsPage) -> Self {
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

    pub struct InternetGatewayIpPoolCreateWhen(::httpmock::When);
    impl InternetGatewayIpPoolCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/internet-gateway-ip-pools$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn gateway(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("gateway", value.to_string()))
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

        pub fn body(self, value: &types::InternetGatewayIpPoolCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InternetGatewayIpPoolCreateThen(::httpmock::Then);
    impl InternetGatewayIpPoolCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::InternetGatewayIpPool) -> Self {
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

    pub struct InternetGatewayIpPoolDeleteWhen(::httpmock::When);
    impl InternetGatewayIpPoolDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::DELETE).path_matches(
                    regex::Regex::new("^/v1/internet-gateway-ip-pools/[^/]*$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn pool(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/internet-gateway-ip-pools/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn cascade<T>(self, value: T) -> Self
        where
            T: Into<Option<bool>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("cascade", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "cascade"))
                        .is_none()
                }))
            }
        }

        pub fn gateway<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a types::NameOrId>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("gateway", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "gateway"))
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

    pub struct InternetGatewayIpPoolDeleteThen(::httpmock::Then);
    impl InternetGatewayIpPoolDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InternetGatewayListWhen(::httpmock::When);
    impl InternetGatewayListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/internet-gateways$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct InternetGatewayListThen(::httpmock::Then);
    impl InternetGatewayListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::InternetGatewayResultsPage) -> Self {
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

    pub struct InternetGatewayCreateWhen(::httpmock::When);
    impl InternetGatewayCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/internet-gateways$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

        pub fn body(self, value: &types::InternetGatewayCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct InternetGatewayCreateThen(::httpmock::Then);
    impl InternetGatewayCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::InternetGateway) -> Self {
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

    pub struct InternetGatewayViewWhen(::httpmock::When);
    impl InternetGatewayViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/internet-gateways/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn gateway(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/internet-gateways/{}$", value.to_string()))
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

    pub struct InternetGatewayViewThen(::httpmock::Then);
    impl InternetGatewayViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::InternetGateway) -> Self {
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

    pub struct InternetGatewayDeleteWhen(::httpmock::When);
    impl InternetGatewayDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/internet-gateways/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn gateway(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/internet-gateways/{}$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn cascade<T>(self, value: T) -> Self
        where
            T: Into<Option<bool>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("cascade", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "cascade"))
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

    pub struct InternetGatewayDeleteThen(::httpmock::Then);
    impl InternetGatewayDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct ProjectIpPoolListWhen(::httpmock::When);
    impl ProjectIpPoolListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/ip-pools$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct ProjectIpPoolListThen(::httpmock::Then);
    impl ProjectIpPoolListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SiloIpPoolResultsPage) -> Self {
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

    pub struct ProjectIpPoolViewWhen(::httpmock::When);
    impl ProjectIpPoolViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/ip-pools/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn pool(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/ip-pools/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct ProjectIpPoolViewThen(::httpmock::Then);
    impl ProjectIpPoolViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SiloIpPool) -> Self {
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

    pub struct LoginLocalWhen(::httpmock::When);
    impl LoginLocalWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/login/[^/]*/local$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct LoginLocalThen(::httpmock::Then);
    impl LoginLocalThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct LogoutWhen(::httpmock::When);
    impl LogoutWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/logout$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct LogoutThen(::httpmock::Then);
    impl LogoutThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct CurrentUserViewWhen(::httpmock::When);
    impl CurrentUserViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/me$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct CurrentUserViewThen(::httpmock::Then);
    impl CurrentUserViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct CurrentUserGroupsWhen(::httpmock::When);
    impl CurrentUserGroupsWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/me/groups$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct CurrentUserGroupsThen(::httpmock::Then);
    impl CurrentUserGroupsThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct CurrentUserSshKeyListWhen(::httpmock::When);
    impl CurrentUserSshKeyListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/me/ssh-keys$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct CurrentUserSshKeyListThen(::httpmock::Then);
    impl CurrentUserSshKeyListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct CurrentUserSshKeyCreateWhen(::httpmock::When);
    impl CurrentUserSshKeyCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/me/ssh-keys$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::SshKeyCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct CurrentUserSshKeyCreateThen(::httpmock::Then);
    impl CurrentUserSshKeyCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct CurrentUserSshKeyViewWhen(::httpmock::When);
    impl CurrentUserSshKeyViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/me/ssh-keys/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn ssh_key(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/me/ssh-keys/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct CurrentUserSshKeyViewThen(::httpmock::Then);
    impl CurrentUserSshKeyViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct CurrentUserSshKeyDeleteWhen(::httpmock::When);
    impl CurrentUserSshKeyDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/me/ssh-keys/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn ssh_key(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/me/ssh-keys/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct CurrentUserSshKeyDeleteThen(::httpmock::Then);
    impl CurrentUserSshKeyDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SiloMetricWhen(::httpmock::When);
    impl SiloMetricWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/metrics/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn metric_name(self, value: types::SystemMetricName) -> Self {
            let re = regex::Regex::new(&format!("^/v1/metrics/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn end_time<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>>,
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
            T: Into<Option<::std::num::NonZeroU32>>,
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

        pub fn order<T>(self, value: T) -> Self
        where
            T: Into<Option<types::PaginationOrder>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("order", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "order"))
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
            T: Into<Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>>,
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

    pub struct SiloMetricThen(::httpmock::Then);
    impl SiloMetricThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InstanceNetworkInterfaceListWhen(::httpmock::When);
    impl InstanceNetworkInterfaceListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/network-interfaces$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct InstanceNetworkInterfaceListThen(::httpmock::Then);
    impl InstanceNetworkInterfaceListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InstanceNetworkInterfaceCreateWhen(::httpmock::When);
    impl InstanceNetworkInterfaceCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/network-interfaces$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct InstanceNetworkInterfaceCreateThen(::httpmock::Then);
    impl InstanceNetworkInterfaceCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InstanceNetworkInterfaceViewWhen(::httpmock::When);
    impl InstanceNetworkInterfaceViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/network-interfaces/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct InstanceNetworkInterfaceViewThen(::httpmock::Then);
    impl InstanceNetworkInterfaceViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InstanceNetworkInterfaceUpdateWhen(::httpmock::When);
    impl InstanceNetworkInterfaceUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/network-interfaces/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct InstanceNetworkInterfaceUpdateThen(::httpmock::Then);
    impl InstanceNetworkInterfaceUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct InstanceNetworkInterfaceDeleteWhen(::httpmock::When);
    impl InstanceNetworkInterfaceDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/network-interfaces/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct InstanceNetworkInterfaceDeleteThen(::httpmock::Then);
    impl InstanceNetworkInterfaceDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct PingWhen(::httpmock::When);
    impl PingWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/ping$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct PingThen(::httpmock::Then);
    impl PingThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Ping) -> Self {
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

    pub struct PolicyViewWhen(::httpmock::When);
    impl PolicyViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct PolicyViewThen(::httpmock::Then);
    impl PolicyViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct PolicyUpdateWhen(::httpmock::When);
    impl PolicyUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::SiloRolePolicy) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct PolicyUpdateThen(::httpmock::Then);
    impl PolicyUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct ProjectListWhen(::httpmock::When);
    impl ProjectListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/projects$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct ProjectListThen(::httpmock::Then);
    impl ProjectListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct ProjectCreateWhen(::httpmock::When);
    impl ProjectCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/projects$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::ProjectCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct ProjectCreateThen(::httpmock::Then);
    impl ProjectCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct ProjectViewWhen(::httpmock::When);
    impl ProjectViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/projects/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/projects/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct ProjectViewThen(::httpmock::Then);
    impl ProjectViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct ProjectUpdateWhen(::httpmock::When);
    impl ProjectUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/projects/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct ProjectUpdateThen(::httpmock::Then);
    impl ProjectUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct ProjectDeleteWhen(::httpmock::When);
    impl ProjectDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/projects/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/projects/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct ProjectDeleteThen(::httpmock::Then);
    impl ProjectDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct ProjectPolicyViewWhen(::httpmock::When);
    impl ProjectPolicyViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/projects/[^/]*/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/projects/{}/policy$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct ProjectPolicyViewThen(::httpmock::Then);
    impl ProjectPolicyViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct ProjectPolicyUpdateWhen(::httpmock::When);
    impl ProjectPolicyUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/projects/[^/]*/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct ProjectPolicyUpdateThen(::httpmock::Then);
    impl ProjectPolicyUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SnapshotListWhen(::httpmock::When);
    impl SnapshotListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/snapshots$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct SnapshotListThen(::httpmock::Then);
    impl SnapshotListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SnapshotCreateWhen(::httpmock::When);
    impl SnapshotCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/snapshots$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("project", value.to_string()))
        }

        pub fn body(self, value: &types::SnapshotCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SnapshotCreateThen(::httpmock::Then);
    impl SnapshotCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SnapshotViewWhen(::httpmock::When);
    impl SnapshotViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/snapshots/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct SnapshotViewThen(::httpmock::Then);
    impl SnapshotViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SnapshotDeleteWhen(::httpmock::When);
    impl SnapshotDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/snapshots/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct SnapshotDeleteThen(::httpmock::Then);
    impl SnapshotDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct PhysicalDiskListWhen(::httpmock::When);
    impl PhysicalDiskListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/hardware/disks$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct PhysicalDiskListThen(::httpmock::Then);
    impl PhysicalDiskListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct PhysicalDiskViewWhen(::httpmock::When);
    impl PhysicalDiskViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/hardware/disks/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn disk_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/hardware/disks/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct PhysicalDiskViewThen(::httpmock::Then);
    impl PhysicalDiskViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::PhysicalDisk) -> Self {
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

    pub struct NetworkingSwitchPortLldpNeighborsWhen(::httpmock::When);
    impl NetworkingSwitchPortLldpNeighborsWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new(
                        "^/v1/system/hardware/rack-switch-port/[^/]*/[^/]*/[^/]*/lldp/neighbors$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn rack_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/hardware/rack-switch-port/{}/.*/.*/lldp/neighbors$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn switch_location(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/hardware/rack-switch-port/.*/{}/.*/lldp/neighbors$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn port(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/hardware/rack-switch-port/.*/.*/{}/lldp/neighbors$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct NetworkingSwitchPortLldpNeighborsThen(::httpmock::Then);
    impl NetworkingSwitchPortLldpNeighborsThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::LldpNeighborResultsPage) -> Self {
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

    pub struct RackListWhen(::httpmock::When);
    impl RackListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/hardware/racks$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct RackListThen(::httpmock::Then);
    impl RackListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct RackViewWhen(::httpmock::When);
    impl RackViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/hardware/racks/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn rack_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/hardware/racks/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct RackViewThen(::httpmock::Then);
    impl RackViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SledListWhen(::httpmock::When);
    impl SledListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/hardware/sleds$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct SledListThen(::httpmock::Then);
    impl SledListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SledAddWhen(::httpmock::When);
    impl SledAddWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/system/hardware/sleds$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::UninitializedSledId) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SledAddThen(::httpmock::Then);
    impl SledAddThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::SledId) -> Self {
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

    pub struct SledViewWhen(::httpmock::When);
    impl SledViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/hardware/sleds/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn sled_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/hardware/sleds/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SledViewThen(::httpmock::Then);
    impl SledViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SledPhysicalDiskListWhen(::httpmock::When);
    impl SledPhysicalDiskListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/hardware/sleds/[^/]*/disks$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn sled_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/hardware/sleds/{}/disks$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct SledPhysicalDiskListThen(::httpmock::Then);
    impl SledPhysicalDiskListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SledInstanceListWhen(::httpmock::When);
    impl SledInstanceListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/hardware/sleds/[^/]*/instances$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn sled_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/hardware/sleds/{}/instances$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct SledInstanceListThen(::httpmock::Then);
    impl SledInstanceListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SledSetProvisionPolicyWhen(::httpmock::When);
    impl SledSetProvisionPolicyWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::PUT).path_matches(
                regex::Regex::new("^/v1/system/hardware/sleds/[^/]*/provision-policy$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn sled_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/hardware/sleds/{}/provision-policy$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::SledProvisionPolicyParams) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SledSetProvisionPolicyThen(::httpmock::Then);
    impl SledSetProvisionPolicyThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SledProvisionPolicyResponse) -> Self {
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

    pub struct SledListUninitializedWhen(::httpmock::When);
    impl SledListUninitializedWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/hardware/sleds-uninitialized$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct SledListUninitializedThen(::httpmock::Then);
    impl SledListUninitializedThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::UninitializedSledResultsPage) -> Self {
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

    pub struct NetworkingSwitchPortListWhen(::httpmock::When);
    impl NetworkingSwitchPortListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/hardware/switch-port$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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
            T: Into<Option<&'a ::uuid::Uuid>>,
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

    pub struct NetworkingSwitchPortListThen(::httpmock::Then);
    impl NetworkingSwitchPortListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct NetworkingSwitchPortLldpConfigViewWhen(::httpmock::When);
    impl NetworkingSwitchPortLldpConfigViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/hardware/switch-port/[^/]*/lldp/config$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn port(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/hardware/switch-port/{}/lldp/config$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn rack_id(self, value: &::uuid::Uuid) -> Self {
            Self(self.0.query_param("rack_id", value.to_string()))
        }

        pub fn switch_location(self, value: &types::Name) -> Self {
            Self(self.0.query_param("switch_location", value.to_string()))
        }
    }

    pub struct NetworkingSwitchPortLldpConfigViewThen(::httpmock::Then);
    impl NetworkingSwitchPortLldpConfigViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::LldpLinkConfig) -> Self {
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

    pub struct NetworkingSwitchPortLldpConfigUpdateWhen(::httpmock::When);
    impl NetworkingSwitchPortLldpConfigUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::POST).path_matches(
                regex::Regex::new("^/v1/system/hardware/switch-port/[^/]*/lldp/config$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn port(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/hardware/switch-port/{}/lldp/config$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn rack_id(self, value: &::uuid::Uuid) -> Self {
            Self(self.0.query_param("rack_id", value.to_string()))
        }

        pub fn switch_location(self, value: &types::Name) -> Self {
            Self(self.0.query_param("switch_location", value.to_string()))
        }

        pub fn body(self, value: &types::LldpLinkConfig) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct NetworkingSwitchPortLldpConfigUpdateThen(::httpmock::Then);
    impl NetworkingSwitchPortLldpConfigUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct NetworkingSwitchPortApplySettingsWhen(::httpmock::When);
    impl NetworkingSwitchPortApplySettingsWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::POST).path_matches(
                regex::Regex::new("^/v1/system/hardware/switch-port/[^/]*/settings$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

        pub fn rack_id(self, value: &::uuid::Uuid) -> Self {
            Self(self.0.query_param("rack_id", value.to_string()))
        }

        pub fn switch_location(self, value: &types::Name) -> Self {
            Self(self.0.query_param("switch_location", value.to_string()))
        }

        pub fn body(self, value: &types::SwitchPortApplySettings) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct NetworkingSwitchPortApplySettingsThen(::httpmock::Then);
    impl NetworkingSwitchPortApplySettingsThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct NetworkingSwitchPortClearSettingsWhen(::httpmock::When);
    impl NetworkingSwitchPortClearSettingsWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::DELETE).path_matches(
                regex::Regex::new("^/v1/system/hardware/switch-port/[^/]*/settings$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

        pub fn rack_id(self, value: &::uuid::Uuid) -> Self {
            Self(self.0.query_param("rack_id", value.to_string()))
        }

        pub fn switch_location(self, value: &types::Name) -> Self {
            Self(self.0.query_param("switch_location", value.to_string()))
        }
    }

    pub struct NetworkingSwitchPortClearSettingsThen(::httpmock::Then);
    impl NetworkingSwitchPortClearSettingsThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct NetworkingSwitchPortStatusWhen(::httpmock::When);
    impl NetworkingSwitchPortStatusWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/hardware/switch-port/[^/]*/status$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn port(self, value: &types::Name) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/hardware/switch-port/{}/status$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn rack_id(self, value: &::uuid::Uuid) -> Self {
            Self(self.0.query_param("rack_id", value.to_string()))
        }

        pub fn switch_location(self, value: &types::Name) -> Self {
            Self(self.0.query_param("switch_location", value.to_string()))
        }
    }

    pub struct NetworkingSwitchPortStatusThen(::httpmock::Then);
    impl NetworkingSwitchPortStatusThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SwitchLinkState) -> Self {
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

    pub struct SwitchListWhen(::httpmock::When);
    impl SwitchListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/hardware/switches$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct SwitchListThen(::httpmock::Then);
    impl SwitchListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SwitchViewWhen(::httpmock::When);
    impl SwitchViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/v1/system/hardware/switches/[^/]*$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn switch_id(self, value: &::uuid::Uuid) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/hardware/switches/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SwitchViewThen(::httpmock::Then);
    impl SwitchViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SiloIdentityProviderListWhen(::httpmock::When);
    impl SiloIdentityProviderListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/identity-providers$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct SiloIdentityProviderListThen(::httpmock::Then);
    impl SiloIdentityProviderListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct LocalIdpUserCreateWhen(::httpmock::When);
    impl LocalIdpUserCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::POST).path_matches(
                regex::Regex::new("^/v1/system/identity-providers/local/users$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("silo", value.to_string()))
        }

        pub fn body(self, value: &types::UserCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct LocalIdpUserCreateThen(::httpmock::Then);
    impl LocalIdpUserCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct LocalIdpUserDeleteWhen(::httpmock::When);
    impl LocalIdpUserDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::DELETE).path_matches(
                regex::Regex::new("^/v1/system/identity-providers/local/users/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn user_id(self, value: &::uuid::Uuid) -> Self {
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

    pub struct LocalIdpUserDeleteThen(::httpmock::Then);
    impl LocalIdpUserDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct LocalIdpUserSetPasswordWhen(::httpmock::When);
    impl LocalIdpUserSetPasswordWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::POST).path_matches(
                    regex::Regex::new(
                        "^/v1/system/identity-providers/local/users/[^/]*/set-password$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn user_id(self, value: &::uuid::Uuid) -> Self {
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

    pub struct LocalIdpUserSetPasswordThen(::httpmock::Then);
    impl LocalIdpUserSetPasswordThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SamlIdentityProviderCreateWhen(::httpmock::When);
    impl SamlIdentityProviderCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::POST).path_matches(
                    regex::Regex::new("^/v1/system/identity-providers/saml$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("silo", value.to_string()))
        }

        pub fn body(self, value: &types::SamlIdentityProviderCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SamlIdentityProviderCreateThen(::httpmock::Then);
    impl SamlIdentityProviderCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SamlIdentityProviderViewWhen(::httpmock::When);
    impl SamlIdentityProviderViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/identity-providers/saml/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
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
    }

    pub struct SamlIdentityProviderViewThen(::httpmock::Then);
    impl SamlIdentityProviderViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct IpPoolListWhen(::httpmock::When);
    impl IpPoolListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/ip-pools$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct IpPoolListThen(::httpmock::Then);
    impl IpPoolListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct IpPoolCreateWhen(::httpmock::When);
    impl IpPoolCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/system/ip-pools$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::IpPoolCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct IpPoolCreateThen(::httpmock::Then);
    impl IpPoolCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct IpPoolViewWhen(::httpmock::When);
    impl IpPoolViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/ip-pools/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn pool(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/system/ip-pools/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct IpPoolViewThen(::httpmock::Then);
    impl IpPoolViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct IpPoolUpdateWhen(::httpmock::When);
    impl IpPoolUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/system/ip-pools/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct IpPoolUpdateThen(::httpmock::Then);
    impl IpPoolUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct IpPoolDeleteWhen(::httpmock::When);
    impl IpPoolDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/system/ip-pools/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn pool(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/system/ip-pools/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct IpPoolDeleteThen(::httpmock::Then);
    impl IpPoolDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct IpPoolRangeListWhen(::httpmock::When);
    impl IpPoolRangeListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/ip-pools/[^/]*/ranges$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct IpPoolRangeListThen(::httpmock::Then);
    impl IpPoolRangeListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct IpPoolRangeAddWhen(::httpmock::When);
    impl IpPoolRangeAddWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::POST).path_matches(
                    regex::Regex::new("^/v1/system/ip-pools/[^/]*/ranges/add$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct IpPoolRangeAddThen(::httpmock::Then);
    impl IpPoolRangeAddThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct IpPoolRangeRemoveWhen(::httpmock::When);
    impl IpPoolRangeRemoveWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::POST).path_matches(
                regex::Regex::new("^/v1/system/ip-pools/[^/]*/ranges/remove$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct IpPoolRangeRemoveThen(::httpmock::Then);
    impl IpPoolRangeRemoveThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct IpPoolSiloListWhen(::httpmock::When);
    impl IpPoolSiloListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/ip-pools/[^/]*/silos$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn pool(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/ip-pools/{}/silos$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct IpPoolSiloListThen(::httpmock::Then);
    impl IpPoolSiloListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::IpPoolSiloLinkResultsPage) -> Self {
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

    pub struct IpPoolSiloLinkWhen(::httpmock::When);
    impl IpPoolSiloLinkWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/system/ip-pools/[^/]*/silos$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn pool(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/ip-pools/{}/silos$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::IpPoolLinkSilo) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct IpPoolSiloLinkThen(::httpmock::Then);
    impl IpPoolSiloLinkThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::IpPoolSiloLink) -> Self {
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

    pub struct IpPoolSiloUpdateWhen(::httpmock::When);
    impl IpPoolSiloUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::PUT).path_matches(
                regex::Regex::new("^/v1/system/ip-pools/[^/]*/silos/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn pool(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/ip-pools/{}/silos/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn silo(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/ip-pools/.*/silos/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::IpPoolSiloUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct IpPoolSiloUpdateThen(::httpmock::Then);
    impl IpPoolSiloUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::IpPoolSiloLink) -> Self {
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

    pub struct IpPoolSiloUnlinkWhen(::httpmock::When);
    impl IpPoolSiloUnlinkWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::DELETE).path_matches(
                regex::Regex::new("^/v1/system/ip-pools/[^/]*/silos/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn pool(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/ip-pools/{}/silos/.*$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn silo(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/ip-pools/.*/silos/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct IpPoolSiloUnlinkThen(::httpmock::Then);
    impl IpPoolSiloUnlinkThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct IpPoolUtilizationViewWhen(::httpmock::When);
    impl IpPoolUtilizationViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/ip-pools/[^/]*/utilization$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn pool(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/ip-pools/{}/utilization$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct IpPoolUtilizationViewThen(::httpmock::Then);
    impl IpPoolUtilizationViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::IpPoolUtilization) -> Self {
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

    pub struct IpPoolServiceViewWhen(::httpmock::When);
    impl IpPoolServiceViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/ip-pools-service$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct IpPoolServiceViewThen(::httpmock::Then);
    impl IpPoolServiceViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct IpPoolServiceRangeListWhen(::httpmock::When);
    impl IpPoolServiceRangeListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/v1/system/ip-pools-service/ranges$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct IpPoolServiceRangeListThen(::httpmock::Then);
    impl IpPoolServiceRangeListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct IpPoolServiceRangeAddWhen(::httpmock::When);
    impl IpPoolServiceRangeAddWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::POST).path_matches(
                regex::Regex::new("^/v1/system/ip-pools-service/ranges/add$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::IpRange) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct IpPoolServiceRangeAddThen(::httpmock::Then);
    impl IpPoolServiceRangeAddThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct IpPoolServiceRangeRemoveWhen(::httpmock::When);
    impl IpPoolServiceRangeRemoveWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::POST).path_matches(
                regex::Regex::new("^/v1/system/ip-pools-service/ranges/remove$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::IpRange) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct IpPoolServiceRangeRemoveThen(::httpmock::Then);
    impl IpPoolServiceRangeRemoveThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SystemMetricWhen(::httpmock::When);
    impl SystemMetricWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/metrics/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn metric_name(self, value: types::SystemMetricName) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/system/metrics/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn end_time<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>>,
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
            T: Into<Option<::std::num::NonZeroU32>>,
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

        pub fn order<T>(self, value: T) -> Self
        where
            T: Into<Option<types::PaginationOrder>>,
        {
            if let Some(value) = value.into() {
                Self(self.0.query_param("order", value.to_string()))
            } else {
                Self(self.0.matches(|req| {
                    req.query_params
                        .as_ref()
                        .and_then(|qs| qs.iter().find(|(key, _)| key == "order"))
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

        pub fn start_time<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a ::chrono::DateTime<::chrono::offset::Utc>>>,
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

    pub struct SystemMetricThen(::httpmock::Then);
    impl SystemMetricThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct NetworkingAddressLotListWhen(::httpmock::When);
    impl NetworkingAddressLotListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/v1/system/networking/address-lot$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct NetworkingAddressLotListThen(::httpmock::Then);
    impl NetworkingAddressLotListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct NetworkingAddressLotCreateWhen(::httpmock::When);
    impl NetworkingAddressLotCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::POST).path_matches(
                    regex::Regex::new("^/v1/system/networking/address-lot$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::AddressLotCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct NetworkingAddressLotCreateThen(::httpmock::Then);
    impl NetworkingAddressLotCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct NetworkingAddressLotDeleteWhen(::httpmock::When);
    impl NetworkingAddressLotDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::DELETE).path_matches(
                regex::Regex::new("^/v1/system/networking/address-lot/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct NetworkingAddressLotDeleteThen(::httpmock::Then);
    impl NetworkingAddressLotDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct NetworkingAddressLotBlockListWhen(::httpmock::When);
    impl NetworkingAddressLotBlockListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/networking/address-lot/[^/]*/blocks$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
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
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct NetworkingAddressLotBlockListThen(::httpmock::Then);
    impl NetworkingAddressLotBlockListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct NetworkingAllowListViewWhen(::httpmock::When);
    impl NetworkingAllowListViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/networking/allow-list$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct NetworkingAllowListViewThen(::httpmock::Then);
    impl NetworkingAllowListViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::AllowList) -> Self {
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

    pub struct NetworkingAllowListUpdateWhen(::httpmock::When);
    impl NetworkingAllowListUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/system/networking/allow-list$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::AllowListUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct NetworkingAllowListUpdateThen(::httpmock::Then);
    impl NetworkingAllowListUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::AllowList) -> Self {
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

    pub struct NetworkingBfdDisableWhen(::httpmock::When);
    impl NetworkingBfdDisableWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::POST).path_matches(
                    regex::Regex::new("^/v1/system/networking/bfd-disable$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::BfdSessionDisable) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct NetworkingBfdDisableThen(::httpmock::Then);
    impl NetworkingBfdDisableThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct NetworkingBfdEnableWhen(::httpmock::When);
    impl NetworkingBfdEnableWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/system/networking/bfd-enable$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::BfdSessionEnable) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct NetworkingBfdEnableThen(::httpmock::Then);
    impl NetworkingBfdEnableThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct NetworkingBfdStatusWhen(::httpmock::When);
    impl NetworkingBfdStatusWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/networking/bfd-status$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct NetworkingBfdStatusThen(::httpmock::Then);
    impl NetworkingBfdStatusThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &::std::vec::Vec<types::BfdStatus>) -> Self {
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

    pub struct NetworkingBgpConfigListWhen(::httpmock::When);
    impl NetworkingBgpConfigListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/networking/bgp$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct NetworkingBgpConfigListThen(::httpmock::Then);
    impl NetworkingBgpConfigListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::BgpConfigResultsPage) -> Self {
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

    pub struct NetworkingBgpConfigCreateWhen(::httpmock::When);
    impl NetworkingBgpConfigCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/system/networking/bgp$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::BgpConfigCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct NetworkingBgpConfigCreateThen(::httpmock::Then);
    impl NetworkingBgpConfigCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::BgpConfig) -> Self {
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

    pub struct NetworkingBgpConfigDeleteWhen(::httpmock::When);
    impl NetworkingBgpConfigDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/system/networking/bgp$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn name_or_id(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("name_or_id", value.to_string()))
        }
    }

    pub struct NetworkingBgpConfigDeleteThen(::httpmock::Then);
    impl NetworkingBgpConfigDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct NetworkingBgpAnnounceSetListWhen(::httpmock::When);
    impl NetworkingBgpAnnounceSetListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/networking/bgp-announce-set$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct NetworkingBgpAnnounceSetListThen(::httpmock::Then);
    impl NetworkingBgpAnnounceSetListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &::std::vec::Vec<types::BgpAnnounceSet>) -> Self {
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

    pub struct NetworkingBgpAnnounceSetUpdateWhen(::httpmock::When);
    impl NetworkingBgpAnnounceSetUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::PUT).path_matches(
                regex::Regex::new("^/v1/system/networking/bgp-announce-set$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::BgpAnnounceSetCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct NetworkingBgpAnnounceSetUpdateThen(::httpmock::Then);
    impl NetworkingBgpAnnounceSetUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::BgpAnnounceSet) -> Self {
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

    pub struct NetworkingBgpAnnounceSetDeleteWhen(::httpmock::When);
    impl NetworkingBgpAnnounceSetDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::DELETE).path_matches(
                regex::Regex::new("^/v1/system/networking/bgp-announce-set/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn announce_set(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/networking/bgp-announce-set/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct NetworkingBgpAnnounceSetDeleteThen(::httpmock::Then);
    impl NetworkingBgpAnnounceSetDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct NetworkingBgpAnnouncementListWhen(::httpmock::When);
    impl NetworkingBgpAnnouncementListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new(
                        "^/v1/system/networking/bgp-announce-set/[^/]*/announcement$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn announce_set(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/networking/bgp-announce-set/{}/announcement$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct NetworkingBgpAnnouncementListThen(::httpmock::Then);
    impl NetworkingBgpAnnouncementListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &::std::vec::Vec<types::BgpAnnouncement>) -> Self {
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

    pub struct NetworkingBgpExportedWhen(::httpmock::When);
    impl NetworkingBgpExportedWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/v1/system/networking/bgp-exported$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct NetworkingBgpExportedThen(::httpmock::Then);
    impl NetworkingBgpExportedThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::BgpExported) -> Self {
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

    pub struct NetworkingBgpMessageHistoryWhen(::httpmock::When);
    impl NetworkingBgpMessageHistoryWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/networking/bgp-message-history$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn asn(self, value: u32) -> Self {
            Self(self.0.query_param("asn", value.to_string()))
        }
    }

    pub struct NetworkingBgpMessageHistoryThen(::httpmock::Then);
    impl NetworkingBgpMessageHistoryThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::AggregateBgpMessageHistory) -> Self {
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

    pub struct NetworkingBgpImportedRoutesIpv4When(::httpmock::When);
    impl NetworkingBgpImportedRoutesIpv4When {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/networking/bgp-routes-ipv4$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn asn(self, value: u32) -> Self {
            Self(self.0.query_param("asn", value.to_string()))
        }
    }

    pub struct NetworkingBgpImportedRoutesIpv4Then(::httpmock::Then);
    impl NetworkingBgpImportedRoutesIpv4Then {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &::std::vec::Vec<types::BgpImportedRouteIpv4>) -> Self {
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

    pub struct NetworkingBgpStatusWhen(::httpmock::When);
    impl NetworkingBgpStatusWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/networking/bgp-status$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct NetworkingBgpStatusThen(::httpmock::Then);
    impl NetworkingBgpStatusThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &::std::vec::Vec<types::BgpPeerStatus>) -> Self {
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

    pub struct NetworkingLoopbackAddressListWhen(::httpmock::When);
    impl NetworkingLoopbackAddressListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/networking/loopback-address$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct NetworkingLoopbackAddressListThen(::httpmock::Then);
    impl NetworkingLoopbackAddressListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct NetworkingLoopbackAddressCreateWhen(::httpmock::When);
    impl NetworkingLoopbackAddressCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::POST).path_matches(
                regex::Regex::new("^/v1/system/networking/loopback-address$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::LoopbackAddressCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct NetworkingLoopbackAddressCreateThen(::httpmock::Then);
    impl NetworkingLoopbackAddressCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct NetworkingLoopbackAddressDeleteWhen(::httpmock::When);
    impl NetworkingLoopbackAddressDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::DELETE).path_matches(
                    regex::Regex::new(
                        "^/v1/system/networking/loopback-address/[^/]*/[^/]*/[^/]*/[^/]*$",
                    )
                    .unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn rack_id(self, value: &::uuid::Uuid) -> Self {
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

        pub fn address(self, value: &::std::net::IpAddr) -> Self {
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

    pub struct NetworkingLoopbackAddressDeleteThen(::httpmock::Then);
    impl NetworkingLoopbackAddressDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct NetworkingSwitchPortSettingsListWhen(::httpmock::When);
    impl NetworkingSwitchPortSettingsListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/networking/switch-port-settings$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct NetworkingSwitchPortSettingsListThen(::httpmock::Then);
    impl NetworkingSwitchPortSettingsListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SwitchPortSettingsIdentityResultsPage) -> Self {
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

    pub struct NetworkingSwitchPortSettingsCreateWhen(::httpmock::When);
    impl NetworkingSwitchPortSettingsCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::POST).path_matches(
                regex::Regex::new("^/v1/system/networking/switch-port-settings$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::SwitchPortSettingsCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct NetworkingSwitchPortSettingsCreateThen(::httpmock::Then);
    impl NetworkingSwitchPortSettingsCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::SwitchPortSettings) -> Self {
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

    pub struct NetworkingSwitchPortSettingsDeleteWhen(::httpmock::When);
    impl NetworkingSwitchPortSettingsDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::DELETE).path_matches(
                regex::Regex::new("^/v1/system/networking/switch-port-settings$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct NetworkingSwitchPortSettingsDeleteThen(::httpmock::Then);
    impl NetworkingSwitchPortSettingsDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct NetworkingSwitchPortSettingsViewWhen(::httpmock::When);
    impl NetworkingSwitchPortSettingsViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/system/networking/switch-port-settings/[^/]*$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct NetworkingSwitchPortSettingsViewThen(::httpmock::Then);
    impl NetworkingSwitchPortSettingsViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SwitchPortSettings) -> Self {
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

    pub struct SystemPolicyViewWhen(::httpmock::When);
    impl SystemPolicyViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct SystemPolicyViewThen(::httpmock::Then);
    impl SystemPolicyViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SystemPolicyUpdateWhen(::httpmock::When);
    impl SystemPolicyUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/system/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::FleetRolePolicy) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SystemPolicyUpdateThen(::httpmock::Then);
    impl SystemPolicyUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct RoleListWhen(::httpmock::When);
    impl RoleListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/roles$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct RoleListThen(::httpmock::Then);
    impl RoleListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct RoleViewWhen(::httpmock::When);
    impl RoleViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/roles/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn role_name(self, value: &str) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/system/roles/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct RoleViewThen(::httpmock::Then);
    impl RoleViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SystemQuotasListWhen(::httpmock::When);
    impl SystemQuotasListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/silo-quotas$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct SystemQuotasListThen(::httpmock::Then);
    impl SystemQuotasListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SiloQuotasResultsPage) -> Self {
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

    pub struct SiloListWhen(::httpmock::When);
    impl SiloListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/silos$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct SiloListThen(::httpmock::Then);
    impl SiloListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SiloCreateWhen(::httpmock::When);
    impl SiloCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/system/silos$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::SiloCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SiloCreateThen(::httpmock::Then);
    impl SiloCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SiloViewWhen(::httpmock::When);
    impl SiloViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/silos/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/system/silos/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SiloViewThen(::httpmock::Then);
    impl SiloViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SiloDeleteWhen(::httpmock::When);
    impl SiloDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/system/silos/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/system/silos/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SiloDeleteThen(::httpmock::Then);
    impl SiloDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SiloIpPoolListWhen(::httpmock::When);
    impl SiloIpPoolListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/silos/[^/]*/ip-pools$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/silos/{}/ip-pools$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct SiloIpPoolListThen(::httpmock::Then);
    impl SiloIpPoolListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SiloIpPoolResultsPage) -> Self {
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

    pub struct SiloPolicyViewWhen(::httpmock::When);
    impl SiloPolicyViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/silos/[^/]*/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/system/silos/{}/policy$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SiloPolicyViewThen(::httpmock::Then);
    impl SiloPolicyViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SiloPolicyUpdateWhen(::httpmock::When);
    impl SiloPolicyUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/system/silos/[^/]*/policy$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct SiloPolicyUpdateThen(::httpmock::Then);
    impl SiloPolicyUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SiloQuotasViewWhen(::httpmock::When);
    impl SiloQuotasViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/silos/[^/]*/quotas$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/system/silos/{}/quotas$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SiloQuotasViewThen(::httpmock::Then);
    impl SiloQuotasViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SiloQuotas) -> Self {
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

    pub struct SiloQuotasUpdateWhen(::httpmock::When);
    impl SiloQuotasUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/system/silos/[^/]*/quotas$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/system/silos/{}/quotas$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::SiloQuotasUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SiloQuotasUpdateThen(::httpmock::Then);
    impl SiloQuotasUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SiloQuotas) -> Self {
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

    pub struct SystemTimeseriesQueryWhen(::httpmock::When);
    impl SystemTimeseriesQueryWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/system/timeseries/query$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::TimeseriesQuery) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct SystemTimeseriesQueryThen(::httpmock::Then);
    impl SystemTimeseriesQueryThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::OxqlQueryResult) -> Self {
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

    pub struct SystemTimeseriesSchemaListWhen(::httpmock::When);
    impl SystemTimeseriesSchemaListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/timeseries/schemas$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct SystemTimeseriesSchemaListThen(::httpmock::Then);
    impl SystemTimeseriesSchemaListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::TimeseriesSchemaResultsPage) -> Self {
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

    pub struct TargetReleaseViewWhen(::httpmock::When);
    impl TargetReleaseViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/update/target-release$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct TargetReleaseViewThen(::httpmock::Then);
    impl TargetReleaseViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::TargetRelease) -> Self {
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

    pub struct TargetReleaseUpdateWhen(::httpmock::When);
    impl TargetReleaseUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/system/update/target-release$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::SetTargetReleaseParams) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct TargetReleaseUpdateThen(::httpmock::Then);
    impl TargetReleaseUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::TargetRelease) -> Self {
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

    pub struct SiloUserListWhen(::httpmock::When);
    impl SiloUserListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/users$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct SiloUserListThen(::httpmock::Then);
    impl SiloUserListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SiloUserViewWhen(::httpmock::When);
    impl SiloUserViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/users/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn user_id(self, value: &::uuid::Uuid) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/system/users/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn silo(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("silo", value.to_string()))
        }
    }

    pub struct SiloUserViewThen(::httpmock::Then);
    impl SiloUserViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct UserBuiltinListWhen(::httpmock::When);
    impl UserBuiltinListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/users-builtin$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct UserBuiltinListThen(::httpmock::Then);
    impl UserBuiltinListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct UserBuiltinViewWhen(::httpmock::When);
    impl UserBuiltinViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/users-builtin/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn user(self, value: &types::NameOrId) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/system/users-builtin/{}$", value.to_string()))
                    .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct UserBuiltinViewThen(::httpmock::Then);
    impl UserBuiltinViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct SiloUtilizationListWhen(::httpmock::When);
    impl SiloUtilizationListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/system/utilization/silos$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct SiloUtilizationListThen(::httpmock::Then);
    impl SiloUtilizationListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SiloUtilizationResultsPage) -> Self {
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

    pub struct SiloUtilizationViewWhen(::httpmock::When);
    impl SiloUtilizationViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner.method(::httpmock::Method::GET).path_matches(
                    regex::Regex::new("^/v1/system/utilization/silos/[^/]*$").unwrap(),
                ),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn silo(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!(
                "^/v1/system/utilization/silos/{}$",
                value.to_string()
            ))
            .unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct SiloUtilizationViewThen(::httpmock::Then);
    impl SiloUtilizationViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::SiloUtilization) -> Self {
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

    pub struct TimeseriesQueryWhen(::httpmock::When);
    impl TimeseriesQueryWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/timeseries/query$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("project", value.to_string()))
        }

        pub fn body(self, value: &types::TimeseriesQuery) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct TimeseriesQueryThen(::httpmock::Then);
    impl TimeseriesQueryThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::OxqlQueryResult) -> Self {
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

    pub struct UserListWhen(::httpmock::When);
    impl UserListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/users$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn group<'a, T>(self, value: T) -> Self
        where
            T: Into<Option<&'a ::uuid::Uuid>>,
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
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct UserListThen(::httpmock::Then);
    impl UserListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct UtilizationViewWhen(::httpmock::When);
    impl UtilizationViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/utilization$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }
    }

    pub struct UtilizationViewThen(::httpmock::Then);
    impl UtilizationViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::Utilization) -> Self {
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

    pub struct VpcFirewallRulesViewWhen(::httpmock::When);
    impl VpcFirewallRulesViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/vpc-firewall-rules$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct VpcFirewallRulesViewThen(::httpmock::Then);
    impl VpcFirewallRulesViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct VpcFirewallRulesUpdateWhen(::httpmock::When);
    impl VpcFirewallRulesUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/vpc-firewall-rules$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct VpcFirewallRulesUpdateThen(::httpmock::Then);
    impl VpcFirewallRulesUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct VpcRouterRouteListWhen(::httpmock::When);
    impl VpcRouterRouteListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/vpc-router-routes$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct VpcRouterRouteListThen(::httpmock::Then);
    impl VpcRouterRouteListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct VpcRouterRouteCreateWhen(::httpmock::When);
    impl VpcRouterRouteCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/vpc-router-routes$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct VpcRouterRouteCreateThen(::httpmock::Then);
    impl VpcRouterRouteCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct VpcRouterRouteViewWhen(::httpmock::When);
    impl VpcRouterRouteViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/vpc-router-routes/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct VpcRouterRouteViewThen(::httpmock::Then);
    impl VpcRouterRouteViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct VpcRouterRouteUpdateWhen(::httpmock::When);
    impl VpcRouterRouteUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/vpc-router-routes/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct VpcRouterRouteUpdateThen(::httpmock::Then);
    impl VpcRouterRouteUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct VpcRouterRouteDeleteWhen(::httpmock::When);
    impl VpcRouterRouteDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/vpc-router-routes/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct VpcRouterRouteDeleteThen(::httpmock::Then);
    impl VpcRouterRouteDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct VpcRouterListWhen(::httpmock::When);
    impl VpcRouterListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/vpc-routers$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct VpcRouterListThen(::httpmock::Then);
    impl VpcRouterListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct VpcRouterCreateWhen(::httpmock::When);
    impl VpcRouterCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/vpc-routers$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct VpcRouterCreateThen(::httpmock::Then);
    impl VpcRouterCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct VpcRouterViewWhen(::httpmock::When);
    impl VpcRouterViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/vpc-routers/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct VpcRouterViewThen(::httpmock::Then);
    impl VpcRouterViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct VpcRouterUpdateWhen(::httpmock::When);
    impl VpcRouterUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/vpc-routers/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct VpcRouterUpdateThen(::httpmock::Then);
    impl VpcRouterUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct VpcRouterDeleteWhen(::httpmock::When);
    impl VpcRouterDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/vpc-routers/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct VpcRouterDeleteThen(::httpmock::Then);
    impl VpcRouterDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct VpcSubnetListWhen(::httpmock::When);
    impl VpcSubnetListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/vpc-subnets$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct VpcSubnetListThen(::httpmock::Then);
    impl VpcSubnetListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct VpcSubnetCreateWhen(::httpmock::When);
    impl VpcSubnetCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/vpc-subnets$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct VpcSubnetCreateThen(::httpmock::Then);
    impl VpcSubnetCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct VpcSubnetViewWhen(::httpmock::When);
    impl VpcSubnetViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/vpc-subnets/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct VpcSubnetViewThen(::httpmock::Then);
    impl VpcSubnetViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct VpcSubnetUpdateWhen(::httpmock::When);
    impl VpcSubnetUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/vpc-subnets/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct VpcSubnetUpdateThen(::httpmock::Then);
    impl VpcSubnetUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct VpcSubnetDeleteWhen(::httpmock::When);
    impl VpcSubnetDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/vpc-subnets/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct VpcSubnetDeleteThen(::httpmock::Then);
    impl VpcSubnetDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct VpcSubnetListNetworkInterfacesWhen(::httpmock::When);
    impl VpcSubnetListNetworkInterfacesWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(inner.method(::httpmock::Method::GET).path_matches(
                regex::Regex::new("^/v1/vpc-subnets/[^/]*/network-interfaces$").unwrap(),
            ))
        }

        pub fn into_inner(self) -> ::httpmock::When {
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
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct VpcSubnetListNetworkInterfacesThen(::httpmock::Then);
    impl VpcSubnetListNetworkInterfacesThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct VpcListWhen(::httpmock::When);
    impl VpcListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/vpcs$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn limit<T>(self, value: T) -> Self
        where
            T: Into<Option<::std::num::NonZeroU32>>,
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

    pub struct VpcListThen(::httpmock::Then);
    impl VpcListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct VpcCreateWhen(::httpmock::When);
    impl VpcCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/vpcs$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn project(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("project", value.to_string()))
        }

        pub fn body(self, value: &types::VpcCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct VpcCreateThen(::httpmock::Then);
    impl VpcCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct VpcViewWhen(::httpmock::When);
    impl VpcViewWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/vpcs/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct VpcViewThen(::httpmock::Then);
    impl VpcViewThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct VpcUpdateWhen(::httpmock::When);
    impl VpcUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/vpcs/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct VpcUpdateThen(::httpmock::Then);
    impl VpcUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct VpcDeleteWhen(::httpmock::When);
    impl VpcDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/vpcs/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
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

    pub struct VpcDeleteThen(::httpmock::Then);
    impl VpcDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct WebhookReceiverCreateWhen(::httpmock::When);
    impl WebhookReceiverCreateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/webhook-receivers$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn body(self, value: &types::WebhookCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct WebhookReceiverCreateThen(::httpmock::Then);
    impl WebhookReceiverCreateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::WebhookReceiver) -> Self {
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

    pub struct WebhookReceiverUpdateWhen(::httpmock::When);
    impl WebhookReceiverUpdateWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::PUT)
                    .path_matches(regex::Regex::new("^/v1/webhook-receivers/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn receiver(self, value: &types::NameOrId) -> Self {
            let re = regex::Regex::new(&format!("^/v1/webhook-receivers/{}$", value.to_string()))
                .unwrap();
            Self(self.0.path_matches(re))
        }

        pub fn body(self, value: &types::WebhookReceiverUpdate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct WebhookReceiverUpdateThen(::httpmock::Then);
    impl WebhookReceiverUpdateThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

    pub struct WebhookSecretsListWhen(::httpmock::When);
    impl WebhookSecretsListWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::GET)
                    .path_matches(regex::Regex::new("^/v1/webhook-secrets$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn receiver(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("receiver", value.to_string()))
        }
    }

    pub struct WebhookSecretsListThen(::httpmock::Then);
    impl WebhookSecretsListThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn ok(self, value: &types::WebhookSecrets) -> Self {
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

    pub struct WebhookSecretsAddWhen(::httpmock::When);
    impl WebhookSecretsAddWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::POST)
                    .path_matches(regex::Regex::new("^/v1/webhook-secrets$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn receiver(self, value: &types::NameOrId) -> Self {
            Self(self.0.query_param("receiver", value.to_string()))
        }

        pub fn body(self, value: &types::WebhookSecretCreate) -> Self {
            Self(self.0.json_body_obj(value))
        }
    }

    pub struct WebhookSecretsAddThen(::httpmock::Then);
    impl WebhookSecretsAddThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
            self.0
        }

        pub fn created(self, value: &types::WebhookSecret) -> Self {
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

    pub struct WebhookSecretsDeleteWhen(::httpmock::When);
    impl WebhookSecretsDeleteWhen {
        pub fn new(inner: ::httpmock::When) -> Self {
            Self(
                inner
                    .method(::httpmock::Method::DELETE)
                    .path_matches(regex::Regex::new("^/v1/webhook-secrets/[^/]*$").unwrap()),
            )
        }

        pub fn into_inner(self) -> ::httpmock::When {
            self.0
        }

        pub fn secret_id(self, value: &::uuid::Uuid) -> Self {
            let re =
                regex::Regex::new(&format!("^/v1/webhook-secrets/{}$", value.to_string())).unwrap();
            Self(self.0.path_matches(re))
        }
    }

    pub struct WebhookSecretsDeleteThen(::httpmock::Then);
    impl WebhookSecretsDeleteThen {
        pub fn new(inner: ::httpmock::Then) -> Self {
            Self(inner)
        }

        pub fn into_inner(self) -> ::httpmock::Then {
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

/// An extension trait for [`MockServer`](::httpmock::MockServer) that
/// adds a method for each operation. These are the equivalent of
/// type-checked [`mock()`](::httpmock::MockServer::mock) calls.
pub trait MockServerExt {
    fn device_auth_request<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::DeviceAuthRequestWhen, operations::DeviceAuthRequestThen);
    fn device_auth_confirm<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::DeviceAuthConfirmWhen, operations::DeviceAuthConfirmThen);
    fn device_access_token<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::DeviceAccessTokenWhen, operations::DeviceAccessTokenThen);
    fn probe_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ProbeListWhen, operations::ProbeListThen);
    fn probe_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ProbeCreateWhen, operations::ProbeCreateThen);
    fn probe_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ProbeViewWhen, operations::ProbeViewThen);
    fn probe_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ProbeDeleteWhen, operations::ProbeDeleteThen);
    fn support_bundle_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SupportBundleListWhen, operations::SupportBundleListThen);
    fn support_bundle_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SupportBundleCreateWhen, operations::SupportBundleCreateThen);
    fn support_bundle_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SupportBundleViewWhen, operations::SupportBundleViewThen);
    fn support_bundle_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SupportBundleDeleteWhen, operations::SupportBundleDeleteThen);
    fn support_bundle_download<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SupportBundleDownloadWhen, operations::SupportBundleDownloadThen);
    fn support_bundle_head<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SupportBundleHeadWhen, operations::SupportBundleHeadThen);
    fn support_bundle_download_file<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::SupportBundleDownloadFileWhen,
            operations::SupportBundleDownloadFileThen,
        );
    fn support_bundle_head_file<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SupportBundleHeadFileWhen, operations::SupportBundleHeadFileThen);
    fn support_bundle_index<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SupportBundleIndexWhen, operations::SupportBundleIndexThen);
    fn login_saml<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::LoginSamlWhen, operations::LoginSamlThen);
    fn affinity_group_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AffinityGroupListWhen, operations::AffinityGroupListThen);
    fn affinity_group_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AffinityGroupCreateWhen, operations::AffinityGroupCreateThen);
    fn affinity_group_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AffinityGroupViewWhen, operations::AffinityGroupViewThen);
    fn affinity_group_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AffinityGroupUpdateWhen, operations::AffinityGroupUpdateThen);
    fn affinity_group_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AffinityGroupDeleteWhen, operations::AffinityGroupDeleteThen);
    fn affinity_group_member_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AffinityGroupMemberListWhen, operations::AffinityGroupMemberListThen);
    fn affinity_group_member_instance_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::AffinityGroupMemberInstanceViewWhen,
            operations::AffinityGroupMemberInstanceViewThen,
        );
    fn affinity_group_member_instance_add<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::AffinityGroupMemberInstanceAddWhen,
            operations::AffinityGroupMemberInstanceAddThen,
        );
    fn affinity_group_member_instance_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::AffinityGroupMemberInstanceDeleteWhen,
            operations::AffinityGroupMemberInstanceDeleteThen,
        );
    fn alert_class_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AlertClassListWhen, operations::AlertClassListThen);
    fn alert_receiver_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AlertReceiverListWhen, operations::AlertReceiverListThen);
    fn alert_receiver_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AlertReceiverViewWhen, operations::AlertReceiverViewThen);
    fn alert_receiver_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AlertReceiverDeleteWhen, operations::AlertReceiverDeleteThen);
    fn alert_delivery_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AlertDeliveryListWhen, operations::AlertDeliveryListThen);
    fn alert_receiver_probe<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AlertReceiverProbeWhen, operations::AlertReceiverProbeThen);
    fn alert_receiver_subscription_add<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::AlertReceiverSubscriptionAddWhen,
            operations::AlertReceiverSubscriptionAddThen,
        );
    fn alert_receiver_subscription_remove<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::AlertReceiverSubscriptionRemoveWhen,
            operations::AlertReceiverSubscriptionRemoveThen,
        );
    fn alert_delivery_resend<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AlertDeliveryResendWhen, operations::AlertDeliveryResendThen);
    fn anti_affinity_group_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AntiAffinityGroupListWhen, operations::AntiAffinityGroupListThen);
    fn anti_affinity_group_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AntiAffinityGroupCreateWhen, operations::AntiAffinityGroupCreateThen);
    fn anti_affinity_group_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AntiAffinityGroupViewWhen, operations::AntiAffinityGroupViewThen);
    fn anti_affinity_group_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AntiAffinityGroupUpdateWhen, operations::AntiAffinityGroupUpdateThen);
    fn anti_affinity_group_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AntiAffinityGroupDeleteWhen, operations::AntiAffinityGroupDeleteThen);
    fn anti_affinity_group_member_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::AntiAffinityGroupMemberListWhen,
            operations::AntiAffinityGroupMemberListThen,
        );
    fn anti_affinity_group_member_instance_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::AntiAffinityGroupMemberInstanceViewWhen,
            operations::AntiAffinityGroupMemberInstanceViewThen,
        );
    fn anti_affinity_group_member_instance_add<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::AntiAffinityGroupMemberInstanceAddWhen,
            operations::AntiAffinityGroupMemberInstanceAddThen,
        );
    fn anti_affinity_group_member_instance_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::AntiAffinityGroupMemberInstanceDeleteWhen,
            operations::AntiAffinityGroupMemberInstanceDeleteThen,
        );
    fn auth_settings_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AuthSettingsViewWhen, operations::AuthSettingsViewThen);
    fn auth_settings_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AuthSettingsUpdateWhen, operations::AuthSettingsUpdateThen);
    fn certificate_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::CertificateListWhen, operations::CertificateListThen);
    fn certificate_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::CertificateCreateWhen, operations::CertificateCreateThen);
    fn certificate_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::CertificateViewWhen, operations::CertificateViewThen);
    fn certificate_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::CertificateDeleteWhen, operations::CertificateDeleteThen);
    fn disk_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::DiskListWhen, operations::DiskListThen);
    fn disk_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::DiskCreateWhen, operations::DiskCreateThen);
    fn disk_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::DiskViewWhen, operations::DiskViewThen);
    fn disk_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::DiskDeleteWhen, operations::DiskDeleteThen);
    fn disk_bulk_write_import<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::DiskBulkWriteImportWhen, operations::DiskBulkWriteImportThen);
    fn disk_bulk_write_import_start<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::DiskBulkWriteImportStartWhen,
            operations::DiskBulkWriteImportStartThen,
        );
    fn disk_bulk_write_import_stop<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::DiskBulkWriteImportStopWhen, operations::DiskBulkWriteImportStopThen);
    fn disk_finalize_import<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::DiskFinalizeImportWhen, operations::DiskFinalizeImportThen);
    fn disk_metrics_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::DiskMetricsListWhen, operations::DiskMetricsListThen);
    fn floating_ip_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::FloatingIpListWhen, operations::FloatingIpListThen);
    fn floating_ip_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::FloatingIpCreateWhen, operations::FloatingIpCreateThen);
    fn floating_ip_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::FloatingIpViewWhen, operations::FloatingIpViewThen);
    fn floating_ip_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::FloatingIpUpdateWhen, operations::FloatingIpUpdateThen);
    fn floating_ip_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::FloatingIpDeleteWhen, operations::FloatingIpDeleteThen);
    fn floating_ip_attach<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::FloatingIpAttachWhen, operations::FloatingIpAttachThen);
    fn floating_ip_detach<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::FloatingIpDetachWhen, operations::FloatingIpDetachThen);
    fn group_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::GroupListWhen, operations::GroupListThen);
    fn group_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::GroupViewWhen, operations::GroupViewThen);
    fn image_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ImageListWhen, operations::ImageListThen);
    fn image_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ImageCreateWhen, operations::ImageCreateThen);
    fn image_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ImageViewWhen, operations::ImageViewThen);
    fn image_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ImageDeleteWhen, operations::ImageDeleteThen);
    fn image_demote<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ImageDemoteWhen, operations::ImageDemoteThen);
    fn image_promote<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ImagePromoteWhen, operations::ImagePromoteThen);
    fn instance_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstanceListWhen, operations::InstanceListThen);
    fn instance_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstanceCreateWhen, operations::InstanceCreateThen);
    fn instance_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstanceViewWhen, operations::InstanceViewThen);
    fn instance_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstanceUpdateWhen, operations::InstanceUpdateThen);
    fn instance_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstanceDeleteWhen, operations::InstanceDeleteThen);
    fn instance_affinity_group_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceAffinityGroupListWhen,
            operations::InstanceAffinityGroupListThen,
        );
    fn instance_anti_affinity_group_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceAntiAffinityGroupListWhen,
            operations::InstanceAntiAffinityGroupListThen,
        );
    fn instance_disk_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstanceDiskListWhen, operations::InstanceDiskListThen);
    fn instance_disk_attach<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstanceDiskAttachWhen, operations::InstanceDiskAttachThen);
    fn instance_disk_detach<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstanceDiskDetachWhen, operations::InstanceDiskDetachThen);
    fn instance_external_ip_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstanceExternalIpListWhen, operations::InstanceExternalIpListThen);
    fn instance_ephemeral_ip_attach<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceEphemeralIpAttachWhen,
            operations::InstanceEphemeralIpAttachThen,
        );
    fn instance_ephemeral_ip_detach<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceEphemeralIpDetachWhen,
            operations::InstanceEphemeralIpDetachThen,
        );
    fn instance_reboot<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstanceRebootWhen, operations::InstanceRebootThen);
    fn instance_serial_console<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstanceSerialConsoleWhen, operations::InstanceSerialConsoleThen);
    fn instance_serial_console_stream<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceSerialConsoleStreamWhen,
            operations::InstanceSerialConsoleStreamThen,
        );
    fn instance_ssh_public_key_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceSshPublicKeyListWhen,
            operations::InstanceSshPublicKeyListThen,
        );
    fn instance_start<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstanceStartWhen, operations::InstanceStartThen);
    fn instance_stop<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstanceStopWhen, operations::InstanceStopThen);
    fn internet_gateway_ip_address_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InternetGatewayIpAddressListWhen,
            operations::InternetGatewayIpAddressListThen,
        );
    fn internet_gateway_ip_address_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InternetGatewayIpAddressCreateWhen,
            operations::InternetGatewayIpAddressCreateThen,
        );
    fn internet_gateway_ip_address_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InternetGatewayIpAddressDeleteWhen,
            operations::InternetGatewayIpAddressDeleteThen,
        );
    fn internet_gateway_ip_pool_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InternetGatewayIpPoolListWhen,
            operations::InternetGatewayIpPoolListThen,
        );
    fn internet_gateway_ip_pool_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InternetGatewayIpPoolCreateWhen,
            operations::InternetGatewayIpPoolCreateThen,
        );
    fn internet_gateway_ip_pool_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InternetGatewayIpPoolDeleteWhen,
            operations::InternetGatewayIpPoolDeleteThen,
        );
    fn internet_gateway_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InternetGatewayListWhen, operations::InternetGatewayListThen);
    fn internet_gateway_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InternetGatewayCreateWhen, operations::InternetGatewayCreateThen);
    fn internet_gateway_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InternetGatewayViewWhen, operations::InternetGatewayViewThen);
    fn internet_gateway_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InternetGatewayDeleteWhen, operations::InternetGatewayDeleteThen);
    fn project_ip_pool_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ProjectIpPoolListWhen, operations::ProjectIpPoolListThen);
    fn project_ip_pool_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ProjectIpPoolViewWhen, operations::ProjectIpPoolViewThen);
    fn login_local<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::LoginLocalWhen, operations::LoginLocalThen);
    fn logout<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::LogoutWhen, operations::LogoutThen);
    fn current_user_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::CurrentUserViewWhen, operations::CurrentUserViewThen);
    fn current_user_groups<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::CurrentUserGroupsWhen, operations::CurrentUserGroupsThen);
    fn current_user_ssh_key_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::CurrentUserSshKeyListWhen, operations::CurrentUserSshKeyListThen);
    fn current_user_ssh_key_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::CurrentUserSshKeyCreateWhen, operations::CurrentUserSshKeyCreateThen);
    fn current_user_ssh_key_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::CurrentUserSshKeyViewWhen, operations::CurrentUserSshKeyViewThen);
    fn current_user_ssh_key_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::CurrentUserSshKeyDeleteWhen, operations::CurrentUserSshKeyDeleteThen);
    fn silo_metric<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SiloMetricWhen, operations::SiloMetricThen);
    fn instance_network_interface_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceListWhen,
            operations::InstanceNetworkInterfaceListThen,
        );
    fn instance_network_interface_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceCreateWhen,
            operations::InstanceNetworkInterfaceCreateThen,
        );
    fn instance_network_interface_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceViewWhen,
            operations::InstanceNetworkInterfaceViewThen,
        );
    fn instance_network_interface_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceUpdateWhen,
            operations::InstanceNetworkInterfaceUpdateThen,
        );
    fn instance_network_interface_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceNetworkInterfaceDeleteWhen,
            operations::InstanceNetworkInterfaceDeleteThen,
        );
    fn ping<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::PingWhen, operations::PingThen);
    fn policy_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::PolicyViewWhen, operations::PolicyViewThen);
    fn policy_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::PolicyUpdateWhen, operations::PolicyUpdateThen);
    fn project_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ProjectListWhen, operations::ProjectListThen);
    fn project_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ProjectCreateWhen, operations::ProjectCreateThen);
    fn project_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ProjectViewWhen, operations::ProjectViewThen);
    fn project_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ProjectUpdateWhen, operations::ProjectUpdateThen);
    fn project_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ProjectDeleteWhen, operations::ProjectDeleteThen);
    fn project_policy_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ProjectPolicyViewWhen, operations::ProjectPolicyViewThen);
    fn project_policy_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ProjectPolicyUpdateWhen, operations::ProjectPolicyUpdateThen);
    fn snapshot_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SnapshotListWhen, operations::SnapshotListThen);
    fn snapshot_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SnapshotCreateWhen, operations::SnapshotCreateThen);
    fn snapshot_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SnapshotViewWhen, operations::SnapshotViewThen);
    fn snapshot_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SnapshotDeleteWhen, operations::SnapshotDeleteThen);
    fn physical_disk_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::PhysicalDiskListWhen, operations::PhysicalDiskListThen);
    fn physical_disk_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::PhysicalDiskViewWhen, operations::PhysicalDiskViewThen);
    fn networking_switch_port_lldp_neighbors<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortLldpNeighborsWhen,
            operations::NetworkingSwitchPortLldpNeighborsThen,
        );
    fn rack_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::RackListWhen, operations::RackListThen);
    fn rack_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::RackViewWhen, operations::RackViewThen);
    fn sled_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SledListWhen, operations::SledListThen);
    fn sled_add<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SledAddWhen, operations::SledAddThen);
    fn sled_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SledViewWhen, operations::SledViewThen);
    fn sled_physical_disk_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SledPhysicalDiskListWhen, operations::SledPhysicalDiskListThen);
    fn sled_instance_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SledInstanceListWhen, operations::SledInstanceListThen);
    fn sled_set_provision_policy<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SledSetProvisionPolicyWhen, operations::SledSetProvisionPolicyThen);
    fn sled_list_uninitialized<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SledListUninitializedWhen, operations::SledListUninitializedThen);
    fn networking_switch_port_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortListWhen,
            operations::NetworkingSwitchPortListThen,
        );
    fn networking_switch_port_lldp_config_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortLldpConfigViewWhen,
            operations::NetworkingSwitchPortLldpConfigViewThen,
        );
    fn networking_switch_port_lldp_config_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortLldpConfigUpdateWhen,
            operations::NetworkingSwitchPortLldpConfigUpdateThen,
        );
    fn networking_switch_port_apply_settings<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortApplySettingsWhen,
            operations::NetworkingSwitchPortApplySettingsThen,
        );
    fn networking_switch_port_clear_settings<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortClearSettingsWhen,
            operations::NetworkingSwitchPortClearSettingsThen,
        );
    fn networking_switch_port_status<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortStatusWhen,
            operations::NetworkingSwitchPortStatusThen,
        );
    fn switch_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SwitchListWhen, operations::SwitchListThen);
    fn switch_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SwitchViewWhen, operations::SwitchViewThen);
    fn silo_identity_provider_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::SiloIdentityProviderListWhen,
            operations::SiloIdentityProviderListThen,
        );
    fn local_idp_user_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::LocalIdpUserCreateWhen, operations::LocalIdpUserCreateThen);
    fn local_idp_user_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::LocalIdpUserDeleteWhen, operations::LocalIdpUserDeleteThen);
    fn local_idp_user_set_password<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::LocalIdpUserSetPasswordWhen, operations::LocalIdpUserSetPasswordThen);
    fn saml_identity_provider_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::SamlIdentityProviderCreateWhen,
            operations::SamlIdentityProviderCreateThen,
        );
    fn saml_identity_provider_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::SamlIdentityProviderViewWhen,
            operations::SamlIdentityProviderViewThen,
        );
    fn ip_pool_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::IpPoolListWhen, operations::IpPoolListThen);
    fn ip_pool_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::IpPoolCreateWhen, operations::IpPoolCreateThen);
    fn ip_pool_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::IpPoolViewWhen, operations::IpPoolViewThen);
    fn ip_pool_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::IpPoolUpdateWhen, operations::IpPoolUpdateThen);
    fn ip_pool_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::IpPoolDeleteWhen, operations::IpPoolDeleteThen);
    fn ip_pool_range_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::IpPoolRangeListWhen, operations::IpPoolRangeListThen);
    fn ip_pool_range_add<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::IpPoolRangeAddWhen, operations::IpPoolRangeAddThen);
    fn ip_pool_range_remove<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::IpPoolRangeRemoveWhen, operations::IpPoolRangeRemoveThen);
    fn ip_pool_silo_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::IpPoolSiloListWhen, operations::IpPoolSiloListThen);
    fn ip_pool_silo_link<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::IpPoolSiloLinkWhen, operations::IpPoolSiloLinkThen);
    fn ip_pool_silo_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::IpPoolSiloUpdateWhen, operations::IpPoolSiloUpdateThen);
    fn ip_pool_silo_unlink<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::IpPoolSiloUnlinkWhen, operations::IpPoolSiloUnlinkThen);
    fn ip_pool_utilization_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::IpPoolUtilizationViewWhen, operations::IpPoolUtilizationViewThen);
    fn ip_pool_service_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::IpPoolServiceViewWhen, operations::IpPoolServiceViewThen);
    fn ip_pool_service_range_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::IpPoolServiceRangeListWhen, operations::IpPoolServiceRangeListThen);
    fn ip_pool_service_range_add<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::IpPoolServiceRangeAddWhen, operations::IpPoolServiceRangeAddThen);
    fn ip_pool_service_range_remove<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::IpPoolServiceRangeRemoveWhen,
            operations::IpPoolServiceRangeRemoveThen,
        );
    fn system_metric<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SystemMetricWhen, operations::SystemMetricThen);
    fn networking_address_lot_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingAddressLotListWhen,
            operations::NetworkingAddressLotListThen,
        );
    fn networking_address_lot_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingAddressLotCreateWhen,
            operations::NetworkingAddressLotCreateThen,
        );
    fn networking_address_lot_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingAddressLotDeleteWhen,
            operations::NetworkingAddressLotDeleteThen,
        );
    fn networking_address_lot_block_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingAddressLotBlockListWhen,
            operations::NetworkingAddressLotBlockListThen,
        );
    fn networking_allow_list_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::NetworkingAllowListViewWhen, operations::NetworkingAllowListViewThen);
    fn networking_allow_list_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingAllowListUpdateWhen,
            operations::NetworkingAllowListUpdateThen,
        );
    fn networking_bfd_disable<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::NetworkingBfdDisableWhen, operations::NetworkingBfdDisableThen);
    fn networking_bfd_enable<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::NetworkingBfdEnableWhen, operations::NetworkingBfdEnableThen);
    fn networking_bfd_status<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::NetworkingBfdStatusWhen, operations::NetworkingBfdStatusThen);
    fn networking_bgp_config_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::NetworkingBgpConfigListWhen, operations::NetworkingBgpConfigListThen);
    fn networking_bgp_config_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingBgpConfigCreateWhen,
            operations::NetworkingBgpConfigCreateThen,
        );
    fn networking_bgp_config_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingBgpConfigDeleteWhen,
            operations::NetworkingBgpConfigDeleteThen,
        );
    fn networking_bgp_announce_set_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingBgpAnnounceSetListWhen,
            operations::NetworkingBgpAnnounceSetListThen,
        );
    fn networking_bgp_announce_set_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingBgpAnnounceSetUpdateWhen,
            operations::NetworkingBgpAnnounceSetUpdateThen,
        );
    fn networking_bgp_announce_set_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingBgpAnnounceSetDeleteWhen,
            operations::NetworkingBgpAnnounceSetDeleteThen,
        );
    fn networking_bgp_announcement_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingBgpAnnouncementListWhen,
            operations::NetworkingBgpAnnouncementListThen,
        );
    fn networking_bgp_exported<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::NetworkingBgpExportedWhen, operations::NetworkingBgpExportedThen);
    fn networking_bgp_message_history<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingBgpMessageHistoryWhen,
            operations::NetworkingBgpMessageHistoryThen,
        );
    fn networking_bgp_imported_routes_ipv4<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingBgpImportedRoutesIpv4When,
            operations::NetworkingBgpImportedRoutesIpv4Then,
        );
    fn networking_bgp_status<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::NetworkingBgpStatusWhen, operations::NetworkingBgpStatusThen);
    fn networking_loopback_address_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingLoopbackAddressListWhen,
            operations::NetworkingLoopbackAddressListThen,
        );
    fn networking_loopback_address_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingLoopbackAddressCreateWhen,
            operations::NetworkingLoopbackAddressCreateThen,
        );
    fn networking_loopback_address_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingLoopbackAddressDeleteWhen,
            operations::NetworkingLoopbackAddressDeleteThen,
        );
    fn networking_switch_port_settings_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortSettingsListWhen,
            operations::NetworkingSwitchPortSettingsListThen,
        );
    fn networking_switch_port_settings_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortSettingsCreateWhen,
            operations::NetworkingSwitchPortSettingsCreateThen,
        );
    fn networking_switch_port_settings_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortSettingsDeleteWhen,
            operations::NetworkingSwitchPortSettingsDeleteThen,
        );
    fn networking_switch_port_settings_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortSettingsViewWhen,
            operations::NetworkingSwitchPortSettingsViewThen,
        );
    fn system_policy_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SystemPolicyViewWhen, operations::SystemPolicyViewThen);
    fn system_policy_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SystemPolicyUpdateWhen, operations::SystemPolicyUpdateThen);
    fn role_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::RoleListWhen, operations::RoleListThen);
    fn role_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::RoleViewWhen, operations::RoleViewThen);
    fn system_quotas_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SystemQuotasListWhen, operations::SystemQuotasListThen);
    fn silo_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SiloListWhen, operations::SiloListThen);
    fn silo_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SiloCreateWhen, operations::SiloCreateThen);
    fn silo_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SiloViewWhen, operations::SiloViewThen);
    fn silo_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SiloDeleteWhen, operations::SiloDeleteThen);
    fn silo_ip_pool_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SiloIpPoolListWhen, operations::SiloIpPoolListThen);
    fn silo_policy_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SiloPolicyViewWhen, operations::SiloPolicyViewThen);
    fn silo_policy_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SiloPolicyUpdateWhen, operations::SiloPolicyUpdateThen);
    fn silo_quotas_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SiloQuotasViewWhen, operations::SiloQuotasViewThen);
    fn silo_quotas_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SiloQuotasUpdateWhen, operations::SiloQuotasUpdateThen);
    fn system_timeseries_query<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SystemTimeseriesQueryWhen, operations::SystemTimeseriesQueryThen);
    fn system_timeseries_schema_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::SystemTimeseriesSchemaListWhen,
            operations::SystemTimeseriesSchemaListThen,
        );
    fn target_release_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::TargetReleaseViewWhen, operations::TargetReleaseViewThen);
    fn target_release_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::TargetReleaseUpdateWhen, operations::TargetReleaseUpdateThen);
    fn silo_user_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SiloUserListWhen, operations::SiloUserListThen);
    fn silo_user_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SiloUserViewWhen, operations::SiloUserViewThen);
    fn user_builtin_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::UserBuiltinListWhen, operations::UserBuiltinListThen);
    fn user_builtin_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::UserBuiltinViewWhen, operations::UserBuiltinViewThen);
    fn silo_utilization_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SiloUtilizationListWhen, operations::SiloUtilizationListThen);
    fn silo_utilization_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SiloUtilizationViewWhen, operations::SiloUtilizationViewThen);
    fn timeseries_query<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::TimeseriesQueryWhen, operations::TimeseriesQueryThen);
    fn user_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::UserListWhen, operations::UserListThen);
    fn utilization_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::UtilizationViewWhen, operations::UtilizationViewThen);
    fn vpc_firewall_rules_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::VpcFirewallRulesViewWhen, operations::VpcFirewallRulesViewThen);
    fn vpc_firewall_rules_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::VpcFirewallRulesUpdateWhen, operations::VpcFirewallRulesUpdateThen);
    fn vpc_router_route_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterRouteListWhen, operations::VpcRouterRouteListThen);
    fn vpc_router_route_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterRouteCreateWhen, operations::VpcRouterRouteCreateThen);
    fn vpc_router_route_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterRouteViewWhen, operations::VpcRouterRouteViewThen);
    fn vpc_router_route_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterRouteUpdateWhen, operations::VpcRouterRouteUpdateThen);
    fn vpc_router_route_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterRouteDeleteWhen, operations::VpcRouterRouteDeleteThen);
    fn vpc_router_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterListWhen, operations::VpcRouterListThen);
    fn vpc_router_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterCreateWhen, operations::VpcRouterCreateThen);
    fn vpc_router_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterViewWhen, operations::VpcRouterViewThen);
    fn vpc_router_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterUpdateWhen, operations::VpcRouterUpdateThen);
    fn vpc_router_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::VpcRouterDeleteWhen, operations::VpcRouterDeleteThen);
    fn vpc_subnet_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::VpcSubnetListWhen, operations::VpcSubnetListThen);
    fn vpc_subnet_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::VpcSubnetCreateWhen, operations::VpcSubnetCreateThen);
    fn vpc_subnet_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::VpcSubnetViewWhen, operations::VpcSubnetViewThen);
    fn vpc_subnet_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::VpcSubnetUpdateWhen, operations::VpcSubnetUpdateThen);
    fn vpc_subnet_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::VpcSubnetDeleteWhen, operations::VpcSubnetDeleteThen);
    fn vpc_subnet_list_network_interfaces<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::VpcSubnetListNetworkInterfacesWhen,
            operations::VpcSubnetListNetworkInterfacesThen,
        );
    fn vpc_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::VpcListWhen, operations::VpcListThen);
    fn vpc_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::VpcCreateWhen, operations::VpcCreateThen);
    fn vpc_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::VpcViewWhen, operations::VpcViewThen);
    fn vpc_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::VpcUpdateWhen, operations::VpcUpdateThen);
    fn vpc_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::VpcDeleteWhen, operations::VpcDeleteThen);
    fn webhook_receiver_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::WebhookReceiverCreateWhen, operations::WebhookReceiverCreateThen);
    fn webhook_receiver_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::WebhookReceiverUpdateWhen, operations::WebhookReceiverUpdateThen);
    fn webhook_secrets_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::WebhookSecretsListWhen, operations::WebhookSecretsListThen);
    fn webhook_secrets_add<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::WebhookSecretsAddWhen, operations::WebhookSecretsAddThen);
    fn webhook_secrets_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::WebhookSecretsDeleteWhen, operations::WebhookSecretsDeleteThen);
}

impl MockServerExt for ::httpmock::MockServer {
    fn device_auth_request<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn device_auth_confirm<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn device_access_token<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn probe_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ProbeListWhen, operations::ProbeListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProbeListWhen::new(when),
                operations::ProbeListThen::new(then),
            )
        })
    }

    fn probe_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ProbeCreateWhen, operations::ProbeCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProbeCreateWhen::new(when),
                operations::ProbeCreateThen::new(then),
            )
        })
    }

    fn probe_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ProbeViewWhen, operations::ProbeViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProbeViewWhen::new(when),
                operations::ProbeViewThen::new(then),
            )
        })
    }

    fn probe_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ProbeDeleteWhen, operations::ProbeDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProbeDeleteWhen::new(when),
                operations::ProbeDeleteThen::new(then),
            )
        })
    }

    fn support_bundle_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SupportBundleListWhen, operations::SupportBundleListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SupportBundleListWhen::new(when),
                operations::SupportBundleListThen::new(then),
            )
        })
    }

    fn support_bundle_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SupportBundleCreateWhen, operations::SupportBundleCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SupportBundleCreateWhen::new(when),
                operations::SupportBundleCreateThen::new(then),
            )
        })
    }

    fn support_bundle_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SupportBundleViewWhen, operations::SupportBundleViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SupportBundleViewWhen::new(when),
                operations::SupportBundleViewThen::new(then),
            )
        })
    }

    fn support_bundle_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SupportBundleDeleteWhen, operations::SupportBundleDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SupportBundleDeleteWhen::new(when),
                operations::SupportBundleDeleteThen::new(then),
            )
        })
    }

    fn support_bundle_download<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SupportBundleDownloadWhen, operations::SupportBundleDownloadThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SupportBundleDownloadWhen::new(when),
                operations::SupportBundleDownloadThen::new(then),
            )
        })
    }

    fn support_bundle_head<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SupportBundleHeadWhen, operations::SupportBundleHeadThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SupportBundleHeadWhen::new(when),
                operations::SupportBundleHeadThen::new(then),
            )
        })
    }

    fn support_bundle_download_file<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::SupportBundleDownloadFileWhen,
            operations::SupportBundleDownloadFileThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SupportBundleDownloadFileWhen::new(when),
                operations::SupportBundleDownloadFileThen::new(then),
            )
        })
    }

    fn support_bundle_head_file<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SupportBundleHeadFileWhen, operations::SupportBundleHeadFileThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SupportBundleHeadFileWhen::new(when),
                operations::SupportBundleHeadFileThen::new(then),
            )
        })
    }

    fn support_bundle_index<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SupportBundleIndexWhen, operations::SupportBundleIndexThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SupportBundleIndexWhen::new(when),
                operations::SupportBundleIndexThen::new(then),
            )
        })
    }

    fn login_saml<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn affinity_group_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AffinityGroupListWhen, operations::AffinityGroupListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AffinityGroupListWhen::new(when),
                operations::AffinityGroupListThen::new(then),
            )
        })
    }

    fn affinity_group_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AffinityGroupCreateWhen, operations::AffinityGroupCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AffinityGroupCreateWhen::new(when),
                operations::AffinityGroupCreateThen::new(then),
            )
        })
    }

    fn affinity_group_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AffinityGroupViewWhen, operations::AffinityGroupViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AffinityGroupViewWhen::new(when),
                operations::AffinityGroupViewThen::new(then),
            )
        })
    }

    fn affinity_group_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AffinityGroupUpdateWhen, operations::AffinityGroupUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AffinityGroupUpdateWhen::new(when),
                operations::AffinityGroupUpdateThen::new(then),
            )
        })
    }

    fn affinity_group_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AffinityGroupDeleteWhen, operations::AffinityGroupDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AffinityGroupDeleteWhen::new(when),
                operations::AffinityGroupDeleteThen::new(then),
            )
        })
    }

    fn affinity_group_member_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AffinityGroupMemberListWhen, operations::AffinityGroupMemberListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AffinityGroupMemberListWhen::new(when),
                operations::AffinityGroupMemberListThen::new(then),
            )
        })
    }

    fn affinity_group_member_instance_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::AffinityGroupMemberInstanceViewWhen,
            operations::AffinityGroupMemberInstanceViewThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AffinityGroupMemberInstanceViewWhen::new(when),
                operations::AffinityGroupMemberInstanceViewThen::new(then),
            )
        })
    }

    fn affinity_group_member_instance_add<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::AffinityGroupMemberInstanceAddWhen,
            operations::AffinityGroupMemberInstanceAddThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AffinityGroupMemberInstanceAddWhen::new(when),
                operations::AffinityGroupMemberInstanceAddThen::new(then),
            )
        })
    }

    fn affinity_group_member_instance_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::AffinityGroupMemberInstanceDeleteWhen,
            operations::AffinityGroupMemberInstanceDeleteThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AffinityGroupMemberInstanceDeleteWhen::new(when),
                operations::AffinityGroupMemberInstanceDeleteThen::new(then),
            )
        })
    }

    fn alert_class_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AlertClassListWhen, operations::AlertClassListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AlertClassListWhen::new(when),
                operations::AlertClassListThen::new(then),
            )
        })
    }

    fn alert_receiver_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AlertReceiverListWhen, operations::AlertReceiverListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AlertReceiverListWhen::new(when),
                operations::AlertReceiverListThen::new(then),
            )
        })
    }

    fn alert_receiver_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AlertReceiverViewWhen, operations::AlertReceiverViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AlertReceiverViewWhen::new(when),
                operations::AlertReceiverViewThen::new(then),
            )
        })
    }

    fn alert_receiver_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AlertReceiverDeleteWhen, operations::AlertReceiverDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AlertReceiverDeleteWhen::new(when),
                operations::AlertReceiverDeleteThen::new(then),
            )
        })
    }

    fn alert_delivery_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AlertDeliveryListWhen, operations::AlertDeliveryListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AlertDeliveryListWhen::new(when),
                operations::AlertDeliveryListThen::new(then),
            )
        })
    }

    fn alert_receiver_probe<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AlertReceiverProbeWhen, operations::AlertReceiverProbeThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AlertReceiverProbeWhen::new(when),
                operations::AlertReceiverProbeThen::new(then),
            )
        })
    }

    fn alert_receiver_subscription_add<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::AlertReceiverSubscriptionAddWhen,
            operations::AlertReceiverSubscriptionAddThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AlertReceiverSubscriptionAddWhen::new(when),
                operations::AlertReceiverSubscriptionAddThen::new(then),
            )
        })
    }

    fn alert_receiver_subscription_remove<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::AlertReceiverSubscriptionRemoveWhen,
            operations::AlertReceiverSubscriptionRemoveThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AlertReceiverSubscriptionRemoveWhen::new(when),
                operations::AlertReceiverSubscriptionRemoveThen::new(then),
            )
        })
    }

    fn alert_delivery_resend<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AlertDeliveryResendWhen, operations::AlertDeliveryResendThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AlertDeliveryResendWhen::new(when),
                operations::AlertDeliveryResendThen::new(then),
            )
        })
    }

    fn anti_affinity_group_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AntiAffinityGroupListWhen, operations::AntiAffinityGroupListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AntiAffinityGroupListWhen::new(when),
                operations::AntiAffinityGroupListThen::new(then),
            )
        })
    }

    fn anti_affinity_group_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AntiAffinityGroupCreateWhen, operations::AntiAffinityGroupCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AntiAffinityGroupCreateWhen::new(when),
                operations::AntiAffinityGroupCreateThen::new(then),
            )
        })
    }

    fn anti_affinity_group_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AntiAffinityGroupViewWhen, operations::AntiAffinityGroupViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AntiAffinityGroupViewWhen::new(when),
                operations::AntiAffinityGroupViewThen::new(then),
            )
        })
    }

    fn anti_affinity_group_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AntiAffinityGroupUpdateWhen, operations::AntiAffinityGroupUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AntiAffinityGroupUpdateWhen::new(when),
                operations::AntiAffinityGroupUpdateThen::new(then),
            )
        })
    }

    fn anti_affinity_group_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AntiAffinityGroupDeleteWhen, operations::AntiAffinityGroupDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AntiAffinityGroupDeleteWhen::new(when),
                operations::AntiAffinityGroupDeleteThen::new(then),
            )
        })
    }

    fn anti_affinity_group_member_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::AntiAffinityGroupMemberListWhen,
            operations::AntiAffinityGroupMemberListThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AntiAffinityGroupMemberListWhen::new(when),
                operations::AntiAffinityGroupMemberListThen::new(then),
            )
        })
    }

    fn anti_affinity_group_member_instance_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::AntiAffinityGroupMemberInstanceViewWhen,
            operations::AntiAffinityGroupMemberInstanceViewThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AntiAffinityGroupMemberInstanceViewWhen::new(when),
                operations::AntiAffinityGroupMemberInstanceViewThen::new(then),
            )
        })
    }

    fn anti_affinity_group_member_instance_add<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::AntiAffinityGroupMemberInstanceAddWhen,
            operations::AntiAffinityGroupMemberInstanceAddThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AntiAffinityGroupMemberInstanceAddWhen::new(when),
                operations::AntiAffinityGroupMemberInstanceAddThen::new(then),
            )
        })
    }

    fn anti_affinity_group_member_instance_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::AntiAffinityGroupMemberInstanceDeleteWhen,
            operations::AntiAffinityGroupMemberInstanceDeleteThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AntiAffinityGroupMemberInstanceDeleteWhen::new(when),
                operations::AntiAffinityGroupMemberInstanceDeleteThen::new(then),
            )
        })
    }

    fn auth_settings_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AuthSettingsViewWhen, operations::AuthSettingsViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AuthSettingsViewWhen::new(when),
                operations::AuthSettingsViewThen::new(then),
            )
        })
    }

    fn auth_settings_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::AuthSettingsUpdateWhen, operations::AuthSettingsUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::AuthSettingsUpdateWhen::new(when),
                operations::AuthSettingsUpdateThen::new(then),
            )
        })
    }

    fn certificate_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn certificate_create<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn certificate_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn certificate_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn disk_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn disk_create<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn disk_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn disk_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn disk_bulk_write_import<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn disk_bulk_write_import_start<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn disk_bulk_write_import_stop<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn disk_finalize_import<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn disk_metrics_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn floating_ip_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::FloatingIpListWhen, operations::FloatingIpListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::FloatingIpListWhen::new(when),
                operations::FloatingIpListThen::new(then),
            )
        })
    }

    fn floating_ip_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::FloatingIpCreateWhen, operations::FloatingIpCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::FloatingIpCreateWhen::new(when),
                operations::FloatingIpCreateThen::new(then),
            )
        })
    }

    fn floating_ip_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::FloatingIpViewWhen, operations::FloatingIpViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::FloatingIpViewWhen::new(when),
                operations::FloatingIpViewThen::new(then),
            )
        })
    }

    fn floating_ip_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::FloatingIpUpdateWhen, operations::FloatingIpUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::FloatingIpUpdateWhen::new(when),
                operations::FloatingIpUpdateThen::new(then),
            )
        })
    }

    fn floating_ip_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::FloatingIpDeleteWhen, operations::FloatingIpDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::FloatingIpDeleteWhen::new(when),
                operations::FloatingIpDeleteThen::new(then),
            )
        })
    }

    fn floating_ip_attach<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::FloatingIpAttachWhen, operations::FloatingIpAttachThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::FloatingIpAttachWhen::new(when),
                operations::FloatingIpAttachThen::new(then),
            )
        })
    }

    fn floating_ip_detach<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::FloatingIpDetachWhen, operations::FloatingIpDetachThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::FloatingIpDetachWhen::new(when),
                operations::FloatingIpDetachThen::new(then),
            )
        })
    }

    fn group_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn group_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn image_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn image_create<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn image_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn image_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn image_demote<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn image_promote<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn instance_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn instance_create<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn instance_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn instance_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InstanceUpdateWhen, operations::InstanceUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceUpdateWhen::new(when),
                operations::InstanceUpdateThen::new(then),
            )
        })
    }

    fn instance_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn instance_affinity_group_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceAffinityGroupListWhen,
            operations::InstanceAffinityGroupListThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceAffinityGroupListWhen::new(when),
                operations::InstanceAffinityGroupListThen::new(then),
            )
        })
    }

    fn instance_anti_affinity_group_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceAntiAffinityGroupListWhen,
            operations::InstanceAntiAffinityGroupListThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceAntiAffinityGroupListWhen::new(when),
                operations::InstanceAntiAffinityGroupListThen::new(then),
            )
        })
    }

    fn instance_disk_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn instance_disk_attach<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn instance_disk_detach<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn instance_external_ip_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn instance_ephemeral_ip_attach<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceEphemeralIpAttachWhen,
            operations::InstanceEphemeralIpAttachThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceEphemeralIpAttachWhen::new(when),
                operations::InstanceEphemeralIpAttachThen::new(then),
            )
        })
    }

    fn instance_ephemeral_ip_detach<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceEphemeralIpDetachWhen,
            operations::InstanceEphemeralIpDetachThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceEphemeralIpDetachWhen::new(when),
                operations::InstanceEphemeralIpDetachThen::new(then),
            )
        })
    }

    fn instance_reboot<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn instance_serial_console<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn instance_serial_console_stream<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn instance_ssh_public_key_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InstanceSshPublicKeyListWhen,
            operations::InstanceSshPublicKeyListThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InstanceSshPublicKeyListWhen::new(when),
                operations::InstanceSshPublicKeyListThen::new(then),
            )
        })
    }

    fn instance_start<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn instance_stop<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn internet_gateway_ip_address_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InternetGatewayIpAddressListWhen,
            operations::InternetGatewayIpAddressListThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InternetGatewayIpAddressListWhen::new(when),
                operations::InternetGatewayIpAddressListThen::new(then),
            )
        })
    }

    fn internet_gateway_ip_address_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InternetGatewayIpAddressCreateWhen,
            operations::InternetGatewayIpAddressCreateThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InternetGatewayIpAddressCreateWhen::new(when),
                operations::InternetGatewayIpAddressCreateThen::new(then),
            )
        })
    }

    fn internet_gateway_ip_address_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InternetGatewayIpAddressDeleteWhen,
            operations::InternetGatewayIpAddressDeleteThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InternetGatewayIpAddressDeleteWhen::new(when),
                operations::InternetGatewayIpAddressDeleteThen::new(then),
            )
        })
    }

    fn internet_gateway_ip_pool_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InternetGatewayIpPoolListWhen,
            operations::InternetGatewayIpPoolListThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InternetGatewayIpPoolListWhen::new(when),
                operations::InternetGatewayIpPoolListThen::new(then),
            )
        })
    }

    fn internet_gateway_ip_pool_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InternetGatewayIpPoolCreateWhen,
            operations::InternetGatewayIpPoolCreateThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InternetGatewayIpPoolCreateWhen::new(when),
                operations::InternetGatewayIpPoolCreateThen::new(then),
            )
        })
    }

    fn internet_gateway_ip_pool_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::InternetGatewayIpPoolDeleteWhen,
            operations::InternetGatewayIpPoolDeleteThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InternetGatewayIpPoolDeleteWhen::new(when),
                operations::InternetGatewayIpPoolDeleteThen::new(then),
            )
        })
    }

    fn internet_gateway_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InternetGatewayListWhen, operations::InternetGatewayListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InternetGatewayListWhen::new(when),
                operations::InternetGatewayListThen::new(then),
            )
        })
    }

    fn internet_gateway_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InternetGatewayCreateWhen, operations::InternetGatewayCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InternetGatewayCreateWhen::new(when),
                operations::InternetGatewayCreateThen::new(then),
            )
        })
    }

    fn internet_gateway_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InternetGatewayViewWhen, operations::InternetGatewayViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InternetGatewayViewWhen::new(when),
                operations::InternetGatewayViewThen::new(then),
            )
        })
    }

    fn internet_gateway_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::InternetGatewayDeleteWhen, operations::InternetGatewayDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::InternetGatewayDeleteWhen::new(when),
                operations::InternetGatewayDeleteThen::new(then),
            )
        })
    }

    fn project_ip_pool_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ProjectIpPoolListWhen, operations::ProjectIpPoolListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectIpPoolListWhen::new(when),
                operations::ProjectIpPoolListThen::new(then),
            )
        })
    }

    fn project_ip_pool_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::ProjectIpPoolViewWhen, operations::ProjectIpPoolViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::ProjectIpPoolViewWhen::new(when),
                operations::ProjectIpPoolViewThen::new(then),
            )
        })
    }

    fn login_local<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn logout<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn current_user_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn current_user_groups<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn current_user_ssh_key_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn current_user_ssh_key_create<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn current_user_ssh_key_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn current_user_ssh_key_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn silo_metric<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SiloMetricWhen, operations::SiloMetricThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloMetricWhen::new(when),
                operations::SiloMetricThen::new(then),
            )
        })
    }

    fn instance_network_interface_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn instance_network_interface_create<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn instance_network_interface_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn instance_network_interface_update<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn instance_network_interface_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn ping<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::PingWhen, operations::PingThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::PingWhen::new(when),
                operations::PingThen::new(then),
            )
        })
    }

    fn policy_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn policy_update<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn project_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn project_create<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn project_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn project_update<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn project_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn project_policy_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn project_policy_update<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn snapshot_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn snapshot_create<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn snapshot_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn snapshot_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn physical_disk_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn physical_disk_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::PhysicalDiskViewWhen, operations::PhysicalDiskViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::PhysicalDiskViewWhen::new(when),
                operations::PhysicalDiskViewThen::new(then),
            )
        })
    }

    fn networking_switch_port_lldp_neighbors<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortLldpNeighborsWhen,
            operations::NetworkingSwitchPortLldpNeighborsThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingSwitchPortLldpNeighborsWhen::new(when),
                operations::NetworkingSwitchPortLldpNeighborsThen::new(then),
            )
        })
    }

    fn rack_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn rack_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn sled_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn sled_add<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SledAddWhen, operations::SledAddThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SledAddWhen::new(when),
                operations::SledAddThen::new(then),
            )
        })
    }

    fn sled_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn sled_physical_disk_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn sled_instance_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn sled_set_provision_policy<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SledSetProvisionPolicyWhen, operations::SledSetProvisionPolicyThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SledSetProvisionPolicyWhen::new(when),
                operations::SledSetProvisionPolicyThen::new(then),
            )
        })
    }

    fn sled_list_uninitialized<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SledListUninitializedWhen, operations::SledListUninitializedThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SledListUninitializedWhen::new(when),
                operations::SledListUninitializedThen::new(then),
            )
        })
    }

    fn networking_switch_port_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn networking_switch_port_lldp_config_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortLldpConfigViewWhen,
            operations::NetworkingSwitchPortLldpConfigViewThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingSwitchPortLldpConfigViewWhen::new(when),
                operations::NetworkingSwitchPortLldpConfigViewThen::new(then),
            )
        })
    }

    fn networking_switch_port_lldp_config_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortLldpConfigUpdateWhen,
            operations::NetworkingSwitchPortLldpConfigUpdateThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingSwitchPortLldpConfigUpdateWhen::new(when),
                operations::NetworkingSwitchPortLldpConfigUpdateThen::new(then),
            )
        })
    }

    fn networking_switch_port_apply_settings<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn networking_switch_port_clear_settings<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn networking_switch_port_status<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingSwitchPortStatusWhen,
            operations::NetworkingSwitchPortStatusThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingSwitchPortStatusWhen::new(when),
                operations::NetworkingSwitchPortStatusThen::new(then),
            )
        })
    }

    fn switch_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn switch_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn silo_identity_provider_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn local_idp_user_create<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn local_idp_user_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn local_idp_user_set_password<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn saml_identity_provider_create<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn saml_identity_provider_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn ip_pool_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn ip_pool_create<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn ip_pool_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn ip_pool_update<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn ip_pool_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn ip_pool_range_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn ip_pool_range_add<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn ip_pool_range_remove<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn ip_pool_silo_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::IpPoolSiloListWhen, operations::IpPoolSiloListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolSiloListWhen::new(when),
                operations::IpPoolSiloListThen::new(then),
            )
        })
    }

    fn ip_pool_silo_link<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::IpPoolSiloLinkWhen, operations::IpPoolSiloLinkThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolSiloLinkWhen::new(when),
                operations::IpPoolSiloLinkThen::new(then),
            )
        })
    }

    fn ip_pool_silo_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::IpPoolSiloUpdateWhen, operations::IpPoolSiloUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolSiloUpdateWhen::new(when),
                operations::IpPoolSiloUpdateThen::new(then),
            )
        })
    }

    fn ip_pool_silo_unlink<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::IpPoolSiloUnlinkWhen, operations::IpPoolSiloUnlinkThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolSiloUnlinkWhen::new(when),
                operations::IpPoolSiloUnlinkThen::new(then),
            )
        })
    }

    fn ip_pool_utilization_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::IpPoolUtilizationViewWhen, operations::IpPoolUtilizationViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::IpPoolUtilizationViewWhen::new(when),
                operations::IpPoolUtilizationViewThen::new(then),
            )
        })
    }

    fn ip_pool_service_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn ip_pool_service_range_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn ip_pool_service_range_add<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn ip_pool_service_range_remove<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn system_metric<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn networking_address_lot_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn networking_address_lot_create<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn networking_address_lot_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn networking_address_lot_block_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn networking_allow_list_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::NetworkingAllowListViewWhen, operations::NetworkingAllowListViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingAllowListViewWhen::new(when),
                operations::NetworkingAllowListViewThen::new(then),
            )
        })
    }

    fn networking_allow_list_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingAllowListUpdateWhen,
            operations::NetworkingAllowListUpdateThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingAllowListUpdateWhen::new(when),
                operations::NetworkingAllowListUpdateThen::new(then),
            )
        })
    }

    fn networking_bfd_disable<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::NetworkingBfdDisableWhen, operations::NetworkingBfdDisableThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingBfdDisableWhen::new(when),
                operations::NetworkingBfdDisableThen::new(then),
            )
        })
    }

    fn networking_bfd_enable<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::NetworkingBfdEnableWhen, operations::NetworkingBfdEnableThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingBfdEnableWhen::new(when),
                operations::NetworkingBfdEnableThen::new(then),
            )
        })
    }

    fn networking_bfd_status<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::NetworkingBfdStatusWhen, operations::NetworkingBfdStatusThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingBfdStatusWhen::new(when),
                operations::NetworkingBfdStatusThen::new(then),
            )
        })
    }

    fn networking_bgp_config_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::NetworkingBgpConfigListWhen, operations::NetworkingBgpConfigListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingBgpConfigListWhen::new(when),
                operations::NetworkingBgpConfigListThen::new(then),
            )
        })
    }

    fn networking_bgp_config_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingBgpConfigCreateWhen,
            operations::NetworkingBgpConfigCreateThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingBgpConfigCreateWhen::new(when),
                operations::NetworkingBgpConfigCreateThen::new(then),
            )
        })
    }

    fn networking_bgp_config_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingBgpConfigDeleteWhen,
            operations::NetworkingBgpConfigDeleteThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingBgpConfigDeleteWhen::new(when),
                operations::NetworkingBgpConfigDeleteThen::new(then),
            )
        })
    }

    fn networking_bgp_announce_set_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingBgpAnnounceSetListWhen,
            operations::NetworkingBgpAnnounceSetListThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingBgpAnnounceSetListWhen::new(when),
                operations::NetworkingBgpAnnounceSetListThen::new(then),
            )
        })
    }

    fn networking_bgp_announce_set_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingBgpAnnounceSetUpdateWhen,
            operations::NetworkingBgpAnnounceSetUpdateThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingBgpAnnounceSetUpdateWhen::new(when),
                operations::NetworkingBgpAnnounceSetUpdateThen::new(then),
            )
        })
    }

    fn networking_bgp_announce_set_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingBgpAnnounceSetDeleteWhen,
            operations::NetworkingBgpAnnounceSetDeleteThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingBgpAnnounceSetDeleteWhen::new(when),
                operations::NetworkingBgpAnnounceSetDeleteThen::new(then),
            )
        })
    }

    fn networking_bgp_announcement_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingBgpAnnouncementListWhen,
            operations::NetworkingBgpAnnouncementListThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingBgpAnnouncementListWhen::new(when),
                operations::NetworkingBgpAnnouncementListThen::new(then),
            )
        })
    }

    fn networking_bgp_exported<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::NetworkingBgpExportedWhen, operations::NetworkingBgpExportedThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingBgpExportedWhen::new(when),
                operations::NetworkingBgpExportedThen::new(then),
            )
        })
    }

    fn networking_bgp_message_history<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingBgpMessageHistoryWhen,
            operations::NetworkingBgpMessageHistoryThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingBgpMessageHistoryWhen::new(when),
                operations::NetworkingBgpMessageHistoryThen::new(then),
            )
        })
    }

    fn networking_bgp_imported_routes_ipv4<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::NetworkingBgpImportedRoutesIpv4When,
            operations::NetworkingBgpImportedRoutesIpv4Then,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingBgpImportedRoutesIpv4When::new(when),
                operations::NetworkingBgpImportedRoutesIpv4Then::new(then),
            )
        })
    }

    fn networking_bgp_status<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::NetworkingBgpStatusWhen, operations::NetworkingBgpStatusThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::NetworkingBgpStatusWhen::new(when),
                operations::NetworkingBgpStatusThen::new(then),
            )
        })
    }

    fn networking_loopback_address_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn networking_loopback_address_create<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn networking_loopback_address_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn networking_switch_port_settings_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn networking_switch_port_settings_create<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn networking_switch_port_settings_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn networking_switch_port_settings_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn system_policy_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn system_policy_update<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn role_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn role_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn system_quotas_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SystemQuotasListWhen, operations::SystemQuotasListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemQuotasListWhen::new(when),
                operations::SystemQuotasListThen::new(then),
            )
        })
    }

    fn silo_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn silo_create<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn silo_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn silo_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn silo_ip_pool_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SiloIpPoolListWhen, operations::SiloIpPoolListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloIpPoolListWhen::new(when),
                operations::SiloIpPoolListThen::new(then),
            )
        })
    }

    fn silo_policy_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn silo_policy_update<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn silo_quotas_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SiloQuotasViewWhen, operations::SiloQuotasViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloQuotasViewWhen::new(when),
                operations::SiloQuotasViewThen::new(then),
            )
        })
    }

    fn silo_quotas_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SiloQuotasUpdateWhen, operations::SiloQuotasUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloQuotasUpdateWhen::new(when),
                operations::SiloQuotasUpdateThen::new(then),
            )
        })
    }

    fn system_timeseries_query<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SystemTimeseriesQueryWhen, operations::SystemTimeseriesQueryThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemTimeseriesQueryWhen::new(when),
                operations::SystemTimeseriesQueryThen::new(then),
            )
        })
    }

    fn system_timeseries_schema_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(
            operations::SystemTimeseriesSchemaListWhen,
            operations::SystemTimeseriesSchemaListThen,
        ),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SystemTimeseriesSchemaListWhen::new(when),
                operations::SystemTimeseriesSchemaListThen::new(then),
            )
        })
    }

    fn target_release_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::TargetReleaseViewWhen, operations::TargetReleaseViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::TargetReleaseViewWhen::new(when),
                operations::TargetReleaseViewThen::new(then),
            )
        })
    }

    fn target_release_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::TargetReleaseUpdateWhen, operations::TargetReleaseUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::TargetReleaseUpdateWhen::new(when),
                operations::TargetReleaseUpdateThen::new(then),
            )
        })
    }

    fn silo_user_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn silo_user_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn user_builtin_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn user_builtin_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn silo_utilization_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SiloUtilizationListWhen, operations::SiloUtilizationListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloUtilizationListWhen::new(when),
                operations::SiloUtilizationListThen::new(then),
            )
        })
    }

    fn silo_utilization_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::SiloUtilizationViewWhen, operations::SiloUtilizationViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::SiloUtilizationViewWhen::new(when),
                operations::SiloUtilizationViewThen::new(then),
            )
        })
    }

    fn timeseries_query<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::TimeseriesQueryWhen, operations::TimeseriesQueryThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::TimeseriesQueryWhen::new(when),
                operations::TimeseriesQueryThen::new(then),
            )
        })
    }

    fn user_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn utilization_view<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::UtilizationViewWhen, operations::UtilizationViewThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::UtilizationViewWhen::new(when),
                operations::UtilizationViewThen::new(then),
            )
        })
    }

    fn vpc_firewall_rules_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn vpc_firewall_rules_update<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn vpc_router_route_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn vpc_router_route_create<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn vpc_router_route_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn vpc_router_route_update<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn vpc_router_route_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn vpc_router_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn vpc_router_create<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn vpc_router_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn vpc_router_update<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn vpc_router_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn vpc_subnet_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn vpc_subnet_create<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn vpc_subnet_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn vpc_subnet_update<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn vpc_subnet_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn vpc_subnet_list_network_interfaces<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn vpc_list<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn vpc_create<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn vpc_view<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn vpc_update<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn vpc_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
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

    fn webhook_receiver_create<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::WebhookReceiverCreateWhen, operations::WebhookReceiverCreateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::WebhookReceiverCreateWhen::new(when),
                operations::WebhookReceiverCreateThen::new(then),
            )
        })
    }

    fn webhook_receiver_update<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::WebhookReceiverUpdateWhen, operations::WebhookReceiverUpdateThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::WebhookReceiverUpdateWhen::new(when),
                operations::WebhookReceiverUpdateThen::new(then),
            )
        })
    }

    fn webhook_secrets_list<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::WebhookSecretsListWhen, operations::WebhookSecretsListThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::WebhookSecretsListWhen::new(when),
                operations::WebhookSecretsListThen::new(then),
            )
        })
    }

    fn webhook_secrets_add<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::WebhookSecretsAddWhen, operations::WebhookSecretsAddThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::WebhookSecretsAddWhen::new(when),
                operations::WebhookSecretsAddThen::new(then),
            )
        })
    }

    fn webhook_secrets_delete<F>(&self, config_fn: F) -> ::httpmock::Mock
    where
        F: FnOnce(operations::WebhookSecretsDeleteWhen, operations::WebhookSecretsDeleteThen),
    {
        self.mock(|when, then| {
            config_fn(
                operations::WebhookSecretsDeleteWhen::new(when),
                operations::WebhookSecretsDeleteThen::new(then),
            )
        })
    }
}
