use axum::Router;
use leptos::{component, view, IntoView};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
struct LocaleStrings {
    hero_title: &'static str,
    hero_subtitle: &'static str,
    cta_primary: &'static str,
    cta_secondary: &'static str,
    feature_quality_title: &'static str,
    feature_quality_body: &'static str,
    feature_speed_title: &'static str,
    feature_speed_body: &'static str,
    feature_support_title: &'static str,
    feature_support_body: &'static str,
    section_products_title: &'static str,
    section_products_body: &'static str,
    section_story_title: &'static str,
    section_story_body: &'static str,
    footer_tagline: &'static str,
    nav_home: &'static str,
    nav_catalog: &'static str,
    nav_about: &'static str,
    nav_contact: &'static str,
}

fn locale_strings(locale: &str) -> LocaleStrings {
    match locale {
        "ru" => LocaleStrings {
            hero_title: "Откройте витрину RusToK",
            hero_subtitle: "Коммерция на Rust с SSR, быстрым поиском и гибкой архитектурой.",
            cta_primary: "Перейти в каталог",
            cta_secondary: "Узнать больше",
            feature_quality_title: "Премиальный опыт",
            feature_quality_body: "Минималистичный дизайн и быстрые ответы без компромиссов.",
            feature_speed_title: "Мгновенная загрузка",
            feature_speed_body: "SSR по умолчанию ускоряет первую отрисовку.",
            feature_support_title: "Готово к масштабированию",
            feature_support_body: "Модули RusToK легко расширяются под бизнес.",
            section_products_title: "Товары, готовые к запуску",
            section_products_body: "Подключите ваши каталоги, цены и промо-акции из ядра.",
            section_story_title: "История бренда",
            section_story_body: "Расскажите о миссии, ценностях и вашем уникальном продукте.",
            footer_tagline: "RusToK — модульная платформа для современной коммерции.",
            nav_home: "Главная",
            nav_catalog: "Каталог",
            nav_about: "О нас",
            nav_contact: "Контакты",
        },
        _ => LocaleStrings {
            hero_title: "Discover the RusToK Storefront",
            hero_subtitle: "Commerce on Rust with SSR, blazing search, and modular growth.",
            cta_primary: "Explore catalog",
            cta_secondary: "Learn more",
            feature_quality_title: "Premium experience",
            feature_quality_body: "Minimal design and fast responses without compromises.",
            feature_speed_title: "Instantly fast",
            feature_speed_body: "SSR-first delivery accelerates your first paint.",
            feature_support_title: "Ready to scale",
            feature_support_body: "RusToK modules grow with your business.",
            section_products_title: "Launch-ready products",
            section_products_body: "Connect catalogs, pricing, and promotions from the core.",
            section_story_title: "Brand story",
            section_story_body: "Share the mission, values, and craft behind your storefront.",
            footer_tagline: "RusToK — the modular platform for modern commerce.",
            nav_home: "Home",
            nav_catalog: "Catalog",
            nav_about: "About",
            nav_contact: "Contact",
        },
    }
}

#[component]
fn StorefrontShell(strings: LocaleStrings) -> impl IntoView {
    view! {
        <div class="min-h-screen bg-base-200 text-base-content">
            <header class="navbar bg-base-100 px-6">
                <div class="flex-1">
                    <span class="text-xl font-bold">"RusToK"</span>
                </div>
                <nav class="menu menu-horizontal gap-4">
                    <a class="link link-hover" href="#home">{strings.nav_home}</a>
                    <a class="link link-hover" href="#catalog">{strings.nav_catalog}</a>
                    <a class="link link-hover" href="#about">{strings.nav_about}</a>
                    <a class="link link-hover" href="#contact">{strings.nav_contact}</a>
                </nav>
            </header>
            <main class="space-y-24">
                <section id="home" class="hero bg-base-100">
                    <div class="hero-content flex-col gap-10 p-10 lg:flex-row">
                        <div class="max-w-xl space-y-6">
                            <h1 class="text-4xl font-bold lg:text-5xl">{strings.hero_title}</h1>
                            <p class="text-lg opacity-80">{strings.hero_subtitle}</p>
                            <div class="flex flex-wrap gap-3">
                                <a class="btn btn-primary" href="#catalog">{strings.cta_primary}</a>
                                <a class="btn btn-ghost" href="#about">{strings.cta_secondary}</a>
                            </div>
                        </div>
                        <div class="card bg-base-200 shadow-xl">
                            <div class="card-body space-y-4">
                                <span class="badge badge-primary">"SSR"</span>
                                <h2 class="card-title">{strings.section_products_title}</h2>
                                <p>{strings.section_products_body}</p>
                                <button class="btn btn-outline btn-primary btn-sm">{strings.cta_primary}</button>
                            </div>
                        </div>
                    </div>
                </section>
                <section id="catalog" class="px-10">
                    <div class="grid gap-6 lg:grid-cols-3">
                        <div class="card bg-base-100 shadow">
                            <div class="card-body">
                                <h3 class="card-title">{strings.feature_quality_title}</h3>
                                <p>{strings.feature_quality_body}</p>
                                <div class="badge badge-outline">"DaisyUI"</div>
                            </div>
                        </div>
                        <div class="card bg-base-100 shadow">
                            <div class="card-body">
                                <h3 class="card-title">{strings.feature_speed_title}</h3>
                                <p>{strings.feature_speed_body}</p>
                                <div class="badge badge-outline">"Tailwind"</div>
                            </div>
                        </div>
                        <div class="card bg-base-100 shadow">
                            <div class="card-body">
                                <h3 class="card-title">{strings.feature_support_title}</h3>
                                <p>{strings.feature_support_body}</p>
                                <div class="badge badge-outline">"Leptos"</div>
                            </div>
                        </div>
                    </div>
                </section>
                <section id="about" class="px-10">
                    <div class="grid gap-6 lg:grid-cols-2">
                        <div class="card bg-base-100 shadow">
                            <div class="card-body">
                                <h3 class="card-title">{strings.section_story_title}</h3>
                                <p>{strings.section_story_body}</p>
                                <button class="btn btn-secondary btn-sm">{strings.cta_secondary}</button>
                            </div>
                        </div>
                        <div class="card bg-base-100 shadow">
                            <div class="card-body">
                                <h3 class="card-title">{strings.nav_contact}</h3>
                                <p>"storefront@rustok.local"</p>
                                <div class="flex gap-2">
                                    <span class="badge badge-outline">"RSS"</span>
                                    <span class="badge badge-outline">"GraphQL"</span>
                                    <span class="badge badge-outline">"API"</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </section>
            </main>
            <footer id="contact" class="footer footer-center bg-base-300 p-8 text-base-content">
                <div class="space-y-2">
                    <p class="text-sm">{strings.footer_tagline}</p>
                    <div class="flex gap-4">
                        <span class="badge badge-primary">"SSR"</span>
                        <span class="badge badge-secondary">"i18n"</span>
                        <span class="badge badge-outline">"Storefront"</span>
                    </div>
                </div>
            </footer>
        </div>
    }
}

#[component]
fn App(locale: String) -> impl IntoView {
    let strings = locale_strings(locale.as_str());
    view! { <StorefrontShell strings=strings /> }
}

fn render_shell(locale: &str) -> String {
    let locale_owned = locale.to_string();
    let app_html = leptos::ssr::render_to_string(move || {
        let locale = locale_owned.clone();
        view! { <App locale=locale /> }
    });
    format!(
        r#"<!DOCTYPE html>
<html lang="{locale}">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>RusToK Storefront</title>
  <script src="https://cdn.tailwindcss.com"></script>
  <script>
    tailwind.config = {{
      theme: {{
        extend: {{}},
      }},
      plugins: [function({{ addVariant }}) {{ addVariant('daisy', '&'); }}],
    }};
  </script>
  <link href="https://cdn.jsdelivr.net/npm/daisyui@4.12.10/dist/full.min.css" rel="stylesheet" type="text/css" />
</head>
<body class="bg-base-200">
  <div id="app">{app_html}</div>
</body>
</html>"#,
        locale = locale,
        app_html = app_html
    )
}

#[tokio::main]
async fn main() {
    let app = Router::new().route(
        "/",
        axum::routing::get(|axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>| async move {
            let locale = params
                .get("lang")
                .map(|value| value.to_lowercase())
                .unwrap_or_else(|| "en".to_string());
            render_shell(locale.as_str())
        }),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3100")
        .await
        .expect("bind storefront");
    axum::serve(listener, app).await.expect("serve storefront");
}
