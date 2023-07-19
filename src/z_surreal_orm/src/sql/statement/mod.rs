pub(crate) mod create;
pub(crate) mod insert;
pub(crate) mod delete;
pub(crate) mod select;
pub(crate) mod update;

pub use create::Create;
pub use insert::Insert;
pub use delete::Delete;
pub use select::Select;
pub use update::Update;

pub trait Statement: std::fmt::Display{}
