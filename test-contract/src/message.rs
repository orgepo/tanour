use minicbor::{Decode, Encode};

#[derive(Clone, Debug, Encode, Decode)]
pub enum TestMsg {
    #[n(0)]
    Mul {
        #[n(0)]
        a: i32,
        #[n(1)]
        b: i32,
    },
    #[n(1)]
    Div {
        #[n(0)]
        a: i32,
        #[n(1)]
        b: i32,
    },
    #[n(2)]
    WriteBuffer(#[n(0)] [u8; 16]),

    #[n(3)]
    ReadBuffer,
}
