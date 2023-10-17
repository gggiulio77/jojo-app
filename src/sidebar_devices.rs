use leptos::*;
use leptos_router::*;

// TODO: find a way to listen devices connected in the backend, like subscribe to an event or something similar

#[component]
pub fn SidebarDevices() -> impl IntoView {
    let (device_id, _device_id_set) = create_signal("");
    let (devices, _devices_set) = create_signal::<Vec<String>>(vec![]);

    view! {
        <div class="drawer-side">
            <label for="devices-drawer" class="drawer-overlay"></label>
            <ul class="menu p-4 w-30 h-full bg-base-200 text-base-content items-center">
                <Show when=move || { devices.get().len() > 0} fallback=|| view! {
                    <li><a>"No devices connected"</a></li>
                }>
                    // TODO: show all connected devices
                    // TODO: navigate to /:id per device listed
                    <li><a>"Device 1"</a></li>
                </Show>
                <div class="divider"></div>
            </ul>
        </div>
        <Outlet/>
    }
}
