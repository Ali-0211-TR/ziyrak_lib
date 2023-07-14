pub(crate) mod create;
pub(crate) mod insert;
pub(crate) mod delete;
pub(crate) mod select;
pub(crate) mod update;

pub(crate) use create::Create;
pub(crate) use insert::Insert;
pub(crate) use delete::Delete;
pub(crate) use select::Select;
pub(crate) use update::Update;

pub trait StatementTrait {
    fn get_string(&self) -> String;
}