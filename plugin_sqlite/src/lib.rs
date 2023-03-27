#[macro_use]
extern crate lazy_static; 
use east_plugin::plugin::{Plugin, DatabasePlugin};
use sqlite_plugin::SqlitePlugin;

extern crate east_plugin;
pub mod sqlite_plugin;
mod db;

mod tests;

#[no_mangle]
pub extern "C" fn install() -> *mut dyn Plugin {
    Box::into_raw(Box::new(SqlitePlugin)) as *mut dyn Plugin
}

#[no_mangle]
pub extern "C" fn create() -> *mut dyn DatabasePlugin {
    Box::into_raw(Box::new(SqlitePlugin)) as *mut dyn DatabasePlugin
}

