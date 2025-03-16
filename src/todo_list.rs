use std::sync::{Arc, Mutex};

use leptos::prelude::*;
use crate::FilterView;
use crate::Todo;

#[component]
pub fn Main(
    todos: ReadSignal<Vec<Todo>>,
    filters: ReadSignal<FilterView>,
    mut on_check: impl FnMut(Todo) + 'static + Send + Clone,
) -> impl IntoView {
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
                            view! {
                                <li>
                                    <div class="view">
                                        <input
                                            class="toggle"
                                            type="checkbox"
                                            prop:checked=todo.done
                                            on:change=move |_| on_check(todo)
                                        />                                        <button
                                            class="destroy"
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