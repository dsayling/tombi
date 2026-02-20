use tombi_schema_store::{SchemaStore, SchemaUri};
use tombi_test_lib::project_root_path;

/// Resolving a schema with circular `$ref` definitions must complete without
/// deadlocking or looping infinitely.
#[tokio::test]
async fn test_resolve_circular_ref_schema() {
    let schema_path = project_root_path()
        .join("schemas")
        .join("circular-ref.schema.json");

    let schema_store = SchemaStore::new();
    let schema_uri = SchemaUri::from_file_path(&schema_path)
        .expect("failed to convert schema path to schema uri");

    // This must return without hanging.  Before the fix the recursive
    // resolve would deadlock on the shared Arc<RwLock> held by cloned
    // definitions with circular oneOf → sequence_task → task → oneOf chains.
    let result = schema_store.try_get_document_schema(&schema_uri).await;
    assert!(
        result.is_ok(),
        "schema resolution should succeed, got: {:?}",
        result
    );
    let doc = result.unwrap();
    assert!(doc.is_some(), "document schema should be Some");
}
