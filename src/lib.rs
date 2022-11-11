use std::str::FromStr;

pub use reflection_proc::*;

pub trait Reflection {
    fn get_field_string(&self, name: &str) -> Result<String, ()>;
    fn get_field_list() -> Vec<&'static str>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateWithStrErr {
    WrongFieldName,
    CannotParseValue,
}

pub trait UpdateWithStr {
    fn update_with_str(&mut self, s: &str) -> Result<(), UpdateWithStrErr>;
}

impl<T> UpdateWithStr for T
where
    T: FromStr,
{
    fn update_with_str(&mut self, s: &str) -> Result<(), UpdateWithStrErr> {
        match Self::from_str(s) {
            Ok(s) => {
                *self = s;
                Ok(())
            }
            Err(_) => Err(UpdateWithStrErr::CannotParseValue),
        }
    }
}

pub trait ReflectionString: UpdateWithStr {
    fn get_field_list() -> Vec<&'static str>;
    fn get_field_string(&self, name: &str) -> Result<String, ()>;
}
