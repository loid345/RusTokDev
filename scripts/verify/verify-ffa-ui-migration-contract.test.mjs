#!/usr/bin/env node

import { mkdtempSync, writeFileSync, mkdirSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { spawnSync } from "node:child_process";

const scriptPath = path.resolve("scripts/verify/verify-ffa-ui-migration-contract.mjs");

function makeFixture({ pipeline }) {
  const root = mkdtempSync(path.join(tmpdir(), "rustok-ffa-verify-"));
  mkdirSync(path.join(root, "docs", "research"), { recursive: true });
  mkdirSync(path.join(root, "docs", "verification"), { recursive: true });

  writeFileSync(
    path.join(root, "docs", "research", "dioxus-ffa-ui-migration-plan.md"),
    [
      "## Фазы реализации",
      "## Принцип исполнения backlog (одна задача за итерацию)",
      "## Политика актуализации verification scripts",
      "## Phase-gate критерии (обязательные переходы между фазами)",
      "## KPI parity (измеримые пороги)",
      "Функциональный parity",
      "Error parity",
      "Performance guard",
      "Contract guard",
      "Docs guard",
      "## RACI (кто принимает phase-gates)",
    ].join("\n"),
  );

  writeFileSync(
    path.join(root, "docs", "research", "dioxus-ffa-pilot-connectivity-map.md"),
    "rustok-pages\nrustok-search\n",
  );

  writeFileSync(
    path.join(root, "docs", "verification", "ffa-ui-parity-checklist.md"),
    [
      "- [ ] Native path (Leptos SSR/hydrate) работает для целевого сценария.",
      "- [ ] GraphQL fallback работает для того же сценария.",
      "- [ ] Headless host path (Next/mobile/external) не сломан.",
      "- [ ] GraphQL/REST surface не удалён и не ослаблен.",
      "- [ ] UI слой не владеет transport/business логикой.",
      "- [ ] Доступ к transport идёт через core ports.",
      "- [ ] Core слой не зависит от `leptos*`.",
      "- [ ] Выполнен `npm run verify:ffa:ui:migration`.",
    ].join("\n"),
  );

  writeFileSync(
    path.join(root, "docs", "index.md"),
    [
      "[plan](./research/dioxus-ffa-ui-migration-plan.md)",
      "[map](./research/dioxus-ffa-pilot-connectivity-map.md)",
      "[check](./verification/ffa-ui-parity-checklist.md)",
    ].join("\n"),
  );

  writeFileSync(
    path.join(root, "package.json"),
    JSON.stringify(
      {
        scripts: {
          "verify:ffa:ui:migration": pipeline,
          "verify:ffa:ui:migration:contract":
            "node scripts/verify/verify-ffa-ui-migration-contract.mjs",
          "verify:ffa:ui:migration:docs": "bash scripts/verify/verify-ffa-ui-doc-patterns.sh",
        },
      },
      null,
      2,
    ),
  );

  return root;
}

function runWithRoot(root) {
  return spawnSync(process.execPath, [scriptPath], {
    env: { ...process.env, RUSTOK_VERIFY_ROOT: root },
    encoding: "utf8",
  });
}

const okRoot = makeFixture({
  pipeline: "npm run verify:ffa:ui:migration:contract && npm run verify:ffa:ui:migration:docs",
});
const ok = runWithRoot(okRoot);
if (ok.status !== 0) {
  console.error("Expected PASS fixture to succeed");
  console.error(ok.stdout);
  console.error(ok.stderr);
  process.exit(1);
}

const badRoot = makeFixture({ pipeline: "npm run verify:ffa:ui:migration:contract" });
const bad = runWithRoot(badRoot);
if (bad.status === 0) {
  console.error("Expected FAIL fixture to fail");
  console.error(bad.stdout);
  console.error(bad.stderr);
  process.exit(1);
}

if (!bad.stderr.includes("verify:ffa:ui:migration:docs")) {
  console.error("Expected missing docs command error in stderr");
  console.error(bad.stderr);
  process.exit(1);
}

console.log("[verify-ffa-ui-migration-contract:test] PASS");
