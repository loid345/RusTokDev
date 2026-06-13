#!/usr/bin/env node
// RusTok AI admin FFA boundary guardrails for the first core slice.

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = process.env.RUSTOK_VERIFY_REPO_ROOT
  ? path.resolve(process.env.RUSTOK_VERIFY_REPO_ROOT)
  : path.resolve(scriptDir, "../..");
const failures = [];

function repoPath(relativePath) {
  return path.join(repoRoot, relativePath);
}

function readRepo(relativePath) {
  return readFileSync(repoPath(relativePath), "utf8");
}

function fail(message) {
  failures.push(message);
}

function assertExists(relativePath, description) {
  if (!existsSync(repoPath(relativePath))) fail(description);
}

function assertContains(text, pattern, description) {
  const found = typeof pattern === "string" ? text.includes(pattern) : pattern.test(text);
  if (!found) fail(description);
}

function assertNotContains(text, pattern, description) {
  const found = typeof pattern === "string" ? text.includes(pattern) : pattern.test(text);
  if (found) fail(description);
}

const libPath = "crates/rustok-ai/admin/src/lib.rs";
const corePath = "crates/rustok-ai/admin/src/core.rs";
const transportModPath = "crates/rustok-ai/admin/src/transport/mod.rs";

assertExists(libPath, `${libPath}: expected AI admin Leptos adapter file`);
assertExists(corePath, `${corePath}: expected AI admin core slice file`);
assertExists(transportModPath, `${transportModPath}: expected AI admin transport facade file`);

const lib = readRepo(libPath);
const core = readRepo(corePath);
const transportMod = readRepo(transportModPath);

assertContains(lib, "mod core;", `${libPath}: crate root must wire core`);
assertContains(lib, "mod transport;", `${libPath}: crate root must wire transport facade`);
assertContains(lib, "use crate::core::{", `${libPath}: Leptos adapter must import core-owned helpers`);
assertContains(lib, "product_attributes_task_payload", `${libPath}: Leptos adapter must call core-owned payload builder`);
assertNotContains(lib, "fn product_attributes_task_payload", `${libPath}: payload builder must not live in the Leptos adapter`);
assertNotContains(lib, "fn parse_csv(value: String)", `${libPath}: CSV request normalization must not live in the Leptos adapter`);
assertNotContains(lib, /t\(\s*locale,\s*locale,/, `${libPath}: i18n helper calls must not pass locale twice`);
assertContains(lib, "transport::fetch_bootstrap", `${libPath}: Leptos adapter must call the AI transport facade`);
assertNotContains(lib, /(^|[^A-Za-z0-9_])api::/, `${libPath}: Leptos adapter must not call the raw pre-FFA api module`);

for (const marker of ["leptos::", "leptos_", "#[component]", "#[server]", "RwSignal", "LocalResource", "web_sys::"]) {
  assertNotContains(core, marker, `${corePath}: core must stay UI/runtime free (${marker})`);
}
assertContains(transportMod, "pub use crate::api::{", `${transportModPath}: transport facade must wrap the existing native api module`);
assertContains(transportMod, "fetch_bootstrap", `${transportModPath}: transport facade must expose bootstrap loading`);
assertContains(transportMod, "run_task_job", `${transportModPath}: transport facade must expose direct job execution`);

for (const marker of [
  "pub fn parse_csv",
  "pub fn optional_text",
  "pub fn alloy_task_payload",
  "pub fn image_task_payload",
  "pub fn product_task_payload",
  "pub fn product_attributes_task_payload",
  "pub fn blog_task_payload",
]) {
  assertContains(core, marker, `${corePath}: expected core-owned helper ${marker}`);
}

if (failures.length > 0) {
  console.error("AI admin boundary verification failed:");
  for (const failure of failures) console.error(`- ${failure}`);
  process.exit(1);
}

console.log("AI admin boundary verification passed");
