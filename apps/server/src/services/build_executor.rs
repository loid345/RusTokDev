use std::path::PathBuf;
use std::process::Stdio;
use std::sync::Arc;

use anyhow::{anyhow, bail, Context};
use serde::Serialize;
use tokio::process::Command;
use uuid::Uuid;

use loco_rs::app::AppContext;

use crate::models::build::{BuildStage, BuildStatus, Model as Build};
use crate::models::release::Model as Release;
use crate::modules::BuildExecutionPlan;
use crate::services::build_event_hub::{build_event_hub_from_context, BuildEventHubPublisher};
use crate::services::build_service::{BuildEventPublisher, BuildService};

const DEFAULT_CARGO_BIN: &str = "cargo";
const BUILD_CARGO_BIN_ENV: &str = "RUSTOK_BUILD_CARGO_BIN";

#[derive(Debug, Clone, Serialize)]
pub struct BuildExecutionReport {
    pub build_id: Uuid,
    pub status: String,
    pub cargo_command: String,
    pub release_id: Option<String>,
    pub release_status: Option<String>,
}

pub struct BuildExecutionService {
    build_service: BuildService,
}

impl BuildExecutionService {
    pub fn new(ctx: &AppContext) -> Self {
        Self::with_event_publisher(
            ctx,
            Arc::new(BuildEventHubPublisher::new(build_event_hub_from_context(
                ctx,
            ))),
        )
    }

    pub fn with_event_publisher(
        ctx: &AppContext,
        event_publisher: Arc<dyn BuildEventPublisher>,
    ) -> Self {
        Self {
            build_service: BuildService::with_event_publisher(ctx.db.clone(), event_publisher),
        }
    }

    pub async fn execute_next_queued_build(
        &self,
        dry_run: bool,
    ) -> anyhow::Result<Option<BuildExecutionReport>> {
        if let Some(running) = self.build_service.running_build().await? {
            bail!("build {} is already running", running.id);
        }

        let Some(build) = self.build_service.next_queued_build().await? else {
            return Ok(None);
        };

        let report = self.execute_build(build.id, dry_run).await?;
        Ok(Some(report))
    }

    pub async fn execute_build(
        &self,
        build_id: Uuid,
        dry_run: bool,
    ) -> anyhow::Result<BuildExecutionReport> {
        let build = self
            .build_service
            .get_build(build_id)
            .await?
            .ok_or_else(|| anyhow!("Build not found"))?;

        if build.is_final() {
            bail!(
                "build {} is already final and cannot be executed again",
                build.id
            );
        }

        if build.status == BuildStatus::Running {
            bail!("build {} is already running", build.id);
        }

        if let Some(running) = self.build_service.running_build().await? {
            if running.id != build.id {
                bail!("build {} is already running", running.id);
            }
        }

        let plan = build_execution_plan(&build)?;
        let spec = BuildCommandSpec::from_plan(&plan);

        if dry_run {
            return Ok(BuildExecutionReport {
                build_id: build.id,
                status: "dry-run".to_string(),
                cargo_command: spec.render(),
                release_id: None,
                release_status: None,
            });
        }

        self.build_service
            .update_build_status(
                build.id,
                BuildStatus::Running,
                Some(BuildStage::Checkout),
                Some(5),
            )
            .await?;

        self.build_service
            .update_build_status(
                build.id,
                BuildStatus::Running,
                Some(BuildStage::Build),
                Some(35),
            )
            .await?;

        let status = run_build_command(&spec)
            .await
            .with_context(|| format!("failed to execute build command for build {}", build.id));

        match status {
            Ok(()) => {
                self.build_service
                    .update_build_status(
                        build.id,
                        BuildStatus::Success,
                        Some(BuildStage::Complete),
                        Some(100),
                    )
                    .await?;

                Ok(BuildExecutionReport {
                    build_id: build.id,
                    status: "success".to_string(),
                    cargo_command: spec.render(),
                    release_id: None,
                    release_status: None,
                })
            }
            Err(error) => {
                self.build_service
                    .fail_build(build.id, error.to_string())
                    .await?;

                Err(error)
            }
        }
    }

    pub async fn ensure_release_for_build(
        &self,
        build_id: Uuid,
        environment: &str,
        activate: bool,
    ) -> anyhow::Result<Release> {
        let build = self
            .build_service
            .get_build(build_id)
            .await?
            .ok_or_else(|| anyhow!("Build not found"))?;

        if build.status != BuildStatus::Success {
            bail!(
                "build {} must be successful before creating a release",
                build.id
            );
        }

        let release = if let Some(release_id) = build.release_id.clone() {
            self.build_service
                .get_release(&release_id)
                .await?
                .ok_or_else(|| anyhow!("release {} referenced by build is missing", release_id))?
        } else {
            let modules = build_module_slugs(&build)?;
            self.build_service
                .create_release(build.id, environment.to_string(), modules)
                .await?
        };

        if activate && release.status != crate::models::release::ReleaseStatus::Active {
            return self.build_service.activate_release(&release.id).await;
        }

        Ok(release)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BuildCommandSpec {
    program: String,
    args: Vec<String>,
    workdir: PathBuf,
    manifest_path: PathBuf,
}

impl BuildCommandSpec {
    fn from_plan(plan: &BuildExecutionPlan) -> Self {
        let program =
            std::env::var(BUILD_CARGO_BIN_ENV).unwrap_or_else(|_| DEFAULT_CARGO_BIN.to_string());
        let workdir = workspace_root();
        let manifest_path = workdir.join("modules.toml");

        let mut args = vec![
            "build".to_string(),
            "-p".to_string(),
            plan.cargo_package.clone(),
        ];
        if plan.cargo_profile == "release" {
            args.push("--release".to_string());
        } else {
            args.push("--profile".to_string());
            args.push(plan.cargo_profile.clone());
        }
        if let Some(target) = &plan.cargo_target {
            args.push("--target".to_string());
            args.push(target.clone());
        }
        if !plan.cargo_features.is_empty() {
            args.push("--features".to_string());
            args.push(plan.cargo_features.join(","));
        }

        Self {
            program,
            args,
            workdir,
            manifest_path,
        }
    }

    fn render(&self) -> String {
        std::iter::once(self.program.as_str())
            .chain(self.args.iter().map(String::as_str))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .map(PathBuf::from)
        .expect("workspace root should be resolvable from apps/server")
}

fn build_execution_plan(build: &Build) -> anyhow::Result<BuildExecutionPlan> {
    let value = build
        .modules_delta
        .as_ref()
        .ok_or_else(|| anyhow!("build {} does not contain execution metadata", build.id))?;

    let plan = value
        .get("execution_plan")
        .ok_or_else(|| anyhow!("build {} is missing execution_plan metadata", build.id))?;

    serde_json::from_value(plan.clone()).map_err(|error| {
        anyhow!(
            "build {} has invalid execution_plan metadata: {error}",
            build.id
        )
    })
}

fn build_module_slugs(build: &Build) -> anyhow::Result<Vec<String>> {
    let value = build
        .modules_delta
        .as_ref()
        .ok_or_else(|| anyhow!("build {} does not contain module metadata", build.id))?;

    let modules = value
        .get("modules")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| anyhow!("build {} is missing modules metadata", build.id))?;

    let mut slugs = modules.keys().cloned().collect::<Vec<_>>();
    slugs.sort();
    Ok(slugs)
}

async fn run_build_command(spec: &BuildCommandSpec) -> anyhow::Result<()> {
    let status = Command::new(&spec.program)
        .args(&spec.args)
        .current_dir(&spec.workdir)
        .env("RUSTOK_MODULES_MANIFEST", &spec.manifest_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await
        .with_context(|| format!("failed to spawn {}", spec.render()))?;

    if !status.success() {
        let exit_code = status
            .code()
            .map(|code| code.to_string())
            .unwrap_or_else(|| "terminated by signal".to_string());
        bail!(
            "build command failed with exit status {exit_code}: {}",
            spec.render()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{build_execution_plan, workspace_root, BuildCommandSpec};
    use crate::models::build::{DeploymentProfile, Model as Build};
    use crate::modules::BuildExecutionPlan;

    fn build_with_plan(plan: &BuildExecutionPlan) -> Build {
        let mut build = Build::new(
            "refs/heads/main".to_string(),
            "hash".to_string(),
            "tester".to_string(),
            DeploymentProfile::Monolith,
        );
        build.modules_delta = Some(serde_json::json!({
            "summary": "+pages",
            "execution_plan": plan,
        }));
        build
    }

    #[test]
    fn parses_execution_plan_from_build_metadata() {
        let plan = BuildExecutionPlan {
            cargo_package: "rustok-server".to_string(),
            cargo_profile: "release".to_string(),
            cargo_target: Some("x86_64-unknown-linux-gnu".to_string()),
            cargo_features: vec!["embed-admin".to_string(), "embed-storefront".to_string()],
            cargo_command: "cargo build -p rustok-server --release".to_string(),
        };

        let parsed = build_execution_plan(&build_with_plan(&plan)).unwrap();
        assert_eq!(parsed, plan);
    }

    #[test]
    fn derives_command_spec_from_plan() {
        let plan = BuildExecutionPlan {
            cargo_package: "rustok-server".to_string(),
            cargo_profile: "release".to_string(),
            cargo_target: Some("x86_64-unknown-linux-gnu".to_string()),
            cargo_features: vec!["embed-admin".to_string()],
            cargo_command: String::new(),
        };

        let spec = BuildCommandSpec::from_plan(&plan);
        assert_eq!(
            spec.args[0..4],
            ["build", "-p", "rustok-server", "--release"]
        );
        assert!(spec.args.contains(&"x86_64-unknown-linux-gnu".to_string()));
        assert!(spec.args.contains(&"embed-admin".to_string()));
    }

    #[test]
    fn extracts_sorted_module_slugs_from_build_metadata() {
        let plan = BuildExecutionPlan {
            cargo_package: "rustok-server".to_string(),
            cargo_profile: "release".to_string(),
            cargo_target: None,
            cargo_features: vec![],
            cargo_command: String::new(),
        };

        let mut build = build_with_plan(&plan);
        build.modules_delta = Some(serde_json::json!({
            "summary": "+pages,+blog",
            "modules": {
                "pages": { "crate_name": "rustok-pages" },
                "blog": { "crate_name": "rustok-blog" }
            },
            "execution_plan": plan,
        }));

        let modules = super::build_module_slugs(&build).unwrap();
        assert_eq!(modules, vec!["blog".to_string(), "pages".to_string()]);
    }

    #[test]
    fn workspace_root_points_to_repo_root() {
        assert!(workspace_root().join("modules.toml").exists());
    }
}
