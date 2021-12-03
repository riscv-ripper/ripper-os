use log::{self, Level, LevelFilter, Log, Metadata, Record};
use core::fmt;

pub fn init() {
    static LOGGER: SimpleLogger = SimpleLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(match option_env!("LOG") {
        Some("error") => LevelFilter::Error,
        Some("warn") => LevelFilter::Warn,
        Some("info") => LevelFilter::Info,
        Some("debug") => LevelFilter::Debug,
        Some("trace") => LevelFilter::Trace,
        _ => LevelFilter::Trace,
    });
}

macro_rules! with_color {
    ($args: ident, $color_code: ident) => {{
        format_args!("\u{1B}[{}m{}\u{1B}[0m", $color_code as u8, $args)
    }};
}

fn print_in_color(args: fmt::Arguments, color_code: u8) {
    crate::console::print(with_color!(args, color_code));
}


struct SimpleLogger;

impl Log for SimpleLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }
    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        /*
        if let Some(tid) = processor().tid_option() {
            print_in_color(
                format_args!(
                    "[{:>5}][{},{}] {}\n",
                    record.level(),
                    crate::arch::cpu::id(),
                    tid,
                    record.args()
                ),
                level_to_color_code(record.level()),
            );
        } else {
            */
        print_in_color(
            format_args!(
                "[{:>5}] {}\n",
                record.level(),
                record.args()
            ),
            level_to_color_code(record.level()),
        );
        //}
    }
    fn flush(&self) {}
}

fn level_to_color_code(level: Level) -> u8 {
    match level {
        Level::Error => 31, // Red
        Level::Warn => 93,  // BrightYellow
        Level::Info => 34,  // Blue
        Level::Debug => 32, // Green
        Level::Trace => 90, // BrightBlack
    }
}
