use fs_extra::error::Error as FseError;
use fs_extra::error::ErrorKind as FseErrorKind;
use std::error::Error as StdError;
use std::fmt;
use std::io::Error as IoError;
use std::io::ErrorKind as IoErrorKind;

/// A list specifying general categories of fs_pro error.
#[derive(Debug)]
pub enum ErrorKind {
  /// An entity was not found.
  NotFound,
  /// The operation lacked the necessary privileges to complete.
  PermissionDenied,
  /// An entity already exists.
  AlreadyExists,
  /// This operation was interrupted.
  Interrupted,
  /// Invalid folder found.
  InvalidFolder,
  /// file is invalid
  InvalidFile,
  /// folder is invalid
  InvalidFileName,
  /// Invalid path.
  InvalidPath,
  /// failed to convert path to &str
  PathToStrConversionFail,
  /// no parent directory found in path
  PathNoParentFound,
  /// no filename found in path
  PathNoFilenameFound,
  /// on extension found in path
  PathNoExtensionFound,
  #[cfg(feature = "json")]
  /// an error happen read file as json
  JsonError(serde_json::error::Error),
  /// any other error
  Other,
}

impl ErrorKind {
  fn as_str(&self) -> &str {
    // let res: &str;
    match *self {
      ErrorKind::NotFound => "entity not found",
      ErrorKind::PermissionDenied => "permission denied",
      ErrorKind::AlreadyExists => "entity already exists",
      ErrorKind::Interrupted => "operation interrupted",
      ErrorKind::Other => "other os error",
      ErrorKind::InvalidFolder => "the file is invalid",
      ErrorKind::InvalidFile => "the folder is invalid",
      ErrorKind::InvalidFileName => "invalid file name error",
      ErrorKind::InvalidPath => "invalid path",
      ErrorKind::PathToStrConversionFail => "path cannot be converted to utf-8 string (&str)",
      ErrorKind::PathNoParentFound => "cannot find any parent directory",
      ErrorKind::PathNoFilenameFound => "cannot find filename",
      ErrorKind::PathNoExtensionFound => "cannot find file extension",
      #[cfg(feature = "json")]
      ErrorKind::JsonError(_) => "an error happen read file as json",
    }
  }
}

#[derive(Debug)]
/// any kind of error
pub struct Error {
  /// the kind of the Error see fs_pro::error::Error
  pub kind: ErrorKind,
  /// the message
  pub message: String,
}

impl Error {
  /// create a new Error
  pub fn new(kind: ErrorKind, message: &str) -> Error {
    Error {
      kind: kind,
      message: message.to_string(),
    }
  }
  /// create new error from ErrorKind and adds default msg
  pub fn new_from_kind(kind: ErrorKind) -> Error {
    let msg = (&kind).as_str().to_string();
    Error {
      kind: kind,
      message: msg,
    }
  }
  /// convert std::io::Error to fs_pro::error::Error
  pub fn from_io(io_err: IoError) -> Error {
    let err_kind: ErrorKind;
    match io_err.kind() {
      IoErrorKind::NotFound => err_kind = ErrorKind::NotFound,
      IoErrorKind::PermissionDenied => err_kind = ErrorKind::PermissionDenied,
      IoErrorKind::AlreadyExists => err_kind = ErrorKind::AlreadyExists,
      IoErrorKind::Interrupted => err_kind = ErrorKind::Interrupted,
      IoErrorKind::Other => err_kind = ErrorKind::Other,
      _ => err_kind = ErrorKind::Other,
    }
    Error::new(err_kind, &io_err.to_string())
  }
  /// converts fs_extra::error::Error to fs_pro::error::Error
  pub fn from_fse_error(fse_error: FseError) -> Error {
    let err_kind: ErrorKind;
    match fse_error.kind {
      FseErrorKind::NotFound => err_kind = ErrorKind::NotFound,
      FseErrorKind::PermissionDenied => err_kind = ErrorKind::PermissionDenied,
      FseErrorKind::AlreadyExists => err_kind = ErrorKind::AlreadyExists,
      FseErrorKind::Interrupted => err_kind = ErrorKind::Interrupted,
      FseErrorKind::InvalidFolder => err_kind = ErrorKind::InvalidFolder,
      FseErrorKind::InvalidFile => err_kind = ErrorKind::InvalidFile,
      FseErrorKind::InvalidFileName => err_kind = ErrorKind::InvalidFileName,
      FseErrorKind::InvalidPath => err_kind = ErrorKind::InvalidPath,
      FseErrorKind::Io(err) => return Error::from_io(err),
      _ => return Error::new(ErrorKind::Other, fse_error.to_string().as_str()),
    };
    Error::new(err_kind, fse_error.to_string().as_str())
  }
}

impl StdError for Error {
  fn description(&self) -> &str {
    self.kind.as_str()
  }
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.message)
  }
}

/// fs_pro's version of Result
pub type Result<T> = ::std::result::Result<T, Error>;

/// convert fs_extra::error::Result to fs_pro::error::Result
pub fn result_from_fse<T>(fse_res: fs_extra::error::Result<T>) -> Result<T> {
  match fse_res {
    Ok(val) => Ok(val),
    Err(e) => Err(Error::from_fse_error(e)),
  }
}

/// converts std::io::Result to fs_pro::error::Result
pub fn result_from_io<T>(io_res: std::io::Result<T>) -> Result<T> {
  match io_res {
    Ok(val) => Ok(val),
    Err(e) => Err(Error::from_io(e)),
  }
}

/// converts Option to fs_pro::error::Result
#[allow(dead_code)]
pub fn result_from_option<T>(maybe_val: Option<T>, err: Error) -> Result<T> {
  if let Some(val) = maybe_val {
    Ok(val)
  } else {
    Err(err)
  }
}

/// convert Option<T> to fs_pro::error::Result<T>
pub fn result_from_option2<T>(maybe_val: Option<T>, kind: ErrorKind) -> Result<T> {
  if let Some(val) = maybe_val {
    Ok(val)
  } else {
    Err(Error::new_from_kind(kind))
  }
}
