pub mod data_handling_lib;
pub mod open_and_save_files;
pub mod webserver;
use webserver::warp_webserver;

#[tokio::main]
async fn main() {
    warp_webserver().await;
}

/*
program panics if it is not run from pickleball_divisons/src
If you get 405 error it might be because you need to be in src when running the program
*/
