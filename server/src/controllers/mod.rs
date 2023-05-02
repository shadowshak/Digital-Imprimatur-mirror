pub mod user;
pub mod data;
pub mod session;
mod doc;

use tokio::sync::{Mutex, MutexGuard};

use once_cell::sync::OnceCell;
pub use user::*;
pub use doc::*;

use self::{data::DatabaseController, session::SessionController, doc::DocumentController};

static USER: OnceCell<Mutex<UserController>> = OnceCell::new();
static SESSION: OnceCell<Mutex<SessionController>> = OnceCell::new();
static DOCUMENT: OnceCell<Mutex<DocumentController>> = OnceCell::new();
static DATABASE: OnceCell<Mutex<DatabaseController>> = OnceCell::new();

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

    ///
    /// Returns a reference to the document controller
    /// 
    pub async fn document<'a>() -> MutexGuard<'a, DocumentController> {
        DOCUMENT
            .get_or_init(Default::default)
            .lock()
            .await
    }
}