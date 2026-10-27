use crc::{CRC_32_MPEG_2, Crc};
use deku::ctx::{Endian, Order};
use deku::prelude::*;
use thiserror::Error;

use crate::ids::{ActionID, BoardID, DataTypeID, DeviceID, Operation, PriorityID};

/// Maximum value fitting in the 6-bit `device_id` field.
pub const MAX_DEVICE_ID: u8 = 63;

#[derive(Debug, Error, PartialEq)]
pub enum FrameError {
    #[error("device_id {0} does not fit in 6 bits (max {MAX_DEVICE_ID})")]
    DeviceIdOutOfRange(u8),
    #[error("payload {0:?} does not match data_type {1:?}")]
    PayloadMismatch(Payload, DataTypeID),
    #[error("payload padding must be zero")]
    NonZeroPadding,
    #[error("frame codec error: {0}")]
    Deku(#[from] DekuError),
    #[error("decoding error: {0}")]
    Decoding(String),
}

#[derive(Debug, Clone, Copy, PartialEq, DekuRead, DekuWrite)]
#[deku(
    ctx = "endian: Endian, order: Order, data_type: DataTypeID",
    endian = "endian",
    bit_order = "order",
    id = "data_type"
)]
pub enum Payload {
    #[deku(id = "DataTypeID::NoData")]
    NoData(
        /// Padding word, always zero on the wire.
        #[deku(bits = 32)]
        u32,
    ),
    #[deku(id = "DataTypeID::Uint32")]
    Uint32(#[deku(endian = "little")] u32),
    #[deku(id = "DataTypeID::Uint16")]
    Uint16(#[deku(endian = "little")] u16, #[deku(bits = 16)] u16),
    #[deku(id = "DataTypeID::Uint8")]
    Uint8(u8, #[deku(bits = 24)] u32),
    #[deku(id = "DataTypeID::Int32")]
    Int32(#[deku(endian = "little")] i32),
    #[deku(id = "DataTypeID::Int16")]
    Int16(#[deku(endian = "little")] i16, #[deku(bits = 16)] u16),
    #[deku(id = "DataTypeID::Int8")]
    Int8(i8, #[deku(bits = 24)] u32),
    #[deku(id = "DataTypeID::Float")]
    Float(#[deku(endian = "little")] f32),
    #[deku(id = "DataTypeID::Int16x2")]
    Int16x2(
        #[deku(endian = "little")] i16,
        #[deku(endian = "little")] i16,
    ),
    #[deku(id = "DataTypeID::Uint16Int16")]
    Uint16Int16(
        #[deku(endian = "little")] u16,
        #[deku(endian = "little")] i16,
    ),
}

impl Payload {
    /// The [`DataTypeID`] this variant encodes as.
    pub fn data_type(self) -> DataTypeID {
        match self {
            Payload::NoData(_) => DataTypeID::NoData,
            Payload::Uint32(_) => DataTypeID::Uint32,
            Payload::Uint16(_, _) => DataTypeID::Uint16,
            Payload::Uint8(_, _) => DataTypeID::Uint8,
            Payload::Int32(_) => DataTypeID::Int32,
            Payload::Int16(_, _) => DataTypeID::Int16,
            Payload::Int8(_, _) => DataTypeID::Int8,
            Payload::Float(_) => DataTypeID::Float,
            Payload::Int16x2(_, _) => DataTypeID::Int16x2,
            Payload::Uint16Int16(_, _) => DataTypeID::Uint16Int16,
        }
    }

    /// Number of meaningful values (mirrors Python's `_valid_payload_len`).
    pub fn len(self) -> usize {
        match self {
            Payload::NoData(_) => 0,
            Payload::Int16x2(_, _) | Payload::Uint16Int16(_, _) => 2,
            _ => 1,
        }
    }

    pub fn is_empty(self) -> bool {
        self.len() == 0
    }

    fn check_padding(self) -> Result<(), FrameError> {
        let ok = match self {
            Payload::NoData(pad) => pad == 0,
            Payload::Uint16(_, pad) | Payload::Int16(_, pad) => pad == 0,
            Payload::Uint8(_, pad) | Payload::Int8(_, pad) => pad == 0,
            _ => true,
        };
        if ok {
            Ok(())
        } else {
            Err(FrameError::NonZeroPadding)
        }
    }
}

impl From<()> for Payload {
    fn from((): ()) -> Self {
        Payload::NoData(0)
    }
}

impl From<u32> for Payload {
    fn from(value: u32) -> Self {
        Payload::Uint32(value)
    }
}

impl From<u16> for Payload {
    fn from(value: u16) -> Self {
        Payload::Uint16(value, 0)
    }
}

impl From<u8> for Payload {
    fn from(value: u8) -> Self {
        Payload::Uint8(value, 0)
    }
}

impl From<i32> for Payload {
    fn from(value: i32) -> Self {
        Payload::Int32(value)
    }
}

impl From<i16> for Payload {
    fn from(value: i16) -> Self {
        Payload::Int16(value, 0)
    }
}

impl From<i8> for Payload {
    fn from(value: i8) -> Self {
        Payload::Int8(value, 0)
    }
}

impl From<f32> for Payload {
    fn from(value: f32) -> Self {
        Payload::Float(value)
    }
}

impl From<(i16, i16)> for Payload {
    fn from((first, second): (i16, i16)) -> Self {
        Payload::Int16x2(first, second)
    }
}

impl From<(u16, i16)> for Payload {
    fn from((first, second): (u16, i16)) -> Self {
        Payload::Uint16Int16(first, second)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "little", bit_order = "lsb")]
pub struct Frame {
    pub destination: BoardID,
    pub priority: PriorityID,
    pub action: ActionID,
    pub source: BoardID,
    pub device_type: DeviceID,
    #[deku(bits = 6)]
    pub device_id: u8,
    pub data_type: DataTypeID,
    #[deku(bits = 8)]
    pub operation: Operation,
    #[deku(ctx = "*data_type")]
    pub payload: Payload,
}

impl Frame {
    pub fn try_new(
        destination: BoardID,
        priority: PriorityID,
        action: ActionID,
        source: BoardID,
        device_type: DeviceID,
        device_id: u8,
        data_type: DataTypeID,
        operation: Operation,
        payload: Payload,
    ) -> Result<Self, FrameError> {
        if device_id > MAX_DEVICE_ID {
            return Err(FrameError::DeviceIdOutOfRange(device_id));
        }
        payload.check_padding()?;
        if payload.data_type() != data_type {
            return Err(FrameError::PayloadMismatch(payload, data_type));
        }
        Ok(Self {
            destination,
            priority,
            action,
            source,
            device_type,
            device_id,
            data_type,
            operation,
            payload,
        })
    }

    pub fn new_unchecked(
        destination: BoardID,
        priority: PriorityID,
        action: ActionID,
        source: BoardID,
        device_type: DeviceID,
        device_id: u8,
        data_type: DataTypeID,
        operation: Operation,
        payload: Payload,
    ) -> Self {
        Self {
            destination,
            priority,
            action,
            source,
            device_type,
            device_id,
            data_type,
            operation,
            payload,
        }
    }

    pub fn new(
        destination: BoardID,
        priority: PriorityID,
        action: ActionID,
        source: BoardID,
        device_type: DeviceID,
        device_id: u8,
        operation: Operation,
        payload: impl Into<Payload>,
    ) -> Result<Self, FrameError> {
        let payload = payload.into();
        Self::try_new(
            destination,
            priority,
            action,
            source,
            device_type,
            device_id,
            payload.data_type(),
            operation,
            payload,
        )
    }

    pub fn as_reversed_frame(self) -> Self {
        Self {
            destination: self.source,
            source: self.destination,
            ..self
        }
    }

    pub fn pack(self) -> Result<Vec<u8>, FrameError> {
        let mut result: Vec<u8> = Vec::new();
        result.push(0x05); // Header
        let mut b = self.to_bytes()?;
        result.append(&mut b); // Values

        let crc = calculate_crc(&result);
        result.extend_from_slice(&crc);

        Ok(result)
    }

    pub fn unpack(raw: &[u8]) -> Result<Frame, FrameError> {
        if raw.len() != 14 {
            return Err(FrameError::Decoding("incorrect length".to_string()));
        }
        let crc = &raw[10..];
        println!();
        for byte in crc {
            print!("{:02x} ", byte);
        }
        println!();
        let data = &raw[..10];
        for byte in data {
            print!("{:02x} ", byte);
        }
        println!();
        if crc != calculate_crc(data) {
            return Err(FrameError::Decoding("incorrect crc".to_string()));
        }

        let data = &raw[1..10];
        let (_, frame) = Frame::from_bytes((data, 0))?;

        Ok(frame)
    }
}

fn append_padding(mut data: Vec<u8>) -> Vec<u8> {
    let padding = data.len() % 4;
    let mut p = vec![0x00u8; padding];
    data.append(&mut p);
    data
}

// Make crc checksum
// Note: it does not check if received data is the correct size
fn calculate_crc(data: &[u8]) -> [u8; 4] {
    let mixed: Vec<u8> = append_padding(data.to_owned())
        .chunks_exact(4)
        .flat_map(|chunk| {
            let val = u32::from_le_bytes(chunk.try_into().unwrap());
            val.to_be_bytes()
        })
        .collect();
    let mpeg = Crc::<u32>::new(&CRC_32_MPEG_2);
    mpeg.checksum(&mixed).to_le_bytes()
}
