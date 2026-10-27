use communication_library_rs::frame::Frame;
use communication_library_rs::ids::{
    ActionID, BoardID, DataTypeID, DeviceID, PriorityID, ServoOperationID,
};
use communication_library_rs::protocol;

fn main() {
    let frame = Frame::new(
        BoardID::Rocket,
        PriorityID::High,
        ActionID::Feed,
        BoardID::Software,
        DeviceID::Servo,
        0,
        ServoOperationID::Position.into(),
        0i16,
    )
    .unwrap();
    assert_eq!(frame.data_type, DataTypeID::Int16);
    let bytes = protocol::encode(&frame).unwrap();
    for byte in &bytes {
        print!("{:02x} ", byte);
    }
    println!();

    let bytes = frame.pack().unwrap();
    for byte in &bytes {
        print!("{:02x} ", byte);
    }
    let decoded = Frame::unpack(&bytes).unwrap();
    assert_eq!(frame, decoded);
    println!();
}
