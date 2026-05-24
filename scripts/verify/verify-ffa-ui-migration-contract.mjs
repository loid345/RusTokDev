#!/usr/bin/env node

import { readFileSync, existsSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const repoRoot = path.resolve(__dirname, "../..");

const requiredDocs = [
  "docs/research/dioxus-ffa-ui-migration-plan.md",
  "docs/research/dioxus-ffa-pilot-connectivity-map.md",
  "docs/verification/ffa-ui-parity-checklist.md",
  "docs/index.md",
];

const requiredPlanHeadings = [
  "Фазы реализации",
  "Принцип исполнения backlog (одна задача за итерацию)",
  "Политика актуализации verification scripts",
  "Phase-gate критерии (обязательные переходы между фазами)",
  "KPI parity (измеримые пороги)",
  "RACI (кто принимает phase-gates)",
];

const requiredChecklistChecks = [
  {
    label: "native path checklist item",
    pattern: /- \[[ xX]\] Native path \(Leptos SSR\/hydrate\) работает для целевого сценария\./,
  },
  {
    label: "graphql fallback checklist item",
    pattern: /- \[[ xX]\] GraphQL fallback работает для того же сценария\./,
  },
  {
    label: "ui/business ownership checklist item",
    pattern: /- \[[ xX]\] UI слой не владеет transport\/business логикой\./,
  },
  {
    label: "transport-through-core checklist item",
    pattern: /- \[[ xX]\] Доступ к transport идёт через core ports\./,
  },
  {
    label: "core-leptos-independence checklist item",
    pattern: /- \[[ xX]\] Core слой не зависит от `leptos\*`\./,
  },
  {
    label: "ffa verify evidence checklist item",
    pattern: /- \[[ xX]\] Выполнен `npm run verify:ffa:ui:migration`\./,
  },
];

const requiredConnectivityMentions = ["rustok-pages", "rustok-search"];

const requiredIndexRefs = [
  "dioxus-ffa-ui-migration-plan.md",
  "dioxus-ffa-pilot-connectivity-map.md",
  "ffa-ui-parity-checklist.md",
];

function assertFileExists(relPath) {
  const fullPath = path.join(repoRoot, relPath);
  if (!existsSync(fullPath)) {
    throw new Error(`Отсутствует обязательный документ: ${relPath}`);
  }
  return fullPath;
}

function normalizeMarkdown(content) {
  return content.replace(/\r\n/g, "\n").replace(/[ \t]+$/gm, "");
}

function getMarkdownHeadings(content) {
  return content
    .split("\n")
    .map((line, index) => {
      const match = line.match(/^#{1,6}\s+(.*)$/);
      return match ? { text: match[1].trim(), line: index + 1 } : null;
    })
    .filter(Boolean);
}

function findLineNumber(content, pattern) {
  const lines = content.split("\n");
  const index = lines.findIndex((line) => pattern.test(line));
  return index >= 0 ? index + 1 : null;
}

function readRequiredDocs() {
  const [planPath, connectivityPath, checklistPath, docsIndexPath] = requiredDocs.map(assertFileExists);

  return {
    plan: normalizeMarkdown(readFileSync(planPath, "utf8")),
    connectivity: normalizeMarkdown(readFileSync(connectivityPath, "utf8")),
    checklist: normalizeMarkdown(readFileSync(checklistPath, "utf8")),
    docsIndex: normalizeMarkdown(readFileSync(docsIndexPath, "utf8")),
  };
}

function collectValidationErrors({ plan, connectivity, checklist, docsIndex }) {
  const errors = [];

  const planHeadingIndex = new Map(
    getMarkdownHeadings(plan).map((heading) => [heading.text, heading.line]),
  );

  requiredPlanHeadings.forEach((heading) => {
    if (!planHeadingIndex.has(heading)) {
      errors.push(`Не найден обязательный heading в migration plan: ${heading}`);
    }
  });

  requiredChecklistChecks.forEach(({ label, pattern }) => {
    const line = findLineNumber(checklist, pattern);
    if (line === null) {
      errors.push(`Не найден обязательный checklist-паттерн (${label})`);
    }
  });

  requiredConnectivityMentions.forEach((mention) => {
    if (!connectivity.includes(mention)) {
      errors.push(`Не найден обязательный пилот в connectivity map: ${mention}`);
    }
  });

  requiredIndexRefs.forEach((refPath) => {
    if (!docsIndex.includes(refPath)) {
      errors.push(`Не найдена обязательная ссылка в docs/index.md: ${refPath}`);
    }
  });

  if (errors.length === 0) {
    requiredPlanHeadings.forEach((heading) => {
      const line = planHeadingIndex.get(heading);
      if (line != null) {
        console.log(`[verify-ffa-ui-migration-contract] heading ok: ${heading} (line ${line})`);
      }
    });
  }

  return errors.sort((a, b) => a.localeCompare(b, "ru"));
}

try {
  const docs = readRequiredDocs();
  const errors = collectValidationErrors(docs);

  if (errors.length > 0) {
    console.error("[verify-ffa-ui-migration-contract] FAIL");
    errors.forEach((error) => console.error(`- ${error}`));
    process.exit(1);
  }

  console.log("[verify-ffa-ui-migration-contract] PASS");
  console.log("Проверены обязательные документы и baseline-контракты FFA migration.");
} catch (error) {
  console.error("[verify-ffa-ui-migration-contract] FAIL");
  console.error(error instanceof Error ? error.message : String(error));
  process.exit(1);
}
