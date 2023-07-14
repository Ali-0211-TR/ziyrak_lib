use crate::data_source::{
    core::vec_to_string_without_scope,
    Output,
    query::statement_models::StatementTrait
};
#[derive(Debug, Default)]
pub struct Update {
    pub what: String,
    pub data: Option<Vec<String>>,
    pub cond: Option<String>,
    pub output: Option<Output>,
    pub timeout: Option<String>,
    pub parallel: bool,
}
pub fn update(what: &str) -> Update {
    Update::init(what)
 }
impl Update {
    pub fn init(what: &str) -> Update {
        let mut t = Update::default();
        t.what = what.to_string();
        t
    }
    pub fn set(mut self, flds: Vec<&str>) -> Update {
        let flds: Vec<String> = flds.iter().map(|s| s.to_string()).collect();
        self.data = Some(flds);
        self
    }
    pub fn filter(mut self, cond: &str) -> Update {
        self.cond = Some(cond.to_string());
        self
    }
    pub fn timeout(mut self, timeout: &str) -> Update {
        self.timeout = Some(timeout.to_string());
        self
    }
    pub fn output(mut self, out: Output) -> Update {
        self.output = Some(out);
        self
    }
    pub fn parallel(mut self) -> Update {
        self.parallel = true;
        self
    }
}

impl std::fmt::Display for Update {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UPDATE {}", self.what)?;
        if let Some(ref v) = self.data {
            let v = vec_to_string_without_scope(&v);
            write!(f, " SET {v}")?;
        }
        if let Some(ref v) = self.cond {
            write!(f, " WHERE {v}")?;
        }

        if let Some(ref output) = self.output {
            write!(f, " {output}")?;
        }

        if let Some(ref timeout) = self.timeout {
            write!(f, " TIMEOUT {timeout}")?;
        }

        if self.parallel {
            f.write_str(" PARALLEL")?;
        }
        Ok(())
    }
}

impl StatementTrait for Update{
    fn get_string(&self) -> String {
        format!("{}", self)
    }
}