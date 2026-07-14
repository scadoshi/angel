#![allow(dead_code)]

mod client;
mod config;
mod endpoint;
mod token;

use mjolnir::models::custom_integration::CustomIntegration;

use crate::{client::AuthenticatedClient, endpoint::Endpoint};

fn main() -> anyhow::Result<()> {
    #[allow(unused_mut, unused_variables)]
    let mut client = AuthenticatedClient::new()?;
    println!("✔  Authenticated");
    let custom_integration = CustomIntegration::new(
        "<<halo_url>>/api",
        "<insert secret for custom integration>",
        8,
    );
    let request_body = vec![custom_integration];
    client.post(Endpoint::CustomIntegration, request_body)?;
    println!("✔  CustomIntegration Posted");
    Ok(())
}
