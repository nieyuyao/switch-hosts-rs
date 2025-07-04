mod debug_log;
mod log;

#[cfg(debug_assertions)]
pub use self::debug_log::init_logger;
#[cfg(not(debug_assertions))]
pub use self::log::init_logger;