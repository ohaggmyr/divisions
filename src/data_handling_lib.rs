use crate::open_and_save_files::{
    load_log_vec, player_data_db_hash_map_to_json_file, player_data_from_file_or_default,
    to_json_file_log,
};
use serde::{Deserialize, Serialize};
use warp::http::StatusCode;

/* Use to send data from web to backend. */
#[derive(Deserialize)]
pub struct Player {
    pub username: Option<String>,
    pub winner: Option<String>,
    pub loser: Option<String>,
    pub points_loser: u32,
}

pub async fn register_result_singles(p: Player) -> Result<impl warp::Reply, warp::Rejection> {
    register_result(p).await
}

pub fn vec_contains_id(v: &[PlayerData], id: &str) -> bool {
    for p in v {
        if p.username == id {
            return true;
        }
    }
    false
}

pub async fn register_result(result_data: Player) -> Result<impl warp::Reply, warp::Rejection> {
    if result_data.winner.clone().unwrap() == result_data.loser.clone().unwrap() {
        //Return asap if the two usernames entered are identical.
        return Ok(StatusCode::BAD_REQUEST);
    }

    //these files are generated but need to exist from the beginning, since
    //it otherwise the standings page would be empty
    let filename_div1 = "./data_files/table_div1.json";
    let filename_div2 = "./data_files/table_div2.json";
    let filename_div3 = "./data_files/table_div3.json";
    let filename_div4 = "./data_files/table_div4.json";
    let mut db_t_1 = match player_data_from_file_or_default(filename_div1) {
        Ok(cxt) => cxt,
        Err(e) => {
            /* should not end up here if from_file_or_default() works as expected  */
            println!("Error with from_file_or_default: {}", e);
            return Ok(StatusCode::BAD_REQUEST);
        }
    };
    let mut db_t_2 = match player_data_from_file_or_default(filename_div2) {
        Ok(cxt) => cxt,
        Err(e) => {
            /* should not end up here if from_file_or_default() works as expected  */
            println!("Error with from_file_or_default: {}", e);
            return Ok(StatusCode::BAD_REQUEST);
        }
    };
    let mut db_t_3 = match player_data_from_file_or_default(filename_div3) {
        Ok(cxt) => cxt,
        Err(e) => {
            /* should not end up here if from_file_or_default() works as expected  */
            println!("Error with from_file_or_default: {}", e);
            return Ok(StatusCode::BAD_REQUEST);
        }
    };

    let mut db_t_4 = match player_data_from_file_or_default(filename_div4) {
        Ok(cxt) => cxt,
        Err(e) => {
            /* should not end up here if from_file_or_default() works as expected  */
            println!("Error with from_file_or_default: {}", e);
            return Ok(StatusCode::BAD_REQUEST);
        }
    };

    //safe unwraps of options
    if vec_contains_id(&db_t_1, &result_data.winner.clone().unwrap())
        && vec_contains_id(&db_t_1, &result_data.loser.clone().unwrap())
    {
        register_result_in_a_specific_division(result_data, &mut db_t_1, filename_div1);
    } else if vec_contains_id(&db_t_2, &result_data.winner.clone().unwrap())
        && vec_contains_id(&db_t_2, &result_data.loser.clone().unwrap())
    {
        register_result_in_a_specific_division(result_data, &mut db_t_2, filename_div2);
    } else if vec_contains_id(&db_t_3, &result_data.winner.clone().unwrap())
        && vec_contains_id(&db_t_3, &result_data.loser.clone().unwrap())
    {
        register_result_in_a_specific_division(result_data, &mut db_t_3, filename_div3);
    } else if vec_contains_id(&db_t_4, &result_data.winner.clone().unwrap())
        && vec_contains_id(&db_t_4, &result_data.loser.clone().unwrap())
    {
        register_result_in_a_specific_division(result_data, &mut db_t_4, filename_div4);
    } else {
        println!("Bad request: there is no player with that name");
        return Ok(StatusCode::BAD_REQUEST);
    }
    Ok(StatusCode::OK)
}
//https://stackoverflow.com/questions/62771576/how-do-i-save-structured-data-to-file
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlayerData {
    pub wins: u32,
    pub sets_won: u32,
    pub sets_lost_negative: i32,
    pub username: String,
    pub name: String,
    pub losses: u32,
}

fn register_result_in_a_specific_division(
    result_data: Player,
    db_t: &mut Vec<PlayerData>,
    filename_data_file: &str,
) {
    let username_of_winner = result_data.winner.clone().unwrap();
    let username_of_loser = result_data.loser.clone().unwrap();

    // i står för instance

    let mut winner_wins = 0;
    let mut winner_losses = 0;
    let mut winner_sets_won = 0;
    let mut winner_sets_lost_negative = 0;
    let mut loser_wins = 0;
    let mut loser_losses = 0;
    let mut loser_sets_won = 0;
    let mut loser_sets_lost_negative = 0;

    let mut db_copy = db_t.clone();
    for i in &mut db_copy {
        if username_of_winner == i.username {
            winner_wins = i.wins;
            winner_losses = i.losses;
            winner_sets_won = i.sets_won;
            winner_sets_lost_negative = i.sets_lost_negative;
            i.wins += 1;
            i.sets_won += 2;
            i.sets_lost_negative -= result_data.points_loser as i32;
        }
        if username_of_loser == i.username {
            loser_wins = i.wins;
            loser_losses = i.losses;
            loser_sets_won = i.sets_won;
            loser_sets_lost_negative = i.sets_lost_negative;
            i.losses += 1;
            i.sets_won += result_data.points_loser;
            i.sets_lost_negative -= 2;
        }
    }

    let new_match_log_row = GameLogRow {
        winner: username_of_winner,
        loser: username_of_loser,
        winner_wins,
        winner_losses,
        winner_sets_won,
        winner_sets_lost_negative,
        loser_wins,
        loser_losses,
        loser_sets_won,
        loser_sets_lost_negative,
        time_registered: get_current_time_as_string(),
    };

    /* Update the the list of all game results. */
    add_result_to_match_log(new_match_log_row);

    /* Update the player database.*/
    db_copy.sort_by(|a, b| b.cmp(a));
    player_data_db_hash_map_to_json_file(&db_copy, filename_data_file);
}
#[derive(Serialize, Deserialize)]
pub struct GameLogRow {
    winner: String,
    loser: String,
    winner_wins: u32,
    winner_losses: u32,
    winner_sets_won: u32,
    winner_sets_lost_negative: i32,
    loser_wins: u32,
    loser_losses: u32,
    loser_sets_won: u32,
    loser_sets_lost_negative: i32,
    time_registered: String,
}

use chrono::{DateTime, Local};
pub fn add_result_to_match_log(row: GameLogRow) {
    //this unwrap is safe
    let file = "./data_files/match_log.json";
    let mut log = load_log_vec(file).unwrap();
    log.insert(0, row); //om detta blev bakvänt så använder jag bara push istället

    //this unwrap is safe
    to_json_file_log(&log, file.to_string()).unwrap();
}

pub fn get_current_time_as_string() -> String {
    let dt: DateTime<Local> = Local::now(); //the rime right now
    let mut sdt = DateTime::to_rfc3339(&dt); //convert the time to a String
    sdt.truncate(19); // remove stuff from the end of the string that we don't want
    let (_, sdt2) = sdt.split_at(11); //remove stuff from the end of the string that we don't want
    sdt2.to_string()
}
