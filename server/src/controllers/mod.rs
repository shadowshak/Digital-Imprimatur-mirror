pub mod user;
pub mod data;
pub mod session;

use tokio::sync::{Mutex, MutexGuard};

use once_cell::sync::OnceCell;
pub use user::*;

use self::{data::DatabaseController, session::SessionController};

static DATABASE: OnceCell<Mutex<DatabaseController>> = OnceCell::new();
static USER: OnceCell<Mutex<UserController>> = OnceCell::new();
static SESSION: OnceCell<Mutex<SessionController>> = OnceCell::new();

pub struct Controller {}

impl Controller {
    ///
    /// Returns a reference to the user controller
    /// 
    pub async fn user<'a>() -> MutexGuard<'a, UserController> {
        USER
            .get_or_init(Default::default)
            .lock()
            .await
    }

    ///
    /// Returns a reference to the database controller
    /// 
    pub async fn database<'a>() -> MutexGuard<'a, DatabaseController> {
        DATABASE
            .get_or_init(Default::default)
            .lock()
            .await
    }

    ///
    /// Returns a reference to the session controller
    /// 
    pub async fn session<'a>() -> MutexGuard<'a, SessionController> {
        SESSION
            .get_or_init(Default::default)
            .lock()
            .await
    }
}