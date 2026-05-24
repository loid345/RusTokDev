import 'package:app_module_contracts/app_module_contracts.dart';

const modulesRootPath = '/modules';

class ModuleRouteEntry {
  const ModuleRouteEntry({
    required this.moduleKey,
    required this.routeSegment,
    required this.path,
    required this.navTitle,
    required this.childRoutes,
  });

  final String moduleKey;
  final String routeSegment;
  final String path;
  final String navTitle;
  final List<ModuleChildRouteEntry> childRoutes;
}

class ModuleChildRouteEntry {
  const ModuleChildRouteEntry({
    required this.subpath,
    required this.path,
    required this.title,
    required this.navLabel,
  });

  final String subpath;
  final String path;
  final String title;
  final String navLabel;
}

List<ModuleRouteEntry> adaptModuleEntries(List<MobileModuleEntry> entries) {
  final adapted = <ModuleRouteEntry>[];

  for (final entry in entries) {
    final routeSegment = _sanitizeSegment(entry.routeSegment);
    if (entry.moduleKey.trim().isEmpty || routeSegment.isEmpty) {
      continue;
    }

    final basePath = '$modulesRootPath/$routeSegment';
    final childRoutes = <ModuleChildRouteEntry>[];

    for (final child in entry.childPages) {
      final subpath = _sanitizeSegment(child.subpath);
      if (subpath.isEmpty) {
        continue;
      }

      childRoutes.add(
        ModuleChildRouteEntry(
          subpath: subpath,
          path: '$basePath/$subpath',
          title: child.title,
          navLabel: child.navLabel ?? child.title,
        ),
      );
    }

    adapted.add(
      ModuleRouteEntry(
        moduleKey: entry.moduleKey,
        routeSegment: routeSegment,
        path: basePath,
        navTitle: entry.nav.title,
        childRoutes: List.unmodifiable(childRoutes),
      ),
    );
  }

  return List.unmodifiable(adapted);
}

String _sanitizeSegment(String value) {
  return value.trim().replaceAll(RegExp(r'^/+|/+$'), '');
}
