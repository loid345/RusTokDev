const API_URL = process.env.NEXT_PUBLIC_API_URL ?? 'http://localhost:5150';

export interface ModuleInfo {
  slug: string;
  name: string;
  description: string;
  version: string;
  kind: 'core' | 'optional';
  dependencies: string[];
  enabled: boolean;
}

export interface ModulesListResponse {
  modules: ModuleInfo[];
}

interface FetchOptions {
  token?: string | null;
  tenantSlug?: string | null;
}

function buildHeaders(opts: FetchOptions): Record<string, string> {
  const headers: Record<string, string> = { 'Content-Type': 'application/json' };
  if (opts.token) headers['Authorization'] = `Bearer ${opts.token}`;
  if (opts.tenantSlug) headers['X-Tenant-Slug'] = opts.tenantSlug;
  return headers;
}

export async function listModules(
  opts: FetchOptions = {}
): Promise<ModulesListResponse> {
  const res = await fetch(`${API_URL}/api/modules`, {
    headers: buildHeaders(opts),
    cache: 'no-store'
  });
  if (!res.ok) throw new Error(`listModules failed: ${res.status}`);
  return res.json();
}

export async function toggleModule(
  slug: string,
  enabled: boolean,
  opts: FetchOptions = {}
): Promise<ModuleInfo> {
  const res = await fetch(`${API_URL}/api/modules/${slug}/toggle`, {
    method: 'PUT',
    headers: buildHeaders(opts),
    body: JSON.stringify({ enabled })
  });
  if (!res.ok) {
    const text = await res.text();
    throw new Error(text || `toggleModule failed: ${res.status}`);
  }
  return res.json();
}
