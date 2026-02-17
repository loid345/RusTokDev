import { registerStorefrontModule } from "./registry";

function ModuleSpotlight() {
  return (
    <div className="rounded-3xl border border-slate-200 bg-slate-50 p-6 shadow-sm">
      <h3 className="text-lg font-semibold text-slate-900">
        Modular storefront sections
      </h3>
      <p className="mt-2 text-sm text-slate-600">
        Extend the home page by registering React components from optional
        packages.
      </p>
      <div className="mt-4 flex flex-wrap gap-2 text-xs">
        <span className="rounded-full bg-sky-100 px-3 py-1 font-semibold text-sky-700">
          Next.js
        </span>
        <span className="rounded-full bg-emerald-100 px-3 py-1 font-semibold text-emerald-700">
          Registry
        </span>
        <span className="rounded-full bg-slate-100 px-3 py-1 font-semibold text-slate-600">
          Packages
        </span>
      </div>
    </div>
  );
}

registerStorefrontModule({
  id: "core-module-spotlight",
  slot: "home:afterHero",
  order: 10,
  render: () => <ModuleSpotlight />,
});
