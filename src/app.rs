use leptos::prelude::*;
use leptos_meta::Stylesheet;

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);

    view! {
        <Stylesheet id="leptos" href="output.css"/>
        <button
            class="p-32 text-red-600"
            on:click=move |_| set_count.update(|val| *val+=1)
        >
            "Click me: "
            {count}
        </button>
        <p class="bg-pink-500 rounded-md text-white">
            "Double count: "
            {move || count.get() * 2}
        </p>
    }
}