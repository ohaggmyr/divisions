use crate::data_handling_lib::{GameLogRow, PlayerData};
use atomicwrites::*;

trait Stringify {
    fn stringify(&self) -> String;
}

impl<T> Stringify for T
where
    T: std::fmt::Debug,
{
    fn stringify(&self) -> String {
        format!("{:?}", self)
    }
}

pub fn load_log_vec(file: &str) -> Result<Vec<GameLogRow>, String> {
    let file = std::fs::File::open(file);
    match file {
        Ok(file) => serde_json::from_reader(file).map_err(|e| e.stringify()),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                let tmp: Vec<GameLogRow> = Vec::new();
                Ok(tmp)
            }
            _ => Err(e.stringify()),
        },
    }
}

pub fn to_json_file_log(log: &[GameLogRow], file: String) -> Result<(), String> {
    let afile = AtomicFile::new(file, OverwriteBehavior::AllowOverwrite);
    afile
        .write(|file| serde_json::to_writer(file, &log))
        .map_err(|e| e.stringify())
}

pub fn player_data_from_file_or_default(file: &str) -> Result<Vec<PlayerData>, String> {
    let file = std::fs::File::open(file);
    match file {
        Ok(file) => serde_json::from_reader(file).map_err(|e| e.stringify()),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => {
                let tmp: Vec<PlayerData> = Vec::new();
                Ok(tmp)
            }
            _ => Err(e.stringify()),
        },
    }
}

pub fn player_data_db_hash_map_to_json_file(db: &[PlayerData], file: &str) {
    let afile = AtomicFile::new(file, OverwriteBehavior::AllowOverwrite);
    let res = afile
        .write(|file| serde_json::to_writer(file, &db))
        .map_err(|e| e.stringify());
    match res {
        Ok(_) => {}
        Err(e) => println!("err: {:?}", e),
    };
}
