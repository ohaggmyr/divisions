use crate::data_handling_lib::register_result_singles;

use serde::Serialize;
use warp::Filter;

pub async fn warp_webserver() {
    let register_result_singles = warp::post()
        .and(warp::path("register_result_singles"))
        .and(warp::body::json())
        .and_then(register_result_singles)
        .with(warp::reply::with::header("Cache-Control", " no-store"));
    let index = warp::get()
        .and(warp::path("pickleball_divisions"))
        .and(warp::fs::file("./html_files/index.html"))
        .with(warp::reply::with::header("Cache-Control", " no-store"));
    let get_table1_json = warp::get()
        .and(warp::path("table_div1"))
        .and(warp::fs::file("./data_files/table_div1.json"))
        .with(warp::reply::with::header("Cache-Control", " no-store"));
    let get_table2_json = warp::get()
        .and(warp::path("table_div2"))
        .and(warp::fs::file("./data_files/table_div2.json"))
        .with(warp::reply::with::header("Cache-Control", " no-store"));
    let get_table3_json = warp::get()
        .and(warp::path("table_div3"))
        .and(warp::fs::file("./data_files/table_div3.json"))
        .with(warp::reply::with::header("Cache-Control", " no-store"));
    let get_table4_json = warp::get()
        .and(warp::path("table_div4"))
        .and(warp::fs::file("./data_files/table_div4.json"))
        .with(warp::reply::with::header("Cache-Control", " no-store"));
    let json_match_log = warp::get()
        .and(warp::path("json_match_log"))
        .and(warp::fs::file("./data_files/match_log.json"))
        .with(warp::reply::with::header("Cache-Control", " no-store"));
    let match_history = warp::get()
        .and(warp::path("match_log"))
        .and(warp::fs::file("./html_files/match_log.html"))
        .with(warp::reply::with::header("Cache-Control", " no-store"));
    let standings_div1 = warp::get()
        .and(warp::path("standings_div1"))
        .and(warp::fs::file("./html_files/standings_div1.html"))
        .with(warp::reply::with::header("Cache-Control", " no-store"));
    let standings_div2 = warp::get()
        .and(warp::path("standings_div2"))
        .and(warp::fs::file("./html_files/standings_div2.html"))
        .with(warp::reply::with::header("Cache-Control", " no-store"));
    let standings_div3 = warp::get()
        .and(warp::path("standings_div3"))
        .and(warp::fs::file("./html_files/standings_div3.html"))
        .with(warp::reply::with::header("Cache-Control", " no-store"));
    let standings_div4 = warp::get()
        .and(warp::path("standings_div4"))
        .and(warp::fs::file("./html_files/standings_div4.html"))
        .with(warp::reply::with::header("Cache-Control", " no-store"));

    // requires modded warp
    let _get_generic_file = warp::path!("file" / String / "type" / String)
        .and(warp::get())
        .and(warp::fs::conditionals())
        .and_then(
            |username: String, file_type: String, conditionals| async move {
                warp::fs::serve_file_reply(
                    format!("./image_files/{}.{}", username, file_type),
                    conditionals,
                )
                .await
            },
        );
    let get_jpg_file = warp::path!("file" / String)
        .and(warp::get())
        .and(warp::fs::conditionals())
        .and_then(|username: String, conditionals| async move {
            warp::fs::serve_file_reply(format!("./image_files/{}.jpg", username), conditionals)
                .await
        });

    let routes = register_result_singles
        .boxed()
        .or(index)
        .boxed()
        .or(get_table1_json)
        .boxed()
        .or(get_table2_json)
        .boxed()
        .or(get_table3_json)
        .boxed()
        .or(get_table4_json)
        .boxed()
        .or(match_history)
        .boxed()
        .or(json_match_log)
        .boxed()
        .boxed()
        .or(standings_div1)
        .boxed()
        .or(standings_div2)
        .boxed()
        .or(standings_div3)
        .boxed()
        .or(standings_div4)
        .boxed()
        .or(get_jpg_file)
        .boxed();

    warp::serve(routes).run(([0, 0, 0, 0], 7001)).await;
}

#[derive(Serialize)]
struct Math {
    request: String,
    reply: String,
    rating_change: String,
}
