mod logs;

use logs::logging;

fn main() {
    logs::init_logger();

    logging::log_info!("Example for info");
    logging::log_debug!("Example for debug");
    logging::log_warn!("Example for warn");
    logging::log_trace!("Example for trace");
    logging::log_error!("Example for error");

    some_function_somewhere_in_code();
    somewhere_deep::something_deep();
}



fn some_function_somewhere_in_code() {
    logging::log_info!("Example somewhere in function");
}


mod somewhere_deep {
    use crate::*;

    pub fn something_deep() {
        logging::log_info!("Example inside somewhere deep");
    }
}



