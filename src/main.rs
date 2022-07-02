fn main() {
    let response = reqwest::blocking::get("https://storage.googleapis.com/juntossomosmais-code-challenge/input-backend.json").unwrap();
    println!("{}", response.text().unwrap());
}
