import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:rustok_modules_mobile/rustok_modules_mobile.dart';

void main() {
  testWidgets('renders GraphQL-backed module summaries and route hints', (
    tester,
  ) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          modulesRepositoryProvider.overrideWithValue(
            const _FakeModulesRepository([
              ModuleSummary(
                slug: 'blog',
                name: 'Blog Module',
                description: 'Editorial content',
                version: '1.2.3',
                kind: 'optional',
                enabled: true,
                ownership: 'platform',
                trustLevel: 'trusted',
                recommendedAdminSurfaces: ['posts'],
                showcaseAdminSurfaces: [],
              ),
            ]),
          ),
        ],
        child: MaterialApp(
          home: Scaffold(
            body: ModulesMobileScreen(
              canManageModules: true,
              resolveModulePath: (module) => '/modules/${module.slug}',
            ),
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('Modules pilot'), findsOneWidget);
    expect(find.text('Blog Module'), findsOneWidget);
    expect(find.textContaining('mobile route: /modules/blog'), findsOneWidget);
    expect(find.text('Enabled'), findsOneWidget);
  });


  testWidgets('runs mutation-backed toggle action and refreshes registry', (
    tester,
  ) async {
    final repository = _MutableModulesRepository();

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          modulesRepositoryProvider.overrideWithValue(repository),
        ],
        child: const MaterialApp(
          home: Scaffold(
            body: ModulesMobileScreen(canManageModules: true),
          ),
        ),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('Enabled'), findsOneWidget);

    await tester.tap(find.text('Disable'));
    await tester.pumpAndSettle();

    expect(repository.toggleRequests, const [false]);
    expect(find.text('Disabled'), findsOneWidget);
    expect(find.text('Enable'), findsOneWidget);
  });

  testWidgets('shows retryable error state', (tester) async {
    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          modulesRepositoryProvider.overrideWithValue(const _FailingRepository()),
        ],
        child: const MaterialApp(
          home: Scaffold(body: ModulesMobileScreen()),
        ),
      ),
    );
    await tester.pumpAndSettle();

    expect(find.text('Failed to load module registry.'), findsOneWidget);
    expect(find.text('Retry'), findsOneWidget);
  });
}

class _FakeModulesRepository implements ModulesRepository {
  const _FakeModulesRepository(this.modules);

  final List<ModuleSummary> modules;

  @override
  Future<List<ModuleSummary>> listModules() async => modules;

  @override
  Future<ModuleToggleResult> toggleModule({
    required String moduleSlug,
    required bool enabled,
  }) async {
    return ModuleToggleResult(
      moduleSlug: moduleSlug,
      enabled: enabled,
      settings: '{}',
    );
  }
}

class _MutableModulesRepository implements ModulesRepository {
  final List<bool> toggleRequests = <bool>[];
  bool _enabled = true;

  @override
  Future<List<ModuleSummary>> listModules() async => [
    ModuleSummary(
      slug: 'blog',
      name: 'Blog Module',
      description: 'Editorial content',
      version: '1.2.3',
      kind: 'optional',
      enabled: _enabled,
      ownership: 'platform',
      trustLevel: 'trusted',
      recommendedAdminSurfaces: const ['posts'],
      showcaseAdminSurfaces: const [],
    ),
  ];

  @override
  Future<ModuleToggleResult> toggleModule({
    required String moduleSlug,
    required bool enabled,
  }) async {
    toggleRequests.add(enabled);
    _enabled = enabled;
    return ModuleToggleResult(
      moduleSlug: moduleSlug,
      enabled: enabled,
      settings: '{}',
    );
  }
}

class _FailingRepository implements ModulesRepository {
  const _FailingRepository();

  @override
  Future<List<ModuleSummary>> listModules() async {
    throw StateError('registry unavailable');
  }

  @override
  Future<ModuleToggleResult> toggleModule({
    required String moduleSlug,
    required bool enabled,
  }) async {
    throw StateError('registry unavailable');
  }
}
