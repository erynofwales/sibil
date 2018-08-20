/* node_parser.rs
 * Eryn Wells <eryn@erynwells.me>
 */

pub trait NodeParser {
}

pub struct IdParser {
}

impl IdParser {
    pub fn new() -> IdParser {
        IdParser { }
    }
}

impl NodeParser for IdParser {
}

pub struct ListParser {
}

impl ListParser {
    pub fn new() -> ListParser {
        ListParser { }
    }
}

impl NodeParser for ListParser {
}
