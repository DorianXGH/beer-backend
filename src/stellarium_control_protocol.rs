#[repr(C, packed)]
struct goto_msg {
    length : u16,
    type : u16,
    time : u64,
    right_ascension : u32,
    declisation : i32
};

#[repr(C, packed)]
struct current_pos_msg {
    length : u16,
    type : u16,
    time : u64,
    right_ascension : u32,
    declisation : i32,
    status : i32
};
