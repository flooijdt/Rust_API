use warp::{Filter, http::Method, query};
mod client;
mod error;
mod storage;
mod route;
use crate::error::return_error;
use crate::storage::get_storage;
use crate::route::{get_clients, update_client, add_client, delete_client};

/** Starts a server via the warp::serve() function on the designated port, currently: localhost/3030;
The warp::path() function receives a String with the adress of the desired path, currently: /clients. */
#[tokio::main]
async fn main() {
    /* Creates cors filter. */
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    /* Gets Clients data from juntos server into the storage variable. */
    let storage = get_storage().await;
    /* Creates a filter for manipulating storage. */
    let storage_filter = warp::any().map(move || storage.clone());
    /* Creates a filter for managing GET Requests for storage data. */
    let get_clients = warp::get()
        /* Serves the filter at the "/clients" path. */
        .and(warp::path("clients"))
        /* Ends the path with a "/". */
        .and(warp::path::end())
        /* Receives pagination queries in the form of a Hashmap<String> via the up designated path. e.g. "/clients?start=3&end=56". */
        .and(query())
        /* Clones the storage so it doesn`t need to be "moved". */
        .and(storage_filter.clone())
        .and_then(get_clients);

    /* Creates a filter for managing POST Requests. */
    let add_client = warp::post()
        .and(warp::path("clients"))
        .and(warp::path::end())
        .and(storage_filter.clone())
        /* Receives th Client to be added in json. */
        .and(warp::body::json())
        .and_then(add_client);


    /* Creates a filter for managing PUT Requests. */
    let update_client = warp::put()
        .and(warp::path("clients"))
        /* Receive parameters via the path and parse them as Strings */
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(storage_filter.clone())
        .and(warp::body::json())
        .and_then(update_client);

    /* Creates a filter for managing DELETE Requests. */
    let delete_client = warp::delete()
        .and(warp::path("clients"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(storage_filter.clone())
        .and_then(delete_client);

    /* Creates route to be served by combining all previous filters plus the error management module. */
    let routes = get_clients.or(update_client).or(add_client).or(delete_client).with(cors).recover(return_error);

    /* Starts server on the below designated port. */
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

