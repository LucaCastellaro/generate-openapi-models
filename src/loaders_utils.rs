use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;

pub const OK: &str = "▪▪▪▪▪";

pub fn get_spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();

    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.034} {msg}")
            .unwrap()
            .tick_strings(&[
                "▪    ",
                " ▪   ",
                "  ▪  ",
                "   ▪ ",
                "    ▪",
                "   ▪ ",
                "  ▪  ",
                " ▪   ",
                OK,
            ]),
    );

    pb.set_message(message.to_string());

    return pb;
}