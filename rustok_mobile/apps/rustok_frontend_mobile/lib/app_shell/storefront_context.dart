import 'package:app_graphql/app_graphql.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:graphql/client.dart';
import 'package:rustok_catalog_mobile/rustok_catalog_mobile.dart';

const _defaultServerBaseUrl = String.fromEnvironment(
  'RUSTOK_STOREFRONT_SERVER_BASE_URL',
  defaultValue: 'http://localhost:8080',
);
const _defaultTenantSlug = String.fromEnvironment(
  'RUSTOK_STOREFRONT_TENANT_SLUG',
  defaultValue: 'default',
);
const _defaultLocale = String.fromEnvironment(
  'RUSTOK_STOREFRONT_LOCALE',
  defaultValue: 'en',
);

Uri _serverBaseUri(String serverBaseUrl) => Uri.parse(serverBaseUrl);

final storefrontRuntimeContextProvider = Provider<StorefrontRuntimeContext>((
  ref,
) {
  return const StorefrontRuntimeContext(
    serverBaseUrl: _defaultServerBaseUrl,
    tenantSlug: _defaultTenantSlug,
    locale: _defaultLocale,
  );
});

final storefrontGraphQlConfigProvider = Provider<GraphQlClientConfig>((ref) {
  final runtime = ref.watch(storefrontRuntimeContextProvider);
  return GraphQlClientConfig(
    baseUri: _serverBaseUri(runtime.serverBaseUrl),
    context: GraphQlRequestContext(
      tenantSlug: runtime.tenantSlug,
      locale: runtime.locale,
    ),
  );
});

final storefrontGraphQlClientProvider = Provider<GraphQLClient>((ref) {
  final config = ref.watch(storefrontGraphQlConfigProvider);
  return const GraphQlClientFactory().create(config);
});

class StorefrontRuntimeContext {
  const StorefrontRuntimeContext({
    required this.serverBaseUrl,
    required this.tenantSlug,
    required this.locale,
  });

  final String serverBaseUrl;
  final String tenantSlug;
  final String locale;
}

final hostStorefrontCatalogRepositoryProvider =
    Provider<StorefrontCatalogRepository>((ref) {
  final runtime = ref.watch(storefrontRuntimeContextProvider);
  return StorefrontPreviewCatalogRepository(locale: runtime.locale);
});

class StorefrontPreviewCatalogRepository implements StorefrontCatalogRepository {
  const StorefrontPreviewCatalogRepository({required this.locale});

  final String locale;

  @override
  Future<List<StorefrontProductSummary>> featuredProducts() async {
    final isRussian = locale.toLowerCase().startsWith('ru');
    return [
      StorefrontProductSummary(
        id: 'creator-kit',
        title: isRussian ? 'Набор автора' : 'Creator kit',
        description: isRussian
            ? 'Витринная карточка каталога из module-owned Flutter package.'
            : 'Storefront catalog card from a module-owned Flutter package.',
        priceLabel: isRussian ? '4 900 ₽' : '49.00 USD',
        badge: isRussian ? 'Новинка' : 'New',
      ),
      StorefrontProductSummary(
        id: 'launch-pack',
        title: isRussian ? 'Launch pack' : 'Launch pack',
        description: isRussian
            ? 'Демонстрационный customer-facing товар без admin UX.'
            : 'Customer-facing demo product without admin UX.',
        priceLabel: isRussian ? '9 900 ₽' : '99.00 USD',
      ),
    ];
  }

  @override
  Future<List<StorefrontCartLine>> cartLines() async {
    final isRussian = locale.toLowerCase().startsWith('ru');
    return [
      StorefrontCartLine(
        productId: 'creator-kit',
        title: isRussian ? 'Набор автора' : 'Creator kit',
        quantity: 1,
        priceLabel: isRussian ? '4 900 ₽' : '49.00 USD',
      ),
    ];
  }
}
