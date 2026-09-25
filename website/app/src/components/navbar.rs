use crate::i18n::Locale;
use crate::i18n::use_i18n;
use leptos::prelude::*;
use leptos_i18n::t;
use leptos_router::components::A;

#[component]
pub fn Navbar() -> impl IntoView {
    let i18n = use_i18n();

    let (community_open, set_community_open) = signal(false);
    let (lang_open, set_lang_open) = signal(false);
    let (theme_open, set_theme_open) = signal(false);
    let current_lang = move || {
        let locale = i18n.get_locale();
        match locale {
            Locale::en => i18n.get_keys().lang_en().into_view(),
            Locale::zh_CN => i18n.get_keys().lang_zh().into_view(),
        }
    };
    let (current_theme, set_current_theme) = signal("light".to_string());
    let current_theme_name = move || {
        let theme = current_theme.get();
        match theme.as_str() {
            "light" => i18n.get_keys().theme_light().into_view(),
            "dark" => i18n.get_keys().theme_dark().into_view(),
            "system" => i18n.get_keys().theme_system().into_view(),
            _ => i18n.get_keys().theme_light().into_view(),
        }
    };

    view! {
        <nav class="navbar">
            <div class="navbar-left">
                <div class="logo-wrapper">
                    <A href="/">
                        <img src="/logo.png" alt="OpenPICL Logo" />
                    </A>
                </div>
                <div class="nav-links">
                    <A href="/home">
                        {t!(i18n, home)}
                    </A>
                    <A href="/hardware">
                        {t!(i18n, hardware)}
                    </A>
                    <A href="/models">
                        {t!(i18n, models)}
                    </A>
                    <A href="/datasets">
                        {t!(i18n, datasets)}
                    </A>
                    <A href="/studio">
                        {t!(i18n, studio)}
                    </A>
                    <A href="/spotlight">
                        {t!(i18n, spotlight)}
                    </A>
                    <a href="https://docs.openpicl.com" target="_blank" rel="noopener noreferrer">
                        {t!(i18n, docs)}
                    </a>
                </div>
                <div class="dropdown">
                    <button class="dropdown-trigger" on:click=move |_| {
                        set_community_open.update(|v| *v = !*v);
                        set_lang_open.set(false);
                        set_theme_open.set(false);
                    }>
                        {t!(i18n, community)}
                    </button>
                    <div class="dropdown-menu" class:open=move || community_open.get()>
                        <A href="/collections" on:click=move |_| set_community_open.set(false)>
                            {t!(i18n, collections)}
                        </A>
                        <A href="/papers" on:click=move |_| set_community_open.set(false)>
                            {t!(i18n, papers)}
                        </A>
                        <A href="/gallery" on:click=move |_| set_community_open.set(false)>
                            {t!(i18n, gallery)}
                        </A>
                        <A href="/organizations" on:click=move |_| set_community_open.set(false)>
                            {t!(i18n, organizations)}
                        </A>
                    </div>
                </div>
            </div>

            <div class="navbar-right">
                <div class="dropdown">
                    <button class="dropdown-trigger" on:click=move |_| {
                        set_lang_open.update(|v| *v = !*v);
                        set_community_open.set(false);
                        set_theme_open.set(false);
                    }>
                        {current_lang}
                    </button>
                    <div class="dropdown-menu" class:open=move || lang_open.get()>
                        <button
                            on:click=move |_| {
                                i18n.set_locale(Locale::zh_CN);
                                set_lang_open.set(false);
                            } class:active=move || i18n.get_locale() == Locale::zh_CN
                        >
                            {move || i18n.get_keys().lang_zh().into_view()}
                        </button>
                        <button
                            on:click=move |_| {
                                i18n.set_locale(Locale::en);
                                set_lang_open.set(false);
                            } class:active=move || i18n.get_locale() == Locale::en
                        >
                            {move || i18n.get_keys().lang_en().into_view()}
                        </button>
                    </div>
                </div>

                <div class="dropdown">
                    <button class="dropdown-trigger" on:click=move |_| {
                        set_theme_open.update(|v| *v = !*v);
                        set_community_open.set(false);
                        set_lang_open.set(false);
                    }>
                        {current_theme_name}
                    </button>
                    <div class="dropdown-menu" class:open=move || theme_open.get()>
                        <button on:click=move |_| {
                            set_current_theme.set("light".to_string());
                            set_theme_open.set(false);
                        } class:active=move || current_theme.get() == "light">
                            {move || i18n.get_keys().theme_light().into_view()}
                        </button>
                        <button on:click=move |_| {
                            set_current_theme.set("dark".to_string());
                            set_theme_open.set(false);
                        } class:active=move || current_theme.get() == "dark">
                            {move || i18n.get_keys().theme_dark().into_view()}
                        </button>
                        <button on:click=move |_| {
                            set_current_theme.set("system".to_string());
                            set_theme_open.set(false);
                        } class:active=move || current_theme.get() == "system">
                            {move || i18n.get_keys().theme_system().into_view()}
                        </button>
                    </div>
                </div>

                <div class="nav-links">
                    <a class="login-btn" href="/login">
                        {t!(i18n, login)}
                    </a>
                    <a class="logout-btn" href="/logout">
                        {t!(i18n, logout)}
                    </a>
                </div>
            </div>
        </nav>
    }
}
