use cfg_if::cfg_if;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
mod routes;
use routes::test::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    view! {
        cx,
        <Stylesheet id="leptos" href="/pkg/tailwind.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=  move |cx| view! {
                    cx,
                    <main class="my-0 mx-auto max-w-3xl text-center">
                        <h2 class="p-6 text-4xl">"Hi"</h2>
                        <h3 class="p-6 text-4xl text-blue-400">"Hi"</h3>
                    </main>
                }/>
                <Route path="test" view=|cx| view! { cx,  <Test/> }/>
            </Routes>
        </Router>
    }
}

cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen]
        pub fn hydrate() {
                console_error_panic_hook::set_once();
                _ = console_log::init_with_level(log::Level::Debug);

                log!("hydrate mode - hydrating");

                leptos::mount_to_body(|cx| {
                    view! { cx,  <App/> }
                });
        }
    }
    else if #[cfg(feature = "csr")] {
        use wasm_bindgen::prelude::wasm_bindgen;

        #[wasm_bindgen(start)]
        pub fn main() {
            use leptos::*;
            _ = console_log::init_with_level(log::Level::Debug);
            console_error_panic_hook::set_once();

            log!("csr mode - mounting to body");

            mount_to_body(|cx| {
                view! { cx, <App /> }
            });
        }
  }
}
