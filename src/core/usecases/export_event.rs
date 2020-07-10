use crate::core::prelude::*;

pub fn export_event<'a>(
    event: Event,
    role: Role,
    moderated_tags: impl IntoIterator<Item = &'a str>,
) -> Event {
    if role < Role::Admin {
        let event = super::filter_event(event, moderated_tags);
        if role < Role::Scout {
            event.strip_contact_details()
        } else {
            event
        }
    } else {
        event
    }
}
