use footer::Footer;
use header::Header;
use todo_list::Main;

use model::{Todo, FilterView};
use leptos::prelude::*;

mod footer;
mod model;
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
                    counter += 1;
                    if key == "Enter" {
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
                on_check=move |todo| {
                    if let Some(index) = todos
                        .get()
                        .iter()
                        .position(|cur| cur.id == todo.id)
                    {
                        set_todos
                            .update(|old_todos| {
                                old_todos[index].done = !(old_todos[index].done);
                            });
                    }
                }
                on_destroy=move |todo| {
                    if let Some(index) = todos
                        .get()
                        .iter()
                        .position(|cur| cur.id == todo.id)
                    {
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

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
