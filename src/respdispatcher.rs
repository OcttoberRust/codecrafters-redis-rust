use crate::resp::RespProtocolDataType;

pub struct RespDispatcher {}

pub enum Command {
    Ping(Option<Vec<u8>>),
    Echo(Vec<u8>),
}

#[derive(Debug)]
pub enum DispatchError {
    NotAnArray,
    MissingCommandName,
    UnknownCommand,
    ArgumentNotBulkString,
    WrongArity,
}

impl RespDispatcher {

    pub fn convert_to_command(
        &mut self,
        value: RespProtocolDataType,
    ) -> Result<Command, DispatchError> {
        if let RespProtocolDataType::Arrays(elements) = value {
            let mut parts = elements.into_iter();
            let command_name = parts.next();

            if let Some(RespProtocolDataType::BulkStrings(name_bytes)) = command_name {
                match name_bytes.to_ascii_uppercase().as_slice() {
                    b"PING" => match Self::parse_ping_args(parts) {
                        Ok(msg) => return Ok(Command::Ping(msg)),
                        Err(e) => return Err(e),
                    },
                    b"ECHO" => match Self::parse_echo_args(parts) {
                        Ok(msg) => return Ok(Command::Echo(msg)),
                        Err(e) => return Err(e),
                    },
                    _ => return Err(DispatchError::UnknownCommand),
                }
            }

            return Err(DispatchError::MissingCommandName);
        }

        Err(DispatchError::NotAnArray)
    }

    fn parse_ping_args(
        mut args: impl Iterator<Item = RespProtocolDataType>,
    ) -> Result<Option<Vec<u8>>, DispatchError> {
        let probes = (args.next(), args.next());

        match probes {
            (None, None) => Ok(None),
            (Some(arg), None) => {
                if let RespProtocolDataType::BulkStrings(msg) = arg {
                    Ok(Some(msg))
                } else {
                    Err(DispatchError::ArgumentNotBulkString)
                }
            }
            _ => Err(DispatchError::WrongArity),
        }
    }

    fn parse_echo_args(
        mut args: impl Iterator<Item = RespProtocolDataType>,
    ) -> Result<Vec<u8>, DispatchError> {
        let probes = (args.next(), args.next());

        match probes {
            (Some(arg), None) => {
                if let RespProtocolDataType::BulkStrings(msg) = arg {
                    Ok(msg)
                } else {
                    Err(DispatchError::ArgumentNotBulkString)
                }
            }
            _ => Err(DispatchError::WrongArity),
        }
    }

    // TODO: both return a RespProtocolDataType.
    pub fn ping_command() {}

    pub fn echo_command() {}
}
