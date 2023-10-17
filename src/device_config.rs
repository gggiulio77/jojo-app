use leptos::logging::log;
use leptos::*;
use leptos_router::*;

#[component]
pub fn DeviceConfig() -> impl IntoView {
    // TODO: create struct Device with { sensibility, button1, id, and some other shit }
    // TODO: create tauri commando to get device information
    // TODO: create all device config tauri commands

    log!("Rendering child");

    let params = use_params_map();
    let id = move || params.with(|params| params.get("id").cloned().unwrap_or_default());

    view! {
        // TODO: create cards -> sensibility, calibrate zero, button 1, invert x/y, turn off socket and clear credentials
        <div class="drawer-content flex flex-col items-center justify-center">
          <div class="card w-96 bg-neutral text-neutral-content">
            <div class="card-body items-center text-center">
              <h2 class="card-title">Cookies!</h2>
              <p>"We are using cookies for no reason."</p>
              <div class="card-actions justify-end">
                <button class="btn btn-primary">Accept</button>
                <button class="btn btn-ghost">Deny</button>
              </div>
            </div>
          </div>
        </div>
    }
}
