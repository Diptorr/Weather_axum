use chrono::prelude::*;
use axum::{routing::get, Router};
use rand::*;
use axum::response::{Json};

struct  Weather {
    time_date : DateTime<Local>,
    temp : i32,
    sky : String,

}

#[tokio::main]
async fn main() {
    //make them routes
    let app: Router = Router::new()
    .route("/",get(hello_ferris))
    .route("/time",get(time))
    .route("/wheater",get(weather));

    //host em
    axum::Server::bind(&"0.0.0.0:8686".parse().unwrap()).serve(app.into_make_service())
    .await
    .unwrap()
    ;
}
//say hi to ferris on route /
async fn hello_ferris() -> String {
    "Hello, Ferris!".to_owned()
}
//get the local time on route /time
async fn time() -> DateTime<Local> {
    let time_date: DateTime<Local> = Local::now();
    time_date.to_owned()
}
//get the definnatly not made up waether
async fn weather() -> Json<Vec<Weather>> {
    let skys = ["Snow", "Raining", "SUN", "Heres k tabuly"];
    let mut weather_vec:Vec<Weather> = Vec::new();
        for x in 1..=5 {
            let wheat = Weather {
                time_date: Local::now(),
                temp : thread_rng().gen_range(-52..100),
                sky : skys[thread_rng().gen_range(0..skys.len())].to_string()
            };  //Hate this
            weather_vec.push(wheat);
        }
    Json(weather_vec)
}
