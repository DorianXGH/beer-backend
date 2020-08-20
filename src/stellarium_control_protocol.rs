use std::net;
use std::io::Read;

#[repr(C, packed)]
pub struct GotoMsg {
    length : u16,
    msg_type : u16,
    pub time : u64,
    pub right_ascension : u32,
    pub declination : i32
}

#[repr(C, packed)]
pub struct CurrentPosMsg {
    length : u16,
    msg_type : u16,
    pub time : u64,
    pub right_ascension : u32,
    pub declination : i32,
    status : i32
}

pub struct Connection <'a> {
    buffer : [u8;std::mem::size_of::<GotoMsg>()],
    stream : &'a mut net::TcpStream,
}

impl<'a> Connection <'a> {
    pub fn from_stream(stream : &'a mut net::TcpStream) -> Connection <'a> {
        let c = Connection {
            buffer : [0;std::mem::size_of::<GotoMsg>()],
            stream : stream
        };
        c
    }

    pub fn readmsg(&mut self) -> Option<GotoMsg> {
        let numawait = self.stream.peek(&mut self.buffer).unwrap();
        if(numawait == std::mem::size_of::<GotoMsg>())
        {
            self.stream.read(&mut self.buffer).unwrap();
            let goto = GotoMsg {
                length : (self.buffer[1] as u16) << 8 | (self.buffer[0] as u16),
                msg_type : (self.buffer[3] as u16) << 8 | (self.buffer[2] as u16),
                time : (self.buffer[11] as u64) << 56 | (self.buffer[10] as u64) << 48 | (self.buffer[9] as u64) << 40 | (self.buffer[8] as u64) << 32 | (self.buffer[7] as u64) << 24 | (self.buffer[6] as u64) << 16 | (self.buffer[5] as u64) << 8 | (self.buffer[4] as u64),
                right_ascension : (self.buffer[15] as u32) << 24 | (self.buffer[14] as u32) << 16 | (self.buffer[13] as u32) << 8 | (self.buffer[12] as u32),
                declination : ((self.buffer[19] as u32) << 24 | (self.buffer[18] as u32) << 16 | (self.buffer[17] as u32) << 8 | (self.buffer[16] as u32)) as i32
            };
            Some(goto)
        } else {
            None
        }

    }
}