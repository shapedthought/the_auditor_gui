// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod auth;
mod models;
mod tcplistener;
mod excel;
use auth::set_up_auth;
use models::{audit::AuditItem, notification::NotificationData, org::{OrgItem, OrgSummary}, group::ItemIds};
use serde::{Serialize, Serializer};
use std::env;
use tauri::http::status::StatusCode;
use vauth::{build_auth_headers, build_url, LogInError, Profile, VClientBuilder, VProfile};

use crate::{models::{group::Group, user::User}, excel::{write::write_excel, read::read_excel}};

struct LoginErrorWrapper(LogInError);

impl Serialize for LoginErrorWrapper {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl From<LogInError> for LoginErrorWrapper {
    fn from(error: LogInError) -> Self {
        LoginErrorWrapper(error)
    }
}

impl From<serde_json::Error> for LoginErrorWrapper {
    fn from(_error: serde_json::Error) -> Self {
        LoginErrorWrapper(LogInError::StatusCodeError(
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}

#[tauri::command]
async fn login(username: &str, password: &str, address: &str) -> Result<String, LoginErrorWrapper> {
    env::set_var("VEEAM_API_PASSWORD", password);
    let mut profile = Profile::get_profile(VProfile::VB365);
    let (_client, login_response) = VClientBuilder::new(&address.to_string(), username.to_string())
        .insecure()
        .build(&mut profile)
        .await?;

    Ok(login_response.access_token)
}

#[tauri::command]
async fn get_audit(address: &str, token: &str, org_id: &str) -> Result<Vec<AuditItem>, LoginErrorWrapper> {
    let profile = Profile::get_profile(VProfile::VB365);
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let audit_string = format!("Organizations/{}/AuditItems", org_id);
    let audit_url = build_url(&address.to_string(), &audit_string, &profile)?;
    let auth_headers = build_auth_headers(&token.to_string(), &profile);
    println!("Audit URL: {}", audit_url);
    let response: Vec<AuditItem> = client
        .get(&audit_url)
        .headers(auth_headers)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    Ok(response)
}

#[tauri::command]
async fn get_users(address: &str, token: &str, path: &str, org_id: &str) -> Result<User, LoginErrorWrapper> {
    let profile = Profile::get_profile(VProfile::VB365);
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let headers = build_auth_headers(&token.to_string(), &profile);
    let user_string = format!("Organizations/{}/Users", org_id);
    let audit_url = build_url(&address.to_string(), &user_string, &profile)?;

    println!("Audit URL: {}", audit_url);
    let response: User = client
        .get(&audit_url)
        .headers(headers)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    write_excel(path.to_string(), Some(response.clone()), None).unwrap();
    Ok(response)
}

#[tauri::command]
async fn get_groups(address: &str, token: &str, path: &str, org_id: &str) -> Result<Group, LoginErrorWrapper> {
    let profile = Profile::get_profile(VProfile::VB365);
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let user_string = format!("Organizations/{}/Groups", org_id);
    let audit_url = build_url(&address.to_string(), &user_string, &profile)?;
    let auth_headers = build_auth_headers(&token.to_string(), &profile);
    println!("Audit URL: {}", audit_url);
    let response: Group = client
        .get(&audit_url)
        .headers(auth_headers)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

        write_excel(path.to_string(), None, Some(response.clone())).unwrap();
    Ok(response)
}

#[tauri::command]
async fn setup_notifications(
    address: &str,
    token: &str,
    user_id: &str,
    from: &str,
    to: &str,
    subject: &str,
) -> Result<String, LoginErrorWrapper> {
    let profile = Profile::get_profile(VProfile::VB365);

    let auth_headers = build_auth_headers(&token.to_string(), &profile);
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .default_headers(auth_headers)
        .build()
        .unwrap();

    let complete_response = set_up_auth(&address.to_string(), &profile, &client)
        .await
        .unwrap();
    
    println!("Setting up notification data to send");
    let nd = NotificationData {
        enable_notification: true,
        authentication_type: "Microsoft365".to_string(),
        to: to.to_string(),
        from: from.to_string(),
        subject: subject.to_string(),
        user_id: user_id.to_string(),
        request_id: complete_response.request_id,
    };

    let url = build_url(
        &address.to_string(),
        &"AuditEmailSettings".to_string(),
        &profile,
    )?;
    
    println!("Sending notification data to {}", url);
    let response = client.put(&url).json(&nd).send().await.unwrap();

    if response.status().is_success() {
        println!("Settings set OK");
        Ok("OK".to_string())
    } else {
        println!("Error in setting up settings");
        Err(LoginErrorWrapper(LogInError::StatusCodeError(
            response.status(),
        )))
    }
}

#[tauri::command]
async fn get_orgs(address: &str, token: &str) -> Result<Vec<OrgSummary>, LoginErrorWrapper> {
    let profiles = Profile::get_profile(VProfile::VB365);
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let headers = build_auth_headers(&token.to_string(), &profiles);

    let end_point = build_url(&address.to_string(), &"Organizations".to_string(), &profiles)?;

    let response: Vec<OrgItem> = client
        .get(&end_point)
        .headers(headers.clone())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let orgs: Vec<OrgSummary> = response.into_iter().map(|org| org.into()).collect();

    Ok(orgs)
}

#[tauri::command]
async fn add_users(address: &str, token: &str, path: &str, org_id: &str) -> Result<String, LoginErrorWrapper> {
    let profile = Profile::get_profile(VProfile::VB365);
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let user_string = format!("Organizations/{}/Users", org_id);
    let audit_url = build_url(&address.to_string(), &user_string, &profile)?;
    let auth_headers = build_auth_headers(&token.to_string(), &profile);
    println!("Audit URL: {}", audit_url);
    let response: User = client
        .get(&audit_url)
        .headers(auth_headers.clone())
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

    let audit_items = read_excel(path.to_string(), Some(response.clone()), None).unwrap();

    let url_str = format!("Organizations/{}/AuditItems", org_id);
    let url = build_url(&address.to_string(), &url_str, &profile)?;

    let res = client.post(url).headers(auth_headers).json(&audit_items).send().await.unwrap();

    let status = res.status();

    if status.is_success() {
        println!("Users added OK");
        Ok("OK".to_string())
    } else {
        println!("Error in adding users");
        let response_str = res.text().await.unwrap().clone();
        println!("Response: {}", response_str);
        Err(LoginErrorWrapper(LogInError::StatusCodeError(
            status,
        )))
    }

}

#[tauri::command]
async fn add_groups(address: &str, token: &str, path: &str, org_id: &str) -> Result<String, LoginErrorWrapper> {
    let profile = Profile::get_profile(VProfile::VB365);
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let user_string = format!("Organizations/{}/Groups", org_id);
    let audit_url = build_url(&address.to_string(), &user_string, &profile)?;
    let auth_headers = build_auth_headers(&token.to_string(), &profile);
    println!("Audit URL: {}", audit_url);
    let response: Group = client
        .get(&audit_url)
        .headers(auth_headers)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();

        let audit_items = read_excel(path.to_string(), None, Some(response.clone())).unwrap();

        let url_str = format!("Organizations/{org_id}/AuditItems");
        let url = build_url(&address.to_string(), &url_str, &profile)?;
    
        let res = client.post(url).json(&audit_items).send().await.unwrap();
    
        if res.status().is_success() {
            println!("Groups added OK");
            Ok("OK".to_string())
        } else {
            println!("Error in adding groups");
            Err(LoginErrorWrapper(LogInError::StatusCodeError(
                res.status(),
            )))
        }
}

#[tauri::command]
async fn delete_audit_item(address: &str, token: &str, id: &str, org_id: &str) -> Result<String, LoginErrorWrapper> {
    let profile = Profile::get_profile(VProfile::VB365);
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let headers = build_auth_headers(&token.to_string(), &profile);

    let url = build_url(
        &address.to_string(),
        &format!("Organizations/{}/AuditItems/remove", org_id),
        &profile,
    )?;

    let item_ids = ItemIds {
        item_ids: vec![id.to_string()],
    };

    println!("Deleting item with id: {}", id);

    let response = client.post(url).headers(headers).json(&item_ids).send().await.unwrap();

    let status = response.status();

    if status.is_success() {
        println!("Items deleted successfully!");
        Ok("OK".to_string())
    } else {
        println!("Error in deleting items");
        let response_str = response.text().await.unwrap().clone();
        println!("Response: {}", response_str);
        Err(LoginErrorWrapper(LogInError::StatusCodeError(
           status,
        )))
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            login, get_audit, get_users, get_groups, setup_notifications, add_users, add_groups, delete_audit_item, get_orgs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
