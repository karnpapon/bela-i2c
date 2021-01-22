#[derive(Debug)]
pub enum OscErrors {
  DeviceNotFound,
  UnrecognizedAddress(String),
  NoSetAction,
  CommandNotFound(String),
  PortNumberIncorrect(String),
}

impl std::fmt::Display for OscErrors {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match *self {
      OscErrors::DeviceNotFound => {
        write!(f, "DeviceNotFound")
      }
      OscErrors::UnrecognizedAddress(ref msg) => {
        write!(f, "UnrecognizedAddress: {}", msg)
      }
      OscErrors::NoSetAction => {
        write!(f, "NoSetAction")
      }
      OscErrors::CommandNotFound(ref msg) => {
        write!(f, "CommandNotFound: {}", msg)
      }
      OscErrors::PortNumberIncorrect(ref msg) => {
        write!(f, "PortNumberIncorrect: {}", msg)
      }
    }
  }
}
