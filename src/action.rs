use jiff::civil::DateTime;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Action {
    who: String,
    datetime: DateTime,
    note_html: String,
    outcome: String,
    ticket_id: u32,
}

impl Action {
    pub fn new(
        who: impl Into<String>,
        datetime: DateTime,
        note_html: impl Into<String>,
        outcome: impl Into<String>,
        ticket_id: u32,
    ) -> Self {
        Self {
            who: who.into(),
            datetime,
            note_html: note_html.into(),
            outcome: outcome.into(),
            ticket_id,
        }
    }
}
