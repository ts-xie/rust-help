use leptos::prelude::*;

#[component]
pub fn Header(
    new_todo: ReadSignal<String>,
    mut on_input: impl FnMut(String) + 'static,
    mut on_key_up: impl FnMut(String) + 'static
) -> impl IntoView {
    view! {
        <header class="header">
            <h1>"todos"</h1>
            <input
                class="new-todo"
                placeholder="What needs to be done?"
                on:input:target=move |ev| on_input(ev.target().value())
                on:keyup=move |ev| { on_key_up(ev.key()) }
                prop:value=new_todo
            />
        </header>
    }
}