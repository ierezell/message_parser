use chrono::prelude::{NaiveDateTime, Timelike};

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Serialize, Deserialize)]
struct FacebookGif {
    uri: String,
}

#[derive(Serialize, Deserialize)]
struct FacebookShare {
    share_text: String,
    link: String,
}

#[derive(Serialize, Deserialize)]
struct FacebookReaction {
    reaction: String,
    actor: String,
}

#[derive(Serialize, Deserialize)]
struct FacebookPhoto {
    uri: String,
    creation_timestamp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct FacebookMessage {
    sender_name: String,
    timestamp_ms: i64,
    content: Option<String>,
    photos: Option<Vec<FacebookPhoto>>,
    reactions: Option<Vec<FacebookReaction>>,
    shares: Option<FacebookShare>,
    gif: Option<Vec<FacebookGif>>,
}

#[derive(Serialize, Deserialize)]
pub struct FacebookParticipant {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct FacebookMessenger {
    pub participants: Vec<FacebookParticipant>,
    pub messages: Vec<FacebookMessage>,
}

pub fn get_counts(
    messages: &Vec<FacebookMessage>,
    participants: &HashSet<String>,
) -> (
    HashMap<String, i32>,
    HashMap<String, i32>,
    HashMap<String, Vec<u32>>,
) {
    let mut msg_count: HashMap<String, i32> = HashMap::new();
    let mut reaction_count = HashMap::new();
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

        if let Some(reactions) = &msg.reactions {
            for reaction in reactions {
                if reaction.actor != sender {
                    match reaction_count.get(&reaction.actor) {
                        Some(count) => {
                            reaction_count.insert(reaction.actor.to_string(), count + 1);
                        }
                        None => {
                            reaction_count.insert(reaction.actor.to_string(), 1);
                        }
                    }
                }
            }
        }
    }

    return (msg_count, reaction_count, date_count);
}

// use std::path::{Path, PathBuf};
// pub fn friend_msg_finder(folder: &String, name: &str) -> Vec<PathBuf> {
//     let mut correct_path: Vec<PathBuf> = [].to_vec();

//     let files_path = Path::new(folder);
//     let paths = files_path.read_dir().expect("Couldn't read directory");

//     for folder_path in paths {
//         let friend_path = folder_path
//             .expect("Cannot read file")
//             .file_name()
//             .to_str()
//             .expect("Cannot read file")
//             .to_owned();

//         let absolute = files_path.join(friend_path.clone());

//         if friend_path.contains(name) {
//             let mut mes_idx: i32 = 1;
//             loop {
//                 let msg_json_path = absolute.join(format!("message_{mes_idx}.json").as_str());

//                 println!("Friend path {:?} ", msg_json_path);
//                 if msg_json_path.exists() {
//                     mes_idx += 1;
//                     println!("{:?} ", msg_json_path);
//                     correct_path.push(msg_json_path);
//                 } else {
//                     break;
//                 }
//             }
//             break;
//         }
//     }
//     return correct_path;
// }
