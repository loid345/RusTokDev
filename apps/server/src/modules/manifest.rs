use rustok_core::ModuleRegistry;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Debug, Deserialize)]
struct ModulesManifest {
    modules: HashMap<String, ModuleSpec>,
}

#[derive(Debug, Deserialize)]
struct ModuleSpec {
    #[serde(rename = "crate")]
    crate_name: String,
    required: Option<bool>,
    depends_on: Option<Vec<String>>,
}

fn is_registry_managed_module(spec: &ModuleSpec) -> bool {
    spec.crate_name != "rustok-outbox"
}

fn normalize_deps(deps: Option<Vec<String>>) -> HashSet<String> {
    deps.unwrap_or_default().into_iter().collect()
}

pub fn validate_registry_vs_manifest(registry: &ModuleRegistry) -> loco_rs::Result<()> {
    let manifest_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../modules.toml");
    let raw = std::fs::read_to_string(&manifest_path).map_err(|error| {
        loco_rs::Error::BadRequest(format!(
            "Failed to read modules manifest {}: {error}",
            manifest_path.display()
        ))
    })?;

    let manifest: ModulesManifest = toml::from_str(&raw).map_err(|error| {
        loco_rs::Error::BadRequest(format!(
            "Failed to parse modules manifest {}: {error}",
            manifest_path.display()
        ))
    })?;

    let missing_in_registry: Vec<String> = manifest
        .modules
        .iter()
        .filter(|(_, spec)| is_registry_managed_module(spec))
        .map(|(slug, _)| slug)
        .filter(|slug| !registry.contains(slug))
        .cloned()
        .collect();

    let missing_in_manifest: Vec<String> = registry
        .list()
        .into_iter()
        .map(|module| module.slug().to_string())
        .filter(|slug| !manifest.modules.contains_key(slug))
        .collect();

    if !missing_in_registry.is_empty() || !missing_in_manifest.is_empty() {
        return Err(loco_rs::Error::BadRequest(format!(
            "modules.toml and ModuleRegistry are out of sync; missing in registry: [{}], missing in manifest: [{}]",
            missing_in_registry.join(", "),
            missing_in_manifest.join(", ")
        )));
    }

    let required_mismatch: Vec<String> = registry
        .list()
        .into_iter()
        .filter_map(|module| {
            manifest.modules.get(module.slug()).and_then(|spec| {
                if !is_registry_managed_module(spec) {
                    None
                } else {
                    Some((
                        module.slug(),
                        spec.required.unwrap_or(false),
                        registry.is_core(module.slug()),
                    ))
                }
            })
        })
        .filter_map(|(slug, required, is_core)| {
            if required == is_core {
                None
            } else {
                Some(format!("{slug} (required={required}, core={is_core})"))
            }
        })
        .collect();

    if !required_mismatch.is_empty() {
        return Err(loco_rs::Error::BadRequest(format!(
            "modules.toml required flags conflict with ModuleRegistry kinds: {}",
            required_mismatch.join(", ")
        )));
    }

    let dependency_mismatch: Vec<String> = registry
        .list()
        .into_iter()
        .filter_map(|module| {
            manifest.modules.get(module.slug()).and_then(|spec| {
                if !is_registry_managed_module(spec) {
                    None
                } else {
                    let manifest_deps = normalize_deps(spec.depends_on.clone());
                    let registry_deps: HashSet<String> = module
                        .dependencies()
                        .iter()
                        .map(|dep| dep.to_string())
                        .collect();

                    if manifest_deps == registry_deps {
                        None
                    } else {
                        Some(format!(
                            "{} (manifest={:?}, registry={:?})",
                            module.slug(),
                            manifest_deps,
                            registry_deps
                        ))
                    }
                }
            })
        })
        .collect();

    if !dependency_mismatch.is_empty() {
        return Err(loco_rs::Error::BadRequest(format!(
            "modules.toml depends_on conflict with ModuleRegistry dependencies: {}",
            dependency_mismatch.join(", ")
        )));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_registry_vs_manifest;
    use crate::modules::build_registry;

    #[test]
    fn registry_matches_modules_manifest() {
        let registry = build_registry();
        let result = validate_registry_vs_manifest(&registry);
        assert!(
            result.is_ok(),
            "modules manifest should match runtime registry: {result:?}"
        );
    }
}
