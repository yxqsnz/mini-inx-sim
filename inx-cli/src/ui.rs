use once_cell::sync::Lazy;
use owo_colors::OwoColorize;

#[macro_export]
macro_rules! info {
    ($($item:tt)*) => {
        $crate::ui::info(format!($($item)*))
    };
}

const INFORMATION: Lazy<String> = Lazy::new(|| "Information:".green().bold().to_string());

pub fn info(text: String) {
    println!("{} {text}", INFORMATION.as_str())
}
