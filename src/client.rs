use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::fmt;

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dob {
    pub age: u32,
    pub date: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location {
    pub city: String,
    pub coordinates: Coordinates,
    pub postcode: u32,
    pub state: String,
    pub street: String,
    pub timezone: Timezone,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Coordinates {
    pub latitude: String,
    pub longitude: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timezone {
    pub description: String,
    pub offset: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Name {
    pub first: String,
    pub last: String,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Picture {
    pub large: String,
    pub medium: String,
    pub thumbnail: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Registered {
    pub age: u32,
    pub date: String,
}
/** Creates intermediary client struct for unifying the `CSV` date with the `JSON` data. */
#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct ClientUnited {
    pub cell: String,
    pub dob: Dob,
    pub email: String,
    pub gender: String,
    pub location: Location,
    pub name: Name,
    pub phone: String,
    pub picture: Picture,
    pub registered: Registered,
}

impl ClientUnited {
    pub fn new(value: Value) -> Self {
        let client: ClientUnited = serde_json::from_value(value).unwrap();
        client
    }
}

/** Receives `CSV` data and organizes it before parsing it to the final Client struct. */
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct ClientCSV {
    pub gender: String,
    pub name__title: String,
    pub name__first: String,
    pub name__last: String,
    pub location__street: String,
    pub location__city: String,
    pub location__state: String,
    pub location__postcode: u32,
    pub location__coordinates__latitude: f64,
    pub location__coordinates__longitude: f64,
    pub location__timezone__offset: String,
    pub location__timezone__description: String,
    pub email: String,
    pub dob__date: String,
    pub dob__age: u32,
    pub registered__date: String,
    pub registered__age: u32,
    pub phone: String,
    pub cell: String,
    pub picture__large: String,
    pub picture__medium: String,
    pub picture__thumbnail: String,
}
/** Implements the new function for `ClientCSV` for parsing clients to `JSON`. */
impl ClientCSV {
    pub fn new(value: Value) -> Self {
        let client: ClientCSV = serde_json::from_value(value).unwrap();
        client
    }
}



/** Creates final Client struct according to desired output. */
#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Client {
    pub id: String,
    pub r#type: String,
    pub gender: String,
    pub name: Name,
    pub location: Location2,
    pub email: String,
    pub birthday: String,
    pub registered: String,
    pub telephoneNumbers: Vec<String>,
    pub mobileNumbers: Vec<String>,
    pub picture: Picture,
    pub nationality: String,
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Location2 {
    pub region: String,
    pub city: String,
    pub coordinates: Coordinates,
    pub postcode: u32,
    pub state: String,
    pub street: String,
    pub timezone: Timezone,
}

pub struct LocationCoordinates {
    pub minlon: f64,
    pub minlat: f64,
    pub maxlon: f64,
    pub maxlat: f64,
}


#[derive(Debug, Deserialize, Clone, Serialize, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct ClientId {pub string: String}

impl fmt::Display for ClientId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self)
    }
}
