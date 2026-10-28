use std::fmt;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, MutexGuard};
use tokio::time::interval;

use communication_library_rs::frame::{Frame, FrameError};
use communication_library_rs::ids::{ActionID, BoardID, DeviceID, RelayOperationID};

use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[derive(Debug, Clone, Default)]
struct Stats {
    fuel_intake: i16,
    oxidizer_intake: i16,
    fuel_main: i16,
    oxidizer_main: i16,
    fuel_level: f32,
    oxidizer_level: f32,
    altitude: f32,
    previous_altitude: f32,
    pressure: f32,
    angle: f32,
    oxidizer_heater: i32,
    igniter: i32,
    parachute: i32,
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Telemetry")?;
        writeln!(
            f,
            "  Fuel intake      {:>8.2}   Oxidizer intake  {:>8.2}",
            self.fuel_intake, self.oxidizer_intake
        )?;
        writeln!(
            f,
            "  Fuel main       {:>8.2}   Oxidizer main    {:>8.2}",
            self.fuel_main, self.oxidizer_main
        )?;
        writeln!(
            f,
            "  Fuel level      {:>8.2}   Oxidizer level   {:>8.2}",
            self.fuel_level, self.oxidizer_level
        )?;
        // This works probably because the simulator sends altitude FEED every second, won't work in real rocket
        writeln!(
            f,
            "  Altitude        {:>8.2}   Velocity    {:>8.2}",
            self.altitude, self.altitude - self.previous_altitude         )?;
        writeln!(
            f,
            "  Pressure        {:>8.2}   Angle            {:>8.2}",
            self.pressure, self.angle
        )?;
        writeln!(
            f,
            "  Oxidizer heater {:>5}   Igniter          {:>5}   Parachute {:>5}",
            self.oxidizer_heater, self.igniter, self.parachute
        )
    }
}

fn match_frame_feed(frame: Frame, mut stats: MutexGuard<'_, Stats>) -> Result<(), FrameError> {
    match (frame.device_type, frame.device_id) {
            (DeviceID::Servo, 0) => stats.fuel_intake = frame.payload.try_into()?,
            (DeviceID::Servo, 1) => stats.oxidizer_intake = frame.payload.try_into()?,
            (DeviceID::Servo, 2) => stats.fuel_main = frame.payload.try_into()?,
            (DeviceID::Servo, 3) => stats.oxidizer_main = frame.payload.try_into()?,
            (DeviceID::Sensor, 0) => stats.fuel_level = frame.payload.try_into()?,
            (DeviceID::Sensor, 1) => stats.oxidizer_level = frame.payload.try_into()?,
            (DeviceID::Sensor, 2) => {
                stats.previous_altitude = stats.altitude;
                stats.altitude = frame.payload.try_into()?
            },
            (DeviceID::Sensor, 3) => stats.pressure = frame.payload.try_into()?,
            (DeviceID::Sensor, 4) => stats.angle = frame.payload.try_into()?,
            _ => return Err(FrameError::DeviceIdOutOfRange(frame.device_id))
    };
    Ok(())
}

fn match_frame_akc(frame: Frame, mut stats: MutexGuard<'_, Stats>) -> Result<(), FrameError> {
    if frame.device_type != DeviceID::Relay {
        return Ok(());
    }
    let value = {
        if frame.operation == RelayOperationID::Open.into() {
            1
        } else {
            0
        }
    };

    match frame.device_id {
        0 => stats.oxidizer_heater = value,
        1 => stats.igniter = value,
        2 => stats.parachute = value,
        id => return Err(FrameError::DeviceIdOutOfRange(id))
    };
    Ok(())
}

fn match_frame_nakc(frame: Frame, _: MutexGuard<'_, Stats>) -> Result<(), FrameError>{
    println!("Error: {:?}", frame);
    Ok(())
}

fn handle_frame(frame: Frame, stats: Arc<Mutex<Stats>>) {
    tokio::spawn(async move {
        if frame.destination != BoardID::Software {
            return;
        }

        let stats = stats.lock().await;
        let r = match frame.action {
            ActionID::Feed => match_frame_feed(frame, stats),
            ActionID::Ack => match_frame_akc(frame, stats),
            ActionID::Nack => match_frame_nakc(frame, stats),
            _ => Ok(())
        };
        if let Err(err) = r {
            println!("Error: {}, frame: {:?}", err, frame);
        }
        
    });
}

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("127.0.0.1:3000").await.unwrap();
    let (mut reader, _writer) = stream.into_split();

    let stats = Arc::new(Mutex::new(Stats::default()));

    let stats_clone = Arc::clone(&stats);
    let read_task = tokio::spawn(async move {
        let mut buf = [0u8; 14];
        loop {
            reader.read_exact(&mut buf).await.unwrap();
            let frame = Frame::unpack(&buf).unwrap();

            handle_frame(frame, stats_clone.clone());
        }
    });
    let _display_task = tokio::spawn(async move {
        let mut ticker = interval(Duration::from_millis(250));

        loop {
            ticker.tick().await;
            let stats = stats.lock().await;
            println!("\n{}", stats);
        }
    });

    let _ = tokio::join!(read_task);
}
