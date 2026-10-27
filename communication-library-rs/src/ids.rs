use deku::ctx::{Endian, Order};
use deku::prelude::*;

pub type Operation = u8;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, DekuRead, DekuWrite)]
#[deku(
    ctx = "endian: Endian, order: Order",
    endian = "endian",
    bit_order = "order",
    id_type = "u8",
    bits = "5"
)]
pub enum BoardID {
    #[deku(id = "0x01")]
    Software = 0x01,
    #[deku(id = "0x02")]
    Rocket = 0x02,
    #[deku(id = "0x09")]
    LastBoard = 0x09,
    #[deku(id = "0x1E")]
    Proxy = 0x1E,
    #[deku(id = "0x1F")]
    Broadcast = 0x1F,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, DekuRead, DekuWrite)]
#[deku(
    ctx = "endian: Endian, order: Order",
    endian = "endian",
    bit_order = "order",
    id_type = "u8",
    bits = "6"
)]
pub enum DeviceID {
    #[deku(id = "0x00")]
    Servo = 0x00,
    #[deku(id = "0x01")]
    Relay = 0x01,
    #[deku(id = "0x02")]
    Sensor = 0x02,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, DekuRead, DekuWrite)]
#[deku(
    ctx = "endian: Endian, order: Order",
    endian = "endian",
    bit_order = "order",
    id_type = "u8",
    bits = "4"
)]
pub enum ActionID {
    #[deku(id = "0x00")]
    Feed = 0x00,
    #[deku(id = "0x01")]
    Service = 0x01,
    #[deku(id = "0x02")]
    Ack = 0x02,
    #[deku(id = "0x03")]
    Nack = 0x03,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, DekuRead, DekuWrite)]
#[deku(
    ctx = "endian: Endian, order: Order",
    endian = "endian",
    bit_order = "order",
    id_type = "u8",
    bits = "4"
)]
pub enum DataTypeID {
    #[deku(id = "0x00")]
    NoData = 0x00,
    #[deku(id = "0x01")]
    Uint32 = 0x01,
    #[deku(id = "0x02")]
    Uint16 = 0x02,
    #[deku(id = "0x03")]
    Uint8 = 0x03,
    #[deku(id = "0x04")]
    Int32 = 0x04,
    #[deku(id = "0x05")]
    Int16 = 0x05,
    #[deku(id = "0x06")]
    Int8 = 0x06,
    #[deku(id = "0x07")]
    Float = 0x07,
    #[deku(id = "0x08")]
    Int16x2 = 0x08,
    #[deku(id = "0x09")]
    Uint16Int16 = 0x09,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, DekuRead, DekuWrite)]
#[deku(
    ctx = "endian: Endian, order: Order",
    endian = "endian",
    bit_order = "order",
    id_type = "u8",
    bits = "2"
)]
pub enum PriorityID {
    #[deku(id = "0x00")]
    High = 0x00,
    #[deku(id = "0x01")]
    Low = 0x01,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum ServoOperationID {
    #[deku(id = "0x01")]
    Open = 0x01,
    #[deku(id = "0x02")]
    Close = 0x02,
    #[deku(id = "0x03")]
    OpenedPos = 0x03,
    #[deku(id = "0x04")]
    ClosedPos = 0x04,
    #[deku(id = "0x05")]
    Position = 0x05,
    #[deku(id = "0x06")]
    Disable = 0x06,
    #[deku(id = "0x07")]
    Range = 0x07,
}

impl Into<u8> for ServoOperationID {
    fn into(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum RelayOperationID {
    #[deku(id = "0x01")]
    Open = 0x01,
    #[deku(id = "0x02")]
    Close = 0x02,
    #[deku(id = "0x03")]
    Status = 0x03,
}

impl Into<u8> for RelayOperationID {
    fn into(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum SensorOperationID {
    #[deku(id = "0x01")]
    Read = 0x01,
}


impl Into<u8> for SensorOperationID {
    fn into(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum AckStatus {
    #[deku(id = "0x00")]
    Disabled = 0x00,
    #[deku(id = "0x01")]
    Waiting = 0x01,
    #[deku(id = "0x02")]
    Ready = 0x02,
    #[deku(id = "0x03")]
    Successful = 0x03,
    #[deku(id = "0x04")]
    Failed = 0x04,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
pub enum LogLevel {
    #[deku(id = "0x00")]
    NotSet = 0x00,
    #[deku(id = "0x0A")]
    Debug = 0x0A,
    #[deku(id = "0x14")]
    Info = 0x14,
    #[deku(id = "0x1E")]
    Warning = 0x1E,
    #[deku(id = "0x28")]
    LogError = 0x28,
    #[deku(id = "0x32")]
    Critical = 0x32,
}
