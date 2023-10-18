use chrono::prelude::{NaiveDateTime, Timelike};

use leptos::logging;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WhatsappMessage {
    pub sender_name: String,
    pub timestamp_ms: i64,
    pub content: String,
}

pub fn parse_whatsapp(text: String) -> Vec<WhatsappMessage> {
    logging::log!("BANANA");
    let date_name_header_regex = Regex::new(
        r"^(?P<date>\d{2}\/\d{2}\/\d{4}\,\s\d{2}\:\d{2})\s-\s(?P<name>[\w\s]+)\:\s(?P<message>.+)$",
    )
    .unwrap();
    let mut messages = Vec::<WhatsappMessage>::new();
    let mut name = "";
    let mut timestamp = 0;
    let mut content = "".to_string();

    for msg_line in text.split("\n") {
        if date_name_header_regex.is_match(msg_line) {
            if name != "" {
                messages.push(WhatsappMessage {
                    sender_name: name.to_string(),
                    timestamp_ms: timestamp,
                    content: content,
                });
            }

            let captures = date_name_header_regex.captures(msg_line).unwrap();
            let date = captures.name("date").unwrap().as_str();
            timestamp = NaiveDateTime::parse_from_str(date, "%d/%m/%Y, %H:%M")
                .unwrap()
                .timestamp_millis();
            name = captures.name("name").unwrap().as_str();
            content = captures.name("message").unwrap().as_str().to_string();
        } else {
            let space = " ".to_string();
            let new_content = space + msg_line;
            content += new_content.as_str();
        }
    }

    if content != "" {
        messages.push(WhatsappMessage {
            sender_name: name.to_string(),
            timestamp_ms: timestamp,
            content: content,
        });
    }

    return messages;
}
pub fn get_counts(
    messages: &Vec<WhatsappMessage>,
    participants: &HashSet<String>,
) -> (HashMap<String, i32>, HashMap<String, Vec<u32>>) {
    let mut msg_count: HashMap<String, i32> = HashMap::new();
    let mut date_count: HashMap<String, Vec<u32>> = HashMap::new();
    for p in participants {
        date_count.insert(p.to_string(), vec![]);
    }

    for msg in messages {
        let sender = msg.sender_name.clone();
        match msg_count.get(&sender) {
            Some(count) => {
                msg_count.insert(sender.to_string(), count + 1);
            }
            None => {
                msg_count.insert(sender.to_string(), 1);
            }
        }

        if let Some(dates_for_user) = date_count.get_mut(&sender) {
            if let Some(datetime) = NaiveDateTime::from_timestamp_millis(msg.timestamp_ms) {
                dates_for_user.push(datetime.hour());
            }
        }
    }

    return (msg_count, date_count);
}
