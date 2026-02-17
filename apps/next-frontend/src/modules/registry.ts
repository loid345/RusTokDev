import type { ReactNode } from "react";

export type StorefrontSlot = "home:afterHero";

export type StorefrontModule = {
  id: string;
  slot: StorefrontSlot;
  order?: number;
  render: () => ReactNode;
};

const registry = new Map<string, StorefrontModule>();

export function registerStorefrontModule(module: StorefrontModule) {
  registry.set(module.id, module);
}

export function getModulesForSlot(slot: StorefrontSlot) {
  return Array.from(registry.values())
    .filter((module) => module.slot === slot)
    .sort((left, right) => {
      const leftOrder = left.order ?? 0;
      const rightOrder = right.order ?? 0;
      if (leftOrder !== rightOrder) {
        return leftOrder - rightOrder;
      }
      return left.id.localeCompare(right.id);
    });
}
