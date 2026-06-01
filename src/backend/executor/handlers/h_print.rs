use crate::backend::executor::Value;

fn display_value(value: &Value) -> String {
    match value {
        Value::Bool(v) => v.to_string(),
        Value::Int(v) => v.to_string(),
        Value::Float(v) => v.to_string(),
        Value::Str(v) => format!("{}", unescape(v)),
        Value::Struct(fields) => {
            let v = fields
                .iter()
                .map(|(k, v)| format!("{}: {}", k, display_value(v)))
                .collect::<Vec<_>>()
                .join(", ");
            format!("{{ {} }}", v)
        }
        Value::Enum(variants) => {
            let inner = variants
                .iter()
                .map(display_value)
                .collect::<Vec<_>>()
                .join("\n");
            format!("{}", inner)
        }
        Value::Array(items) => {
            let inner = items
                .iter()
                .map(display_value)
                .collect::<Vec<_>>()
                .join(", ");
            format!("[{}]", inner)
        }
    }
}

fn unescape(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.next() {
                Some('n') => result.push('\n'),
                Some('t') => result.push('\t'),
                Some('r') => result.push('\r'),
                Some('\\') => result.push('\\'),
                Some('"') => result.push('"'),
                Some('0') => result.push('\0'),
                Some(other) => {
                    // unknown escape: keep as-is
                    result.push('\\');
                    result.push(other);
                }
                None => result.push('\\'),
            }
        } else {
            result.push(c);
        }
    }
    result
}

pub fn execute_print(value: &Value) {
    println!("{}", display_value(value));
}
