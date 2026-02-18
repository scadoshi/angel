mod action;
mod client;
mod config;
mod token;

use crate::{action::Action, client::AuthenticatedClient};
use jiff::{tz::TimeZone, Timestamp};

fn main() -> anyhow::Result<()> {
    let mut client = AuthenticatedClient::new()?;
    println!("✔  Authenticated");
    let action = Action::new(
        "Rust App",
        Timestamp::now().to_zoned(TimeZone::UTC).datetime(),
        "<p><b>Testing actions</b></p><p><ol><li>Item 1</li><li>Item 2</li></ol></p>",
        "Note Added via API",
        2997,
    );
    client.post_action(action)?;
    println!("✔  Action Posted");
    Ok(())
}
