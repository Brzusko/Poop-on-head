use std::collections::HashMap;
use serde_json::Value;
//examples to parse
// --text arg1 arg2 ... <- Command with args
// --text <- Command without args
//tokens
// -- <- prefix
//  <- blank is separator
// ssd13421 <- string

enum TokenTypes
{
    Prefix,
    Separator,
    String,
    EndLine,
    Continue,
    Terminate,
}

struct Token
{
    pub symbol: String,
    pub length: usize,
    pub token_type: TokenTypes,
    pub value: Option<String>,
}

trait TokenizerState
{
    fn check_string_slice(string_slice: &str) -> Token;
    fn check_for_transition() -> TokenTypes;
}

struct PrefixTokenizerState;

impl TokenizerState for PrefixTokenizerState {
    fn check_string_slice(string_slice: &str) -> Token {
        todo!()
    }

    fn check_for_transition() -> TokenTypes {
        todo!()
    }
}

struct SeparatorTokenizerState;

impl TokenizerState for SeparatorTokenizerState {
    fn check_string_slice(string_slice: &str) -> Token {
        todo!()
    }

    fn check_for_transition() -> TokenTypes {
        todo!()
    }
}

struct StringTokenizerState;

impl TokenizerState for StringTokenizerState {
    fn check_string_slice(string_slice: &str) -> Token {
        todo!()
    }

    fn check_for_transition() -> TokenTypes {
        todo!()
    }
}

struct EndLineTokenizerState;

impl TokenizerState for EndLineTokenizerState {
    fn check_string_slice(string_slice: &str) -> Token {
        todo!()
    }

    fn check_for_transition() -> TokenTypes {
        todo!()
    }
}

struct TokenizerStateMachine
{
    current_state_id: TokenTypes,
    states_collection: HashMap<TokenTypes, dyn TokenizerState>
}