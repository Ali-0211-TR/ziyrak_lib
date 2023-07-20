use std::fmt;
use crate::{
    core::{
        obj_to_string, vec_to_string_without_scope, separate_fields, separate_values
    },
    sql::{Statement,Output}
};

#[derive(Debug, Default, Clone)]
pub struct Insert {
    pub into: String,
    pub data: Option<Vec<String>>,
    pub fields: Option<Vec<String>>,
    pub ignore: bool,
    pub update: Option<Vec<String>>,
    pub output: Option<Output>,
    pub tmout: Option<String>,
    pub paral: bool,
}
pub fn insert(into: &str) -> Insert {
    Insert::init(into)
}
impl Insert {
    pub fn init(into: &str) -> Insert {
        let mut t = Insert::default();
        t.into = into.to_string();
        t
    }
    pub fn ignore(mut self) -> Insert {
        self.ignore = true;
        self
    }
    pub fn into_vec_str(mut self, fields: Vec<&str>) -> Insert {
        let fields: Vec<String> = fields.iter().map(|s| String::from(*s)).collect();
        self.fields = Some(fields);
        self
    }
    pub fn fields<Obj: serde::Serialize>(mut self, obj: Obj) -> Insert {
        let obj = match obj_to_string(&obj){
            Ok(v) => v,
            Err(e) => {eprintln!("Error from separate fields: {e}"); "".to_string()},
        };
        let t =  match separate_fields(&obj) {
            Ok(val) => val,
            Err(e) => {eprintln!("Error from separate fields: {e}"); vec![]},
        };
        self.fields = Some(t);
        self
    }
    pub fn value<T: serde::Serialize>(mut self, obj: T) -> Insert {
        let obj = match obj_to_string(obj) {
            Ok(str_data) => str_data,
            ////////>>>>Стоит пересмотреть!
            Err(e) => {
                eprintln!("Error from sql CREATE::CONTENT: {e}\nReturned CANCEL TRANSACTION");
                "{Error: 'Create contnent error'}; CANCEL TRANSACTION;".to_string()
            }
        };
        if let Some(ref mut d) = self.data {
            d.push(obj);
        } else {
            self.data = Some(vec![obj]);
        }
        self
    }
    pub fn values<T: serde::Serialize>(mut self, data: Vec<T>) -> Insert {
        let mut data = data
            .iter()
            .map(|d| {
                match obj_to_string(&d) {
                    Ok(str_data) => str_data,
                    ////////>>>>Стоит пересмотреть!
                    Err(e) => {
                        eprintln!(
                            "Error from sql CREATE::CONTENT: {e}\nReturned CANCEL TRANSACTION"
                        );
                        "{Error: 'Create contnent error'}; CANCEL TRANSACTION;".to_string()
                    }
                }
            })
            .collect();
        if let Some(ref mut d) = self.data {
            d.append(&mut data);
        } else {
            self.data = Some(data);
        }
        self
    }

    pub fn output(mut self, out: Output) -> Insert {
        self.output = Some(out);
        self
    }

    pub fn parallel(mut self) -> Insert {
        self.paral = true;
        self
    }

    pub fn timeout(mut self, tmout: &str) -> Insert {
        self.tmout = Some(tmout.to_string());
        self
    }
}
impl fmt::Display for Insert {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("INSERT")?;

        if self.ignore {
            f.write_str(" IGNORE")?;
        }

        write!(f, " INTO {}", self.into.as_str())?;

        if let Some(ref v) = self.fields {
            write!(f, " ({}) VALUES", vec_to_string_without_scope(v))?;
            if let Some(ref d) = self.data {
                let mut temp = vec![];
                for i in d {
                    let values = match separate_values(i) {
                        Ok(val) => val,
                        Err(e) => return Err(std::fmt::Error),
                    };

                    temp.push(format!("({})", vec_to_string_without_scope(&values)));
                }
                write!(f, " {}", vec_to_string_without_scope(&temp))?;
            }
        } else if let Some(ref d) = self.data {
            write!(f, " {:?}", d)?;
        }

        if let Some(ref output) = self.output {
            write!(f, " {output}")?;
        }

        if let Some(ref timeout) = self.tmout {
            write!(f, " TIMEOUT {timeout}")?;
        }

        if self.paral {
            f.write_str(" PARALLEL")?;
        }
        Ok(())
    }
}

impl Statement for Insert{}

// #[cfg(test)]
// mod tests {

//     use super::*;

//     #[test]
//     fn insert_statement() {
//         let obj1 = vec![
//             crate::lib_core::func_for_testing::generate_random_camera(),
//             crate::lib_core::func_for_testing::generate_random_camera(),
//             crate::lib_core::func_for_testing::generate_random_camera(),
//             crate::lib_core::func_for_testing::generate_random_camera(),
//         ];
//         let a: Vec<String> = obj1
//             .iter()
//             .map(|item| serde_json::to_string(item).unwrap_or_else(|_| String::new()))
//             .collect();

//         let sql = format!("INSERT INTO camera {:?}", a);
//         let res = Insert::init("camera").values(obj1);

//         println!("{}\n\n\n{}", sql, res);
//         assert_eq!(sql, format!("{}", res));
//     }

//     #[test]
//     fn insert_statement_2() {
//         #[derive(Debug, Default, serde::Serialize)]
//         struct TestPerson {
//             name: String,
//             phone: String,
//             age: u32,
//         }

//         let obj1 = vec![
//             TestPerson {
//                 name: "Test_01".to_string(),
//                 phone: "9987776".to_string(),
//                 age: 20,
//             },
//             TestPerson {
//                 name: "qwer_01".to_string(),
//                 phone: "99er776".to_string(),
//                 age: 30,
//             },
//         ];
//         let a: Vec<String> = obj1
//             .iter()
//             .map(|item| serde_json::to_string(item).unwrap_or_else(|_| String::new()))
//             .collect();

//         let sql = r#"INSERT INTO person (age, name, phone) VALUES (20, "Test_01", "9987776"), (30, "qwer_01", "99er776")"#;
//         let res = Insert::init("person")
//             .fields(obj1.get(0).unwrap())
//             .values(obj1);

//         println!("{}\n\n\n{}", sql, res);
//         assert_eq!(sql, format!("{}", res));
//     }
// }
