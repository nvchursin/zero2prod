#![recursion_limit = "512"]

use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "page", rename_all = "snake_case")]
pub enum PageData {
    Home,
    Login {
        messages: Vec<String>,
    },
    Dashboard {
        username: String,
    },
    Password {
        messages: Vec<String>,
    },
    Newsletters {
        messages: Vec<String>,
        idempotency_key: String,
    },
    SubscriptionPending,
    Confirmation,
}

#[component]
pub fn App(page: PageData) -> impl IntoView {
    match page {
        PageData::Home => view! { <HomePage /> }.into_any(),
        PageData::Login { messages } => view! { <LoginPage messages=messages /> }.into_any(),
        PageData::Dashboard { username } => {
            view! { <DashboardPage username=username /> }.into_any()
        }
        PageData::Password { messages } => view! { <PasswordPage messages=messages /> }.into_any(),
        PageData::Newsletters {
            messages,
            idempotency_key,
        } => view! {
            <NewsletterPage messages=messages idempotency_key=idempotency_key />
        }
        .into_any(),
        PageData::SubscriptionPending => view! { <SubscriptionPendingPage /> }.into_any(),
        PageData::Confirmation => view! { <ConfirmationPage /> }.into_any(),
    }
}

#[component]
fn Brand() -> impl IntoView {
    view! {
        <a class="brand" href="/" aria-label="The Dispatch home">
            <span class="brand-mark" aria-hidden="true">"D"</span>
            <span>"The Dispatch"</span>
        </a>
    }
}

#[component]
fn PublicHeader() -> impl IntoView {
    view! {
        <header class="site-header">
            <div class="container header-inner">
                <Brand />
                <a class="text-link" href="/login">"Editor login"</a>
            </div>
        </header>
    }
}

#[component]
fn Alerts(messages: Vec<String>) -> impl IntoView {
    view! {
        <div class="alerts" aria-live="polite">
            {messages
                .into_iter()
                .map(|message| {
                    let is_success = message.contains("successfully")
                        || message.contains("accepted")
                        || message.contains("changed");
                    let class = if is_success {
                        "alert alert-success"
                    } else {
                        "alert alert-error"
                    };
                    view! { <p class=class>{message}</p> }
                })
                .collect_view()}
        </div>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <PublicHeader />
        <main>
            <section class="hero">
                <div class="container hero-grid">
                    <div class="hero-copy">
                        <p class="eyebrow">"A thoughtful letter for curious people"</p>
                        <h1>"Ideas worth opening your inbox for."</h1>
                        <p class="lede">
                            "A concise, considered newsletter about building useful things, making better decisions, and noticing what matters."
                        </p>
                        <form class="subscribe-form" action="/subscriptions" method="post">
                            <div class="field">
                                <label for="subscriber-name">"Your name"</label>
                                <input
                                    id="subscriber-name"
                                    name="name"
                                    type="text"
                                    autocomplete="name"
                                    placeholder="Ada Lovelace"
                                    required
                                />
                            </div>
                            <div class="field">
                                <label for="subscriber-email">"Email address"</label>
                                <input
                                    id="subscriber-email"
                                    name="email"
                                    type="email"
                                    autocomplete="email"
                                    placeholder="ada@example.com"
                                    required
                                />
                            </div>
                            <button class="button button-primary" type="submit">
                                "Join the readers"
                                <span aria-hidden="true">"→"</span>
                            </button>
                            <p class="form-note">"One useful letter at a time. No noise, unsubscribe whenever you like."</p>
                        </form>
                    </div>
                    <aside class="issue-preview" aria-label="Latest issue preview">
                        <div class="preview-meta">
                            <span>"Issue 024"</span>
                            <span>"6 min read"</span>
                        </div>
                        <p class="preview-kicker">"From the archive"</p>
                        <h2>"The quiet advantage of doing less, better"</h2>
                        <p>
                            "A practical note on subtraction, attention, and why the clearest products often begin with a smaller promise."
                        </p>
                        <div class="preview-rule"></div>
                        <blockquote>
                            "Clarity is not the absence of ambition. It is ambition with an editor."
                        </blockquote>
                    </aside>
                </div>
            </section>
            <section class="principles">
                <div class="container">
                    <p class="eyebrow">"What to expect"</p>
                    <div class="principle-grid">
                        <article>
                            <span>"01"</span>
                            <h3>"Useful by design"</h3>
                            <p>"Every issue leaves you with one idea you can put to work."</p>
                        </article>
                        <article>
                            <span>"02"</span>
                            <h3>"Brief, not shallow"</h3>
                            <p>"Carefully edited writing that respects your time and attention."</p>
                        </article>
                        <article>
                            <span>"03"</span>
                            <h3>"Human, always"</h3>
                            <p>"No content mill, no daily churn—just a clear note from one person to another."</p>
                        </article>
                    </div>
                </div>
            </section>
        </main>
        <footer class="site-footer">
            <div class="container footer-inner">
                <Brand />
                <p>"Independent notes on thoughtful work."</p>
            </div>
        </footer>
    }
}

#[component]
fn AuthShell(children: Children) -> impl IntoView {
    view! {
        <main class="auth-shell">
            <div class="auth-brand"><Brand /></div>
            <section class="auth-card">{children()}</section>
            <a class="back-link" href="/">"← Back to the publication"</a>
        </main>
    }
}

#[component]
fn LoginPage(messages: Vec<String>) -> impl IntoView {
    view! {
        <AuthShell>
            <p class="eyebrow">"Private editorial desk"</p>
            <h1>"Welcome back."</h1>
            <p class="card-intro">"Sign in to write and send the next issue."</p>
            <Alerts messages=messages />
            <form class="stacked-form" action="/login" method="post">
                <div class="field">
                    <label for="username">"Username"</label>
                    <input
                        id="username"
                        name="username"
                        type="text"
                        autocomplete="username"
                        placeholder="Your username"
                        required
                        autofocus
                    />
                </div>
                <div class="field">
                    <label for="password">"Password"</label>
                    <input
                        id="password"
                        name="password"
                        type="password"
                        autocomplete="current-password"
                        placeholder="Your password"
                        required
                    />
                </div>
                <button class="button button-primary button-full" type="submit">"Sign in →"</button>
            </form>
        </AuthShell>
    }
}

#[component]
fn AdminHeader(active: &'static str) -> impl IntoView {
    let nav_class = move |item| {
        if active == item {
            "admin-nav-link is-active"
        } else {
            "admin-nav-link"
        }
    };

    view! {
        <header class="admin-header">
            <div class="container admin-header-inner">
                <Brand />
                <nav class="admin-nav" aria-label="Admin navigation">
                    <a class=nav_class("dashboard") href="/admin/dashboard">"Overview"</a>
                    <a class=nav_class("newsletters") href="/admin/newsletters">"New issue"</a>
                    <a class=nav_class("password") href="/admin/password">"Account"</a>
                </nav>
                <form action="/admin/logout" method="post">
                    <button class="button button-quiet" type="submit">"Sign out"</button>
                </form>
            </div>
        </header>
    }
}

#[component]
fn DashboardPage(username: String) -> impl IntoView {
    view! {
        <AdminHeader active="dashboard" />
        <main class="admin-main">
            <div class="container admin-container">
                <div class="page-heading">
                    <div>
                        <p class="eyebrow">"Editorial overview"</p>
                        <h1>"Welcome "{username}"."</h1>
                    </div>
                    <a class="button button-primary" href="/admin/newsletters">"Write a new issue →"</a>
                </div>
                <section class="dashboard-grid">
                    <a class="action-card action-card-featured" href="/admin/newsletters">
                        <span class="action-number">"01"</span>
                        <div>
                            <p class="eyebrow">"Publish"</p>
                            <h2>"Compose the next newsletter"</h2>
                            <p>"Write both the plain-text and HTML editions, then send them to confirmed readers."</p>
                        </div>
                        <span class="action-arrow" aria-hidden="true">"↗"</span>
                    </a>
                    <a class="action-card" href="/admin/password">
                        <span class="action-number">"02"</span>
                        <div>
                            <p class="eyebrow">"Security"</p>
                            <h2>"Update your password"</h2>
                            <p>"Keep access to the editorial desk secure."</p>
                        </div>
                        <span class="action-arrow" aria-hidden="true">"↗"</span>
                    </a>
                </section>
            </div>
        </main>
    }
}

#[component]
fn PasswordPage(messages: Vec<String>) -> impl IntoView {
    let (new_password, set_new_password) = signal(String::new());
    let (confirmation, set_confirmation) = signal(String::new());
    let passwords_match = move || {
        let password = new_password.get();
        let confirmation = confirmation.get();
        !confirmation.is_empty() && password == confirmation
    };

    view! {
        <AdminHeader active="password" />
        <main class="admin-main">
            <div class="container narrow-container">
                <div class="page-heading compact-heading">
                    <div>
                        <p class="eyebrow">"Account security"</p>
                        <h1>"Change password"</h1>
                        <p class="lede-small">"Choose a strong password you do not use anywhere else."</p>
                    </div>
                </div>
                <section class="form-card">
                    <Alerts messages=messages />
                    <form class="stacked-form" action="/admin/password" method="post">
                        <div class="field">
                            <label for="current-password">"Current password"</label>
                            <input
                                id="current-password"
                                name="current_password"
                                type="password"
                                autocomplete="current-password"
                                required
                            />
                        </div>
                        <div class="field">
                            <label for="new-password">"New password"</label>
                            <input
                                id="new-password"
                                name="new_password"
                                type="password"
                                autocomplete="new-password"
                                minlength="8"
                                required
                                on:input=move |event| set_new_password.set(event_target_value(&event))
                            />
                        </div>
                        <div class="field">
                            <label for="new-password-check">"Confirm new password"</label>
                            <input
                                id="new-password-check"
                                name="new_password_check"
                                type="password"
                                autocomplete="new-password"
                                minlength="8"
                                required
                                on:input=move |event| set_confirmation.set(event_target_value(&event))
                            />
                            <p class=move || {
                                if passwords_match() {
                                    "field-hint hint-success"
                                } else {
                                    "field-hint"
                                }
                            }>
                                {move || {
                                    if confirmation.get().is_empty() {
                                        "Enter the new password once more."
                                    } else if passwords_match() {
                                        "Passwords match."
                                    } else {
                                        "Passwords do not match yet."
                                    }
                                }}
                            </p>
                        </div>
                        <div class="form-actions">
                            <button class="button button-primary" type="submit">"Update password"</button>
                            <a class="button button-quiet" href="/admin/dashboard">"Cancel"</a>
                        </div>
                    </form>
                </section>
            </div>
        </main>
    }
}

#[component]
fn NewsletterPage(messages: Vec<String>, idempotency_key: String) -> impl IntoView {
    let (title, set_title) = signal(String::new());
    let (text_content, set_text_content) = signal(String::new());
    let (html_content, set_html_content) = signal(String::new());

    view! {
        <AdminHeader active="newsletters" />
        <main class="admin-main">
            <div class="container editor-container">
                <div class="page-heading compact-heading">
                    <div>
                        <p class="eyebrow">"New edition"</p>
                        <h1>"Compose an issue"</h1>
                        <p class="lede-small">"Give readers a useful idea in both accessible formats."</p>
                    </div>
                </div>
                <Alerts messages=messages />
                <form class="editor-form" action="/admin/newsletters" method="post">
                    <input type="hidden" name="idempotency_key" value=idempotency_key />
                    <section class="form-card editor-title-card">
                        <div class="field">
                            <div class="label-row">
                                <label for="newsletter-title">"Subject line"</label>
                                <span>{move || title.get().chars().count()}" / 120"</span>
                            </div>
                            <input
                                class="title-input"
                                id="newsletter-title"
                                name="title"
                                type="text"
                                maxlength="120"
                                placeholder="A clear promise for this issue"
                                required
                                on:input=move |event| set_title.set(event_target_value(&event))
                            />
                        </div>
                    </section>
                    <div class="editor-grid">
                        <section class="form-card editor-pane">
                            <div class="pane-heading">
                                <div>
                                    <p class="eyebrow">"Plain text"</p>
                                    <h2>"Accessible edition"</h2>
                                </div>
                                <span>{move || text_content.get().chars().count()}" characters"</span>
                            </div>
                            <textarea
                                id="text-content"
                                name="text_content"
                                rows="16"
                                placeholder="Write the plain-text version here…"
                                required
                                on:input=move |event| set_text_content.set(event_target_value(&event))
                            ></textarea>
                        </section>
                        <section class="form-card editor-pane">
                            <div class="pane-heading">
                                <div>
                                    <p class="eyebrow">"HTML"</p>
                                    <h2>"Rich edition"</h2>
                                </div>
                                <span>{move || html_content.get().chars().count()}" characters"</span>
                            </div>
                            <textarea
                                id="html-content"
                                name="html_content"
                                rows="16"
                                placeholder="<p>Write the HTML version here…</p>"
                                required
                                spellcheck="false"
                                on:input=move |event| set_html_content.set(event_target_value(&event))
                            ></textarea>
                        </section>
                    </div>
                    <div class="editor-actions">
                        <p>"Publishing queues one delivery for every confirmed subscriber."</p>
                        <div class="form-actions">
                            <a class="button button-quiet" href="/admin/dashboard">"Save for later"</a>
                            <button class="button button-primary" type="submit">"Publish issue →"</button>
                        </div>
                    </div>
                </form>
            </div>
        </main>
    }
}

#[component]
fn ConfirmationPage() -> impl IntoView {
    view! {
        <PublicHeader />
        <main class="confirmation-shell">
            <section class="confirmation-card">
                <span class="confirmation-mark" aria-hidden="true">"✓"</span>
                <p class="eyebrow">"Subscription confirmed"</p>
                <h1>"You’re on the list."</h1>
                <p class="card-intro">
                    "Thank you for confirming. The next issue of The Dispatch will arrive in your inbox."
                </p>
                <a class="button button-primary" href="/">"Return home →"</a>
            </section>
        </main>
    }
}

#[component]
fn SubscriptionPendingPage() -> impl IntoView {
    view! {
        <PublicHeader />
        <main class="confirmation-shell">
            <section class="confirmation-card">
                <span class="confirmation-mark confirmation-mark-mail" aria-hidden="true">"✉"</span>
                <p class="eyebrow">"One last step"</p>
                <h1>"Check your inbox."</h1>
                <p class="card-intro">
                    "We sent you a confirmation link. Open it to complete your subscription to The Dispatch."
                </p>
                <a class="button button-primary" href="/">"Return home →"</a>
            </section>
        </main>
    }
}

#[cfg(feature = "ssr")]
pub fn render_document(page: PageData) -> String {
    let page_json = serde_json::to_string(&page)
        .expect("Page data should be serializable")
        .replace('<', "\\u003c")
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029");
    let body = view! { <App page=page /> }.to_html();

    format!(
        r##"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <meta name="theme-color" content="#f5f0e8">
  <meta name="description" content="The Dispatch — thoughtful notes on useful work.">
  <title>The Dispatch</title>
  <link rel="icon" href="/assets/favicon.svg" type="image/svg+xml">
  <link rel="preconnect" href="https://fonts.googleapis.com">
  <link rel="preconnect" href="https://fonts.gstatic.com" crossorigin>
  <link href="https://fonts.googleapis.com/css2?family=DM+Sans:wght@400;500;600&family=Newsreader:opsz,wght@6..72,400;6..72,500;6..72,600&display=swap" rel="stylesheet">
  <link rel="stylesheet" href="/pkg/zero2prod.css">
  <script id="page-data" type="application/json">{page_json}</script>
  <script type="module">
    import init, {{ hydrate }} from '/pkg/zero2prod.js';
    await init({{ module_or_path: '/pkg/zero2prod.wasm' }});
    hydrate();
  </script>
</head>
<body>{body}</body>
</html>"##
    )
}

#[cfg(feature = "hydrate")]
fn page_data_from_document() -> PageData {
    let document = web_sys::window()
        .expect("window should exist")
        .document()
        .expect("document should exist");
    let element = document
        .get_element_by_id("page-data")
        .expect("page-data script should exist");
    serde_json::from_str(&element.text_content().unwrap_or_default())
        .expect("page-data should contain valid JSON")
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    console_error_panic_hook::set_once();
    let page = page_data_from_document();
    leptos::mount::hydrate_body(move || view! { <App page=page.clone() /> });
}

#[cfg(all(test, feature = "ssr"))]
mod tests {
    use super::{PageData, render_document};

    #[test]
    fn renders_a_complete_hydratable_document() {
        let html = render_document(PageData::Home);

        assert!(html.starts_with("<!doctype html>"));
        assert!(html.contains("Ideas worth opening your inbox for."));
        assert!(html.contains("/pkg/zero2prod.js"));
        assert!(html.contains("/pkg/zero2prod.css"));
        assert!(html.contains("/pkg/zero2prod.wasm"));
        assert!(html.contains(r#"action="/subscriptions""#));
        assert!(!html.contains("<!--bo-"));
    }

    #[test]
    fn escapes_page_data_embedded_in_the_script_element() {
        let html = render_document(PageData::Login {
            messages: vec!["</script><script>alert('nope')</script>".to_owned()],
        });

        assert!(!html.contains("</script><script>alert"));
        assert!(html.contains(r#"\u003c/script>"#));
    }
}
