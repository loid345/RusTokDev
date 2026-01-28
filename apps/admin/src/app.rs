use leptos::*;
use leptos_router::{Route, Router, Routes};

use crate::pages::{dashboard::Dashboard, login::Login};
use crate::providers::auth::{provide_auth_context, use_auth};

#[component]
pub fn App() -> impl IntoView {
    provide_auth_context();

    view! {
        <Router>
            <main class="app-shell">
                <Style />
                <Routes>
                    <Route path="/login" view=Login />
                    <Route path="/dashboard" view=ProtectedDashboard />
                    <Route path="" view=ProtectedDashboard />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn ProtectedDashboard() -> impl IntoView {
    let auth = use_auth();

    view! {
        <Show
            when=move || auth.token.get().is_some()
            fallback=|| view! { <Login /> }
        >
            <Dashboard />
        </Show>
    }
}

#[component]
fn Style() -> impl IntoView {
    view! {
        <style>
            ":root {\n"
            "  color-scheme: light;\n"
            "  font-family: 'Inter', system-ui, -apple-system, sans-serif;\n"
            "  background-color: #f5f6fb;\n"
            "}\n"
            "* { box-sizing: border-box; }\n"
            "body { margin: 0; }\n"
            ".app-shell { min-height: 100vh; color: #0f172a; }\n"
            ".auth-grid { display: grid; grid-template-columns: 1.2fr 1fr; min-height: 100vh; }\n"
            ".auth-visual { padding: 72px; background: radial-gradient(circle at top left, #1e3a8a, #0f172a); color: #fff; display: flex; flex-direction: column; justify-content: center; gap: 24px; }\n"
            ".auth-visual h1 { font-size: 2.5rem; margin: 0; }\n"
            ".auth-visual p { margin: 0; font-size: 1.05rem; opacity: 0.85; }\n"
            ".auth-form { padding: 72px 80px; display: flex; flex-direction: column; justify-content: center; gap: 28px; background: #f8fafc; }\n"
            ".auth-card { background: #fff; border-radius: 24px; padding: 32px; box-shadow: 0 24px 60px rgba(15, 23, 42, 0.12); display: flex; flex-direction: column; gap: 20px; }\n"
            ".auth-card h2 { margin: 0; font-size: 1.75rem; }\n"
            ".auth-card p { margin: 0; color: #64748b; }\n"
            ".input-group { display: flex; flex-direction: column; gap: 8px; }\n"
            ".input-group label { font-size: 0.9rem; color: #475569; }\n"
            ".input-group input { padding: 12px 16px; border-radius: 12px; border: 1px solid #e2e8f0; font-size: 0.95rem; }\n"
            ".primary-button { background: #2563eb; color: #fff; border: none; padding: 12px 18px; border-radius: 12px; font-weight: 600; cursor: pointer; }\n"
            ".primary-button.w-full { width: 100%; }\n"
            ".ghost-button { background: transparent; color: #2563eb; border: 1px solid #cbd5f5; }\n"
            ".secondary-link { color: #2563eb; text-decoration: none; font-size: 0.9rem; }\n"
            ".badge { background: #e2e8f0; padding: 6px 12px; border-radius: 999px; font-size: 0.85rem; color: #475569; }\n"
            ".alert { background: #fee2e2; color: #b91c1c; padding: 10px 14px; border-radius: 12px; font-size: 0.9rem; }\n"
            ".dashboard { padding: 32px 40px 56px; }\n"
            ".dashboard-header { display: flex; justify-content: space-between; align-items: center; margin-bottom: 32px; gap: 16px; flex-wrap: wrap; }\n"
            ".dashboard-header h1 { margin: 8px 0 0; font-size: 2rem; }\n"
            ".dashboard-actions { display: flex; gap: 12px; align-items: center; flex-wrap: wrap; }\n"
            ".stats-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr)); gap: 20px; margin-bottom: 32px; }\n"
            ".stat-card { background: #fff; padding: 20px; border-radius: 18px; box-shadow: 0 16px 30px rgba(15, 23, 42, 0.08); }\n"
            ".stat-card h3 { margin: 0 0 8px; font-size: 0.95rem; color: #64748b; }\n"
            ".stat-card strong { font-size: 1.5rem; }\n"
            ".dashboard-panels { display: grid; grid-template-columns: 1.4fr 1fr; gap: 24px; }\n"
            ".panel { background: #fff; border-radius: 20px; padding: 24px; box-shadow: 0 18px 36px rgba(15, 23, 42, 0.08); }\n"
            ".panel h4 { margin: 0 0 16px; }\n"
            ".activity-item { display: flex; justify-content: space-between; align-items: center; padding: 12px 0; border-bottom: 1px solid #e2e8f0; }\n"
            ".activity-item:last-child { border-bottom: none; }\n"
            ".quick-actions { display: grid; gap: 12px; }\n"
            ".quick-actions button { background: #f1f5f9; border: none; padding: 12px 16px; border-radius: 12px; text-align: left; font-weight: 600; }\n"
            "@media (max-width: 960px) {\n"
            "  .auth-grid { grid-template-columns: 1fr; }\n"
            "  .auth-form { padding: 48px 32px; }\n"
            "  .auth-visual { padding: 48px 32px; }\n"
            "  .dashboard-panels { grid-template-columns: 1fr; }\n"
            "}\n"
        </style>
    }
}
