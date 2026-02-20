use std::time::Duration;

use tokio::time::timeout;
use tombi_schema_store::{SchemaStore, SchemaUri};

#[tokio::test]
async fn test_recursive_external_schema_fetch_does_not_hang() {
    let temp_dir = std::env::temp_dir().join(format!("tombi-recursive-schema-{}", std::process::id()));
    std::fs::create_dir_all(&temp_dir).expect("failed to create temp dir");

    let schema_a_path = temp_dir.join("a.schema.json");
    let schema_b_path = temp_dir.join("b.schema.json");

    let schema_a_uri =
        SchemaUri::from_file_path(&schema_a_path).expect("failed to convert schema A path");
    let schema_b_uri =
        SchemaUri::from_file_path(&schema_b_path).expect("failed to convert schema B path");

    std::fs::write(
        &schema_a_path,
        format!(
            r#"{{
  "oneOf": [
    {{ "$ref": "{schema_b_uri}" }}
  ]
}}"#
        ),
    )
    .expect("failed to write schema A");

    std::fs::write(
        &schema_b_path,
        format!(
            r#"{{
  "oneOf": [
    {{ "$ref": "{schema_a_uri}" }}
  ]
}}"#
        ),
    )
    .expect("failed to write schema B");

    let schema_store = SchemaStore::new();
    let result = timeout(
        Duration::from_secs(5),
        schema_store.try_get_document_schema(&schema_a_uri),
    )
    .await;

    assert!(result.is_ok(), "schema resolution timed out");
    let result = result.unwrap();
    assert!(result.is_ok(), "schema resolution returned an error");
    assert!(result.unwrap().is_some(), "root schema should be resolved");

    std::fs::remove_dir_all(&temp_dir).expect("failed to remove temp dir");
}
