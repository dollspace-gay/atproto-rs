use proptest::prelude::*;

use proto_blue_xrpc::error::ResponseType;

proptest! {
    /// Any u16 status code should never panic in from_http_status.
    #[test]
    fn from_http_status_never_panics(status in any::<u16>()) {
        let _ = ResponseType::from_http_status(status);
    }

    /// ResponseType::name() should always return a non-empty string.
    #[test]
    fn name_always_returns_non_empty_string(status in any::<u16>()) {
        let rt = ResponseType::from_http_status(status);
        assert!(!rt.name().is_empty());
    }
}
