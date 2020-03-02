use crate::auth::User;
use leaf::models;

markup::define! {
    Layout<'title, 'user, Body: markup::Render>(body: Body, title: &'title str, user: Option<&'user User>) {
        {markup::doctype()}
        html[lang="en"] {
            head {
                meta[charset="utf-8"];
                meta[name="viewport", content="width=device-width, initial-scale=1"];
                title { { title } " – Leaf" }
                link[rel="stylesheet", href="app.css", type="text/css", charset="utf-8"];
            }
            body {
                header.center {
                    h1 { { title } }
                }
                main {
                    { body }
                }
                footer.center {
                    div.copyright {
                        a[href="https://github.com/wezm/leaf"] {"Leaf Tasks"}
                        @if user.is_some() {
                            " — "
                            form.logout[action="/logout", method="POST"] {
                                input[type="submit", name="submit", value="Sign Out"];
                            }
                        }
                    }
                }
            }
        }
    }
    Index<'tasks>(tasks: &'tasks [models::Task]) {
        form[action="/tasks", method="POST"] {
            ul."task-list" {
                li."new-task" {
                    span.ornament {{markup::raw("➕&#xFE0E; ")}}
                    input[type="text", name="description", placeholder="New task"];
                }
                @for task in *(tasks) {
                    {Task { id: task.id.to_string(), description: &task.description }}
                }
            }

            div.actions {
                input[type="submit", name="submit", value="Save"];
            }
        }
    }
    Task<'a>(id: String, description: &'a str) {
        li {
            label {
                input[type="checkbox", name=format!("complete_{}", id), value=id];
                " "
                {description}
            }
        }
    }
    Login<'a>(flash: Option<&'a str>) {
        form.login.center[action="/login", method="POST"] {
            @if let Some(ref message) = *(flash) {
                .flash.center { { message } }
            }
            label[for="password"] { "Password" }
            input#password[type="password", name="password", required?=true];

            input[type="submit", name="submit", value="Sign In"];
        }
    }
}
