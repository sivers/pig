pub trait ErrStr<T> {
    fn err_str<S: Into<Box<str>>>(self, s: S) -> Result<T, Error>;
}

impl<T, E: std::fmt::Debug> ErrStr<T> for Result<T, E> {
    fn err_str<S: Into<Box<str>>>(self, s: S) -> Result<T, Error> {
        match self {
            Ok(t) => Ok(t),
            Err(e) => {
                println!("{:?}", e);
                Err(Error(s.into()))
            }
        }
    }
}

impl<T> ErrStr<T> for Option<T> {
    fn err_str<S: Into<Box<str>>>(self, s: S) -> Result<T, Error> {
        match self {
            Some(t) => Ok(t),
            None => Err(Error(s.into())),
        }
    }
}

#[derive(Debug)]
pub struct Error(pub Box<str>);

impl warp::reject::Reject for Error {}

impl std::convert::From<Error> for warp::Rejection {
    fn from(e: Error) -> warp::Rejection {
        warp::reject::custom(e)
    }
}

use warp::http::StatusCode;
use warp::{reject, Rejection, Reply};

#[derive(Serialize)]
pub struct ErrMsg<'a> {
    error: &'a Box<str>,
}

pub async fn customize_error(err: Rejection) -> Result<impl Reply, Rejection> {
    if err.is_not_found() {
        #[derive(Serialize)]
        struct Empty {}
        let json = warp::reply::json(&Empty {});
        Ok(warp::reply::with_status(json, StatusCode::NOT_FOUND))
    } else if let Some(err) = err.find::<Error>() {
        let json = warp::reply::json(&ErrMsg { error: &err.0 });
        let code = StatusCode::INTERNAL_SERVER_ERROR;
        Ok(warp::reply::with_status(json, code))
    } else {
        Err(err)
    }
}
