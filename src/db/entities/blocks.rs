#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub blockhash: String,
    pub parenthash: Option<String>,
    pub is_current: Option<bool>,
    pub number: Option<i32>,
}
