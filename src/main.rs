use serde_json::Value;
use serde::Deserialize;
use csv::{self, Reader};

fn main() {
    // get json containing user data from source
    let response = reqwest::blocking::get("https://storage.googleapis.com/juntossomosmais-code-challenge/input-backend.json").expect("unable to get the origin json.");
    // convert json to serde_json Value
    let json_response: Value = serde_json::from_str(response.text().expect("unable to converte Response to String.").as_str()).expect("unable to convert &str to serde_json::Value.");

    // println!("{}",json_response);
    //get csv containing user data from source
    let response_csv = reqwest::blocking::get("https://storage.googleapis.com/juntossomosmais-code-challenge/input-backend.csv").expect("unable to get the origin csv.");
    let mut rdr= csv::Reader::from_reader(response_csv);

    println!("{:#?}", rdr.headers().expect("failed to get headers."));

    // for result in rdr.records() {
    //     // An error may occur, so abort the program in an unfriendly way.
    //     // We will make this more friendly later!
    //     let record = result.expect("failed to read record");
    //     // Print a debug version of the record.
    //     println!("{:#?}", record);
    // }
    
    for result in rdr.deserialize(){
        let result: (String, String) = result.expect("error deserializing into tuple.");
        println!("{:#?}", result);
    }

    // Parse clients into structs
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
       location__timezone__offset: f64,
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
    
    for result in rdr.deserialize(){
        let result: (String, String) = result.expect("error deserializing into tuple.");
        println!("{:#?}", result);
    }


}
