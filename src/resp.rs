pub struct RespParser{
    pub buf: [u8; 512],
    pub pos: usize,
    pub bytes_read: usize,
}


#[derive(Debug)]
pub enum RespError {
    UnexpectedEof,
    UnexpectedByte,
}

#[derive(Debug)]
pub enum RespProtocolDataType {
    SimpleString,
    SimpleErrors,
    Integers,
    BulkStrings(Vec<u8>),
    NullBulkStrings,
    Arrays(Vec<RespProtocolDataType>),
    Nulls,
    Booleans,
    Doubles,
    BigNumbers,
    BulkErrors,
    VerbatimStrings,
    Maps,
    Attributes,
    Sets,
    Pushes,
}
impl RespParser {

    pub fn parse_input(&mut self) -> Result<RespProtocolDataType, RespError> {
        
        match self.buf[self.pos] {
            b'*' => self.array(),
            b'$' => self.bulk_string(),
            _ => Err(RespError::UnexpectedEof) //come up with a better error

        }
    }

    pub fn bulk_string(&mut self) ->  Result<RespProtocolDataType, RespError>{

        let bulk_string_length = Self::read_length(self)?;
        let mut payload = Vec::new();

        if self.pos >= self.bytes_read {
            return Err(RespError::UnexpectedEof);
        }

        for _i in 0..bulk_string_length {
            if self.pos > self.bytes_read {
                return Err(RespError::UnexpectedEof);
            }

            payload.push(self.buf[self.pos]);

            self.pos = self.pos + 1;
        }
        
        self.pos = self.pos + 2;

        Ok(RespProtocolDataType::BulkStrings(payload))
    }

    pub fn read_length(&mut self) -> Result<u8, RespError>{

        let mut tracked_value_to_return: u8 = 0;
        let base: u8 = 10;

        self.pos = self.pos + 1;

        loop{

            if self.pos + 1 >= self.bytes_read {
                return Err(RespError::UnexpectedEof);
            }

            if self.buf[self.pos] == b'\r' && self.buf[self.pos+1] == b'\n'
            {
                break;
            }

             if !self.buf[self.pos].is_ascii_digit() {
                 return Err(RespError::UnexpectedByte);
             }

             tracked_value_to_return = tracked_value_to_return * base +  (self.buf[self.pos] - b'0');

             self.pos = self.pos + 1;

        }

        self.pos = self.pos + 2;

        if self.pos > self.bytes_read {
                return Err(RespError::UnexpectedEof);
        }

        Ok(tracked_value_to_return)
    }

    pub fn array(&mut self) -> Result<RespProtocolDataType, RespError> {

        let array_size = Self::read_length(self)?;

        let mut elements = Vec::new();

        for _i in 0..array_size {
            elements.push(self.parse_input()?);
        }


        Ok(RespProtocolDataType::Arrays(elements))
    }
}
