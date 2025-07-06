use std::time::Duration;

use leptos::{prelude::*, reactive::spawn_local};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
struct UptimeResponse {
    status: String,
    duration: u64,
    ratio: Option<f64>,
}

#[component]
fn App() -> impl IntoView {
    let (uptime, set_uptime) = signal(UptimeResponse {
        status: "Loading...".to_string(),
        duration: 0,
        ratio: None,
    });

    let load_uptime = move || {
        spawn_local(async move {
            let client = reqwest::Client::new();

            set_uptime
                .update(|uptime| uptime.status = "Loading...".to_string());

            let res = client
                .get("http://localhost:8081/api/uptime")
                .send()
                .await
                .expect("Failed to fetch uptime")
                .json::<UptimeResponse>()
                .await
                .expect("Failed to parse JSON");

            set_uptime.set(res);
        });
    };

    let resource = Resource::new(
        move || {},
        move |_| async move {
            load_uptime();
        },
    );

    set_interval(
        move || {
            resource.refetch();
        },
        Duration::from_secs(5),
    );

    view! { cx,
        <div class="container mx-auto p-4">
            <h1 class="text-2xl font-bold">"Website Uptime Status"</h1>
            <p>"Status: " {move || uptime.get().status}</p>
            <p>"Duration: " {move || uptime.get().duration}</p>
            <p>"Ratio: " {move || uptime.get().ratio.map(|v| format!("{v}%")).unwrap_or("Unknown".to_string())}</p>
        </div>
    }
    // view! {
    //         <div class="min-h-screen bg-gray-100 flex items-center justify-center p-4">
    //             <div class="bg-white rounded-lg shadow-lg p-6 max-w-md w-full">
    //                 <h1 class="text-3xl font-bold text-gray-800 mb-4 text-center">
    //                     "Website Uptime Status"
    //                 </h1>
    //
    //                 // Progress bar during loading
    //                 {move || uptime_resource.loading().get().then(|| view! {
    //                     <div class="w-full bg-gray-200 rounded-full h-2.5 mb-4">
    //                         <div class="bg-blue-600 h-2.5 rounded-full animate-pulse" style="width: 100%"></div>
    //                     </div>
    //                 })}
    //
    //                 // Uptime data or error
    //                 {move || match uptime_resource.get() {
    //                     None => view! {
    //                         <p class="text-gray-500 text-center">"Loading..."</p>
    //                     }.into_view(),
    //                     Some(Ok(Some(data))) => view! {
    //                         <div class="space-y-3">
    //                             <p class="text-lg">
    //                                 <span class="font-semibold">"Status: "</span>
    //                                 <span class={move || format!(
    //                                     "font-medium {}",
    //                                     if data.status == "up" { "text-green-600" } else { "text-red-600" }
    //                                 )}>
    //                                     {data.status}
    //                                 </span>
    //                             </p>
    //                             <p class="text-lg">
    //                                 <span class="font-semibold">"Uptime Ratio: "</span>
    //                                 {data.uptime_ratio} "%"
    //                             </p>
    //                             <p class="text-lg">
    //                                 <span class="font-semibold">"Last Downtime: "</span>
    //                                 {data.last_downtime.unwrap_or("None".to_string())}
    //                             </p>
    //                         </div>
    //                     }.into_view(),
    //                     Some(Ok(None)) => view! {
    //                         <p class="text-gray-500 text-center">"No data available"</p>
    //                     }.into_view(),
    //                     Some(Err(error)) => view! {
    //                         <p class="text-red-500 text-center">"Error: " {error}</p>
    //                     }.into_view(),
    //                 }}
    //             </div>
    //         </div>
    //     }
}

pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
