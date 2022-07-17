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

    pub struct LocationCorrdinates {
        minlon: f64,
        minlat: f64,
        maxlon: f64,
        maxlat: f64,
    }


