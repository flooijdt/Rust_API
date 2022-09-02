use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::StatusCode,
    reject::Reject,
    Rejection, Reply,
};

/** Details possible problems deploying and operating the server. */
#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    ClientNotFound,
}
/** Implements the Display trait to Error, thus allowing the custom errors to be printed. */
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::ParseError(ref err) => write!(f, "Cannot parse parameter: {}", err),
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::ClientNotFound => write!(f, "Client not found"),
        }
    }
}

impl Reject for Error {}
/** Returns the errors and status' code according to the error origin.  */
pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    }else if let Some(error) = r.find::<CorsForbidden>() {
            Ok(warp::reply::with_status(
                error.to_string(),
                StatusCode::FORBIDDEN,
            ))
        } else if let Some(error) = r.find::<BodyDeserializeError>() {
                Ok(warp::reply::with_status(
                    error.to_string(),
                    StatusCode::UNPROCESSABLE_ENTITY,
                ))
            }  else {
                    Ok(warp::reply::with_status(
                        "Route not found".to_string(),
                        StatusCode::NOT_FOUND,
                    ))
                }
}


