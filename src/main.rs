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
use crate::client::{Dob, Location, Location2, LocationCoordinates, ClientId, Client, ClientCSV, Coordinates, ClientUnited, Timezone, Picture, Registered, Name};
use crate::error::Error;
use crate::error::return_error;
use crate::storage::Storage;


async fn get_clients(params: HashMap<String, String>, mut storage: Storage) -> Result<warp::reply::Json, Rejection>{

    // get Response containing user data from source.
    let resp: String = task::spawn_blocking(|| {
    // do some compute-heavy work or call synchronous code
        let client = ClientDl::new();
        client.get("https://storage.googleapis.com/juntossomosmais-code-challenge/input-backend.json").send().unwrap().text().unwrap()
    }).await.unwrap();

    let resp2: String = task::spawn_blocking(|| { 
        let client = ClientDl::new();
        client.get("https://storage.googleapis.com/juntossomosmais-code-challenge/input-backend.csv").send().unwrap().text().unwrap()
    }).await.unwrap();
    

    // convert Response to json.
    let mut json: Value = serde_json::from_str(resp.as_str()).unwrap();
    // create list with for Client structs.
    let mut json_clients_list: Vec<ClientUnited> = Vec::new();
    // clone json as an array for iteration.
    // let mut json_array = json.clone();

    // println!("{:?}", &json_array);//-----------------------------------------does not print

    // let mut json2: Value = resp2.into();//-------------------------------------o problema aqui é que esses dados sao CSV - será que é por isso que nao sao iteraveis??
    // println!("{:?}", &json["results"]);//---------------------------- still prints!
    // iterate json_array in order to fill json_clients_list.
    // let mut json_array = json.as_object_mut();

    for object in json["results"].as_array_mut().iter() {
        // println!("{:?}", &object);
        for objectling in object.iter() {
            // println!("antes objectling");
            // println!("{:?}", &objectling);
            // println!("depois objectling");
            let client = ClientUnited::new(objectling.clone());
            json_clients_list.push(client);
        }
    }
    // println!("{:?}", &json_clients_list);
    // get csv containing user data from source.
    // let mut json2: Value = resp2.into();//-------------------------------------o problema aqui é que esses dados sao CSV - será que é por isso que nao sao iteraveis??
    // convert response to Reader, for file tempering.
    // let mut json2  = csv::Reader::from_reader(response_csv);

    let mut json2  = csv::Reader::from_reader(resp2.as_bytes());
    // println!("{:?}", &json2);// -----------------------------------------------até aqui (json2) print os customers.
    // convert ClientCSV to Client struct.
    for result in json2.deserialize::<Vec<ClientCSV>>() {
        // println!("{:?}", &result);
    // 
        for result in result.unwrap() {
    //         // println!("{:?}", &result);
            // println!("{:?}", &result);    
            // let mut result = ClientCSV::new(result);
            // println!("{:?}", &result);   
    
            // result = result.clone();
            let mut result: ClientUnited = ClientUnited {
                cell: result.cell,
                dob: Dob {
                    age: result.dob__age,
                    date: result.dob__date,
                },
                email: result.email,
                gender: result.gender,
                location: Location {
                    city: result.location__city,
                    coordinates: Coordinates {
                        latitude: result.location__coordinates__latitude.to_string(),
                        longitude: result.location__coordinates__longitude.to_string(),
                    },
                    postcode: result.location__postcode,
                    state: result.location__state,
                    street: result.location__street,
                    timezone: Timezone {
                        description: result.location__timezone__description,
                        offset: result.location__timezone__offset,
                    },
                },
                name: Name {
                    first: result.name__first,
                    last: result.name__last,
                    title: result.name__title,
                },
                phone: result.phone,
                picture: Picture {
                    large: result.picture__large,
                    medium: result.picture__medium,
                    thumbnail: result.picture__thumbnail,
                },
                registered: Registered {
                    age: result.registered__age,
                    date: result.registered__date,
                },
            };
            // println!("{:#?}", &result);
        json_clients_list.push(result);
    }}
    // println!("{:?}", &json_clients_list);
    // create final Client struct according to desired output.
    let special1 = LocationCoordinates {
        minlon: -2.196998,
        minlat: -46.361899,
        maxlon: -15.411580,
        maxlat: -34.276938,
    };

    let special2 = LocationCoordinates {
        minlon: -19.766959,
        minlat: -52.997614,
        maxlon: -23.966413,
        maxlat: -44.428305,
    };

    let normal = LocationCoordinates {
        minlon: -26.155681,
        minlat: -54.777426,
        maxlon: -34.016466,
        maxlat: -46.603598,
    };

    let mut id_counter = 0;

    for client in json_clients_list.iter() {        
        let client = client.clone();
        let mut client: Client = Client {
            id: ClientId(String::from("placeholder")),
            r#type: String::from("placeholder"),
            gender: client.gender,
            name: Name {
                title: client.name.title,
                first: client.name.first,
                last: client.name.last,
            },
            location: Location2 {
                region: String::from("placeholder"),
                street: client.location.street,
                city: client.location.city,
                state: client.location.state,
                postcode: client.location.postcode,
                coordinates: Coordinates {
                    latitude: client.location.coordinates.latitude,
                    longitude: client.location.coordinates.longitude,
                },
                timezone: Timezone {
                    offset: client.location.timezone.offset,
                    description: client.location.timezone.description,
                },
            },
            email: client.email,
            birthday: client.dob.date,
            registered: client.registered.date,
            telephoneNumbers: vec![client.phone],
            mobileNumbers: vec![client.cell],
            picture: Picture {
                large: client.picture.large,
                medium: client.picture.medium,
                thumbnail: client.picture.thumbnail,
            },
            nationality: String::from("BR"),
        };

        if client.gender == String::from("male") {
            client.gender = String::from("m");
        } else if client.gender == String::from("female") {
            client.gender = String::from("f");
        }

        if client.location.state == "rio grande do sul"
            || client.location.state == "santa catarina"
            || client.location.state == "paraná"
        {
            client.location.region = String::from("sul");
        } else if client.location.state == "espírito santo"
            || client.location.state == "rio de janeiro"
            || client.location.state == "minas gerais"
            || client.location.state == "são paulo"
        {
            client.location.region = String::from("sudeste");
        } else if client.location.state == "mato grosso"
            || client.location.state == "mato grosso do sul"
            || client.location.state == "goiás"
            || client.location.state == "distrito federal"
        {
            client.location.region = String::from("centro-oeste");
        } else if client.location.state == "acre"
            || client.location.state == "amazonas"
            || client.location.state == "rondônia"
            || client.location.state == "amapá"
            || client.location.state == "roraima"
            || client.location.state == "pará"
            || client.location.state == "tocantins"
        {
            client.location.region = String::from("norte");
        } else if client.location.state == "bahia"
            || client.location.state == "sergipe"
            || client.location.state == "alagoas"
            || client.location.state == "paraíba"
            || client.location.state == "pernambuco"
            || client.location.state == "rio grande do norte"
            || client.location.state == "ceará"
            || client.location.state == "piauí"
            || client.location.state == "maranhão"
        {
            client.location.region = String::from("nordeste");
        }

        if special1.minlat <= client.location.coordinates.latitude.parse::<f64>().unwrap()
            && client.location.coordinates.latitude.parse::<f64>().unwrap() <= special1.maxlat
            && special1.minlon
                <= client
                    .location
                    .coordinates
                    .longitude
                    .parse::<f64>()
                    .unwrap()
            && client
                .location
                .coordinates
                .longitude
                .parse::<f64>()
                .unwrap()
                <= special1.maxlon
        {
            client.r#type = String::from("special");
        } else if special2.minlat <= client.location.coordinates.latitude.parse::<f64>().unwrap()
            && client.location.coordinates.latitude.parse::<f64>().unwrap() <= special2.maxlat
            && special2.minlon
                <= client
                    .location
                    .coordinates
                    .longitude
                    .parse::<f64>()
                    .unwrap()
            && client
                .location
                .coordinates
                .longitude
                .parse::<f64>()
                .unwrap()
                <= special2.maxlon
        {
            client.r#type = String::from("special");
        } else if normal.minlat <= client.location.coordinates.latitude.parse::<f64>().unwrap()
            && client.location.coordinates.latitude.parse::<f64>().unwrap() <= normal.maxlat
            && normal.minlon
                <= client
                    .location
                    .coordinates
                    .longitude
                    .parse::<f64>()
                    .unwrap()
            && client
                .location
                .coordinates
                .longitude
                .parse::<f64>()
                .unwrap()
                <= normal.maxlon
        {
            client.r#type = String::from("normal");
        } else {
            client.r#type = String::from("labourious");
        }
        // correct phone format.
        client.telephoneNumbers[0] = client.telephoneNumbers[0].replace("(", "");
        client.telephoneNumbers[0] = client.telephoneNumbers[0].replace(")", "");
        client.telephoneNumbers[0] = client.telephoneNumbers[0].replace(" ", "");
        client.telephoneNumbers[0] = client.telephoneNumbers[0].replace("-", "");
        let mut brcode: String = String::from("+55");
        brcode.push_str(client.telephoneNumbers[0].clone().as_str());
        client.telephoneNumbers[0] = brcode;
        // correct mobile numbers.
        client.mobileNumbers[0] = client.mobileNumbers[0].replace("(", "");
        client.mobileNumbers[0] = client.mobileNumbers[0].replace(")", "");
        client.mobileNumbers[0] = client.mobileNumbers[0].replace(" ", "");
        client.mobileNumbers[0] = client.mobileNumbers[0].replace("-", "");
        let mut brcode: String = String::from("+55");
        brcode.push_str(client.mobileNumbers[0].clone().as_str());
        client.mobileNumbers[0] = brcode;

        client.id = ClientId(id_counter.to_string());

        // println!("{:#?}", &client);
        storage.clients.write().await.insert(client.id.clone(), client);

        id_counter += 1;
    }
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




    let exclientid: ClientId = ClientId(String::from("34"));
    // println!("{:#?}", &storage.clients.read().await.get(&exclientid));
    let res: Vec<Client> = storage.clients.read().await.values().cloned().collect();

    // let res = &res[params.get("start").unwrap()..params.get("end").unwrap()];
    // println!("{:#?}", &res);
    // println!("{:#?}", params);
    Ok(warp::reply::json(&res))
}

async fn add_client(storage: Storage, client: Client) -> Result<impl warp::Reply, warp::Rejection> {
    storage.clients.write().await.insert(client.id.clone(), client);
 
    Ok(warp::reply::with_status(
        "Client added",
        StatusCode::OK,
    ))
}

async fn update_client(id: String, storage: Storage, client: Client) -> Result<impl warp::Reply, warp::Rejection> {
    match storage.clients.write().await.get_mut(&ClientId(id)) {
        Some(c) => *c = client,
        None => return Err(warp::reject::custom(Error::ClientNotFound)),
    }
 
    Ok(warp::reply::with_status(
        "Client updated",
        StatusCode::OK,
    ))
}

async fn delete_client(
    id: String,
    storage: Storage,
) -> Result<impl warp::Reply, warp::Rejection> {
    match storage.clients.write().await.remove(&ClientId(id)) {
        Some(_) => Ok(warp::reply::with_status("Client deleted", StatusCode::OK)),
        None => Err(warp::reject::custom(Error::ClientNotFound)),
    }
}


#[tokio::main]
async fn main() {
/* Creates cors filter */
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let mut storage = Storage::new();

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

