#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct StatusCode(u16);

#[derive(Debug)]
pub struct InvalidStatusCode {
    _priv: (),
}

impl InvalidStatusCode {
    fn new() -> InvalidStatusCode {
        InvalidStatusCode { _priv: () }
    }
}

impl StatusCode {
    #[inline]
    pub fn from_u16(src: u16) -> Result<StatusCode, InvalidStatusCode> {
        if src < 100 || src >= 600 {
            return Err(InvalidStatusCode::new());
        }

        Ok(StatusCode(src))
    }

    #[inline]
    pub fn as_u16(&self) -> u16 {
        (*self).into()
    }

    #[inline]
    pub fn is_success(&self) -> bool {
        self.0 < (StatusCode::BAD_REQUEST).0
    }

    #[inline]
    pub fn is_retriable(&self) -> bool {
        self.0 == StatusCode::THROTTLED || self.0 == StatusCode::SERVER_ERROR
    }
}

impl Default for StatusCode {
    #[inline]
    fn default() -> StatusCode {
        StatusCode::OK
    }
}

impl PartialEq<u16> for StatusCode {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        self.as_u16() == *other
    }
}

impl PartialEq<StatusCode> for u16 {
    #[inline]
    fn eq(&self, other: &StatusCode) -> bool {
        *self == other.as_u16()
    }
}

impl From<StatusCode> for u16 {
    #[inline]
    fn from(status: StatusCode) -> u16 {
        status.0
    }
}

macro_rules! status_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident);
        )+
    ) => {
        impl StatusCode {
        $(
            $(#[$docs])*
            pub const $konst: StatusCode = StatusCode($num);
        )+

        }
    }
}

status_codes! {
  // Default); unset value
  (0, UNKNOWN);

  // Service success codes
  (200, OK);
  (202, ACCEPTED);
  (204, NO_CONTENT);

  // Service error codes
  (400, BAD_REQUEST);
  (401, UNAUTHORIZED);
  (403, FORBIDDEN);
  (404, NOT_FOUND);
  (405, NOT_ALLOWED);
  (409, NOT_CONFLICT);
  (412, PRECONDITION_FAILED);
  (413, REQUEST_TOO_LARGE);
  (415, UNSUPPORTED_TYPE);
  (429, THROTTLED);
  (499, CLIENT_CLOSED);
  (500, SERVER_ERROR);
  (502, BAD_GATEWAY);
  (503, SERVICE_UNAVAILABLE);
  (504, TIMEOUT);
}

#[cfg(test)]
mod tests_conversions {
    use super::*;
    #[test]
    fn status_code_can_be_compared_to_integers() {
        assert_eq!(StatusCode::OK, 200);
    }

    #[test]
    fn status_code_can_be_cast_as_integers() {
        assert_eq!(StatusCode::OK.as_u16(), 200);
    }

    #[test]
    fn status_code_can_be_created_from_integers() {
        assert_eq!(StatusCode::from_u16(200).unwrap_or_default(), 200);
    }

    #[test]
    fn parsing_an_unkown_results_in_value() {
        assert_eq!(StatusCode::from_u16(599).unwrap(), 599);
    }

    #[test]
    fn parsing_a_bad_code_results_in_error() {
        assert!(StatusCode::from_u16(99).is_err());
        assert!(StatusCode::from_u16(100).is_ok());
        assert!(StatusCode::from_u16(599).is_ok());
        assert!(StatusCode::from_u16(600).is_err());
        assert!(StatusCode::from_u16(601).is_err());
    }
}

#[cfg(test)]
mod tests_success {
    use super::*;
    #[test]
    fn status_codes_less_than_bad_request_are_success() {
        assert!(StatusCode::OK.is_success());
        assert!(StatusCode::ACCEPTED.is_success());
        assert!(StatusCode::NO_CONTENT.is_success());
        assert!(!StatusCode::BAD_REQUEST.is_success());
        assert!(!StatusCode::UNAUTHORIZED.is_success());
    }
}

#[cfg(test)]
mod tests_retriable {
    use super::*;
    #[test]
    fn throttled_requests_are_retriable() {
        assert!(StatusCode::THROTTLED.is_retriable());
    }

    #[test]
    fn server_error_requests_are_retriable() {
        assert!(StatusCode::SERVER_ERROR.is_retriable());
    }
}
