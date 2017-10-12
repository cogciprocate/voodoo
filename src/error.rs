use std::error::Error as StdError;
use std::result::Result as StdResult;
use ::CallResult;

pub type Result<T> = StdResult<T, Error>;

pub enum ErrorKind {
    Void,
    CallResult(CallResult),
    String(String),
    Nul(::std::ffi::NulError),
    Io(::std::io::Error),
    FromUtf8Error(::std::string::FromUtf8Error),
    UnspecifiedDimensions,
    IntoStringError(::std::ffi::IntoStringError),
}


/// An Error.
pub struct Error {
    pub kind: ErrorKind,
    pub cause: Option<Box<self::Error>>,
}

impl self::Error {
    /// Returns an `Error` with the `UnspecifiedDimensions` kind variant.
    pub fn unspecified_dimensions() -> Error {
        Error { kind: ErrorKind::UnspecifiedDimensions, cause: None }
    }

    /// Creates a new error with this error as its cause.
    pub fn chain<E: Into<Error>>(self, err: E) -> Self {
        // let desc = format!("{}: {}", pre, self.description());
        let err = err.into();
        assert!(err.cause.is_none(), "Cannot chain an error that already has a cause.");
        Error { kind: err.kind, cause: Some(Box::new(self)) }
    }

    /// Returns the error variant and contents.
    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    /// Returns the immediate cause of this error (e.g. the next error in the
    /// chain).
    pub fn cause(&self) -> Option<&self::Error> {
        // match self.cause {
        //     Some(ref bc) => Some(&*bc),
        //     None => None,
        // }
        self.cause.as_ref().map(|c| &**c)
    }

    /// Writes the error message for this error to a formatter.
    fn write_msg(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match self.kind {
                ErrorKind::Void => write!(f, "Error"),
                ErrorKind::CallResult(ref res) => write!(f, "Vulkan API call result: {:?}", res),
                ErrorKind::Nul(ref err) => write!(f, "{}", err.description()),
                ErrorKind::Io(ref err) => write!(f, "{}", err.description()),
                ErrorKind::FromUtf8Error(ref err) => write!(f, "{}", err.description()),
                ErrorKind::IntoStringError(ref err) => write!(f, "{}", err.description()),
                ErrorKind::String(ref desc) => write!(f, "{}", desc),
                ErrorKind::UnspecifiedDimensions => write!(f, "Cannot convert to a valid set of \
                    dimensions. Please specify some dimensions."),
            }
        }

    /// Writes the error message for this error and its cause to a formatter.
    fn _fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self.cause {
            Some(ref cause) => {
                self.write_msg(f)?;
                write!(f, ": ")?;
                cause._fmt(f)
            },
            None => self.write_msg(f)
        }
    }
}

impl ::std::fmt::Debug for self::Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self._fmt(f)
    }
}

impl ::std::fmt::Display for self::Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self._fmt(f)
    }
}

impl StdError for self::Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::Void => "Vulkan error",
            ErrorKind::CallResult(ref _res) => "Vulkan API call error",
            ErrorKind::Nul(ref err) => err.description(),
            ErrorKind::Io(ref err) => err.description(),
            ErrorKind::FromUtf8Error(ref err) => err.description(),
            ErrorKind::IntoStringError(ref err) => err.description(),
            ErrorKind::String(ref desc) => desc.as_str(),
            ErrorKind::UnspecifiedDimensions => "Cannot convert to a valid set of dimensions. \
                Please specify some dimensions.",
            // _ => panic!("OclErrorKind::description()"),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        match self.cause {
            Some(ref bc) => Some(&*bc),
            None => None,
        }
    }
}

impl From<i32> for self::Error {
    fn from(res: i32) -> Self {
        Error { kind: self::ErrorKind::CallResult(res.into()), cause: None }
    }
}

impl From<()> for self::Error {
    fn from(_: ()) -> Self {
        Error { kind: self::ErrorKind::Void, cause: None }
    }
}

impl From<String> for self::Error {
    fn from(desc: String) -> Self {
        Error { kind: self::ErrorKind::String(desc), cause: None }
    }
}

impl From<self::Error> for String {
    fn from(err: self::Error) -> String {
        format!("{}", err)
    }
}

impl<'a> From<&'a str> for self::Error {
    fn from(desc: &'a str) -> Self {
        Error { kind: self::ErrorKind::String(String::from(desc)), cause: None }
    }
}

impl From<::std::ffi::NulError> for self::Error {
    fn from(err: ::std::ffi::NulError) -> Self {
        Error { kind: self::ErrorKind::Nul(err), cause: None }
    }
}

impl From<::std::io::Error> for self::Error {
    fn from(err: ::std::io::Error) -> Self {
        Error { kind: self::ErrorKind::Io(err), cause: None }
    }
}

impl From<::std::string::FromUtf8Error> for self::Error {
    fn from(err: ::std::string::FromUtf8Error) -> Self {
        Error { kind: self::ErrorKind::FromUtf8Error(err), cause: None }
    }
}

impl From<::std::ffi::IntoStringError> for self::Error {
    fn from(err: ::std::ffi::IntoStringError) -> Self {
        Error { kind: self::ErrorKind::IntoStringError(err), cause: None }
    }
}

unsafe impl ::std::marker::Send for self::Error {}




/// An chainable error.
pub trait ChainErr<T, E> {
    /// If the `Result` is an `Err` then `chain_err` evaluates the closure,
    /// which returns *some type that can be converted to `ErrorKind`*, boxes
    /// the original error to store as the cause, then returns a new error
    /// containing the original error.
    //
    // Blatantly ripped off from the `error-chain` crate.
    fn chain_err<F, IE>(self, callback: F) -> ::std::result::Result<T, Error>
        where F: FnOnce() -> IE, IE: Into<Error>;
}

impl<T> ChainErr<T, Error> for self::Result<T> {
    fn chain_err<F, E>(self, callback: F) -> self::Result<T>
            where F: FnOnce() -> E, E: Into<self::Error>
      {
        self.map_err(move |e| {
            let err = callback().into();
            assert!(err.cause.is_none());
            self::Error { kind: err.kind, cause: Some(Box::new(e)) }
        })
    }
}