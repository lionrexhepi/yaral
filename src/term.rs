use crate::context::AlgebraContext;

pub trait TermContent {
    fn simplify(self, context: &AlgebraContext) -> Term;
}

#[derive(PartialEq, Clone)]
pub struct Value(f64);

impl TermContent for Value {
    fn simplify(self, _context: &AlgebraContext) -> Term {
        Term::Value(self)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value(value)
    }
}

#[derive(PartialEq, Clone)]
pub struct Identifier(String);

impl TermContent for Identifier {
    fn simplify(self, context: &AlgebraContext) -> Term {
        if let Some(value) = context.get_value(&self.0) {
            Term::Value(value)
        } else {
            Term::Identifier(self)
        }
    }
}


#[derive(PartialEq, Clone)]
pub struct Addition(Vec<Term>);

impl TermContent for Addition {
    fn simplify(self, context: &AlgebraContext) -> Term {
        let mut content = Vec::with_capacity(self.0.capacity());

        for (i, term) in self.0.iter().enumerate() {
            content[i] = term.clone().simplify(context)
        }


        

        todo!()
    }
}


#[derive(PartialEq, Clone)]
pub struct Multiplication(Vec<Term>);


#[derive(PartialEq, Clone)]
pub struct Division(Box<Term>, Box<Term>);

#[derive(PartialEq, Clone)]
pub struct Exponentation(Box<Term>, Box<Term>);


#[derive(PartialEq, Clone)]
pub struct RootExtraction(Box<Term>, Box<Term>);

#[derive(PartialEq, Clone)]
pub enum Term {
    Value(Value),
    Identifier(Identifier),
    Addition(Addition),
    Multiplication(Multiplication),
    Division(Box<Term>, Box<Term>),
    Exponentation(Exponentation),
    RootExtraction(RootExtraction),
}




impl Term {
    pub fn simplify(self, context: &AlgebraContext) -> Self {
        match self {
            Term::Value(_) => todo!(),
            Term::Identifier(_) => todo!(),
            Term::Addition(_) => todo!(),
            Term::Multiplication(_) => todo!(),
            Term::Division(_, _) => todo!(),
            Term::Exponentation(_) => todo!(),
            Term::RootExtraction(_) => todo!(),
        }
    }
} 

