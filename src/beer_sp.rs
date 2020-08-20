use serial;
pub enum BeerMsg {
    GotoAbs(i32,i32),
    GotoRel(i32,i32),
    ZeroAbs(i32,i32),
    ZeroRel(i32,i32)
}

impl BeerMsg {
    pub fn to_triplet(&self) -> (u16,i32,i32) 
    {
        match &self {
            BeerMsg::GotoAbs(a,b) => (0,*a,*b),
            BeerMsg::GotoRel(a,b) => (1,*a,*b),
            BeerMsg::ZeroAbs(a,b) => (2,*a,*b),
            BeerMsg::ZeroRel(a,b) => (3,*a,*b),
            _ => (std::u16::MAX,0,0)
        }
    }
    pub fn send_msg(&self, port : &mut dyn serial::SerialPort)
    {
        let (a,b,c) = &self.to_triplet();
        port.write(&a.to_le_bytes()).expect("Err while sending order to port");
        port.write(&b.to_le_bytes()).expect("Err while sending order to port");
        port.write(&c.to_le_bytes()).expect("Err while sending order to port");
    }
}