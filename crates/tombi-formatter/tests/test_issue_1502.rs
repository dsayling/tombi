use itertools::Either;
use tombi_config::TomlVersion;
use tombi_formatter::Formatter;
use tombi_schema_store::{AssociateSchemaOptions, SchemaStore, SchemaUri};
use tombi_test_lib::{project_root_path, pyproject_schema_path};

#[tokio::test]
async fn test_format_pyproject_with_tool_poe_schema() {
    tombi_test_lib::init_log();

    let schema_store = SchemaStore::new();
    let schema_uri = SchemaUri::from_file_path(pyproject_schema_path())
        .expect("failed to convert pyproject schema path to schema uri");
    schema_store
        .associate_schema(
            schema_uri,
            vec!["*.toml".to_string()],
            &AssociateSchemaOptions::default(),
        )
        .await;

    let source = r#"
[tool.poetry]
name = "company-app"
version = "0.0.0"

[tool.poetry.dependencies]
python = ">=3.12, <4.0"
django = "^5.1.6"

[tool.poe.tasks.install]
help = "Run default poetry install"
sequence = [{ cmd = "poetry install" }]

[tool.poe.tasks.format]
help = "Auto-format Python tests with Ruff."
sequence = [
  { cmd = "ruff format ." },
  { cmd = "ruff check . --fix" },
]
ignore_fail = "return_non_zero"

[tool.ruff.format]
preview = true

[tool.mypy]
ignore_missing_imports = true

[tool.typos.default]
extend-ignore-re = ["(?Rm)^.*# noqa: typos$"]

[tool.coverage.report]
fail_under = 86
"#;

    let source_path = project_root_path().join("test.toml");
    let format_options = Default::default();
    let formatter = Formatter::new(
        TomlVersion::default(),
        &format_options,
        Some(Either::Right(source_path.as_path())),
        &schema_store,
    );

    let formatted = formatter
        .format(source)
        .await
        .expect("format should succeed");
    assert!(formatted.contains("[tool.poe.tasks.install]"));
}
