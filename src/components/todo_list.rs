use std::sync::{Arc, Mutex};

use leptos::{html, prelude::*};

use crate::model::{FilterView, Todo};

#[component]
pub fn Main(
    todos: ReadSignal<Vec<Todo>>,
    filters: ReadSignal<FilterView>,
    on_update: impl FnMut((usize, String)) + 'static + Send + Sync,
    on_check: impl FnMut(usize) + 'static + Send,
    on_destroy: impl FnMut(usize) + 'static + Send,
    on_toggle_all: impl FnMut() + 'static + Send + Sync
) -> impl IntoView {
    let on_update = Arc::new(Mutex::new(on_update));
    let on_check = Arc::new(Mutex::new(on_check));
    let on_destroy = Arc::new(Mutex::new(on_destroy));
    let on_toggle_all = Arc::new(Mutex::new(on_toggle_all));

    view! {
        <section class="main">
            <Show when=move|| { todos.get().len() > 0 }>
                {
                    let on_toggle_all = on_toggle_all.clone();
                    view! {
                        <div class="toggle-all-container">
                            <input
                                id="toggle-all"
                                class="toggle-all"
                                type="checkbox"
                                on:change=move |_| (on_toggle_all.lock().unwrap())()
                                prop:checked=move|| todos.get().iter().all(|todo| todo.done)
                            />
                            <label for="toggle-all" class="toggle-all-label">
                                "Mark all as complete"
                            </label>
                        </div>
                    }
                }
            </Show>
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
                        let is_completed = Memo::new(move |_| {
                            todos
                                .with(|item| {
                                    item.iter()
                                        .find(|item| item.id == id)
                                        .map(|d| d.done)
                                        .unwrap_or(false)
                                })
                        });
                        let desc = Memo::new(move |_| {
                            todos
                                .with(|item| {
                                    item.iter()
                                        .find(|item| item.id == id)
                                        .map(|d| d.description.clone())
                                        .unwrap_or("".to_string())
                                })
                        });
                        let edit_ref: NodeRef<html::Input> = NodeRef::new();
                        let (editing, set_editing) = signal(false);
                        Effect::new(move |prev: Option<bool>| {
                            if prev == Some(false) && editing.get() == true {
                                let _ = edit_ref.get().unwrap().focus();
                            }
                            editing.get()
                        });

                        view! {
                            <li class:completed=is_completed class:editing=move || editing.get()>
                                <div class="view">
                                    <input
                                        class="toggle"
                                        type="checkbox"
                                        prop:checked=is_completed
                                        on:change=move |_| (on_check.lock().unwrap())(todo.id)
                                    />
                                    <label on:dblclick=move |_| set_editing.set(true)>{desc}</label>
                                    <button
                                        class="destroy"
                                        on:click=move |_| (on_destroy.lock().unwrap())(todo.id)
                                    />
                                </div>
                                {
                                    let on_key_down_update = on_update.clone();
                                    let on_blur_update = on_update.clone();
                                    view! {
                                        <input
                                            class="edit"
                                            value=todo.description
                                            node_ref=edit_ref
                                            on:keydown=move |ev| {
                                                if ev.key() == "Enter" {
                                                    (on_key_down_update
                                                        .lock()
                                                        .unwrap())((id, edit_ref.get().unwrap().value()));
                                                    set_editing.set(false);
                                                }
                                                if ev.key() == "Escape" {
                                                    edit_ref.get().unwrap().set_value(&desc.get());
                                                    set_editing.set(false);
                                                }
                                            }
                                            on:blur=move |_| {
                                                (on_blur_update
                                                    .lock()
                                                    .unwrap())((id, edit_ref.get().unwrap().value()));
                                                set_editing.set(false);
                                            }
                                        />
                                    }
                                }
                            </li>
                        }
                    }
                />
            </ul>
        </section>
    }
}