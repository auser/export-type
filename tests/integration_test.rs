#![allow(unused)]
use export_type::ExportType;
use serde_json::Value;

#[derive(ExportType)]
#[export_type(path = "target/test_exports", lang = "typescript")]
struct TestUser {
    id: i32,
    name: String,
    #[export_type(rename = "emailAddress")]
    email: Option<String>,
    roles: Vec<String>,
    #[export_type(rename = "custom_headers")]
    custom_headers: std::collections::HashMap<String, String>,
    created_at: chrono::DateTime<chrono::Utc>,
    details: Option<Value>,
}

#[derive(sqlx::FromRow, ExportType)]
#[export_type(path = "target/test_exports", lang = "typescript")]
struct DatabaseModel {
    id: i32,
    name: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(ExportType)]
#[export_type(path = "target/test_exports", lang = "typescript")]
enum TestStatus {
    Active,
    Inactive,
    Pending { reason: String },
}

#[test]
fn test_generated_files_exist() {
    // These files should be generated during compilation
    assert!(std::path::Path::new("target/test_exports/index.ts").exists());
}

#[test]
fn test_generated_struct_types() {
    let user_content = std::fs::read_to_string("target/test_exports/index.ts")
        .expect("Should read user typescript file");

    assert!(user_content.contains("export interface TestUser"));
    assert!(user_content.contains("id: number"));
    assert!(user_content.contains("emailAddress?: string"));
    assert!(user_content.contains("roles: string[]"));
    assert!(user_content.contains("created_at: Date"));
    assert!(user_content.contains("details?: Record<any, any> | undefined"));
}

#[test]
fn test_generated_enum_types() {
    let status_content = std::fs::read_to_string("target/test_exports/index.ts")
        .expect("Should read status typescript file");

    assert!(status_content.contains("export type TestStatus"));
    assert!(status_content.contains("| \"Active\""));
    assert!(status_content.contains("| { type: \"Pending\";     reason: string; }"));
}

#[test]
fn test_sqlx_type_is_skipped_in_database_model_struct() {
    let database_model_content = std::fs::read_to_string("target/test_exports/index.ts")
        .expect("Should read database model typescript file");

    assert!(database_model_content.contains("export interface DatabaseModel"));
    assert!(database_model_content.contains("created_at: Date"));
}
