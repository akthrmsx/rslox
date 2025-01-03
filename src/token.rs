#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LeftParen { line: usize },
    RightParen { line: usize },

    LeftBrace { line: usize },
    RightBrace { line: usize },

    Dot { line: usize },
    Comma { line: usize },
    Semicolon { line: usize },

    Plus { line: usize },
    Minus { line: usize },
    Asterisk { line: usize },
    Slash { line: usize },

    Equal { line: usize },
    Bang { line: usize },

    EqualEqual { line: usize },
    NotEqual { line: usize },
    LessThan { line: usize },
    LessEqual { line: usize },
    GreaterThan { line: usize },
    GreaterEqual { line: usize },

    Identifier { line: usize, literal: String },
    String { line: usize, literal: String },
    Number { line: usize, literal: String },

    True { line: usize },
    False { line: usize },
    Nil { line: usize },

    And { line: usize },
    Or { line: usize },

    If { line: usize },
    Else { line: usize },
    For { line: usize },
    While { line: usize },

    Class { line: usize },
    This { line: usize },
    Super { line: usize },

    Fun { line: usize },
    Return { line: usize },
    Var { line: usize },
    Print { line: usize },

    Eof { line: usize },
}

impl Token {
    pub fn lexeme(&self) -> String {
        use Token::*;

        match self {
            LeftParen { .. } => "(".into(),
            RightParen { .. } => ")".into(),

            LeftBrace { .. } => "{".into(),
            RightBrace { .. } => "}".into(),

            Dot { .. } => ".".into(),
            Comma { .. } => ",".into(),
            Semicolon { .. } => ";".into(),

            Plus { .. } => "+".into(),
            Minus { .. } => "-".into(),
            Asterisk { .. } => "*".into(),
            Slash { .. } => "/".into(),

            Equal { .. } => "=".into(),
            Bang { .. } => "!".into(),

            EqualEqual { .. } => "==".into(),
            NotEqual { .. } => "!=".into(),
            LessThan { .. } => "<".into(),
            LessEqual { .. } => "<=".into(),
            GreaterThan { .. } => ">".into(),
            GreaterEqual { .. } => ">=".into(),

            Identifier { literal, .. } => literal.into(),
            String { literal, .. } => format!("\"{}\"", literal),
            Number { literal, .. } => literal.into(),

            True { .. } => "true".into(),
            False { .. } => "false".into(),
            Nil { .. } => "nil".into(),

            And { .. } => "and".into(),
            Or { .. } => "or".into(),

            If { .. } => "if".into(),
            Else { .. } => "else".into(),
            For { .. } => "for".into(),
            While { .. } => "while".into(),

            Class { .. } => "class".into(),
            This { .. } => "this".into(),
            Super { .. } => "super".into(),

            Fun { .. } => "fun".into(),
            Return { .. } => "return".into(),
            Var { .. } => "var".into(),
            Print { .. } => "print".into(),

            Eof { .. } => ">".into(),
        }
    }
}
