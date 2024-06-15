use std::borrow::Cow;

use leptos::{component, view, IntoView};

pub struct TechStackItem {
    name: Cow<'static, str>,
    url: Cow<'static, str>,
}

impl TechStackItem {
    pub fn new(name: impl Into<Cow<'static, str>>, url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name: name.into(),
            url: url.into(),
        }
    }
}

#[component]
pub fn TechStackList(items: Vec<TechStackItem>) -> impl IntoView {
    let render_item = |item: TechStackItem| {
        view! {
            <li>
                <a href={item.url} class="after:content-['[↗]'] after:text-[0.5em] after:text-blue-600">
                    {item.name}
                </a>
            </li>
        }
    };

    view! {
        <ul class="list-disc ps-10">
            {
                items.into_iter()
                    .map(render_item)
                    .collect::<Vec<_>>()
            }
        </ul>
    }
}
