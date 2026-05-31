#!/usr/bin/env node

import { spawnSync } from "node:child_process";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const repoRoot = path.resolve(__dirname, "..", "..", "..", "..");

const args = [
  "test",
  "-p",
  "rustok-pages",
  "--test",
  "page_service_kind_guard",
  "pages_builder_fallback",
];

console.log(
  `[verify-page-builder-pages-fallback-gate] running cargo ${args.join(" ")}`,
);
const run = spawnSync("cargo", args, {
  cwd: repoRoot,
  stdio: "inherit",
});

if (run.status !== 0) {
  console.error("[verify-page-builder-pages-fallback-gate] FAIL");
  process.exit(run.status ?? 1);
}

console.log("[verify-page-builder-pages-fallback-gate] PASS");
