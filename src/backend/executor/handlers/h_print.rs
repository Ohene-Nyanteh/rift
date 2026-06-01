use crate::backend::executor::Value;

fn display_value(value: &Value) -> String {
    match value {
        Value::Bool(v) => v.to_string(),
        Value::Int(v) => v.to_string(),
        Value::Float(v) => v.to_string(),
        Value::Str(v) => format!("\"{}\"", v),
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

pub fn execute_print(value: &Value) {
    println!("{}", display_value(value));
}
