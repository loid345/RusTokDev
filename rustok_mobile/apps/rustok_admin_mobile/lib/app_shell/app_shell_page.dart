import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'auth_bootstrap.dart';

class AppShellPage extends ConsumerWidget {
  const AppShellPage({super.key, required this.child});

  final Widget child;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final probe = ref.watch(authBootstrapProbeProvider);
    return Scaffold(
      appBar: AppBar(title: const Text('RusTok Admin Mobile')),
      body: probe.when(
        data: (data) {
          if (!data.isAuthenticated) {
            return const _UnauthenticatedView();
          }
          return Column(
            children: [
              _AuthContextBanner(data: data),
              Expanded(child: child),
            ],
          );
        },
        loading: () => const Center(child: CircularProgressIndicator()),
        error: (error, _) => Center(child: Text('Bootstrap error: $error')),
      ),
    );
  }
}

class _UnauthenticatedView extends StatelessWidget {
  const _UnauthenticatedView();

  @override
  Widget build(BuildContext context) {
    return const Center(
      child: Text(
        'No active session. Provide valid auth session in host bootstrap.',
        textAlign: TextAlign.center,
      ),
    );
  }
}

class _AuthContextBanner extends StatelessWidget {
  const _AuthContextBanner({required this.data});

  final BootstrapProbeResult data;

  @override
  Widget build(BuildContext context) {
    final meEmail = data.me?['email']?.toString() ?? 'unknown';
    final tenantSlug = data.currentTenant?['slug']?.toString() ?? 'unknown';
    return Material(
      color: Theme.of(context).colorScheme.surfaceContainerHighest,
      child: ListTile(
        dense: true,
        title: Text('me: $meEmail'),
        subtitle: Text('tenant: $tenantSlug'),
      ),
    );
  }
}
