mod user;
mod data;

use std::{sync::{Mutex, MutexGuard}};

use once_cell::sync::OnceCell;
pub use user::*;

use self::data::DatabaseController;

static DATABASE: OnceCell<Mutex<DatabaseController>> = OnceCell::new();
static USER: OnceCell<Mutex<UserController>> = OnceCell::new();

pub struct Controller {}

impl Controller {
    pub fn user<'a>() -> MutexGuard<'a, UserController> {
        USER
            .get_or_init(|| todo!())
            .lock()
            .unwrap()
    }

    ///
    /// Returns a reference to the database controller
    /// 
    pub fn database<'a>() -> MutexGuard<'a, DatabaseController> {
        DATABASE
            .get_or_init(|| todo!())
            .lock()
            .unwrap()

    }
}