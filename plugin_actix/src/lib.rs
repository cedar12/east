
// extern crate east_sqlite;
use actix_plugin::ActixPlugin;
use east_plugin::plugin::{Plugin, WebPlugin};

#[cfg(test)]
mod tests;

mod actix_plugin;
mod route;
mod auth;
mod user_data;
mod model;

// mod tests;

#[no_mangle]
pub extern "C" fn install() -> *mut dyn Plugin {
  Box::into_raw(Box::new(ActixPlugin)) as *mut dyn Plugin
}


#[no_mangle]
pub extern "C" fn create() -> *mut dyn WebPlugin{
    Box::into_raw(Box::new(ActixPlugin)) as *mut dyn WebPlugin
}

