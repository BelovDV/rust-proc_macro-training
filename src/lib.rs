use std::str::FromStr;

pub use reflection_proc::*;

pub trait Reflection<'a> {
    type Field;

    fn get_field(&'a mut self, name: &str) -> Self::Field;
    fn get_field_list(&self) -> Vec<String>;
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
