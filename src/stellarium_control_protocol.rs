use std::net

#[repr(C, packed)]
struct GotoMsg {
    length : u16,
    type : u16,
    time : u64,
    right_ascension : u32,
    declisation : i32
};

#[repr(C, packed)]
struct CurrentPosMsg {
    length : u16,
    type : u16,
    time : u64,
    right_ascension : u32,
    declisation : i32,
    status : i32
};

struct Connection {
    buffer : [u8;std::mem::size_of(GotoMsg)],
    stream : &mut net::TcpStream,
};

impl Connection {
    fn from_stream(&mut net::TcpStream stream) -> Connection {
        Connection {
            buffer : [0;std::mem::size_of(GotoMsg)],
            stream : stream
        }
    }

    fn readmsg($self) -> Option<GotoMsg> {
        let numawait = self.stream.peek(&mut self.buffer).unwrap();
        if(numawait == std::mem::size_of(GotoMsg))
        {
            self.stream.read(&mut self.buffer).unwrap();
            let goto = GotoMsg {
                length : self.buffer[0] << 8 | self.buffer[1],
                type : self.buffer[2] << 8 | self.buffer[3],
                time : self.buffer[4] << 56 | self.buffer[5] << 48 | self.buffer[6] << 40 | self.buffer[7] << 32 | self.buffer[8] << 24 | self.buffer[9] << 16 | self.buffer[10] << 8 | self.buffer[11],
                right_ascension : self.buffer[12] << 24 | self.buffer[13] << 16 | self.buffer[14] << 8 | self.buffer[15],
                declisation : self.buffer[16] << 24 | self.buffer[17] << 16 | self.buffer[18] << 8 | self.buffer[19]
            };
            Some(goto)
        } else {
            None
        }

    }
}