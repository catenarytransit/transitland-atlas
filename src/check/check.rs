use dmfr::*;
use serde_json::Error as SerdeError;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let feed_entries = fs::read_dir(format!("feeds/"))?;

    let mut is_err = false;

    for entry in feed_entries {
        if let Ok(entry) = entry {
            if let Some(file_name) = entry.file_name().to_str() {
                let contents = fs::read_to_string(format!("feeds/{}", file_name));
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
            }
        }
    }

    match is_err {
        true => Err(Box::new(std::io::Error::from(
            std::io::ErrorKind::InvalidData,
        ))),
        false => Ok(()),
    }
}
