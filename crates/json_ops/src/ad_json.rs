use crate::adopter::*;
use serde_json::Value;
use serde_json::json;

/// Create json pointer directely from `json::Value`.
impl ValuePath for Value {
    /// Get value from array by index.
    fn get_index<'tr>(&'tr self, i: usize) -> Option<&'tr Self>
    {
        self.get(i)
    }

    /// Get value from map by key.
    fn get_key<'tr>(&'tr self, k: &str) -> Option<&'tr Self>
    {
        self.get(k)
    }

    /// Get value from array by index.
    fn get_index_mut<'tr>(&'tr mut self, i: usize) -> Option<&'tr mut Self>
    {
        self.get_mut(i)
    }

    /// Get value from map by key.
    fn get_key_mut<'tr>(&'tr mut self, k: &str) -> Option<&'tr mut Self>
    {
        self.get_mut(k)
    }
}

impl ValueReader for Value {
    /// operator `| &str`
    fn get_str<'tr>(&'tr self, rhs: &'tr str) -> &'tr str {
        match self.as_str() {
            Some(val) => val,
            None => rhs,
        }
    }

    /// operator `| String`.
    fn get_string(&self, rhs: String) -> String {
        match self {
            Value::String(s) => s.to_string(),
            Value::Number(i) if rhs == "0" && i.is_i64() => i.to_string(),
            Value::Number(u) if rhs == "0" && u.is_u64() => u.to_string(),
            Value::Number(f) if rhs == "0.0" && f.is_f64() => f.to_string(),
            Value::Bool(tf) if rhs == "bool" => tf.to_string(),
            Value::Array(_) if rhs == "[]" => self.to_string(),
            Value::Object(_) if rhs == "{}" => self.to_string(),
            _ if rhs.is_empty() => self.to_string(),
            _ => rhs
        }
    }

    /// operator `| i64`.
    fn get_i64(&self, rhs: i64) -> i64 {
        match self {
            Value::Number(n) if n.is_i64() => n.as_i64().unwrap_or(rhs),
            Value::String(s) => s.parse().unwrap_or(rhs),
            Value::Bool(tf) => if *tf { 1 } else { 0 },
            _ => rhs
        }
    }

    /// operator `| f64`.
    fn get_f64(&self, rhs: f64) -> f64 {
        match self {
            Value::Number(n) if n.is_f64() => n.as_f64().unwrap_or(rhs),
            Value::String(s) => s.parse().unwrap_or(rhs),
            _ => rhs
        }
    }

    /// operator `| bool`.
    fn get_bool(&self, rhs: bool) -> bool {
        match self {
            Value::Bool(tf) => *tf,
            Value::Number(n) if n.is_i64() => n.as_i64().unwrap_or(0) != 0,
            Value::Number(n) if n.is_u64() => true,
            Value::String(s) => s.parse().unwrap_or(rhs),
            _ => rhs
        }
    }
}

impl ValueWriter for Value {
    /// For operator `<<` with scalar string, integer, float, bool and unit.
    fn put_value<T>(&mut self, rhs: T) -> &mut Self where Value: From<T> , T: ScalarValue {
        *self = Value::from(rhs);
        self
    }

    /// For operator `<<` to jsob objecet.
    fn push_object<K: ToString, T>(&mut self, key: K, val: T) -> &mut Self where Value: From<T> {
        if !self.is_object() {
            *self = json!({});
        }
        if let Some(v) = self.as_object_mut() {
            v.insert(key.to_string(), Value::from(val));
        }
        self
    }

    /// For operator `<<` to jsob array.
    fn push_array<T>(&mut self, val: T) -> &mut Self where Value: From<T> {
        if !self.is_array() {
            *self = json!([]);
        }
        if let Some(v) = self.as_array_mut() {
            v.push(Value::from(val));
        }
        self
    }

}
