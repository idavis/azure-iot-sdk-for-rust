pub const AZ_ERROR_INSUFFICIENT_SPAN_SIZE: &str = "The size of the provided span is too small";

use core::fmt::{Debug, Display};

pub trait Error: Debug + Display {}
