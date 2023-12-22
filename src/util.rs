#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        use colorful::Color;
        use colorful::Colorful;
        let prompt = ">".color(Color::LightGreen);
        println!("{prompt} {}", format!($($arg)*));
    }};
}
