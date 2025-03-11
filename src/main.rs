use leptos::{prelude::*};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Todo {
    id: usize,
    description: String,
    done: bool
}

#[derive(Debug, Default, Clone, PartialEq, Copy)]
enum FilterView {
    #[default]
    All,
    Active,
    Completed
}

impl FilterView {
    const VALUES: [Self;3] = [Self::All, Self::Active, Self::Completed];

    fn as_string(&self) -> String {
        match self {
            Self::All => "All".to_string(),
            Self::Active => "Active".to_string(),
            Self::Completed => "Completed".to_string()
        }
    }

    fn get_link(&self) -> String {
      match self {
          Self::All => "#/".to_string(),
          Self::Active => "#/active".to_string(),
          Self::Completed => "#/completed".to_string()
      }
  }
}

#[component]
pub fn App() -> impl IntoView {
    let mut counter = 0;
    let todo_list: Vec<Todo> = vec![];
    let (new_todo, set_new_todo) = signal(String::new());
    let (todos, set_todos) = signal(todo_list);
    let (filter_view, set_filter_view) = signal(FilterView::All);
    view! {
        <section class="todoapp">
            <header class="header">
                <h1>"todos"</h1>
                <input
                    class="new-todo"
                    on:input:target=move |ev| {
                        set_new_todo.set(ev.target().value());
                    }
                    on:keyup=move |ev| {
                        if ev.key() == "Enter" {
                            set_todos
                                .update(|todos| {
                                    counter += 1;
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
                    prop:value=new_todo
                />
            </header>
            <section class="main">
                <input id="toggle-all" class="toggle-all" type="checkbox" />
                <label for="toggle-all">"Mark all as complete"</label>
                <ul class="todo-list">
                    <For
                        each=move || {
                            let what_filter = filter_view.get();
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
                                            on:change=move |_| {
                                                if let Some(index) = todos
                                                    .get()
                                                    .iter()
                                                    .position(|cur| cur.id == todo.id)
                                                {
                                                    set_todos
                                                        .update(|old_todos| {
                                                            leptos::logging::log!("done? {}", old_todos[index].done);
                                                            old_todos[index].done = !(old_todos[index].done);
                                                        });
                                                }
                                            }
                                        />
                                        <label>{todo.description}</label>
                                        <button
                                            class="destroy"
                                            on:click=move |_| {
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
                                    </div>
                                </li>
                            }
                        }
                    />
                </ul>
            </section>
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
                            view! {
                                <li on:click=move |_| { set_filter_view.set(filter_method) }>
                                    <a
                                        class:selected=move || {
                                            filter_view.get() == filter_method
                                        }
                                        href=filter_method.get_link()
                                    >
                                        {filter_method.as_string()}
                                    </a>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
                <button
                    class="clear-completed"
                    on:click=move |_| {
                        let new_todos = todos
                            .get()
                            .into_iter()
                            .filter(|todo| !todo.done)
                            .collect::<Vec<_>>();
                        set_todos.set(new_todos);
                    }
                >
                    "Clear completed"
                </button>
            </footer>
        </section>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount::mount_to_body(App);
}
