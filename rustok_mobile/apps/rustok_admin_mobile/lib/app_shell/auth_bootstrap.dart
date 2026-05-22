import 'package:app_graphql/app_graphql.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:graphql/client.dart';

const _defaultServerBaseUrl = String.fromEnvironment(
  'RUSTOK_SERVER_BASE_URL',
  defaultValue: 'http://localhost:8080',
);
const _defaultTenantSlug = String.fromEnvironment(
  'RUSTOK_TENANT_SLUG',
  defaultValue: 'default',
);
const _defaultLocale = String.fromEnvironment(
  'RUSTOK_LOCALE',
  defaultValue: 'en',
);

Uri _serverBaseUri() => Uri.parse(_defaultServerBaseUrl);

final authSessionStoreProvider = Provider<AuthSessionStore>((ref) {
  return InMemoryAuthSessionStore();
});

final refreshClientProvider = Provider<GraphQLClient>((ref) {
  final config = GraphQlClientConfig(
    baseUri: _serverBaseUri(),
    context: const GraphQlRequestContext(
      tenantSlug: _defaultTenantSlug,
      locale: _defaultLocale,
    ),
  );
  return const GraphQlClientFactory().createHttpOnly(config);
});

final refreshTokenServiceProvider = Provider<RefreshTokenService>((ref) {
  final client = ref.watch(refreshClientProvider);
  return GraphQlRefreshTokenService(client: client);
});

final authSessionManagerProvider = Provider<AuthSessionManager>((ref) {
  final store = ref.watch(authSessionStoreProvider);
  final refreshService = ref.watch(refreshTokenServiceProvider);
  return AuthSessionManager(store: store, refreshTokenService: refreshService);
});

final authSessionProvider = FutureProvider<AuthSession?>((ref) async {
  final manager = ref.watch(authSessionManagerProvider);
  return manager.readValidSession();
});

final graphQlConfigProvider = Provider<GraphQlClientConfig>((ref) {
  final session = ref.watch(authSessionProvider).valueOrNull;
  return GraphQlClientConfig(
    baseUri: _serverBaseUri(),
    context: GraphQlRequestContext(
      tenantSlug: _defaultTenantSlug,
      locale: _defaultLocale,
      accessToken: session?.accessToken,
    ),
  );
});

final graphQlClientProvider = Provider<GraphQLClient>((ref) {
  final config = ref.watch(graphQlConfigProvider);
  return const GraphQlClientFactory().create(config);
});

final authBootstrapProbeProvider = FutureProvider<BootstrapProbeResult>((ref) async {
  final session = await ref.watch(authSessionProvider.future);
  if (session == null) {
    return const BootstrapProbeResult.unauthenticated();
  }

  final client = ref.watch(graphQlClientProvider);
  const document = r'''
    query BootstrapProbe {
      me {
        id
        email
      }
      currentTenant {
        id
        slug
      }
    }
  ''';

  final result = await client.query(
    QueryOptions(document: gql(document), fetchPolicy: FetchPolicy.networkOnly),
  );
  if (result.hasException) {
    throw result.exception!;
  }

  final payload = result.data ?? const <String, dynamic>{};
  return BootstrapProbeResult.authenticated(
    me: payload['me'] as Map<String, dynamic>?,
    currentTenant: payload['currentTenant'] as Map<String, dynamic>?,
  );
});

class BootstrapProbeResult {
  const BootstrapProbeResult._({
    required this.isAuthenticated,
    this.me,
    this.currentTenant,
  });

  const BootstrapProbeResult.unauthenticated()
    : this._(isAuthenticated: false);

  const BootstrapProbeResult.authenticated({
    Map<String, dynamic>? me,
    Map<String, dynamic>? currentTenant,
  }) : this._(
         isAuthenticated: true,
         me: me,
         currentTenant: currentTenant,
       );

  final bool isAuthenticated;
  final Map<String, dynamic>? me;
  final Map<String, dynamic>? currentTenant;
}
