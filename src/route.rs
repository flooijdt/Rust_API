use serde_json::Value;
use std::vec::Vec;
use warp::{Rejection, http::StatusCode};
use tokio::task;
use std::collections::HashMap;
use reqwest::blocking::Client as ClientDl;
use crate::client::{Dob, Location, Location2, LocationCoordinates, ClientId, Client, ClientCSV, Coordinates, ClientUnited, Timezone, Picture, Registered, Name};
use crate::error::Error;
use crate::storage::Storage;


pub async fn get_clients(params: HashMap<String, String>, mut storage: Storage) -> Result<warp::reply::Json, Rejection>{

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

    let mut json2  = csv::Reader::from_reader(resp2.as_bytes());
    // convert ClientCSV to Client struct.
    for result in json2.deserialize::<Vec<ClientCSV>>() {
        for result in result.unwrap() {
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

