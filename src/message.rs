#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Message {
    SetAbsolute { amount: i32, percent: bool },
    SetRelative { amount: i32, percent: bool },
    None,
}

// fuck serde, fuck json, fuck rust serialization bullshit
// I'ma just fucking copy the goddamn bytes myself
impl Message {
    pub fn serialize(self) -> [u8; ::core::mem::size_of::<Self>()] {
        unsafe { std::mem::transmute(self) }
    }
    pub fn deserialize(bytes: [u8; ::core::mem::size_of::<Self>()]) -> Self {
        unsafe { std::mem::transmute(bytes) }
    }
}

#[test]
fn message_serialize_compat() {
    let msg = Message::SetRelative {
        amount: 8430289,
        percent: true,
    };

    let serialised = msg.serialize();
    let converted = Message::deserialize(serialised);

    assert_eq!(msg, converted);
}
