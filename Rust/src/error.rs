pub trait ErrStr<T> {
    fn err_str<S: Into<Box<str>>>(self, s: S) -> Result<T, Error>;
}

impl<T, E: std::fmt::Debug> ErrStr<T> for Result<T, E> {
    fn err_str<S: Into<Box<str>>>(self, s: S) -> Result<T, Error> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => {
                println!("{:?}", e);
                Err(Error(500, s.into()))
            }
        }
    }
}

#[derive(Debug)]
pub struct Error(pub u16, pub Box<str>);

pub fn custom_err<S: std::convert::Into<Box<str>>>(code: u16, msg: S) -> Error {
    Error(code, msg.into())
}

impl warp::reject::Reject for Error {}

impl std::convert::From<Error> for warp::Rejection {
    fn from(e: Error) -> warp::Rejection {
        warp::reject::custom(e)
    }
}

use warp::http::StatusCode;
use warp::{Rejection, Reply};

#[derive(Serialize)]
pub struct ErrMsg<'a> {
    error: &'a Box<str>,
}

#[derive(Serialize)]
struct Empty {}

pub async fn customize_error(err: Rejection) -> Result<impl Reply, Rejection> {
    if err.is_not_found() {
        let json = warp::reply::json(&Empty {});
        Ok(warp::reply::with_status(json, StatusCode::NOT_FOUND))
    } else if let Some(err) = err.find::<Error>() {
        let code = err.0;
        if code == 404 {
            let json = warp::reply::json(&Empty {});
            Ok(warp::reply::with_status(json, StatusCode::NOT_FOUND))
        } else {
            let json = warp::reply::json(&ErrMsg { error: &err.1 });
            Ok(warp::reply::with_status(
                json,
                StatusCode::from_u16(code).expect("Invalid HTTP code"),
            ))
        }
    } else {
        Err(err)
    }
}
