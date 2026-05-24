import 'package:app_module_contracts/app_module_contracts.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:rustok_admin_mobile/registry/module_entry_adapter.dart';

void main() {
  test('adapts module entries and child routes into canonical module paths', () {
    final entries = <MobileModuleEntry>[
      const MobileModuleEntry(
        moduleKey: 'rustok_blog',
        routeSegment: '/blog/',
        nav: MobileNavMeta(title: 'Blog', icon: 'article'),
        childPages: [
          MobileChildPage(subpath: '/new/', title: 'Add Blog Post'),
          MobileChildPage(
            subpath: 'posts',
            title: 'All Blog Posts',
            navLabel: 'All Posts',
          ),
        ],
      ),
    ];

    final adapted = adaptModuleEntries(entries);

    expect(adapted, hasLength(1));
    final blog = adapted.first;
    expect(blog.routeSegment, 'blog');
    expect(blog.path, '/modules/blog');
    expect(blog.navTitle, 'Blog');
    expect(blog.childRoutes, hasLength(2));
    expect(blog.childRoutes.first.subpath, 'new');
    expect(blog.childRoutes.first.path, '/modules/blog/new');
    expect(blog.childRoutes.first.navLabel, 'Add Blog Post');
    expect(blog.childRoutes.last.navLabel, 'All Posts');
  });

  test('skips invalid module and child segments after sanitization', () {
    final entries = <MobileModuleEntry>[
      const MobileModuleEntry(
        moduleKey: ' ',
        routeSegment: '/blog/',
        nav: MobileNavMeta(title: 'Blog', icon: 'article'),
      ),
      const MobileModuleEntry(
        moduleKey: 'rustok_pages',
        routeSegment: '///',
        nav: MobileNavMeta(title: 'Pages', icon: 'module'),
      ),
      const MobileModuleEntry(
        moduleKey: 'rustok_search',
        routeSegment: '/search/',
        nav: MobileNavMeta(title: 'Search', icon: 'search'),
        childPages: [
          MobileChildPage(subpath: '///', title: 'Invalid child'),
          MobileChildPage(subpath: '/analytics/', title: 'Analytics'),
        ],
      ),
    ];

    final adapted = adaptModuleEntries(entries);

    expect(adapted, hasLength(1));
    final search = adapted.single;
    expect(search.moduleKey, 'rustok_search');
    expect(search.path, '/modules/search');
    expect(search.childRoutes, hasLength(1));
    expect(search.childRoutes.single.path, '/modules/search/analytics');
  });
}
