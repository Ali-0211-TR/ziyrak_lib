use crate::data_source::{
    Output,
    query::statement_models::StatementTrait
};

#[derive(Debug, Default)]
pub struct Delete {
    pub(crate) what: String,
    pub(crate) cond: Option<String>,
    pub(crate) output: Option<Output>,
    pub(crate) timeout: Option<String>,
    pub(crate) parallel: bool,
}
pub fn delete(what: &str) -> Delete{
    Delete::init(what)
}
impl Delete {
    pub fn init(what: &str) -> Delete {
        let mut t = Delete::default();
        t.what = what.to_string();
        t
    }
    pub fn timeout(mut self, tmout: &str) -> Delete {
        self.timeout = Some(tmout.to_string());
        self
    }
    pub fn filter(mut self, cnd: &str) -> Delete {
        self.cond = Some(cnd.to_string());
        self
    }
    pub fn output(mut self, out: Output) -> Delete {
        self.output = Some(out);
        self
    }
    pub fn parallel(mut self) -> Delete {
        self.parallel = true;
        self
    }
}
impl std::fmt::Display for Delete {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DELETE {}", self.what)?;
        if let Some(ref cnd) = self.cond {
            write!(f, " WHERE {}", cnd)?
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

impl StatementTrait for Delete{
    fn get_string(&self) -> String {
        format!("{}", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn just_delete_test() {
        let sql = "DELETE person";
        let qry = Delete::init("person");
        assert_eq!(sql, format!("{qry}").as_str());
    }
    #[test]
    fn filter_delete_test() {
        let sql = "DELETE city WHERE name = 'London'";
        let qry = Delete::init("city").filter("name = 'London'");
        assert_eq!(sql, format!("{qry}").as_str());
    }
    #[test]
    fn filter_return_delete_test() {
        let sql = "DELETE user WHERE age < 18 RETURN NONE";
        let qry = Delete::init("user").filter("age < 18").output(Output::None);
        assert_eq!(sql, format!("{qry}").as_str());
    }
    #[test]
    fn timeout_delete_test() {
        let sql = "DELETE person WHERE influencer = false TIMEOUT 5s";
        let qry = Delete::init("person")
            .filter("influencer = false")
            .timeout("5s");
        assert_eq!(sql, format!("{qry}").as_str());
    }
}
