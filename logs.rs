use env_logger;
use chrono::{Local};
use std::io::{Write};



pub mod logging {
    #[macro_export]
    macro_rules! full_path_with_line {
        () => {{
            let loc = std::panic::Location::caller();
            let line = loc.line();

            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }

            fn f() {}
            let full_name = type_name_of(f);
            let mut parts: Vec<&str> = full_name.split("::").collect();

            while let Some(last) = parts.last() {
                if *last == "f" || *last == "{{closure}}" {
                    parts.pop();
                } else {
                    break;
                }
            }

            let function_name = parts.last().unwrap_or(&"unknown");
            let module_path = module_path!();

            format!("{}::{}::{}", module_path, function_name, line)
        }};
    }


    #[macro_export]
    macro_rules! log_trace {
        ($($arg:tt)+) => {{
            let path = $crate::full_path_with_line!();
            log::trace!(target: &path, $($arg)+);
        }};
    }

    #[macro_export]
    macro_rules! log_debug {
        ($($arg:tt)+) => {{
            let path = $crate::full_path_with_line!();
            log::debug!(target: &path, $($arg)+);
        }};
    }

    #[macro_export]
    macro_rules! log_info {
        ($($arg:tt)+) => {{
            let path = $crate::full_path_with_line!();
            log::info!(target: &path, $($arg)+);
        }};
    }

    #[macro_export]
    macro_rules! log_warn {
        ($($arg:tt)+) => {{
            let path = $crate::full_path_with_line!();
            log::warn!(target: &path, $($arg)+);
        }};
    }

    #[macro_export]
    macro_rules! log_error {
        ($($arg:tt)+) => {{
            let path = $crate::full_path_with_line!();
            log::error!(target: &path, $($arg)+);
        }};
    }

    pub(crate) use log_trace;
    pub(crate) use log_debug;
    pub(crate) use log_info;
    pub(crate) use log_warn;
    pub(crate) use log_error;
}




fn init_logger() {
    env_logger::Builder::new()
        .format(|buf, record| {

            let module = record.target();

            let timestamp = Local::now().format("%d-%m-%Y %H:%M:%S%.3f");
            let level = format!("{:<5}", record.level());
            let module_formatted = format!("{:<50}", module);

            let timestamp_color = "\x1b[90m";
            let level_color = match record.level() {
                log::Level::Error => "\x1b[31m",
                log::Level::Warn => "\x1b[33m",
                log::Level::Info => "\x1b[32m",
                log::Level::Debug => "\x1b[36m",
                log::Level::Trace => "\x1b[90m",
            };

            let module_color = "\x1b[1;96m";
            let reset = "\x1b[0m";

            writeln!(
                buf,
                "[{}{}{} {}{}{} {}{}{}] {}",
                timestamp_color,
                timestamp,
                reset,
                level_color,
                level,
                reset,
                module_color,
                module_formatted,
                reset,
                record.args()
            )
        })
        .parse_filters(&std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()))
        .init();
}
