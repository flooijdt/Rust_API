use serde_json::Value;
use serde::Deserialize;
use csv::{self, Reader};

fn main() {
    let response = reqwest::blocking::get("https://storage.googleapis.com/juntossomosmais-code-challenge/input-backend.json").expect("unable to get the origin json.");
    let json_response: Value = serde_json::from_str(response.text().expect("unable to converte Response to String.").as_str()).expect("unable to convert &str to serde_json::Value.");

    // println!("{}",json_response);



    let response_csv = reqwest::blocking::get("https://storage.googleapis.com/juntossomosmais-code-challenge/input-backend.csv").expect("unable to get the origin csv.");
    let mut rdr= csv::Reader::from_reader(response_csv);

    println!("{:#?}", rdr.headers().expect("failed to get headers."));

    for result in rdr.records() {
        // An error may occur, so abort the program in an unfriendly way.
        // We will make this more friendly later!
        let record = result.expect("a CSV record");
        // Print a debug version of the record.
        println!("{:#?}", record);
    }
}
