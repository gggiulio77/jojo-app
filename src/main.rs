mod app;
mod device_config;
mod sidebar_devices;

use leptos::*;

use crate::app::App;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
