#[derive(Debug)]
pub enum Mutation {
    ChangeEditedBy(String),
    ChangeIsDayTime(String),
    ChangeColor(String),
    ChangeNumberIncrease(String),
    ChangeNumberDecrease(String),
    Nothing(),
}
