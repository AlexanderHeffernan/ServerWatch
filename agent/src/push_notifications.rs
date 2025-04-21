/*!
 * The Push Notifications module handles the sending of push notifications to users via Alertzy (https://alertzy.app).
 */
use reqwest::Client;
use serde::Serialize;
use serde_json;
use std::env;

#[derive(Serialize)]
struct AlertzyPayload {
    accountKey: String,
    title: String,
    message: String,
    priority: i32,
}

pub async fn send_alertzy_notification(title: &str, message: &str, priority: i32) -> Result<(), String> {
    // Check if the SERVERWATCH_ALERTZY_KEY environment variable is set
    let alertzy_key = match env::var("SERVERWATCH_ALERTZY_KEY") {
        Ok(key) => key,
        Err(_) => return Err("SERVERWATCH_ALERTZY_KEY environment variable is not set.".to_string()),
    };

    let payload = AlertzyPayload {
        accountKey: alertzy_key,
        title: title.to_string(),
        message: message.to_string(),
        priority,
    };

    let response = Client::new()
        .post("https://alertzy.app/send")
        .form(&payload)
        .send()
        .await
        .map_err(|e| format!("Failed to send Alertzy notification: {}", e))?;

    if response.status().is_success() {
        let json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse Alertzy response: {}", e))?;
        if json["response"] == "success" {
            Ok(())
        } else {
            Err(format!("Alertzy API error: {:?}", json["error"]))
        }
    } else {
        Err(format!("Alertzy request failed with status: {}", response.status()))
    }
}