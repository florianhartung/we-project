use leptos::{component, view, IntoView, MaybeProp, MaybeSignal, TextProp};

#[component]
pub fn TextInputField(
    #[prop(into)] input_name: &'static str,
    #[prop(into)] label: MaybeSignal<String>,
    #[prop(into, optional)] placeholder: MaybeProp<TextProp>,
    #[prop(optional)] is_password: bool,
) -> impl IntoView {
    let input_type = is_password
        .then(|| "password")
        .unwrap_or("text");

    view! {
        <label class="flex flex-col">
            {label}
            <input
                type=input_type
                name=input_name
                placeholder=placeholder
                class="border-2 border-black rounded-sm shadow-sm placeholder:text-gray-400 p-2"
            />
        </label>
    }
}

#[component]
pub fn SubmitButton(
    #[prop(into)]
    label: MaybeSignal<String>
) -> impl IntoView {
    view! {
        <input
            type="submit"
            class="bg-gray-200 border-black border-2 rounded-sm shadow-sm hover:bg-slate-200 px-6 py-2"
            value=label
        />
    }
}
