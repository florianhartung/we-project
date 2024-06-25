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
        <label class="flex flex-col text-white">
            {label}
            <input
                type=input_type
                name=input_name
                placeholder=placeholder
                class="bg-gray-600 border-2 border-gray-700 rounded-md shadow-sm placeholder:text-black/50 p-2"
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
            class="bg-gray-400 border-white rounded-full border-2 shadow-sm hover:bg-gray-600 px-6 py-2 text-white"
            value=label
        />
    }
}
