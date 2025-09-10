use eyre::Result;
use reqwest::Client;
use serde_json::json;
use tracing::{error, info};

pub async fn send_new_user_notification(username: &str) -> Result<()> {
    let webhook_url = match std::env::var("SLACK_WEBHOOK_URL") {
        Ok(url) => url,
        Err(_) => {
            info!("SLACK_WEBHOOK_URL not configured, skipping Slack notification");
            return Ok(());
        }
    };

    let client = Client::new();
    let payload = json!({
        "text": format!("New gh user: **{}**", username),
        "username": "MEVlog Bot",
        "icon_emoji": ":goat:"
    });

    match client.post(&webhook_url).json(&payload).send().await {
        Ok(response) => {
            if response.status().is_success() {
                info!(
                    "Successfully sent Slack notification for new user: {}",
                    username
                );
            } else {
                error!(
                    "Failed to send Slack notification. Status: {}",
                    response.status()
                );
            }
        }
        Err(e) => {
            error!("Error sending Slack notification: {:?}", e);
        }
    }

    Ok(())
}
