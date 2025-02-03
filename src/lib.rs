mod app;
mod components;
mod pages;

pub use app::*;
pub use components::*;
pub use pages::*;

#[cfg(feature = "ssr")]
mod server;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::App;

    console_error_panic_hook::set_once();

    leptos::mount::hydrate_body(App);
}
