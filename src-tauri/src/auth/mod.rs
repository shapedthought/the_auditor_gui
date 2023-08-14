use regex::Regex;
use reqwest::Client;
use vauth::{build_url, Profile};

use crate::{
    models::oauth::{AuthRequest, AuthResponse, CompleteRequest, CompleteResponse},
    tcplistener::run_tcp_listener,
};

pub async fn set_up_auth(
    address: &String,
    profile: &Profile,
    client: &Client,
) -> Result<CompleteResponse, anyhow::Error> {
    let auth_request = AuthRequest {
        authentication_service_kind: "Microsoft365".to_string(),
        tenant_id: None,
        client_id: None,
        client_secret: None,
        redirect_url: "http://localhost".to_string(),
    };
    let url = build_url(
        address,
        &"AuditEmailSettings/PrepareOAuthSignIn".to_string(),
        profile,
    )?;
    let response = client
        .post(&url)
        .json(&auth_request)
        .send()
        .await?
        .json::<AuthResponse>()
        .await?;

    println!("Opening browser to sign in...");
    webbrowser::open(&response.sign_in_url)?;

    println!("Please sign in, this program will listen for the data from the call back.");

    let url_string = run_tcp_listener("http://localhost".to_string()).await?;
    if url_string.is_empty() {
        return Err(anyhow::anyhow!("The URL was empty"));
    }
    let pattern = r"=([^&]+)";
    let regex = Regex::new(pattern).unwrap();
    let matches: Vec<&str> = regex
        .captures_iter(&url_string)
        .map(|capture| capture.get(1).unwrap().as_str())
        .collect();
    if matches.len() != 3 {
        return Err(anyhow::anyhow!("Invalid URL"));
    }
    let complete_request = CompleteRequest {
        code: matches[0].to_string(),
        state: matches[1].to_string(),
    };
    let url_string = build_url(
        address,
        &"AuditEmailSettings/CompleteOAuthSignIn".to_string(),
        profile,
    )?;
    let complete_response = client
        .post(&url_string)
        .json(&complete_request)
        .send()
        .await?;

    if complete_response.status().is_success() {
        println!("Authentication successful!");
        let complete_response = complete_response.json::<CompleteResponse>().await?;
        Ok(complete_response)
    } else {
        println!("Authentication unsuccessful!");
        let reason = complete_response.text().await?;
        Err(anyhow::anyhow!("Authentication failed! {:?}", reason))
    }

}
