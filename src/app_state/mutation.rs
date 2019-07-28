#[derive(Debug)]
pub enum Mutation {
    ChangeEditedBy(String),
    ChangeIsDayTime(String),
    ChangeColor(String),
    ChangeNumberIncrement(String),
    ChangeNumberDecrement(String),

    Nothing(),
}
