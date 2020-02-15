use leaf::models;

markup::define! {
    Layout<'title, Body: markup::Render>(body: Body, title: &'title str) {
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
                        "Copyright © 2020 Wesley Moore — "
                        a[href="https://github.com/wezm/wezm.net"] {"Source on GitHub"}
                    }
                }
            }
        }
    }
    Index<'tasks>(tasks: &'tasks [models::Task]) {
        form[action="/tasks", method="POST"] {
            ul."task-list" {
                li."new-task" {
                    span.ornament {{markup::raw("➕&#xFE0E;")}}
                    input[type="text", name="newtask", placeholder="New task"];
                }
                @for task in *(tasks) {
                    {Task { description: &task.description }}
                }
            }

            div.actions {
                input[type="submit", name="submit", value="Save"];
            }
        }
    }
    Task<'a>(description: &'a str) {
        li {
            label {
                input[type="checkbox", name="completed", value="1"];
                {description}
            }
        }
    }
    Login(flash: Option<String>) {
        form.login.center[action="/login", method="POST"] {
            @if let Some(ref message) = *(flash) {
                .flash.center { { message } }
            }
            label[for="password"] { "Password" }
            input#password[type="password", name="password", required?=true];

            input[type="submit", name="submit", value="Login"];
        }
    }
}
