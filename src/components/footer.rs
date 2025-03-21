use std::sync::{Arc, Mutex};
use leptos::{ev::MouseEvent, prelude::*};

use crate::model::{Todo, FilterView};

#[component]
pub fn Footer(
    todos: ReadSignal<Vec<Todo>>,
    filters: ReadSignal<FilterView>,
    on_filter_update: impl FnMut(FilterView) + 'static,
    on_clear_completed: impl Fn(MouseEvent) + 'static + Send + Sync
) -> impl IntoView {
    let on_filter_update = Arc::new(Mutex::new(on_filter_update));
    let on_clear_completed = Arc::new(on_clear_completed);
    view! {
        <footer class="footer">
            <span class="todo-count">
                {move || {
                    let n = todos.get().iter().filter(|todo| !todo.done).count();
                    format!("{n} {} left!", if n > 1 { "items" } else { "item" })
                }}
            </span>
            <ul class="filters">
                {FilterView::VALUES
                    .into_iter()
                    .map(|filter_method| {
                        let on_filter_update = Arc::clone(&on_filter_update);
                        view! {
                            <li on:click=move |_| (on_filter_update.lock().unwrap())(filter_method)>
                                <a
                                    class:selected=move || { filters.get() == filter_method }
                                    href=filter_method.get_link()
                                >
                                    {filter_method.as_string()}
                                </a>
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
            <Show when=move || {
                todos.get().iter().any(|todo| todo.done)
            }>
                {
                    let on_clear_completed = on_clear_completed.clone();
                    view! {
                        <button class="clear-completed" on:click=move |ev| on_clear_completed(ev)>
                            "Clear completed"
                        </button>
                    }
                }
            </Show>
        </footer>
    }
}