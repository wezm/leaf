use std::fmt;

use leaf::models;
use markup::Render;
use regex::Regex;

use crate::auth::User;

struct AutoLink<'a>(&'a str);

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
                    input[type="text", name="description", placeholder="New task", autofocus?=true];
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
                {AutoLink(description)}
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

impl<'a> Render for AutoLink<'a> {
    fn render(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Source http://www.urlregex.com/ (Python version)
        lazy_static! {
            static ref RE: Regex = Regex::new(
                "http[s]?://(?:[a-zA-Z]|[0-9]|[$-_@.&+]|[!*(),]|(?:%[0-9a-fA-F][0-9a-fA-F]))+"
            )
            .unwrap();
        }

        let mut start = 0;
        for url_match in RE.find_iter(self.0) {
            // Write out the text preceding the URL escaped
            &self.0[start..url_match.start()].render(f)?;
            // Write out the URL as a link, unescaped
            markup::raw(format!(
                r#"<a href="{url}" target="_blank">{url}</a>"#,
                url = url_match.as_str()
            ))
            .render(f)?;
            // Update the start marker
            start = url_match.end()
        }

        if start < self.0.len() {
            &self.0[start..].render(f)?;
        }

        Ok(())
    }

    fn is_none(&self) -> bool {
        self.0.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl<'a> fmt::Display for AutoLink<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.render(f)
        }
    }

    #[test]
    fn test_autolink() {
        assert_eq!(AutoLink("").to_string(), String::from(""));
        assert_eq!(AutoLink("no url").to_string(), String::from("no url"));
        assert_eq!(AutoLink("url.com").to_string(), String::from("url.com"));
        assert_eq!(
            AutoLink("https://example.com/").to_string(),
            String::from(
                r#"<a href="https://example.com/" target="_blank">https://example.com/</a>"#
            )
        );
        assert_eq!(
            AutoLink("https://example.com/ after url").to_string(),
            String::from(
                r#"<a href="https://example.com/" target="_blank">https://example.com/</a> after url"#
            )
        );
        assert_eq!(
            AutoLink("before url https://example.com/").to_string(),
            String::from(
                r#"before url <a href="https://example.com/" target="_blank">https://example.com/</a>"#
            )
        );
        assert_eq!(
            AutoLink("http://example.com/ https://example.com/").to_string(),
            String::from(
                r#"<a href="http://example.com/" target="_blank">http://example.com/</a> <a href="https://example.com/" target="_blank">https://example.com/</a>"#
            )
        );
    }
}
