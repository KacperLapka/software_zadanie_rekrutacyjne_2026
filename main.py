from communication_library import ids
from communication_library.communication_manager import CommunicationManager
from communication_library.exceptions import (
    TransportTimeoutError,
    UnregisteredCallbackError,
)
from communication_library.tcp_transport import TcpSettings
from communication_library.frame import Frame
from communication_library.transport import TransportType

import curses
from enum import Enum
import threading
import time


class Toggle(Enum):
    OFF = 0
    ON = 1

class Servo(Enum):
    FUEL_INTAKE = 0
    OXIDIZER_INTAKE = 1
    FUEL_MAIN = 2
    OXIDIZER_MAIN = 3

class ServoManager:
    cm: CommunicationManager

    def __init__(self, cm: CommunicationManager) -> None:
        self.cm = cm

    def set(self, device: Servo, position: int):
        f = Frame(
            ids.BoardID.ROCKET,
            ids.PriorityID.HIGH,
            ids.ActionID.FEED,
            ids.BoardID.SOFTWARE,
            ids.DeviceID.SERVO,
            device.value,
            ids.DataTypeID.INT16,
            ids.OperationID.SERVO.value.POSITION,
            (position, ), # type: ignore
        )

        self.cm.push(f)
        self.cm.send()

class Relay(Enum):
    OXIDIZER_HEATER = 0
    IGNITER = 1
    PARACHUTE = 2

class RelaysManager:
    cm: CommunicationManager

    def __init__(self, cm: CommunicationManager) -> None:
        self.cm = cm

    def set(self, device: Relay, option: Toggle):
        operation = None
        if option == Toggle.OFF:
            operation = ids.OperationID.RELAY.value.CLOSE
        else:
            operation = ids.OperationID.RELAY.value.OPEN

        f = Frame(
            ids.BoardID.ROCKET,
            ids.PriorityID.HIGH,
            ids.ActionID.FEED,
            ids.BoardID.SOFTWARE,
            ids.DeviceID.RELAY,
            device.value,
            ids.DataTypeID.INT16,
            operation,
        )

        self.cm.push(f)
        self.cm.send()
    


class Stats:
    fuel_intake: float
    oxidizer_intake: float
    fuel_main: float
    oxidizer_main: float
    fuel_level: float
    oxidizer_level: float
    altitude: float
    previous_altitude: float
    pressure: float
    angle: float
    oxidizer_heater: int
    igniter: int
    parachute: int

    def __init__(self) -> None:
        self.fuel_intake = 0.0
        self.oxidizer_intake = 0.0
        self.fuel_main = 0.0
        self.oxidizer_main = 0.0
        self.fuel_level = 0.0
        self.oxidizer_level = 0.0
        self.altitude = 0.0
        self.pressure = 0.0
        self.angle = 0.0
        self.oxidizer_heater = 0
        self.igniter = 0
        self.parachute = 0
        self.previous_altitude = 0.0
    
    @property
    def velocity(self):
        return (s.altitude - s.previous_altitude)


s = Stats()


def print_info(stdscr: curses.window, s: Stats):
    stdscr.clear()
    stdscr.addstr(0, 0, "------------------------")
    stdscr.addstr(2, 0, f"altitude: {s.altitude:.2f}")
    stdscr.addstr(3, 0, f"velocity: {s.velocity:.2f}")
    stdscr.addstr(4, 0, f"fuel_level: {s.fuel_level:.2f}")
    stdscr.addstr(5, 0, f"oxidizer_level: {s.oxidizer_level:.2f}")
    stdscr.addstr(6, 0, f"pressure: {s.pressure:.2f}")
    stdscr.addstr(7, 0, f"angle: {s.angle:.2f}")
    stdscr.addstr(8, 0, "servos:")
    stdscr.addstr(9, 0, f"  fuel_intake={s.fuel_intake:.2f}, oxidizer_intake={s.oxidizer_intake:.2f}, fuel_main={s.fuel_main:.2f}, oxidizer_main={s.oxidizer_main:.2f}")
    stdscr.addstr(10, 0, f"relays: oxidizer_heater={s.oxidizer_heater}, igniter={s.igniter}, parachute={s.parachute}")
    stdscr.addstr(0, 0, "------------------------")
    stdscr.refresh()

def parse_altitude_frame(frame: Frame):
    if isinstance(frame.data, (int, float)):
        s.altitude = frame.data


def parse_servo_frame(frame: Frame):
    if not isinstance(frame.data, (int, float)):
        return

    match frame.device_id:
        case 0:
            s.fuel_intake = float(frame.data)
        case 1:
            s.oxidizer_intake = float(frame.data)
        case 2:
            s.fuel_main = float(frame.data)
        case 3:
            s.oxidizer_main = float(frame.data)
        case _:
            pass


def parse_sensor_frame(frame: Frame):
    if not isinstance(frame.data, (int, float)):
        return

    match frame.device_id:
        case 0:
            s.fuel_level = float(frame.data)
        case 1:
            s.oxidizer_level = float(frame.data)
        case 2:
            s.previous_altitude = s.altitude
            s.altitude = float(frame.data)
        case 3:
            s.pressure = float(frame.data)
        case 4:
            s.angle = float(frame.data)
        case _:
            pass


def parse_relay_frame(frame: Frame):
    if not isinstance(frame.data, int):
        return

    if frame.operation == ids.OperationID.RELAY.value.OPEN:
        value = 1
    else:
        value = 0
    match frame.device_id:
        case 0:
            s.oxidizer_heater = value
        case 1:
            s.igniter = value
        case 2:
            s.parachute = value
        case _:
            pass



def setup_callbacks(cm: CommunicationManager):
    for i in range(4):
        frame = Frame(ids.BoardID.SOFTWARE, 
                           ids.PriorityID.LOW, 
                           ids.ActionID.FEED, 
                           ids.BoardID.ROCKET, 
                           ids.DeviceID.SERVO, 
                           i, # servo sensors
                           ids.DataTypeID.FLOAT,
                           ids.OperationID.SERVO.value.POSITION)
        cm.register_callback(parse_servo_frame, frame) # type: ignore
    
    for i in range(5):
        frame = Frame(ids.BoardID.SOFTWARE, 
                           ids.PriorityID.LOW, 
                           ids.ActionID.FEED, 
                           ids.BoardID.ROCKET, 
                           ids.DeviceID.SENSOR, 
                           i, # sensor sensors
                           ids.DataTypeID.FLOAT,
                           ids.OperationID.SENSOR.value.READ)
        cm.register_callback(parse_sensor_frame, frame) # type: ignore

    for i in range(1):
        frame = Frame(ids.BoardID.SOFTWARE, 
                           ids.PriorityID.HIGH, 
                           ids.ActionID.ACK, 
                           ids.BoardID.ROCKET, 
                           ids.DeviceID.RELAY, 
                           0, # relay sensors
                           ids.DataTypeID.INT16,
                           ids.OperationID.RELAY.value.OPEN,
                           )
        cm.register_callback(parse_relay_frame, frame) # type: ignore
        frame = Frame(ids.BoardID.SOFTWARE, 
                           ids.PriorityID.HIGH, 
                           ids.ActionID.ACK, 
                           ids.BoardID.ROCKET, 
                           ids.DeviceID.RELAY, 
                           0, # relay sensors
                           ids.DataTypeID.INT16,
                           ids.OperationID.RELAY.value.CLOSE,
                           )
        cm.register_callback(parse_relay_frame, frame) # type: ignore

def setup_flight(stats: Stats, rl: RelaysManager, sl: ServoManager):
    sl.set(Servo.OXIDIZER_INTAKE, 0)
    while stats.oxidizer_level < 100:
        time.sleep(0.1)
    sl.set(Servo.OXIDIZER_INTAKE, 100)
    sl.set(Servo.FUEL_INTAKE, 0)
    while stats.fuel_level < 100:
        time.sleep(0.1)
    sl.set(Servo.FUEL_INTAKE, 100)
    rl.set(Relay.OXIDIZER_HEATER, Toggle.ON)

def listening_thread(cm: CommunicationManager):
    while True:
        try:
            _ = cm.receive()
            time.sleep(0.1)
        except TransportTimeoutError:
            pass
        except UnregisteredCallbackError as e:
            print(f"Unregistered callback error: {str(e)}")

def preasure_watchdog(rl: RelaysManager, s: Stats):
    while True:
        if s.pressure > 63 and s.oxidizer_heater == 1:
            rl.set(Relay.OXIDIZER_HEATER, Toggle.OFF)
        elif s.pressure < 57 and s.oxidizer_heater == 0:
            rl.set(Relay.OXIDIZER_HEATER, Toggle.ON)

        time.sleep(0.1)

def launch(rl: RelaysManager, sl: ServoManager, s: Stats):
    while s.pressure < 55:
        time.sleep(0.5)
    sl.set(Servo.FUEL_MAIN, 0)
    sl.set(Servo.OXIDIZER_MAIN, 0)
    time.sleep(0.5)
    rl.set(Relay.IGNITER, Toggle.ON)
    while s.fuel_level > 0 or s.oxidizer_level > 0:
        time.sleep(0.1)

def landing(rl: RelaysManager, s: Stats):
    while s.velocity > 0:
        time.sleep(0.1)
    rl.set(Relay.PARACHUTE, Toggle.ON)

def stats_thread(stdscr: curses.window, s: Stats):
    while True:
        print_info(stdscr, s)
        time.sleep(1)

def main(stdscr: curses.window):
    curses.curs_set(0)
    stdscr.timeout(1000)

    cm = CommunicationManager()
    cm.change_transport_type(TransportType.TCP)

    cm.connect(TcpSettings("127.0.0.1", 3000))

    setup_callbacks(cm)

    rl = RelaysManager(cm)
    sl = ServoManager(cm)

    threading.Thread(target=listening_thread, args=(cm,)).start()
    threading.Thread(target=stats_thread, args=(stdscr, s)).start()

    setup_flight(s, RelaysManager(cm), ServoManager(cm))
    threading.Thread(target=preasure_watchdog, args=(rl, s)).start()


    launch(rl, sl, s)
    landing(rl, s)

    while True:
        time.sleep(0.1)


curses.wrapper(main)
