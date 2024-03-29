#[macro_export]
macro_rules! character_patterns {
    (Whitespace) => {
        // 空白文字にマッチする全てのパターン
        '\u{0009}'
            | '\u{000B}'
            | '\u{000C}'
            | '\u{FEFF}'
            | '\u{0020}'
            | '\u{00A0}'
            | '\u{1680}'
            | '\u{2000}'..='\u{200A}' | '\u{202F}' | '\u{205f}' | '\u{3000}'
    };

    (LineTerminator) => {
        // 改行文字にマッチする全てのパターン
        '\u{000A}' | '\u{000D}' | '\u{2028}' | '\u{2029}'
    };

    (LineTerminator without CR) => {
        // <CR>を除いたパターン
        '\u{000A}' | '\u{2028}' | '\u{2029}'
    };

    (Punctuator) => {
        // 割り算文字、右波括弧以外の全ての演算子の開始文字
        '{' | '('
            | ')'
            | '['
            | ']'
            | '.'
            | ';'
            | ','
            | '<'
            | '>'
            | '='
            | '!'
            | '+'
            | '-'
            | '*'
            | '%'
            | '&'
            | '|'
            | '^'
            | '~'
            | '?'
    };

    (Punctuator div) => {
        '/'
    };

    (Punctuator right brace) => {
        '}'
    };

    (Numeric) => {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
    };
}
