use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
    path,
};

pub mod components;
pub mod pages;

use crate::components::Navbar;
use crate::pages::{
    CollectionsPage, DatasetsPage, GalleryPage, HardwarePage, HomePage, ModelsPage,
    OrganizationsPage, PapersPage, SpotlightPage, StudioPage,
};

include!(concat!(env!("OUT_DIR"), "/i18n/mod.rs"));
use i18n::*;

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[allow(non_snake_case)]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/website.css"/>

        <Title text="OpenPICL"/>

        <I18nContextProvider>
            <Router>
                <Suspense fallback=move || view! { <div>Loading...</div> }>
                    <Navbar />
                </Suspense>

                <main>
                    <Routes fallback=|| "Page not found.".into_view()>
                        <Route path=StaticSegment("") view=HomePage/>
                        <Route path=path!("/home") view=HomePage/>
                        <Route path=path!("/hardware") view=HardwarePage/>
                        <Route path=path!("/models") view=ModelsPage/>
                        <Route path=path!("/datasets") view=DatasetsPage/>
                        <Route path=path!("/studio") view=StudioPage/>
                        <Route path=path!("/spotlight") view=SpotlightPage/>
                        <Route path=path!("/collections") view=CollectionsPage/>
                        <Route path=path!("/papers") view=PapersPage/>
                        <Route path=path!("/gallery") view=GalleryPage/>
                        <Route path=path!("/organizations") view=OrganizationsPage/>
                    </Routes>
                </main>
            </Router>
        </I18nContextProvider>
    }
}
