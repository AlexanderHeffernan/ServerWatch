/*!
 * The Push Notifications module handles the sending of push notifications to users via Alertzy (https://alertzy.app).
 */
use reqwest::Client;
use serde::Serialize;
use std::env;

#[derive(Serialize)]
struct AlertzyPayload {
    #[serde(rename = "accountKey")]
    account_key: String,
    title: String,
    message: String,
    priority: i32,
}

pub async fn send_alertzy_notification(
    title: &str,
    message: &str,
    priority: i32,
) -> Result<(), String> {
    // Check if the SERVERWATCH_ALERTZY_KEY environment variable is set
    let alertzy_key = match env::var("SERVERWATCH_ALERTZY_KEY") {
        Ok(key) if !key.trim().is_empty() => key,
        _ => {
            return Err("SERVERWATCH_ALERTZY_KEY environment variable is not set.".to_string());
        }
    };

    let payload = AlertzyPayload {
        account_key: alertzy_key,
        title: title.to_string(),
        message: message.to_string(),
        priority,
    };

    let response = Client::builder()
        .connect_timeout(std::time::Duration::from_secs(1))
        .timeout(std::time::Duration::from_secs(2))
        .build()
        .map_err(|_| "Failed to create notification client".to_string())?
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
        Err(format!(
            "Alertzy request failed with status: {}",
            response.status()
        ))
    }
}
