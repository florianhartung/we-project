use leptos::{
    component, create_resource, create_server_action, view, IntoView, Resource, SignalGet, Suspense,
};

use crate::{api, app::components::common::PillButton};

/// This is a counter that uses a counter state stored on the server.
/// All counter actions (get, increment) are performed through server fns.
#[component]
pub fn ServerCounter() -> impl IntoView {
    let increment_action = create_server_action::<api::counter::Increment>();
    let incremented_value = move || increment_action.value().get().transpose().unwrap();

    // Store both the initial and future count states in a resource.
    // This way during SSR the server can directly call `api::counter::get` and stream it to the client through out-of-order streaming.
    let current_count: Resource<_, u32> = create_resource(
        move || incremented_value(),
        |increment_action_value| async move {
            if let Some(incremented_value) = increment_action_value {
                incremented_value
            } else {
                api::counter::get().await.unwrap()
            }
        },
    );

    view! {
        <div class="flex flex-col items-center space-y-2 border border-gray-300 rounded-md w-fit p-2 shadow-sm">
            <div class="text-2xl">
                <Suspense fallback = || "Loading...">
                        {current_count}
                </Suspense>
            </div>

            <PillButton class="bg-violet-700 text-white font-bold uppercase" on_press=move |()| {
                increment_action.dispatch(api::counter::Increment {})
            }>
            Add one
            </PillButton>
        </div>
    }
}
