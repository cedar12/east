use actix_plugin::ActixPlugin;
use east_plugin::plugin::{Plugin, WebPlugin};


mod actix_plugin;
mod route;


#[no_mangle]
pub extern "C" fn install() -> *mut dyn Plugin {
  Box::into_raw(Box::new(ActixPlugin)) as *mut dyn Plugin
}


#[no_mangle]
pub extern "C" fn create() -> *mut dyn WebPlugin{
    Box::into_raw(Box::new(ActixPlugin)) as *mut dyn WebPlugin
}

