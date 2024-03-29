use csv::{self, Reader};
use reqwest::blocking::Response;
use serde::{de::IntoDeserializer, Deserialize, Deserializer, Serialize};
use serde_json::{json, Map, Value, Value::Object};
use std::collections::HashMap;
use std::vec::Vec;
use tokio::task::spawn_blocking;
use warp::{
    filters::cors::CorsForbidden, http::StatusCode, reject::Reject, Error, Filter, Rejection, Reply,
};
//comment
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Dob {
    age: u32,
    date: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Location {
    city: String,
    coordinates: Coordinates,
    postcode: u32,
    state: String,
    street: String,
    timezone: Timezone,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Coordinates {
    latitude: String,
    longitude: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Timezone {
    description: String,
    offset: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Name {
    first: String,
    last: String,
    title: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Picture {
    large: String,
    medium: String,
    thumbnail: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Registered {
    age: u32,
    date: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ClientUnited {
    cell: String,
    dob: Dob,
    email: String,
    gender: String,
    location: Location,
    name: Name,
    phone: String,
    picture: Picture,
    registered: Registered,
}

impl ClientUnited {
    fn new(value: Value) -> Self {
        let client: ClientUnited = serde_json::from_value(value).unwrap();
        client
    }
}

// convert response to Reader, for file tempering.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ClientCSV {
    gender: String,
    name__title: String,
    name__first: String,
    name__last: String,
    location__street: String,
    location__city: String,
    location__state: String,
    location__postcode: u32,
    location__coordinates__latitude: f64,
    location__coordinates__longitude: f64,
    location__timezone__offset: String,
    location__timezone__description: String,
    email: String,
    dob__date: String,
    dob__age: u32,
    registered__date: String,
    registered__age: u32,
    phone: String,
    cell: String,
    picture__large: String,
    picture__medium: String,
    picture__thumbnail: String,
}
// create final Client struct according to desired output.
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Client {
    id: ClientId,
    r#type: String,
    gender: String,
    name: Name,
    location: Location2,
    email: String,
    birthday: String,
    registered: String,
    telephoneNumbers: Vec<String>,
    mobileNumbers: Vec<String>,
    picture: Picture,
    nationality: String,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Location2 {
    region: String,
    city: String,
    coordinates: Coordinates,
    postcode: u32,
    state: String,
    street: String,
    timezone: Timezone,
}
// clasification in regard to coordinates: special, labourious or normal.

pub struct LocationCoordinates {
    minlon: f64,
    minlat: f64,
    maxlon: f64,
    maxlat: f64,
}

#[derive(Debug, Deserialize, Clone, Serialize, Eq, PartialEq, Hash)]
pub struct ClientId(String);

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Storage {
    clients: HashMap<ClientId, Client>,
}

impl Storage {
    fn new() -> Self {
        Storage {
            clients: HashMap::new(),
        }
    }

    fn add_client(mut self, client: Client) -> Self {
        self.clients.insert(client.id.clone(), client);
        self
    }
}
// impl warp::Reply for Storage {
//     fn into_response(self) -> warp::reply::Response {
//         Response::new(format!("{}", self.json).into())
//     }
// }
//

fn get_clients() -> Storage {
    // get Response containing user data from source.

    let response_json = reqwest::blocking::get(
        "https://storage.googleapis.com/juntossomosmais-code-challenge/input-backend.json",
    )
    .expect("unable to get the origin json.");

    // convert Response to json.
    let mut json: Value = serde_json::from_reader(response_json).unwrap();
    // create list with for Client structs.
    let mut json_clients_list: Vec<ClientUnited> = Vec::new();
    // clone json as an array for iteration.
    let json_array: Value = serde_json::from_value(json["results"].clone()).unwrap();
    // iterate json_array in order to fill json_clients_list.
    for object in json_array.as_array() {
        for objectling in object {
            let client = ClientUnited::new(objectling.clone());
            json_clients_list.push(client);
        }
    }
    // get csv containing user data from source.
    let mut response_csv = reqwest::blocking::get(
        "https://storage.googleapis.com/juntossomosmais-code-challenge/input-backend.csv",
    )
    .expect("unable to get the origin csv.");
    // convert response to Reader, for file tempering.
    let mut rdr = csv::Reader::from_reader(response_csv);

    // convert ClientCSV to Client struct.
    for result in rdr.deserialize() {
        let mut result: ClientCSV = result.unwrap();
        result = result.clone();
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
    }

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

    let mut storage = Storage::new();

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

        storage.clients.insert(client.id.clone(), client);

        id_counter += 1;
    }

    // let json_clients_list: Vec<Client> = json_clients_list.into();
    // json_clients_list
    storage
    // Ok::<warp::reply::Json, warp::Rejection>(warp::reply::json(&storage))
}

// async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
//     if let Some(_InvalidId) = r.find() {
//         Ok(warp::reply::with_status(
//             "No valid ID presented",
//             StatusCode::UNPROCESSABLE_ENTITY,
//         ))
//     } else {
//         Ok(warp::reply::with_status(
//             "Route not found",
//             StatusCode::NOT_FOUND,
//         ))
//     }
// }

#[tokio::main]
async fn main() {
    use tokio::task::spawn_blocking;
    use warp::http::Method;

    //
    // #[derive(Debug, Serialize, Deserialize, Clone)]
    // struct Replai {json: Result<String, Error>}
    //
    // impl warp::Reply for Replai {
    // fn into_response(self) -> warp::reply::Response {
    //     Response::new(format!("{}", self.json).into())
    // }
    // }

    // let clientslist: Vec<Client> = clientslist().await;

    // let mut clientts: Replai = Replai {json: spawn_blocking( || {clientslist()}).await.unwrap().await};
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    // let mut clients_spawn = spawn_blocking(move || {get_clients()}).await.unwrap();
    //
    // let route = warp::get().and(warp::path("clients")).and(warp::path::end()).and_then(match get_clients() {
    //        Err(_) =>  {
    //            Err(warp::reject::reject())
    //        },
    //        Ok(storage) => {
    //            Ok(warp::reply::json(
    //                &get_clients().unwrap()
    //            ))
    //        }
    //   }
    // );
    // warp::serve(route).run(([127, 0, 0, 1], 3030)).await;

    //FUNCIONANDO
    #[derive(Debug)]
    struct InvalidId;
    impl Reject for InvalidId {}

    async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
        if let Some(error) = r.find::<CorsForbidden>() {
            Ok(warp::reply::with_status(
                error.to_string(),
                StatusCode::FORBIDDEN,
            ))
        } else if let Some(InvalidId) = r.find() {
            Ok(warp::reply::with_status(
                "No valid ID presented".to_string(),
                StatusCode::UNPROCESSABLE_ENTITY,
            ))
        } else {
            Ok(warp::reply::with_status(
                "Route not found".to_string(),
                StatusCode::NOT_FOUND,
            ))
        }
    }

    let mut clients_spawn = spawn_blocking(move || get_clients()).await.unwrap().clone();

    // let route = warp::get().and(warp::path!("clients" / usize)).and(warp::path::end()).map(move |id| warp::reply::json(&clients_spawn.0[id])).with(cors).recover(return_error)   ;

    let route = warp::get()
        .and(warp::path("clients"))
        .and(warp::path::end())
        .map(move || warp::reply::json(&clients_spawn.clients[&ClientId(1.to_string())]))
        .with(cors)
        .recover(return_error);

    warp::serve(route).run(([127, 0, 0, 1], 3030)).await;
    //FUNCIONANDO
}
