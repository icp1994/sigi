use crate::sigi::actions::Action;
use crate::sigi::items::Items;
use std::error::Error;
use std::io::ErrorKind;
use std::{env, fs, path::Path};

// TODO: Alternate backends? Right now it's just JSON files. Maybe SQLite? PilaDB?
//       Something new? Daemon(s) could prevent many loads and handle locking.
// TODO: Configurable data location?
// TODO: Allow an idea of "stack of stacks"
// TODO: Figure out a good naming algorithm (maybe numeric?)

// TODO: Custom error. This is returning raw filesystem errors.
pub fn save(action: &Action, items: Items) -> Result<(), impl Error> {
    let data_path: String = sigi_file(&action.topic);
    let json: String = serde_json::to_string(&items).unwrap();
    let result = fs::write(&data_path, &json);
    if result.is_err() && result.as_ref().unwrap_err().kind() == ErrorKind::NotFound {
        fs::create_dir_all(sigi_path()).unwrap();
        fs::write(data_path, json)
    } else {
        result
    }
}

// TODO: Custom error. This is returning raw serialization errors.
pub fn load(action: &Action) -> Result<Items, impl Error> {
    let data_path: String = sigi_file(&action.topic);
    let read_result = fs::read_to_string(data_path);
    if read_result.is_err() && read_result.as_ref().unwrap_err().kind() == ErrorKind::NotFound {
        Ok(vec![])
    } else {
        let json = read_result.unwrap();
        serde_json::from_str(&json)
    }
}

fn sigi_path() -> String {
    let home = env::var("HOME").or_else(|_| env::var("HOMEDRIVE")).unwrap();
    let path = format!("{}/.local/share/sigi", home);
    Path::new(&path).to_string_lossy().to_string()
}

fn sigi_file(filename: &str) -> String {
    let path = format!("{}/{}.json", sigi_path(), filename);
    Path::new(&path).to_string_lossy().to_string()
}