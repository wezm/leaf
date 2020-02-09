use crate::store;

markup::define! {
    Index<'tasks>(tasks: &'tasks [&'tasks store::Task]) {
        {markup::doctype()}
        html[lang = "en"] {
            head {
                meta[charset="utf-8"] {
                    meta[name="viewport", content="width=device-width, initial-scale=1"] {
                        title { "🍃 Tasks" }
                        link[rel="stylesheet", href="css/app.css", type="text/css", charset="utf-8"];
                    }
                    body {
                        header {
                            h1 {"🍃 Tasks"}
                        }
                        main {
                            form[action="/tasks", method="POST"] {
                                ul."task-list" {
                                    li."new-task" {
                                        span.ornament {"➕&#xFE0E;"}
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
                        footer.center {
                            div.copyright {
                                "Copyright © 2020 Wesley Moore &mdash;"
                                    a[href="https://github.com/wezm/wezm.net"] {"Source on GitHub"}
                            }
                        }
                    }
                }
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
}
