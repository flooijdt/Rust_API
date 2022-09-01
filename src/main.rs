use csv::{self, Reader};
use reqwest::blocking::Response;
use serde::{de::IntoDeserializer, Deserialize, Deserializer, Serialize};
use serde_json::{Map, Value, Value::Object, json};
use std::vec::Vec;
use warp::{Filter, Rejection, Reply, http::StatusCode, reject::Reject, http::Method, filters::{body::BodyDeserializeError, cors::CorsForbidden}, query};
use tokio::{sync::RwLock, task};
use std::sync::Arc;
use std::collections::HashMap;
use reqwest::blocking::Client as ClientDl;
mod client;
mod error;
mod storage;
mod route;
use crate::client::{Dob, Location, Location2, LocationCoordinates, ClientId, Client, ClientCSV, Coordinates, ClientUnited, Timezone, Picture, Registered, Name};
use crate::error::Error;
use crate::error::return_error;
use crate::storage::{Storage, get_storage};
use crate::route::{get_clients, update_client, add_client, delete_client};


#[tokio::main]
async fn main() {
/* Creates cors filter */
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    // let mut storage = Storage::new();
    let storage = get_storage();

    let storage_filter = warp::any().map(move || storage.clone());

    let get_clients = warp::get()
        .and(warp::path("clients"))
        .and(warp::path::end())
        .and(query())
        .and(storage_filter.clone())
        .and_then(get_clients);


    let add_client = warp::post()
        .and(warp::path("clients"))
        .and(warp::path::end())
        .and(storage_filter.clone())
        .and(warp::body::json())
        .and_then(add_client);
 

    let update_client = warp::put()
        .and(warp::path("clients"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(storage_filter.clone())
        .and(warp::body::json())
        .and_then(update_client);
 
    let delete_client = warp::delete()
        .and(warp::path("clients"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(storage_filter.clone())
        .and_then(delete_client);


    let routes = get_clients.or(update_client).or(add_client).or(delete_client).with(cors).recover(return_error);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

