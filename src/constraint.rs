#[derive(Clone, Copy, PartialEq)]
pub enum ConstraintDatatypes {
    STRING,
    INT,
}

#[derive(Clone, Copy, PartialEq)]
#[warn(non_camel_case_types)]
pub enum Constraints {
    PRIMARY_KEY,
    NOT_NULL,
}

pub fn constraint_to_string(constraint: &ConstraintDatatypes) -> String {
    match constraint {
        ConstraintDatatypes::STRING => return String::from("String"),
        ConstraintDatatypes::INT => return String::from("Integer"),
    }
}

impl Clone for Constraint {
    fn clone(&self) -> Self {
        return Constraint {
            datatype: self.datatype,
            constraints: self.constraints.clone(),
        };
    }
}

pub struct Constraint {
    pub datatype: ConstraintDatatypes,
    pub constraints: Vec<Constraints>,
}
pub fn make_constraint(datatype: ConstraintDatatypes) -> Constraint {
    return Constraint {
        datatype: datatype,
        constraints: vec![],
    };
}

impl Constraint {
    pub fn contains(&self, constraint: Constraints) -> bool {
        for check_constraint in &self.constraints {
            if check_constraint.eq(&constraint) {
                return true;
            }
        }
        return false;
    }
    pub fn make_not_null(mut self) -> Self {
        self.constraints.push(Constraints::NOT_NULL);
        return self;
    }
}
