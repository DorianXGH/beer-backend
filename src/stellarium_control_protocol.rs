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
    fn readmsg($self) -> GotoMsg {
        let numawait = self.stream.peek(&mut self.buffer).unwrap();
        if(numawait == std::mem::size_of(GotoMsg))
        {
            self.stream.read(&mut self.buffer).unwrap();
            
        }

    }
}