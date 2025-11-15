use std::fmt::Debug;

#[allow(unused)]
pub enum EvaluationResult {
    Number,
    String,
    Decimal,
    Boolean,
    // I might introduce support for these types later on
    /*
    Array,
    Object,
    Function,
    Null,
     */
}

#[allow(unused)]
pub trait Expression: Debug {
    fn get_evaluation_type(&self) -> EvaluationResult;
}
