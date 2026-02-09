use core::fmt;


#[derive(PartialEq, Debug)]
pub struct Data {
    pub id: i32,
    pub name: String
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} | {}", self.id, self.name)
    }
}
