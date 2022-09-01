use std::vec::Vec;
use warp::{Rejection, http::StatusCode};
use std::collections::HashMap;
use crate::client::{ClientId, Client};
use crate::error::Error;
use crate::storage::Storage;


pub async fn get_clients(params: HashMap<String, String>, mut storage: Storage) -> Result<warp::reply::Json, Rejection>{

    /* Applying pagination parameters provided by query*/
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        let res: Vec<Client> = storage.clients.read().await.values().cloned().collect();
        let res = &res[pagination.start..pagination.end];
        return Ok(warp::reply::json(&res));
    } else {
        let res: Vec<Client> = storage.clients.read().await.values().cloned().collect();
        return Ok(warp::reply::json(&res));
    }

    #[derive(Debug)]
    struct Pagination {
        start: usize,
        end: usize,
    }

    fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
        if params.contains_key("start") && params.contains_key("end") {
            return Ok(Pagination {
                start: params
                    .get("start")
                    .unwrap()
                    .parse::<usize>()
                    .map_err(Error::ParseError)?,
                end: params
                    .get("end")
                    .unwrap()
                    .parse::<usize>()
                    .map_err(Error::ParseError)?,
            });
        }
     
        Err(Error::MissingParameters)
    }

    let res: Vec<Client> = storage.clients.read().await.values().cloned().collect();

    Ok(warp::reply::json(&res))
}

pub async fn add_client(storage: Storage, client: Client) -> Result<impl warp::Reply, warp::Rejection> {
    storage.clients.write().await.insert(client.id.clone(), client);
 
    Ok(warp::reply::with_status(
        "Client added",
        StatusCode::OK,
    ))
}

pub async fn update_client(id: String, storage: Storage, client: Client) -> Result<impl warp::Reply, warp::Rejection> {
    match storage.clients.write().await.get_mut(&ClientId(id)) {
        Some(c) => *c = client,
        None => return Err(warp::reject::custom(Error::ClientNotFound)),
    }
 
    Ok(warp::reply::with_status(
        "Client updated",
        StatusCode::OK,
    ))
}

pub async fn delete_client(
    id: String,
    storage: Storage,
) -> Result<impl warp::Reply, warp::Rejection> {
    match storage.clients.write().await.remove(&ClientId(id)) {
        Some(_) => Ok(warp::reply::with_status("Client deleted", StatusCode::OK)),
        None => Err(warp::reject::custom(Error::ClientNotFound)),
    }
}

