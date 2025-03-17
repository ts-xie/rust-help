use std::sync::{Arc, Mutex};

use leptos::{ev::MouseEvent, prelude::*};

use crate::model::{FilterView, Todo};

#[component]
pub fn Main(
    todos: ReadSignal<Vec<Todo>>,
    filters: ReadSignal<FilterView>,
    on_check: impl FnMut(usize) + 'static + Send,
    on_destroy: impl FnMut(usize) + 'static + Send,
    on_toggle_all: impl FnMut(MouseEvent) + 'static
) -> impl IntoView {
    let on_check = Arc::new(Mutex::new(on_check));
    let on_destroy = Arc::new(Mutex::new(on_destroy));

    view! {
        <section class="main">
            <input id="toggle-all" class="toggle-all" type="checkbox" />
            <label for="toggle-all" on:click=on_toggle_all>
                "Mark all as complete"
            </label>
            <ul class="todo-list">
                <For
                    each=move || {
                        let filter = filters.get();
                        todos
                            .get()
                            .into_iter()
                            .filter(move |todo| match filter {
                                FilterView::All => true,
                                FilterView::Active => !todo.done,
                                FilterView::Completed => todo.done,
                            })
                            .map(|todo| (todo.id, todo))
                    }
                    key=|(_, todo)| todo.id
                    children=move |(id, todo)| {
                        let on_check = on_check.clone();
                        let on_destroy = on_destroy.clone();
                        let value = Memo::new(move |_| {
                            todos
                                .with(|item| {
                                    item
                                        .iter()
                                        .find(|item| item.id == id)
                                        .map(|d| d.done)
                                        .unwrap_or(false)
                                })
                        });
                        view! {
                            <li class:completed=value>
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