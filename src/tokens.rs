
#![allow(dead_code)]
#[derive(Debug)]
pub enum Tokens {
    //Newline, // Newline & Carriage Return[Enter]
    Id {value: String}, // Object identifier- ex. Functions,Loops,Arrays,
    Op {value: String}, // Operations (+,-,*,%,<,>,=)
    Num {value: String},
    Null{value: String},
    
}