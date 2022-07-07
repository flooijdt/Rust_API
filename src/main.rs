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
// create Client struct and sub-structs for easier data manipulation.
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
    struct ClientUnited {
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
        fn new(value: Value) -> Self{
            let client: ClientUnited = serde_json::from_value(value).unwrap();
            client
        }
    }

    // get csv containing user data from source.
    let response_csv = reqwest::blocking::get("https://storage.googleapis.com/juntossomosmais-code-challenge/input-backend.csv").expect("unable to get the origin csv.");
    // convert response to Reader, for file tempering.
    let mut rdr= csv::Reader::from_reader(response_csv);

    #[derive(Debug, Deserialize, Clone, Serialize)]
    struct ClientCSV {
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
    // convert ClientCSV to Client struct.
    for result in rdr.deserialize(){
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
                title:result.name__title,
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
        println!("{:#?}", &result);
        json_clients_list.push(result);
    }

    // create final Client struct according to desired output.
    #[derive(Debug, Deserialize, Clone, Serialize)]
    struct Client {
        r#type: String,
        gender: String,
        name: Name {
            title: String,
            first: String,
            last: String
        },
        location: Location2 {
            region: String,
            street: String,
            city: String,
            state: String,
            postcode: u32,
            coordinates: Coordinates {
                latitude: String,
                longitude: String
            },
            timezone: Timezone {
                offset: String,
                description: String
            }
        },
        email: String,
        birthday: String,
        registered: String,
        telephoneNumbers: [
            String
        ],
        mobileNumbers: [
            String
        ],
        picture: Picture {
            large: String,
            medium: String,
            thumbnail: String
        },
        nationality: String
    }
     struct Location2 {
        region: String,
        city: String,
        coordinates: Coordinates,
        postcode: u32,
        state: String,
        street: String,
        timezone: Timezone,
    }
    // clasification in regard to coordinates: special, labourious or normal.
    
    struct LocationCorrdinates {
        minlon: f64,
        minlat: f64,
        maxlon: f64,
        maxlat: f64,
    }
    

    let special1 = LocationCorrdinates {
        minlon: -2.196998,
        minlat -46.361899,
        maxlon: -15.411580,
        maxlat: -34.276938,
    };

    let special2 = LocationCorrdinates {
        minlon: -19.766959,
        minlat -52.997614,
        maxlon: -23.966413,
        maxlat: -44.428305,
    };

    let normal = LocationCorrdinates {
        minlon: -26.155681,
        minlat -54.777426,
        maxlon: -34.016466,
        maxlat: -46.603598,
    };



    for client in json_clients_list.iter() {
        let mut client: Client = Client {
            r#type: "placeholder",
            gender: client.gender,
            name: Name {
                title: client.name.title,
                first: client.name.first,
                last: client.name.last
            },
            location: Location2 {
                region: "placeholder",
                street: client.location.street,
                city: client.location.city,
                state: client.location.state,
                postcode: client.location.postcode,
                coordinates: Coordinates {
                    latitude: client.location.coordinates.latitude,
                    longitude: client.location.coordinates.longitude
                },
                timezone: Timezone {
                    offset: client.location.timezone.offset,
                    description: client.location.timezone.description
                }
            },
            email: client.email,
            birthday: client.dob.date,
            registered: client.registered.date,
            telephoneNumbers: [
                client.phone
            ],
            mobileNumbers: [
                client.cell
            ],
            picture: {
                large: client.picture.large,
                medium: client.picture.medium,
                thumbnail: client.picture.thumbnail
            },
            nationality: "BR"
        }:
        
        if client.location.state == "rio grande do sul" || "santa catarina" || "paraná" {
        client.location.region = "sul";
        }
        else if client.location.state == "espírito santo" || "rio de janeiro" || "minas gerais" || "são paulo" {
        client.location.region = "sudeste";
        }
        else if client.location.state == "mato grosso" || "mato grosso do sul" || "goiás" || "distrito federal" {
        client.location.region = "centro-oeste";
        }
        else if client.location.state == "acre" || "amazonas" || "rondônia" || "amapá" || "roraima" || "pará" || "tocantins" {
        client.location.region = "norte";
        }
        else if client.location.state == "bahia" || "sergipe" || "alagoas" || "paraíba" || "pernambuco" || "rio grande do norte" || "ceará" || "piauí" || "maranhão" {
        client.location.region = "nordeste";
        }
        
    if special1.minlat <= client.location.coordinates.latitude <= special1.maxlat && special1.minlon <= client.location.coordinates.longitude <= special1.maxlon {
        client.r#type = "special";
    }





    }



}
