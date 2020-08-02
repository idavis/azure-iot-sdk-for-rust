#![no_std]
#![deny(rust_2018_idioms, warnings)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(
    clippy::default_trait_access,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::pub_enum_variant_names,
    clippy::similar_names,
    clippy::too_many_lines,
    clippy::use_self,
    clippy::cast_possible_truncation
)]
// Consider removing these
#![allow(dead_code)]
#![allow(clippy::large_enum_variant)]

pub mod provisioning;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
