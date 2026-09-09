use leptos::prelude::*;

#[component]
pub fn HomePage() -> impl IntoView {
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to OpenPICL!"</h1>
        <button on:click=on_click>"Click Me: "</button>
    }
}
