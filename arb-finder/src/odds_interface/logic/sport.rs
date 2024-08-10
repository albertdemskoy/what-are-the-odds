use serde::Deserialize;

use super::event::Event;

#[derive(Deserialize, Debug, Clone)]
pub struct Sport {
    key: String,
    group: String,
    title: String,
    description: String,
    active: bool,
    has_outrights: bool,
}
