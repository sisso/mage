#[derive(Debug)]
pub enum GameError {
    Msg(String),
    Str(&'static str),
}
