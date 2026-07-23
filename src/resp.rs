pub struct RespParser{
    pub buf: [u8; 512],
    pub pos: usize,
    pub bytes_read: usize
}

#[derive(Debug)]
pub enum RespError {
    UnexpectedEof,
    UnexpectedByte,
}

pub enum RespProtocolDataType {

}

impl RespParser {

    pub fn ParseInput(&mut self) -> Result<(), RespError> {

        let arraysizevalue = Self::findArraySize(self)?;
        match self.buf[self.pos] {
            b'$' => Self::parseString(self, arraysizevalue),
            _ => Ok(())

        }
    }

    pub fn findArraySize(&mut self) -> Result<u8, RespError>{

        let mut trackedValueToReturn: u8 = 0;
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

             trackedValueToReturn = trackedValueToReturn * base +  (self.buf[self.pos] - b'0');

             self.pos = self.pos + 1;

        }

        self.pos = self.pos + 2;
        Ok(trackedValueToReturn)
    }

    pub fn parseString(&mut self, mut arraysizevalue: u8) -> Result<(), RespError> {

        let mut vec: Vec<u8> = Vec::new();
        while arraysizevalue != 0 {
            if self.pos + 1 >= self.bytes_read {
                return Err(RespError::UnexpectedEof);
            }

            if self.buf[self.pos] == b'\r' && self.buf[self.pos+1] == b'\n'
            {
                arraysizevalue = arraysizevalue - 1;
                self.pos = self.pos + 2;
                continue;
            }

             self.pos = self.pos + 1;

        }
        Ok(())
    }
}
