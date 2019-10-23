use std::sync::Mutex;

#[derive(Debug)]
pub struct RjOption {
    pub is_debug: bool,
}

impl RjOption {
    pub fn new() -> RjOption {
        RjOption { is_debug: false }
    }
}

lazy_static! {
    pub static ref RJ_OPTION: Mutex<RjOption> = Mutex::new(RjOption::new());
}
