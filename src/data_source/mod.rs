pub(crate) mod core;
pub(crate) mod query;

pub(crate) mod datastore_manager;
pub(crate) mod output;

pub use output::Output;
pub use datastore_manager::DataBaseManager;
pub use query::{
    statement_models::{
        create::{create, Create},
        select::{select, Select},
        update::{update, Update},
        insert::{insert, Insert},
        delete::{delete, Delete},
    },
    Query,
};