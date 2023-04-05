use openapi_struct_gen::generate;

fn main() {
    generate(
        format!(
            "{}/{}",
            std::env::var("CARGO_MANIFEST_DIR").unwrap(),
            "starling-api.yaml"
        ),
        format!(
            "{}/{}",
            std::env::var("OUT_DIR").unwrap(),
            "starling_api.rs"
        ),
        Some(&["Clone", "Serialize", "Deserialize"]),
        Some(&[("serde", "Serialize"), ("serde", "Deserialize")]),
        Some(&[(r#"#[skip_serializing_none]"#, None)]),
        Some(&[(
            r#"#[serde(rename_all = "camelCase")]"#,
            Some(&["Struct"]),
        )]),
    )
    .unwrap();
}
