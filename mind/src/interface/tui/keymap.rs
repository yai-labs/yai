#[derive(Debug, Clone, Copy)]
pub struct KeyMap;

impl KeyMap {
    pub const NEXT_VIEW: char = '\t';
    pub const GRAPH: char = 'g';
    pub const LOGS: char = 'l';
    pub const EVENTS: char = 'e';
    pub const DB: char = 'd';
    pub const PROVIDERS: char = 'p';
    pub const CONTRACTS: char = 'c';
    pub const SEARCH: char = '/';
    pub const PALETTE: char = ':';
    pub const QUIT: char = 'q';
}
