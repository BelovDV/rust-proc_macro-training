use std::str::FromStr;

pub use reflection_proc::*;

pub trait Reflection {
    fn get_field_string(&self, name: &str) -> Result<String, ()>;
    fn get_field_list() -> Vec<String>;
}

pub trait UpdateWithStr {
    type Err;

    fn update_with_str(&mut self, s: &str) -> Result<(), Self::Err>;
}

impl<T> UpdateWithStr for T
where
    T: FromStr,
{
    type Err = <T as FromStr>::Err;

    fn update_with_str(&mut self, s: &str) -> Result<(), Self::Err> {
        match Self::from_str(s) {
            Ok(s) => {
                *self = s;
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}
