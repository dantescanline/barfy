
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

    let mut i = 3.0;

    print!("{}", termion::clear::All);

    loop {
        let size = termion::terminal_size().unwrap();
        i += 0.1;

        let min_x = 6;
        let max_x = size.0 - 3;
        let min_y = 4;
        let max_y = size.1 - 2;

        for y in min_y..max_y {
            for x in min_x..max_x {
                let mut s1 = f32::sin(x as f32 / 8.0 + i);
                let mut s2 = f32::sin((y as f32 / 6.0) + 1.0 + i);
                let final_s = (s1 + s2) / 4.0 + 0.5;

                // let this_color = colors[i];
                let mut this_color = Color::White;
                
                let mut char = "O";
                if(x == min_x || x == max_x -1) {
                    char = "|";
                } else if(y == min_y || y == max_y-1) {
                    char ="=";
                } else {
                    this_color = colors[ (final_s * colors.len() as f32) as usize ];
                }
                let some_text = char.color(this_color);


                print!("{}{}", termion::cursor::Goto(x, y), some_text);
            }
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}
