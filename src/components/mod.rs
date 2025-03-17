use leptos::prelude::*;
use crate::model::{Todo, FilterView};
use header::Header;
use footer::Footer;
use todo_list::Main;

mod footer;
mod header;
mod todo_list;

#[component]
pub fn App() -> impl IntoView {
    let todo_list: Vec<Todo> = vec![];
    let mut counter = 0;
    let (new_todo, set_new_todo) = signal(String::new());
    let (todos, set_todos) = signal(todo_list);
    let (filters, set_filter) = signal(FilterView::All);
    view! {
        <section class="todoapp">
            <Header
                new_todo=new_todo
                on_input=move |value| set_new_todo.set(value)
                on_key_up=move |key| {
                    if key == "Enter" {
                        counter += 1;
                        set_todos
                            .update(|todos| {
                                todos
                                    .push(Todo {
                                        id: counter,
                                        description: new_todo.get(),
                                        done: false,
                                    });
                            });
                        set_new_todo.set(String::new());
                    }
                }
            />
            <Main
                todos=todos
                filters=filters
                on_check=move |id| {
                    if let Some(index) = todos.get().iter().position(|cur| cur.id == id) {
                        set_todos
                            .update(|old_todos| {
                                old_todos[index].toggle_completed();
                            });
                    }
                }
                on_toggle_all=move |_| {
                    match todos.get().iter().all(|item| item.done) {
                        true => {
                            set_todos
                                .update(|todos| {
                                    todos
                                        .iter_mut()
                                        .for_each(|todo| {
                                            todo.done = false;
                                        })
                                })
                        }
                        false => {
                            set_todos
                                .update(|todos| {
                                    todos
                                        .iter_mut()
                                        .for_each(|todo| {
                                            todo.done = true;
                                        })
                                })
                        }
                    }
                }
                on_destroy=move |id| {
                    if let Some(index) = todos.get().iter().position(|cur| cur.id == id) {
                        set_todos
                            .update(|old_todos| {
                                old_todos.remove(index);
                            });
                    }
                }
            />
            <Footer
                todos=todos
                filters=filters
                on_filter_update=move |filter_method| set_filter.set(filter_method)
                on_clear_completed=move |_| {
                    let new_todos = todos
                        .get()
                        .into_iter()
                        .filter(|todo| !todo.done)
                        .collect::<Vec<_>>();
                    set_todos.set(new_todos);
                }
            />
        </section>
    }
}