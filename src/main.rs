use serde_json::Value;

fn main() {
    let response = reqwest::blocking::get("https://storage.googleapis.com/juntossomosmais-code-challenge/input-backend.json").expect("unable to get the origin json.");
    let json_response: Value = serde_json::from_str(response.text().expect("unable to converte Response to String.").as_str()).expect("unable to convert &str to serde_json::Value.");

    println!("{}",json_response);
}
