use std::borrow::Cow;

use leptos::{component, ev::MouseEvent, view, Callable, Callback, Children, IntoView};
use tailwind_fuse::tw_merge;

#[component]
pub fn PillButton(
    #[prop(into, optional)] on_press: Option<Callback<()>>,
    #[prop(into, optional)] class: Option<Cow<'static, str>>,
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    let on_click_handler = move |_: MouseEvent| {
        if let Some(on_press) = on_press {
            on_press.call(());
        }
    };

    // Note that tw_merge still has some issues. If any issues arise, consider using `tw_join!` instead.
    let class = tw_merge!("bg-green-500 px-[2em] py-[0.8em] rounded-full", class); 

    view! {
        <button class=class on:click=on_click_handler>
            {children.map(|c| c())}
        </button>
    }
}
