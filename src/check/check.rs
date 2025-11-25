use dmfr::*;
use serde_json::Error as SerdeError;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let feed_entries = fs::read_dir(format!("feeds/"))?;

    let mut is_err = false;

    let mut url_to_feed_ids: HashMap<String, HashSet<String>> = HashMap::new();
    let mut feed_id_to_file_name: HashMap<String, String> = HashMap::new();

    for entry in feed_entries {
        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().to_str() {
                let contents = fs::read_to_string(format!("feeds/{}", &file_name));
                if contents.is_err() {
                    eprintln!(
                        "Error Reading Feed File {}: {:#?}",
                        file_name,
                        contents.unwrap_err()
                    );
                    continue;
                }
                let dmfrinfo: Result<dmfr::DistributedMobilityFeedRegistry, SerdeError> =
                    serde_json::from_str(&contents.unwrap());

                if let Err(dmfrerr) = &dmfrinfo {
                    eprintln!("Error in {}, {:?}", file_name, dmfrerr);

                    is_err = true;
                }

                if let Ok(dmfrinfo) = &dmfrinfo {
                    for feed in &dmfrinfo.feeds {
                        if feed.spec == dmfr::FeedSpec::GtfsRt {
                            if feed.urls.static_current.is_some() {
                                eprintln!(
                                    "{}, Spec is GtfsRt but static_current is some",
                                    &file_name
                                );
                            }
                        }

                        feed_id_to_file_name.insert(feed.id.clone(), file_name.to_string());

                        for url in &feed.urls.static_current {
                            if feed.urls.static_current.is_some() {
                                let static_current =
                                    feed.urls.static_current.as_ref().unwrap().deref();
                                match url_to_feed_ids.entry(static_current.clone()) {
                                    std::collections::hash_map::Entry::Occupied(mut oe) => {
                                        let oe_mutable = oe.get_mut();

                                        oe_mutable.insert(static_current.clone());
                                    }
                                    std::collections::hash_map::Entry::Vacant(ve) => {
                                        ve.insert(HashSet::from([static_current.clone()]));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    
    for (url, feed_ids) in &url_to_feed_ids {
        if feed_ids.len() > 1 {
            println!("{:?} are sharing {}", feed_ids, url);
        }
    }

    match is_err {
        true => Err(Box::new(std::io::Error::from(
            std::io::ErrorKind::InvalidData,
        ))),
        false => Ok(()),
    }
}
