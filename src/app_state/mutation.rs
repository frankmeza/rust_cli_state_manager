#[derive(Debug)]
pub enum Mutation {
    ChangeEditedBy(String),
    ChangeIsDayTime(String),
    ChangeColor(String),
    ChangeNumber(String, String),

    Nothing(),
}
