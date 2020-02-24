use rocket::request::{FormItems, FromForm};

use leaf::models::TaskId;

pub struct TasksForm {
    pub new_task: Option<String>,
    pub completed_ids: Vec<TaskId>,
}

impl<'f> FromForm<'f> for TasksForm {
    // In practice, we'd use a more descriptive error type.
    type Error = (); // FIXME

    fn from_form(items: &mut FormItems<'f>, strict: bool) -> Result<TasksForm, ()> {
        let mut description = None;
        let mut completed_ids = Vec::new();

        for item in items {
            match item.key.as_str() {
                "description" if description.is_none() => {
                    if !item.value.is_empty() {
                        let decoded = item.value.url_decode().map_err(|_| ())?;
                        description = Some(decoded);
                    }
                }
                key if key.starts_with("complete") => {
                    let id = item
                        .value
                        .url_decode()
                        .map_err(|_| ()) // FIXME err
                        .and_then(|value| value.parse().map_err(|_| ()))?; // FIXME err
                    completed_ids.push(id)
                }
                _ if strict => return Err(()),
                _ => { /* allow extra value when not strict */ }
            }
        }

        Ok(TasksForm {
            new_task: description,
            completed_ids,
        })
    }
}
