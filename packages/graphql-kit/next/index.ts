export const GRAPHQL_ENDPOINT = "/api/graphql";
export const TENANT_HEADER = "X-Tenant-Slug";
export const AUTH_HEADER = "Authorization";

export type GraphqlRequest<V = Record<string, unknown>> = {
  query: string;
  variables?: V;
};

export type GraphqlError = {
  message: string;
};

export type GraphqlResponse<T> = {
  data?: T;
  errors?: GraphqlError[];
};

export type GraphqlFetchOptions<V> = {
  baseUrl?: string;
  token?: string;
  tenant?: string;
  request: GraphqlRequest<V>;
};

export async function fetchGraphql<T, V = Record<string, unknown>>(
  options: GraphqlFetchOptions<V>,
): Promise<GraphqlResponse<T>> {
  const { baseUrl = "", token, tenant, request } = options;
  const headers: Record<string, string> = {
    "Content-Type": "application/json",
  };

  if (tenant) {
    headers[TENANT_HEADER] = tenant;
  }

  if (token) {
    headers[AUTH_HEADER] = `Bearer ${token}`;
  }

  const response = await fetch(`${baseUrl}${GRAPHQL_ENDPOINT}`, {
    method: "POST",
    headers,
    body: JSON.stringify(request),
  });

  if (!response.ok) {
    return { errors: [{ message: `HTTP ${response.status}` }] };
  }

  return (await response.json()) as GraphqlResponse<T>;
}
