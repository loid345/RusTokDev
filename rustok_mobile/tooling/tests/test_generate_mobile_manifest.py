import pathlib
import tempfile
import textwrap
import unittest

from rustok_mobile.tooling.scripts.generate_mobile_manifest import (
    render,
    render_snapshot_json,
    scan_modules,
)


def write_module_manifest(root: pathlib.Path, crate: str, manifest: str) -> None:
    manifest_path = root / f"crates/{crate}/rustok-module.toml"
    manifest_path.parent.mkdir(parents=True, exist_ok=True)
    manifest_path.write_text(textwrap.dedent(manifest).strip(), encoding="utf-8")


class GenerateMobileManifestTests(unittest.TestCase):
    def test_scan_modules_filters_and_sorts(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            write_module_manifest(
                root,
                "mod-a",
                """
                [module]
                slug = "auth"
                name = "Auth"

                [provides.admin_ui]
                route_segment = "Auth"
                nav_label = "Auth"

                [[provides.admin_ui.child_pages]]
                subpath = "Users"
                title = "Users"
                """,
            )
            write_module_manifest(
                root,
                "mod-b",
                """
                [module]
                slug = "blog"
                name = "Blog"

                [provides.admin_ui]
                route_segment = "blog"
                nav_label = "Blog"
                """,
            )
            write_module_manifest(
                root,
                "mod-c",
                """
                [module]
                slug = "service-only"
                """,
            )

            modules = scan_modules(root)
            self.assertEqual([m["route_segment"] for m in modules], ["auth", "blog"])
            self.assertEqual(modules[0]["icon"], "shield")
            self.assertEqual(modules[0]["child_pages"][0]["subpath"], "users")

    def test_render_escapes_quotes(self):
        content = render(
            [
                {
                    "module_key": "rustok_test",
                    "route_segment": "test",
                    "nav_label": "Owner's",
                    "icon": "module",
                }
            ]
        )
        self.assertIn("Owner\\'s", content)

    def test_render_includes_child_pages_and_contract_metadata(self):
        content = render(
            [
                {
                    "module_key": "rustok_test",
                    "route_segment": "test",
                    "nav_label": "Test",
                    "icon": "module",
                    "locale_namespace": "test_module",
                    "permissions": ["test.read"],
                    "child_pages": [
                        {"subpath": "items", "title": "Items", "nav_label": "All items"}
                    ],
                }
            ]
        )
        self.assertIn("localeNamespace: 'test_module'", content)
        self.assertIn("permissions: <String>[", content)
        self.assertIn("'test.read'", content)
        self.assertIn("childPages: <MobileChildPage>[", content)
        self.assertIn("subpath: 'items'", content)
        self.assertIn("navLabel: 'All items'", content)

    def test_render_snapshot_contains_required_ffa_keys(self):
        payload = render_snapshot_json(
            [
                {
                    "module_key": "rustok_blog",
                    "route_segment": "blog",
                    "nav_label": "Blog",
                    "icon": "article",
                    "child_pages": [{"subpath": "posts", "title": "Posts"}],
                }
            ]
        )
        self.assertIn('"module_slug": "blog"', payload)
        self.assertIn('"surface_kind": "admin_mobile"', payload)
        self.assertIn('"route_segment": "blog"', payload)
        self.assertIn('"nav_icon": "article"', payload)
        self.assertIn('"child_pages"', payload)
        self.assertNotIn('"builder_surface"', payload)

    def test_scan_modules_includes_builder_surface_metadata(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            write_module_manifest(
                root,
                "mod-a",
                """
                [module]
                slug = "pages"

                [dependencies.page_builder]
                module = "page-builder"
                contract = "grapesjs_v1"
                contract_version = "1.0"
                required_capabilities = ["preview", "tree", "preview"]

                [provides.admin_ui]
                route_segment = "pages"

                [fba.builder_consumer]
                builder_contract_version = "1.0"

                [fba.builder_consumer.degraded_modes]
                builder_disabled = "readonly"

                [fba.builder_consumer.toggle_profiles]
                builder_off = ["builder.enabled=false", "builder.enabled=false"]
                """,
            )

            modules = scan_modules(root)
            builder_surface = modules[0]["builder_surface"]
            self.assertEqual(builder_surface["provider_module"], "page-builder")
            self.assertEqual(builder_surface["contract"], "grapesjs_v1")
            self.assertEqual(builder_surface["capabilities"], ["preview", "tree"])
            self.assertEqual(
                builder_surface["degraded_modes"], {"builder_disabled": "readonly"}
            )
            self.assertEqual(
                builder_surface["toggle_profiles"],
                {"builder_off": ["builder.enabled=false"]},
            )

    def test_scan_modules_omits_incomplete_builder_surface_metadata(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            write_module_manifest(
                root,
                "mod-a",
                """
                [module]
                slug = "forum"

                [dependencies]
                page_builder = { version_req = ">=0.1.0" }

                [provides.admin_ui]
                route_segment = "forum"

                [fba.builder_consumer]
                builder_contract_version = "1.0"
                """,
            )

            modules = scan_modules(root)
            self.assertIsNone(modules[0]["builder_surface"])

    def test_render_includes_builder_surface_metadata(self):
        content = render(
            [
                {
                    "module_key": "rustok_pages",
                    "route_segment": "pages",
                    "nav_label": "Pages",
                    "icon": "module",
                    "builder_surface": {
                        "provider_module": "page-builder",
                        "contract": "grapesjs_v1",
                        "contract_version": "1.0",
                        "builder_contract_version": "1.0",
                        "capabilities": ["preview"],
                        "degraded_modes": {"builder_disabled": "readonly"},
                        "toggle_profiles": {"builder_off": ["builder.enabled=false"]},
                    },
                }
            ]
        )
        self.assertIn("builderSurface: MobileBuilderSurfaceMeta(", content)
        self.assertIn("providerModule: 'page-builder'", content)
        self.assertIn("capabilities: <String>[", content)
        self.assertIn("'builder_off': <String>[", content)

    def test_render_snapshot_includes_builder_surface_metadata(self):
        payload = render_snapshot_json(
            [
                {
                    "module_key": "rustok_pages",
                    "module_slug": "pages",
                    "route_segment": "pages",
                    "nav_label": "Pages",
                    "icon": "module",
                    "child_pages": [],
                    "builder_surface": {
                        "provider_module": "page-builder",
                        "contract": "grapesjs_v1",
                        "contract_version": "1.0",
                        "builder_contract_version": "1.0",
                        "capabilities": ["preview"],
                        "degraded_modes": {"builder_disabled": "readonly"},
                        "toggle_profiles": {"builder_off": ["builder.enabled=false"]},
                    },
                }
            ]
        )
        self.assertIn('"builder_surface": {', payload)
        self.assertIn('"provider_module": "page-builder"', payload)
        self.assertIn('"builder_contract_version": "1.0"', payload)

    def test_scan_modules_raises_on_duplicate_route_segment(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            write_module_manifest(
                root,
                "mod-a",
                """
                [module]
                slug = "blog"

                [provides.admin_ui]
                route_segment = "content"
                """,
            )
            write_module_manifest(
                root,
                "mod-b",
                """
                [module]
                slug = "forum"

                [provides.admin_ui]
                route_segment = "content"
                """,
            )

            with self.assertRaises(ValueError) as ctx:
                scan_modules(root)
            self.assertIn("content", str(ctx.exception))
            self.assertIn("already declared in", str(ctx.exception))

    def test_scan_modules_includes_permissions_and_locale_namespace(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            write_module_manifest(
                root,
                "mod-a",
                """
                [module]
                slug = "blog"

                [provides.admin_ui]
                route_segment = "blog"
                locale_namespace = "content_blog"
                permissions = ["modules.read", "modules.read", " "]
                """,
            )

            modules = scan_modules(root)
            self.assertEqual(modules[0]["locale_namespace"], "content_blog")
            self.assertEqual(modules[0]["permissions"], ["modules.read"])

    def test_render_snapshot_uses_module_locale_namespace_fallback(self):
        payload = render_snapshot_json(
            [
                {
                    "module_key": "rustok_blog",
                    "module_slug": "blog",
                    "route_segment": "content",
                    "nav_label": "Blog",
                    "icon": "article",
                    "permissions": ["blog.read"],
                    "child_pages": [],
                }
            ]
        )
        self.assertIn('"locale_namespace": "blog"', payload)
        self.assertIn('"permissions": [', payload)
        self.assertIn('"blog.read"', payload)

    def test_scan_modules_normalizes_locale_namespace_and_sorts_permissions(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            write_module_manifest(
                root,
                "mod-a",
                """
                [module]
                slug = "blog"

                [provides.admin_ui]
                route_segment = "blog"
                locale_namespace = "Content Blog"
                permissions = ["z.read", "a.read", "z.read"]
                """,
            )

            modules = scan_modules(root)
            self.assertEqual(modules[0]["locale_namespace"], "content_blog")
            self.assertEqual(modules[0]["permissions"], ["a.read", "z.read"])

    def test_scan_modules_normalizes_permissions_to_lowercase(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            write_module_manifest(
                root,
                "mod-a",
                """
                [module]
                slug = "blog"

                [provides.admin_ui]
                route_segment = "blog"
                permissions = ["Blog.Read", "BLOG.READ", "blog.write"]
                """,
            )

            modules = scan_modules(root)
            self.assertEqual(modules[0]["permissions"], ["blog.read", "blog.write"])

    def test_scan_modules_drops_permissions_with_invalid_format(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            write_module_manifest(
                root,
                "mod-a",
                """
                [module]
                slug = "blog"

                [provides.admin_ui]
                route_segment = "blog"
                permissions = ["blog.read", "blog/read", "bad space", "ok:perm"]
                """,
            )

            modules = scan_modules(root)
            self.assertEqual(modules[0]["permissions"], ["blog.read", "ok:perm"])

    def test_scan_modules_falls_back_locale_namespace_when_normalized_empty(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = pathlib.Path(tmp)
            write_module_manifest(
                root,
                "mod-a",
                """
                [module]
                slug = "blog"

                [provides.admin_ui]
                route_segment = "blog"
                locale_namespace = "!!!"
                """,
            )

            modules = scan_modules(root)
            self.assertEqual(modules[0]["locale_namespace"], "blog")


if __name__ == "__main__":
    unittest.main()
