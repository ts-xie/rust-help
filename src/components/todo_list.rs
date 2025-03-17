use std::sync::{Arc, Mutex};

use leptos::prelude::*;
use crate::model::{FilterView, Todo};

#[component]
pub fn Main(
    todos: ReadSignal<Vec<Todo>>,
    filters: ReadSignal<FilterView>,
    on_check: impl FnMut(usize) + 'static + Send,
    on_destroy: impl FnMut(usize) + 'static + Send,
) -> impl IntoView {
    let on_check = Arc::new(Mutex::new(on_check));
    let on_destroy = Arc::new(Mutex::new(on_destroy));

    view! {
        <section class="main">
            <input id="toggle-all" class="toggle-all" type="checkbox" />
            <label for="toggle-all">"Mark all as complete"</label>
            <ul class="todo-list">
                <For
                    each=move || {
                        let what_filter = filters.get();
                        todos
                            .get()
                            .into_iter()
                            .filter(move |todo| match what_filter {
                                FilterView::All => true,
                                FilterView::Active => !todo.done,
                                FilterView::Completed => todo.done,
                            })
                    }
                    key=|todo| todo.id
                    children=move |todo| {
                        let on_check = on_check.clone();
                        let on_destroy = on_destroy.clone();
                        view! {
                            <li>
                                <div class="view">
                                    <input
                                        class="toggle"
                                        type="checkbox"
                                        prop:checked=todo.done
                                        on:change=move |_| (on_check.lock().unwrap())(todo.id)
                                    />
                                    <label>{todo.description}</label>
                                    <button
                                        class="destroy"
                                        on:click=move |_| (on_destroy.lock().unwrap())(todo.id)
                                    />
                                </div>
                                <input class="edit" value="Create a TodoMVC template" />
                            </li>
                        }
                    }
                />
            </ul>
        </section>
    }
}