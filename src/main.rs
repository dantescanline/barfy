use colored::Color;
use colored::Colorize;
use rand::random;

fn main() {
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

    for x in 1..20002 {
        let this_color = colors[i];
        let some_text = "#".color(this_color);
        print!("{}", some_text);
        if rand::random::<f32>() > 0.8 {
            i += 1;
        }
        i = i % colors.len()
    }
    println!("")
}
