#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]

mod api;

pub use api::*;

mod models {
    openapi_struct_gen::include!("starling_api");
}
