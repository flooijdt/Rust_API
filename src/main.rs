use serde_json::{Value, Value::Object, Map};
use json;
use serde::{Serialize, Deserialize, de::IntoDeserializer, Deserializer};
use csv::{self, Reader};
use reqwest::blocking::Response;
use std::vec::Vec;

fn main() {
    // get Response containing user data from source.
    let response_json = reqwest::blocking::get("https://storage.googleapis.com/juntossomosmais-code-challenge/input-backend.json").expect("unable to get the origin json.");
    // convert Response to json.
    let mut json: Value = serde_json::from_reader(response_json).expect("unable to parse json from the Response's body.");
    // create list with for Client_json structs.
    let mut json_clients_list: Vec<Client_json> = Vec::new();
    // clone json as an array for iteration.
    let json_array: Value = serde_json::from_value(json["results"].clone()).unwrap();
    // iterate json_array in order to fill json_clients_list.
    for object in json_array.as_array() {
        for objectling in object {
            let client = Client_json::new(objectling.clone());
            json_clients_list.push(client);
        }
    }
// create Client_json struct and sub-structs for easier data manipulation.
    #[derive(Debug, Deserialize, Clone, Serialize)]
    struct Dob {
        age: u32,
        date: String,
    }
    
    #[derive(Debug, Deserialize, Clone, Serialize)]
    struct Location {
        city: String,
        coordinates: Coordinates,
        postcode: u32,
        state: String,
        street: String,
        timezone: Timezone,

    }

    #[derive(Debug, Deserialize, Clone, Serialize)]
    struct Coordinates {
        latitude: String,
        longitude: String,
    }

    #[derive(Debug, Deserialize, Clone, Serialize)]
    struct Timezone {
        description: String,
        offset: String,
    }
    
    #[derive(Debug, Deserialize, Clone, Serialize)]
    struct Name {
        first: String,
        last: String,
        title: String,
    }

    #[derive(Debug, Deserialize, Clone, Serialize)]
    struct Picture {
        large: String,
        medium: String,
        thumbnail: String,
    }
    
    #[derive(Debug, Deserialize, Clone, Serialize)]
    struct Registered {
        age: u32,
        date: String,
    }

    #[derive(Debug, Deserialize, Clone, Serialize)]
    struct Client_json {
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

    impl Client_json {
        fn new(value: Value) -> Self{
            let client: Client_json = serde_json::from_value(value).unwrap();
            client
        }
    }

    // get csv containing user data from source.
    let response_csv = reqwest::blocking::get("https://storage.googleapis.com/juntossomosmais-code-challenge/input-backend.csv").expect("unable to get the origin csv.");
    // convert response to Reader, for file tempering.
    let mut rdr= csv::Reader::from_reader(response_csv);

    #[derive(Debug, Deserialize)]
    struct Client {
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
    // convert clients to Client struct.
    for result in rdr.deserialize(){
        let result: Client = result.expect("error deserializing clients into structs.");
        println!("{:#?}", result);
    }


}
