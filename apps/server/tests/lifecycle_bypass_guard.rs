use std::fs;
use std::path::{Path, PathBuf};

fn collect_rust_files(root: &Path, out: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or_default();
                if name == "target" || name == ".git" || name == "node_modules" {
                    continue;
                }
                collect_rust_files(&path, out);
            } else if path.extension().and_then(|e| e.to_str()) == Some("rs") {
                out.push(path);
            }
        }
    }
}

#[test]
fn bypass_toggle_api_is_not_used_in_production_paths() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root");

    let apps_server_root = repo_root.join("apps/server");
    let apps_admin_root = repo_root.join("apps/admin");
    let allowed_files = [apps_server_root.join("src/models/tenant_modules.rs")];
    let ignored_files = [apps_server_root.join("tests/lifecycle_bypass_guard.rs")];
    let forbidden_pattern = "upsert_flag_without_lifecycle_for_migrations_only(";

    let mut rust_files = Vec::new();
    collect_rust_files(&apps_server_root, &mut rust_files);
    collect_rust_files(&apps_admin_root, &mut rust_files);

    let mut offenders = Vec::new();
    for file in rust_files {
        if allowed_files.iter().any(|allowed| allowed == &file) {
            continue;
        }
        if ignored_files.iter().any(|ignored| ignored == &file) {
            continue;
        }

        if let Ok(content) = fs::read_to_string(&file) {
            if content.contains(forbidden_pattern) {
                let rel = file
                    .strip_prefix(repo_root)
                    .map(|path| path.display().to_string())
                    .unwrap_or_else(|_| file.display().to_string());
                offenders.push(rel);
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "Forbidden bypass API usage found outside tenant_modules model file: {offenders:?}"
    );
}

#[test]
fn admin_native_toggle_endpoint_is_not_reintroduced() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root");
    let admin_modules_api = repo_root.join("apps/admin/src/features/modules/api.rs");
    let content = fs::read_to_string(&admin_modules_api)
        .expect("apps/admin modules api source should be readable");

    assert!(
        !content.contains("endpoint = \"admin/toggle-module\""),
        "Forbidden native toggle endpoint declaration found in apps/admin modules api."
    );
    assert!(
        !content.contains("fn toggle_module_native("),
        "Forbidden native toggle helper reintroduced in apps/admin modules api."
    );
}

#[test]
fn admin_toggle_module_uses_graphql_without_native_fallback() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root");
    let admin_modules_api = repo_root.join("apps/admin/src/features/modules/api.rs");
    let content = fs::read_to_string(&admin_modules_api)
        .expect("apps/admin modules api source should be readable");

    let toggle_fn = extract_function_block(&content, "pub async fn toggle_module(")
        .expect("toggle_module function block must exist");

    assert!(
        !toggle_fn.contains("combine_native_and_graphql_error"),
        "toggle_module must not compose native+graphql fallback errors; canonical GraphQL path only."
    );
    assert!(
        toggle_fn.contains("request("),
        "toggle_module must call GraphQL request path."
    );
}

fn extract_function_block<'a>(content: &'a str, signature: &str) -> Option<&'a str> {
    let start = content.find(signature)?;
    let rest = &content[start..];
    let open_rel = rest.find('{')?;
    let mut depth = 0usize;
    let mut end_rel = None;

    for (idx, ch) in rest.char_indices().skip(open_rel) {
        match ch {
            '{' => depth += 1,
            '}' => {
                if depth == 0 {
                    return None;
                }
                depth -= 1;
                if depth == 0 {
                    end_rel = Some(idx + ch.len_utf8());
                    break;
                }
            }
            _ => {}
        }
    }

    end_rel.map(|end| &rest[..end])
}

#[test]
fn extract_function_block_handles_nested_braces() {
    let source = r#"
pub async fn toggle_module() -> Result<(), ()> {
    if true {
        let _nested = || {
            let _map = std::collections::BTreeMap::<String, String>::new();
            _map
        };
        _nested();
    }
    Ok(())
}

pub async fn update_module_settings() {}
"#;

    let extracted = extract_function_block(source, "pub async fn toggle_module()")
        .expect("toggle_module function should be extracted");
    assert!(extracted.contains("BTreeMap::<String, String>::new()"));
    assert!(extracted.trim_end().ends_with('}'));
    assert!(!extracted.contains("pub async fn update_module_settings()"));
}

#[test]
fn extract_function_block_returns_none_for_missing_signature() {
    let source = "pub async fn update_module_settings() {}";
    assert!(extract_function_block(source, "pub async fn toggle_module(").is_none());
}

#[test]
fn graphql_mutations_do_not_reintroduce_duplicate_platform_composition_mapping_tests() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root");
    let mutations_rs = repo_root.join("apps/server/src/graphql/mutations.rs");
    let content = fs::read_to_string(&mutations_rs).expect("mutations.rs should be readable");

    let expected_unique_tests = [
        "fn platform_composition_error_maps_revision_conflict_with_expected_and_current()",
        "fn platform_composition_build_error_maps_build_failures_to_internal_error()",
        "fn platform_composition_build_error_maps_manifest_validation_to_user_facing_message()",
        "fn platform_composition_build_error_maps_revision_conflict_to_conflict_message()",
    ];

    for signature in expected_unique_tests {
        let occurrences = content.matches(signature).count();
        assert_eq!(
            occurrences, 1,
            "Expected exactly one `{signature}` test, found {occurrences}."
        );
    }

    assert!(
        !content.contains("\"queue unavailable\""),
        "Obsolete platform composition build error fixture (`queue unavailable`) reintroduced."
    );

    let forbidden_legacy_tests = [
        "fn platform_composition_error_maps_revision_conflict_to_conflict_message()",
        "fn platform_composition_build_error_maps_enqueue_failures_to_internal_error()",
        "fn platform_composition_build_error_maps_composition_conflict_consistently()",
    ];

    for signature in forbidden_legacy_tests {
        assert!(
            !content.contains(signature),
            "Legacy/duplicate platform composition mapping test signature reintroduced: {signature}"
        );
    }
}
