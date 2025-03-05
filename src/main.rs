#![no_std]
#![no_main]

pub mod game;

use embedded_hal::{delay::DelayNs, digital::InputPin};
use game::Game;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{timer::Timer, Delay, Uarte},
};
extern crate panic_semihosting;

#[cortex_m_rt::entry]
fn main() -> ! {
    let mut board = Board::take().unwrap();
    let mut delay = Delay::new(board.SYST);

    let mut display = Display::new(board.display_pins);

    let mut game = Game::new();

    let mut button_c = board.edge.e12.into_pullup_input();
    let mut button_d = board.pins.p0_17.into_pullup_input();
    let mut button_e = board.pins.p0_01.into_pullup_input();
    let mut button_f = board.pins.p0_13.into_pullup_input();

    loop {
        if button_c.is_low().unwrap() {
            game.cursor_left();
        }
        if button_e.is_low().unwrap() {
            game.cursor_up();
        }
        if button_d.is_low().unwrap() {
            game.cursor_down();
        }
        if button_f.is_low().unwrap() {
            game.cursor_right();
        }
        if board.buttons.button_a.is_low().unwrap() {
            game.tick();
        }
        if board.buttons.button_b.is_low().unwrap() {
            game.place();
        }
        let drawn = game.draw();
        display.show(&mut delay, drawn, 100);
    }
}
