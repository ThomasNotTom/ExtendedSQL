use crate::constraint::Constraint;

pub struct Header {
    pub name: String,
    pub constraint: Constraint,
}

impl Clone for Header {
    fn clone(&self) -> Self {
        return Header {
            name: self.name.clone(),
            constraint: self.constraint.clone(),
        };
    }
}

impl Header {
    pub fn new(name: String, constraint: Constraint) -> Self {
        Header { name, constraint }
    }
}
