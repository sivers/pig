pub trait ErrStr<T> {
    fn err_str<S: Into<Box<str>>>(self, s: S) -> Result<T, Box<str>>;
}

impl<T, E> ErrStr<T> for Result<T, E> {
    fn err_str<S: Into<Box<str>>>(self, s: S) -> Result<T, Box<str>> {
        match self {
            Ok(t) => Ok(t),
            Err(_) => Err(s.into()),
        }
    }
}

impl<T> ErrStr<T> for Option<T> {
    fn err_str<S: Into<Box<str>>>(self, s: S) -> Result<T, Box<str>> {
        match self {
            Some(t) => Ok(t),
            None => Err(s.into()),
        }
    }
}
