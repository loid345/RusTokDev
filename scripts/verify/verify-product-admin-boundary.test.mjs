#!/usr/bin/env node

import { test } from "node:test";
import assert from "node:assert/strict";
import { mkdtempSync, mkdirSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { spawnSync } from "node:child_process";

const scriptPath = path.resolve("scripts/verify/verify-product-admin-boundary.mjs");

function writeFixtureFile(root, relativePath, content) {
  const filePath = path.join(root, relativePath);
  mkdirSync(path.dirname(filePath), { recursive: true });
  writeFileSync(filePath, content);
}

function libSource() {
  return `
mod api;
mod core;
mod i18n;
mod model;
mod transport;
mod ui;

pub use ui::leptos::ProductAdmin;
`;
}

function coreSource({ includeLeptos = false, omitOpenProduct = false } = {}) {
  return `
${includeLeptos ? "use leptos::prelude::*;" : ""}
pub(crate) struct ProductAdminSaveCommand;
pub(crate) struct ProductAdminEditorFormState;
pub(crate) struct ProductAdminStatusMutationResultViewModel;
pub(crate) struct ProductAdminDeleteResultViewModel;
pub(crate) struct ProductAdminSeoPanelCopy;
pub(crate) struct ProductAdminRouteQueryIntent;
pub(crate) fn parse_product_admin_inventory_quantity_input(value: &str) -> i32 { 0 }
${omitOpenProduct ? "" : "pub(crate) enum ProductAdminOpenProductViewModel { Ready, Empty }"}
pub(crate) fn product_admin_pricing_preview_state_from_result() {}
`;
}

function uiSource({ rawApiCall = false, rawServiceCall = false } = {}) {
  return `
use crate::core::{build_product_admin_save_command, ProductAdminOpenProductViewModel, product_admin_pricing_preview_state_from_result};
use crate::transport;

pub fn ProductAdmin() {
    let _transport = transport::fetch_products;
    let _save = build_product_admin_save_command;
    let _open = ProductAdminOpenProductViewModel::Empty;
    let _pricing = product_admin_pricing_preview_state_from_result;
    ${rawApiCall ? "let _raw = api::fetch_products;" : ""}
    ${rawServiceCall ? "let _service = ProductService::new;" : ""}
}
`;
}

function transportSource({ includeServerEndpoint = false } = {}) {
  return `
use crate::api;

pub async fn fetch_bootstrap() { api::fetch_bootstrap().await; }
pub async fn fetch_products() { api::fetch_products().await; }
pub async fn fetch_product() { api::fetch_product().await; }
pub async fn fetch_product_pricing() { api::fetch_product_pricing().await; }
pub async fn fetch_shipping_profiles() { api::fetch_shipping_profiles().await; }
pub async fn create_product() { api::create_product().await; }
pub async fn update_product() { api::update_product().await; }
pub async fn change_product_status() { api::change_product_status().await; }
pub async fn delete_product() { api::delete_product().await; }
${includeServerEndpoint ? '#[server(prefix = "/api/fn", endpoint = "bad")] async fn bad() {}' : ""}
`;
}

function apiSource() {
  return `
use leptos_graphql::GraphqlRequest;
pub async fn fetch_bootstrap() {}
pub async fn fetch_products() {}
pub async fn fetch_product() {}
pub async fn fetch_product_pricing() {}
pub async fn fetch_shipping_profiles() {}
pub async fn create_product() {}
pub async fn update_product() {}
pub async fn change_product_status() {}
pub async fn delete_product() {}
`;
}

function withFixture(options = {}) {
  const root = mkdtempSync(path.join(tmpdir(), "rustok-product-boundary-"));
  writeFixtureFile(root, "crates/rustok-product/admin/src/lib.rs", libSource());
  writeFixtureFile(root, "crates/rustok-product/admin/src/core.rs", coreSource(options));
  writeFixtureFile(root, "crates/rustok-product/admin/src/ui/leptos.rs", uiSource(options));
  writeFixtureFile(root, "crates/rustok-product/admin/src/transport.rs", transportSource(options));
  writeFixtureFile(root, "crates/rustok-product/admin/src/api.rs", apiSource());
  writeFixtureFile(root, "crates/rustok-product/docs/implementation-plan.md", "verify-product-admin-boundary.mjs");
  writeFixtureFile(root, "docs/modules/registry.md", "verify-product-admin-boundary.mjs");
  return root;
}

function runVerifier(root) {
  return spawnSync("node", [scriptPath], {
    cwd: path.resolve("."),
    env: { ...process.env, RUSTOK_VERIFY_REPO_ROOT: root },
    encoding: "utf8",
  });
}

test("product admin boundary verifier passes canonical fixture", () => {
  const root = withFixture();
  try {
    const result = runVerifier(root);
    assert.equal(result.status, 0, result.stderr || result.stdout);
    assert.match(result.stdout, /product admin boundary verification passed/);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("product admin boundary verifier rejects Leptos-specific core", () => {
  const root = withFixture({ includeLeptos: true });
  try {
    const result = runVerifier(root);
    assert.notEqual(result.status, 0, "Expected Leptos core fixture to fail");
    assert.match(result.stderr, /core must stay Leptos\/server-function free/);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("product admin boundary verifier rejects raw api calls from UI", () => {
  const root = withFixture({ rawApiCall: true });
  try {
    const result = runVerifier(root);
    assert.notEqual(result.status, 0, "Expected raw UI api fixture to fail");
    assert.match(result.stderr, /UI adapter must not call raw transport or services/);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("product admin boundary verifier rejects missing core open-result policy", () => {
  const root = withFixture({ omitOpenProduct: true });
  try {
    const result = runVerifier(root);
    assert.notEqual(result.status, 0, "Expected missing open-result helper fixture to fail");
    assert.match(result.stderr, /ProductAdminOpenProductViewModel/);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});

test("product admin boundary verifier rejects server functions in transport facade", () => {
  const root = withFixture({ includeServerEndpoint: true });
  try {
    const result = runVerifier(root);
    assert.notEqual(result.status, 0, "Expected transport server-function fixture to fail");
    assert.match(result.stderr, /server\/native endpoints must not live in the product admin transport facade/);
  } finally {
    rmSync(root, { recursive: true, force: true });
  }
});
