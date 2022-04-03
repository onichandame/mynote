use sea_orm::Condition;

pub trait FilterBuilder {
    fn build(&self) -> Condition;
}
