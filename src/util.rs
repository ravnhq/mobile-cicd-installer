#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        use colorful::{Color, Colorful};
        let prompt = ">".color(Color::LightGreen);
        println!("{prompt} {}", format!($($arg)*));
    }};
}
