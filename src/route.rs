use std::vec::Vec;
use serde::de::IntoDeserializer;
use warp::http::response;
use warp::{Rejection, http::StatusCode};
use std::collections::HashMap;
use tracing::{instrument, info};
use crate::client::{ClientId, Client};
use crate::error::Error;
use crate::storage::{Storage, self};
use crate::get_response::{GetResponse, self};

// pub async fn get_clients(params: HashMap<String, String>, mut storage: Storage) -> Result<warp::reply::Json, Rejection>{
//     println!("{:#?}", params);
//     info!("Start querying questions");
//     /* Applies pagination parameters provided by query. */
//     if !params.is_empty() {
//         let pagination = extract_pagination(params)?;
//         info!(pagination = true);
//         let res: Vec<Client> = storage.clients.read().await.values().cloned().collect();
//         let res = &res[pagination.start..pagination.end];
//         return Ok(warp::reply::json(&res));
//     } else {
//         info!(pagination = false);
//         let res: Vec<Client> = storage.clients.read().await.values().cloned().collect();
//         return Ok(warp::reply::json(&res));
//     }


/** Implements GET function. */
// tenho que de algum modo avisar o usuário de que apenas os parametros type e region devem ser oferecidos e sao levados em consideração
#[instrument]
pub async fn get_clients(params: HashMap<String, String>, mut storage: Storage) -> Result<warp::reply::Json, Rejection>{
    info!("Start querying clients");
    /* Applies pagination parameters provided by query. */
    if !params.is_empty() {
        let clients_iter = storage.clients.read().await;
        let clients_iter = clients_iter.values().cloned();

        let mut clients_vec = Vec::<Client>::new();

        for client in clients_iter {
            let region = client.location.region.clone();
            let r#type = client.r#type.clone();
            // println!("{:?}", /*params.get("type").unwrap() ==*/ &client.r#type.clone());
            if params.get("type").expect("could not get type.") == &r#type && params.get("region").expect("could not get region.") == &region {
                clients_vec.push(client.clone());
            }
        }
        // println!("{:#?}", clients_vec);
        let mut res = clients_vec;
        /* Pagination data */ 
        let mut warp_response = GetResponse::new();

        warp_response.totalCount = res.len();

        if warp_response.totalCount <= 10 {
            warp_response.pageNumber = 1;
            warp_response.pageSize = warp_response.totalCount;
            warp_response.clients = res;
        } else if warp_response.totalCount > 10 {
            warp_response.pageNumber = 1;
            warp_response.pageSize = warp_response.totalCount;
            
            res.sort_by_key(|client| client.id.parse::<usize>().expect("Could not convert to usize."));
            warp_response.clients = res;
            
        }

        return Ok(warp::reply::json(&warp_response));
    } else {
        info!(params = false);
        let res: Vec<Client> = storage.clients.read().await.values().cloned().collect();
        return Ok(warp::reply::json(&res));
    }
 



    /** Creates a pagination struct in order to organize the incoming parameters. */
    #[derive(Debug)]
    struct Pagination {
        start: usize,
        end: usize,
    }

    /** Organizes the params into the `Pagination` struct. */
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
    /* Gets clients data and organizes it in a `Vector` for display. */
    let res: Vec<Client> = storage.clients.read().await.values().cloned().collect();
    /* Converts `Vector` into `JSON`. */
    Ok(warp::reply::json(&res))
}
/** Implements the POST function. */
pub async fn add_client(storage: Storage, client: Client) -> Result<impl warp::Reply, warp::Rejection> {
    storage.clients.write().await.insert(ClientId{ string: client.id.clone()}, client);

    Ok(warp::reply::with_status(
        "Client added",
        StatusCode::OK,
    ))
}
/** Implements the PUT function. */
pub async fn update_client(id: String, storage: Storage, client: Client) -> Result<impl warp::Reply, warp::Rejection> {
    match storage.clients.write().await.get_mut(&ClientId{string: id}) {
        Some(c) => *c = client,
        None => return Err(warp::reject::custom(Error::ClientNotFound)),
    }

    Ok(warp::reply::with_status(
        "Client updated",
        StatusCode::OK,
    ))
}
/** Implements the DELETE function. */
pub async fn delete_client(
    id: String,
    storage: Storage,
) -> Result<impl warp::Reply, warp::Rejection> {
    match storage.clients.write().await.remove(&ClientId{string: id}) {
        Some(_) => Ok(warp::reply::with_status("Client deleted", StatusCode::OK)),
        None => Err(warp::reject::custom(Error::ClientNotFound)),
    }
}

