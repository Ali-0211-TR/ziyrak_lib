use std::fmt;
use crate::data_source::{
    core::vec_to_string_without_scope,
    query::statement_models::StatementTrait
};


#[derive(Debug, Default)]
pub struct Select {
    pub expr: Vec<String>,
    pub what: Option<Vec<String>>,
    pub cond: Option<String>,
    pub splt: Option<Vec<String>>,
    pub grp: Option<Vec<String>>,
    pub ordr: Option<Vec<String>>,
    pub lmt: Option<u32>,
    pub strt: Option<u32>,
    pub ftch: Option<Vec<String>>,
    pub vrsn: Option<String>,
    pub tmout: Option<String>,
    pub paral: bool,
}
pub fn select(expr: Vec<&str>) -> Select {
   Select::init(expr)
}
impl Select {
    pub fn init(expr: Vec<&str>) -> Select {
        let mut temp = Select::default();
        let expr = expr.iter().map(|s| s.to_string()).collect();
        temp.expr = expr;
        temp
    }

    pub fn from(mut self, what: Vec<&str>) -> Select {
        let what = what.iter().map(|s| s.to_string()).collect();
        self.what = Some(what);
        self
    }

    pub fn filter(mut self, cnd: &str) -> Select {
        self.cond = Some(cnd.to_string());
        self
    }

    pub fn split(mut self, splt: Vec<&str>) -> Select {
        let splt = splt.iter().map(|s| s.to_string()).collect();
        self.splt = Some(splt);
        self
    }

    pub fn group(mut self, grp: Vec<&str>) -> Select {
        let grp = grp.iter().map(|s| s.to_string()).collect();
        self.grp = Some(grp);
        self
    }

    pub fn order(mut self, ord: Vec<&str>) -> Select {
        let ord = ord.iter().map(|s| s.to_string()).collect();
        self.ordr = Some(ord);
        self
    }

    pub fn limit(mut self, lmt: u32) -> Select {
        self.lmt = Some(lmt);
        self
    }

    pub fn start(mut self, strt: u32) -> Select {
        self.strt = Some(strt);
        self
    }

    pub fn fetch(mut self, ftch: Vec<&str>) -> Select {
        let ftch = ftch.iter().map(|s| s.to_string()).collect();
        self.ftch = Some(ftch);
        self
    }

    pub fn timeout(mut self, tmout: &str) -> Select {
        self.tmout = Some(tmout.to_string());
        self
    }

    pub fn parallel(mut self, prlel: bool) -> Select {
        self.paral = prlel;
        self
    }

    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}


impl fmt::Display for Select {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.expr.is_empty() {
            f.write_str("SELECT *")?;
        } else {
            write!(f, "SELECT {}", vec_to_string_without_scope(&self.expr))?;
        }

        match &self.what {
            Some(wh) => {
                write!(f, " FROM {}", vec_to_string_without_scope(wh))?;
            }
            None => {
                return Err(fmt::Error);
            }
        }

        if let Some(ref v) = self.cond {
            write!(f, " WHERE {}", v)?;
        }
        if let Some(ref v) = self.splt {
            write!(f, " SPLIT ON {}", vec_to_string_without_scope(v))?;
        }
        if let Some(ref v) = self.grp {
            write!(f, " GROUP {}", vec_to_string_without_scope(v))?;
        }
        if let Some(ref v) = self.ordr {
            write!(f, " {}", vec_to_string_without_scope(v))?;
        }
        if let Some(v) = self.lmt {
            write!(f, " LIMIT {}", v)?;
            if let Some(v) = self.strt {
                write!(f, " START {}", v)?;
            }
        }
        if let Some(ref v) = self.ftch {
            write!(f, " FETCH {}", vec_to_string_without_scope(v))?;
        }
        if let Some(ref v) = self.vrsn {
            write!(f, " VERSION {}", v)?;
        }
        if let Some(ref v) = self.tmout {
            write!(f, " TIMEOUT {}", v)?;
        }
        if self.paral {
            write!(f, " PARALLEL")?;
        }
        Ok(())
    }
}
impl StatementTrait for Select{
    fn get_string(&self) -> String {
        format!("{}", self)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug, serde::Serialize)]
    struct Name<'a> {
        first: &'a str,
        last: &'a str,
    }
    #[derive(Debug, serde::Serialize)]
    struct Person<'a> {
        title: &'a str,
        name: Name<'a>,
        marketing: bool,
    }
    #[test]
    fn select_all_fields_test() {
        let slct = Select::init(vec![]).from(vec!["person"]);
        let sql = "SELECT * FROM person";
        assert_eq!(sql, slct.to_string().as_str());
    }
    #[test]
    fn elect_specific_fields_test() {
        let sql = "SELECT name, address, email FROM person";
        let slct = Select::init(vec!["name", "address", "email"]).from(vec!["person"]);
        assert_eq!(sql, slct.to_string().as_str());
    }
}
