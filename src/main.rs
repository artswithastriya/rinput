pub mod keycode;
use clap::Parser;
use mouse_keyboard_input::VirtualDevice;

/// Simple program to greet a person
#[derive(Parser, Debug)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    key: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
enum Command {
    Click(Args),
    Press(Args),
    Release(Args),
    MoveMouse(MoveArgs),
    ScrollVertical(ScrollArgs),
    ScrollHorizontal(ScrollArgs),
}

#[derive(Parser, Debug)]
struct MoveArgs {
    #[arg(short, long)]
    x: i32,
    #[arg(short, long)]
    y: i32,
}

#[derive(Parser, Debug)]
struct ScrollArgs {
    #[arg(short, long)]
    scroll: i32,
}
fn main() {
    let command = Command::parse();

    let mut device = VirtualDevice::default().unwrap();
    match &command {
        Command::Click(args) | Command::Press(args) | Command::Release(args) => {
            let Some(key) = keycode::match_str(&args.key) else {
                return println!("{} can't be used as a key input. It has to be either an integer or one of the supported values", args.key);
            };

            match &command {
                Command::Click(_) => device.click(key),
                Command::Press(_) => device.press(key),
                Command::Release(_) => device.release(key),
                _ => unreachable!(),
            }
            .unwrap();
        }

        Command::MoveMouse(args) => device.move_mouse(args.x, args.y).unwrap(),
        Command::ScrollVertical(args) => device.scroll_vertical(args.scroll).unwrap(),
        Command::ScrollHorizontal(args) => device.scroll_horizontal(args.scroll).unwrap(),
    }
}
