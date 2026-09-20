use notify_rust::{Notification, Hint, Urgency};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Notifications {
    pub on_success: bool,
    pub on_error: bool,
}

impl Default for Notifications {
    fn default() -> Self {
        Self {
            on_success: false,
            on_error: true,
        }
    }
}



pub fn notification_success(summary: &str, body: &str) {
    Notification::new()
        .appname("Rice")
        .summary(summary)
        .body(body)
        .icon("emblem-success")
        .hint(Hint::Category("transfer.complete".to_string()))
        .urgency(Urgency::Normal)
        .show()
        .unwrap();
}

pub fn notification_error(summary: &str, body: &str) {
    Notification::new()
        .appname("Rice")
        .summary(summary)
        .body(body)
        .icon("dialog-error")
        .hint(Hint::Category("transfer.error".to_string()))
        .urgency(Urgency::Critical)
        .show()
        .unwrap();
}
