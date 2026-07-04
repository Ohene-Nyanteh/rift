use crate::backend::tokens::{Keywords, NonAtomic, Operations, Primary, Tokens};

pub fn convert_tokens_to_values(token: &Tokens)-> String {
    match token {
        Tokens::Atomic(op)=> convert_operations_to_values(op),
        Tokens::Keyword(keyword) => convert_keywords_to_values(keyword),
        Tokens::Primary(value) => convert_primary_to_value(value),
        Tokens::NonAtomic(value) => convert_non_atomic_to_values(value),
        Tokens::Variable(name) => name.clone(),
        Tokens::EOF => String::from("End Of File")
    }
}



fn convert_operations_to_values(op: &Operations) -> String {
    match op {
        Operations::Add => String::from("+"),
        Operations::Sub => String::from("-"),
        Operations::Mul => String::from("*"),
        Operations::Mod => String::from("%"),
        Operations::Div => String::from("/"),

        Operations::And => String::from("&"),
        Operations::Or => String::from("|"),


        Operations::Not => String::from("!"),
        Operations::EqualTo => String::from("=="),
        Operations::LessOrEquals=> String::from("<="),
        Operations::GreaterOrEquals => String::from(">="),
        Operations::GreaterThan => String::from(">"),
        Operations::LessThan => String::from("<"),
        Operations::NotEqualTo => String::from("!=")

    }
}



fn convert_keywords_to_values(keyword: &Keywords) -> String {
    match keyword {
        Keywords::Let => String::from("let"),
        Keywords::Fn => String::from("fn"),
        Keywords::Enum => String::from("enum"),
        Keywords::Struct => String::from("struct"),
        Keywords::Match => String::from("match"),
        Keywords::If => String::from("if"),
        Keywords::Elif => String::from("elif"),
        Keywords::Else => String::from("else"),
        Keywords::Loop => String::from("loop"),
        Keywords::While => String::from("while"),
        Keywords::For => String::from("for"),
        Keywords::From => String::from("from"),
        Keywords::Break => String::from("break"),
        Keywords::Continue => String::from("continue"),
        Keywords::Print => String::from("print"),
        Keywords::Return => String::from("return"),
        Keywords::In => String::from("in")
    }
}


fn convert_primary_to_value(value: &Primary) -> String {
    match value {
        Primary::Bool(v) => v.to_string(),
        Primary::Float(v) => v.to_string(),
        Primary::Int(v) => v.to_string(),
        Primary::Str(v) => v.to_string()
    }
}


fn convert_non_atomic_to_values(value: &NonAtomic) -> String {
    match value {
        NonAtomic::Colon => String::from(":"),
        NonAtomic::Comma => String::from(","),
        NonAtomic::SemiColon => String::from(";"),
        NonAtomic::LParen => String::from("("),
        NonAtomic::RParen => String::from(")"),
        NonAtomic::LCurlyBraces => String::from("{"),
        NonAtomic::RCurlyBraces => String::from("}"),
        NonAtomic::LSquareBraces => String::from("["),
        NonAtomic::RSquareBraces => String::from("]"),
        NonAtomic::Assignment => String::from("="),
        NonAtomic::FatArrow => String::from("=>"),
        NonAtomic::Dot => String::from(".")
    }
}
