use warp::Filter;

#[tokio::main]
async fn main() {
    let route = warp::path!("wharever" / String).map(|param: String| format!("whareva {}", param));
    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
}
