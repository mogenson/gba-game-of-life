#![no_std]
#![feature(start)]

use gba_game_of_life::Universe;
use gba::io::{
    display::{DisplayControlSetting, DisplayMode, DISPCNT},
    keypad::KEYINPUT,
    timers::{TimerControlSetting, TM0CNT_H, TM0CNT_L},
};
use panic_abort as _;
use voladdress::VolAddress;

const BG2PA: VolAddress<u16> = unsafe { VolAddress::new(0x400_0020) };
const BG2PD: VolAddress<u16> = unsafe { VolAddress::new(0x400_0026) };

#[start]
fn main(_argc: isize, _argv: *const *const u8) -> isize {
    DISPCNT.write(
        DisplayControlSetting::new()
            .with_mode(DisplayMode::Mode5)
            .with_bg2(true),
    );

    // scale background
    let scale: u16 = 1 << 7;
    BG2PA.write(scale);
    BG2PD.write(scale);

    let mut universe = Universe::new();
    universe.populate(0xDEADBEEF);

    // start free-running timer
    TM0CNT_H.write(TimerControlSetting::new().with_enabled(true));
    loop {
        // any button pressed
        if KEYINPUT.read() < 0x03FF {
            let seed = TM0CNT_L.read() as u64; // current timer count
            universe.populate(seed); // repopulate universe
        }
        universe.step();
    }
}
