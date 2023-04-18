use std::time::Duration;

use colored::Color;
use colored::Colorize;
use rand::random;

extern crate termion;

/// This is the main function... obviously
fn main() {
    // todo: use some clever enum to array thingy to do this automatically
    let colors = [
        Color::Black,
        Color::Red,
        Color::Green,
        Color::Yellow,
        Color::Blue,
        Color::Magenta,
        Color::Cyan,
        Color::White,
        Color::BrightBlack,
        Color::BrightRed,
        Color::BrightGreen,
        Color::BrightYellow,
        Color::BrightBlue,
        Color::BrightMagenta,
        Color::BrightCyan,
        Color::BrightWhite,
    ];

    let mut i = 3;

    print!("{}", termion::clear::All);

    let size = termion::terminal_size().unwrap();

    loop {
        for y in 4..(size.1 - 2) {
            for x in 6..(size.0 - 3) {
                let this_color = colors[i];

                let chars = ["#"];

                let index = (random::<f32>() * (chars.len() as f32)) as usize;
                let char: &str = chars[index];
                let some_text = char.color(this_color);

                //     // Do some random here
                if random::<f32>() > 0.2 {
                    i += 1;
                }

                i %= colors.len();
                print!("{}{}", termion::cursor::Goto(x, y), some_text);
            }
        }
        std::thread::sleep(Duration::from_millis(90));
    }
}
