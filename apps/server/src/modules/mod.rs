use rustok_blog::BlogModule;
use rustok_commerce::CommerceModule;
use rustok_content::ContentModule;
use rustok_core::ModuleRegistry;
use rustok_forum::ForumModule;
use rustok_pages::PagesModule;

pub fn build_registry() -> ModuleRegistry {
    ModuleRegistry::new()
        .register(ContentModule)
        .register(CommerceModule)
        .register(BlogModule)
        .register(ForumModule)
        .register(PagesModule)
}
