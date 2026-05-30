import 'package:graphql/client.dart';

import 'module_summary.dart';

const moduleRegistryQuery = r'''
  query ModuleRegistry {
    moduleRegistry {
      moduleSlug
      name
      description
      version
      kind
      dependencies
      enabled
      ownership
      trustLevel
      recommendedAdminSurfaces
      showcaseAdminSurfaces
    }
  }
''';

const toggleModuleMutation = r'''
  mutation ToggleModule($moduleSlug: String!, $enabled: Boolean!) {
    toggleModule(moduleSlug: $moduleSlug, enabled: $enabled) {
      moduleSlug
      enabled
      settings
    }
  }
''';

abstract interface class ModulesRepository {
  Future<List<ModuleSummary>> listModules();

  Future<ModuleToggleResult> toggleModule({
    required String moduleSlug,
    required bool enabled,
  });
}

class GraphQlModulesRepository implements ModulesRepository {
  const GraphQlModulesRepository(this._client);

  final GraphQLClient _client;

  @override
  Future<List<ModuleSummary>> listModules() async {
    final result = await _client.query(
      QueryOptions(
        document: gql(moduleRegistryQuery),
        fetchPolicy: FetchPolicy.cacheAndNetwork,
      ),
    );

    if (result.hasException) {
      throw result.exception!;
    }

    final payload = result.data?['moduleRegistry'];
    if (payload is! List) {
      return const <ModuleSummary>[];
    }

    return List.unmodifiable(
      payload.whereType<Map<String, dynamic>>().map(ModuleSummary.fromJson),
    );
  }

  @override
  Future<ModuleToggleResult> toggleModule({
    required String moduleSlug,
    required bool enabled,
  }) async {
    final result = await _client.mutate(
      MutationOptions(
        document: gql(toggleModuleMutation),
        variables: <String, dynamic>{
          'moduleSlug': moduleSlug,
          'enabled': enabled,
        },
      ),
    );

    if (result.hasException) {
      throw result.exception!;
    }

    final payload = result.data?['toggleModule'];
    if (payload is! Map<String, dynamic>) {
      throw const FormatException('toggleModule response payload is missing.');
    }

    return ModuleToggleResult.fromJson(payload);
  }
}

class ModuleToggleResult {
  const ModuleToggleResult({
    required this.moduleSlug,
    required this.enabled,
    required this.settings,
  });

  final String moduleSlug;
  final bool enabled;
  final String settings;

  factory ModuleToggleResult.fromJson(Map<String, dynamic> json) {
    return ModuleToggleResult(
      moduleSlug: _readToggleString(json, 'moduleSlug'),
      enabled: json['enabled'] == true,
      settings: _readToggleString(json, 'settings'),
    );
  }
}

String _readToggleString(Map<String, dynamic> json, String key) {
  final value = json[key];
  return value is String ? value : '';
}
