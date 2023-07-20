use std::fmt;
use crate::{
    core::obj_to_string,
    sql::{
        Output,
        Statement
    }
};

#[derive(Debug, Default, Clone)]
pub struct Create {
    what: String,
    data: Option<String>,
    output: Option<Output>,
    tmout: Option<String>,
    paral: bool,
}

pub fn create(what: &str) -> Create {
    Create::init(what)
}

impl Create {
    pub fn content<T: serde::Serialize>(mut self, obj: T) -> Create {
        self.data = Some(match obj_to_string(obj) {
            Ok(str_data) => str_data,
            ////////>>>>Стоит пересмотреть!
            Err(e) => {
                eprintln!("Error from sql CREATE::CONTENT: {e}\nReturned CANCEL TRANSACTION");
                "{Error: 'Create contnent error'}; CANCEL TRANSACTION;".to_string()
            }
        });
        self
    }

    pub fn init(what: &str) -> Create {
        let mut t = Create::default();
        t.what = what.to_string();
        t
    }

    pub fn output(mut self, out: Output) -> Create {
        self.output = Some(out);
        self
    }

    pub fn parallel(mut self) -> Create {
        self.paral = true;
        self
    }

    pub fn timeout(mut self, tmout: &str) -> Create {
        self.tmout = Some(tmout.to_string());
        self
    }
}

impl fmt::Display for Create {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CREATE {}", self.what.as_str())?;

        if let Some(ref v) = self.data {
            write!(f, " CONTENT {}", v)?;
        }

        if let Some(ref v) = self.output {
            write!(f, " {}", v)?;
        }

        if let Some(ref v) = self.tmout {
            write!(f, " TIMEOUT {}", v)?
        }

        if self.paral {
            write!(f, " PARALLEL")?;
        }

        Ok(())
    }
}

impl Statement for Create{}

// #[cfg(test)]
// mod tests {

//     use super::*;
//     #[test]
//     fn create_to_string_test(){
//         let obj = crate::lib_core::func_for_testing::generate_random_camera();
//         let sql = format!(
//             "CREATE camera CONTENT {} RETURN AFTER",
//             obj_to_string(&obj).unwrap()
//         );
//         let res = Create::init("camera").content(&obj).output(Output::After);

//         assert_eq!(sql, res.get_string());

//     }
//     #[test]
//     fn create_statement() {
//         let obj = crate::lib_core::func_for_testing::generate_random_camera();
//         let sql = format!(
//             "CREATE camera CONTENT {} RETURN AFTER",
//             obj_to_string(&obj).unwrap()
//         );

//         let res = Create::init("camera").content(&obj).output(Output::After);
//         //println!("{}\n{}", sql, res);
//         assert_eq!(sql, format!("{}", res));
//     }
// }