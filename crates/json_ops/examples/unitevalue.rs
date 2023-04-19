use std::collections::HashMap;
use json_ops::{ValuePath, ValueReader, ValueWriter, ScalarValue};

// a simple value tree mode
#[derive(Debug)]
enum Value
{
    Cell(Cell),
    Table(Table),
}

#[derive(Debug)]
enum Cell
{
    Number(i64),
    Text(String),
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Number(0)
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Cell(Cell::default())
    }
}

impl Value {
    fn is_table(&self) -> bool {
        match self {
            Value::Table(_) => true,
            _ => false
        }
    }
}

// something like lua table that act both array and map
#[derive(Debug)]
struct Table
{
    seq: Vec<Value>,
    map: HashMap<String, Value>,
}

impl Table
{
    pub fn new() -> Self {
        Self { seq: Vec::new(), map: HashMap::new() }
    }

    pub fn pushi(&mut self, v: Value) -> &Self {
        self.seq.push(v);
        self
    }

    pub fn pushk(&mut self, k: &str, v: Value) -> &Self {
        self.map.insert(k.to_string(), v);
        self
    }
}

impl ValuePath for Value {
    fn get_index<'tr>(&'tr self, i: usize) -> Option<&'tr Self>
    {
        match self {
            Value::Table(t) => t.seq.get(i),
            _ => None
        }
    }

    /// Get value from map by key.
    fn get_key<'tr>(&'tr self, k: &str) -> Option<&'tr Self>
    {
        match self {
            Value::Table(t) => t.map.get(k),
            _ => None
        }
    }

    /// Get value from array by index.
    fn get_index_mut<'tr>(&'tr mut self, i: usize) -> Option<&'tr mut Self>
    {
        match self {
            Value::Table(t) => t.seq.get_mut(i),
            _ => None
        }
    }

    /// Get value from map by key.
    fn get_key_mut<'tr>(&'tr mut self, k: &str) -> Option<&'tr mut Self>
    {
        match self {
            Value::Table(t) => t.map.get_mut(k),
            _ => None
        }
    }
}

impl ValueReader for Value {
    fn get_str<'tr>(&'tr self, rhs: &'tr str) -> &'tr str {
        match self {
            Value::Cell(Cell::Text(s)) => s.as_str(),
            _ => rhs
        }
    }
    fn get_string(&self, rhs: String) -> String {
        match self {
            Value::Cell(Cell::Text(s)) => s.clone(),
            _ => rhs
        }
    }
    fn get_i64(&self, rhs: i64) -> i64 {
        match self {
            Value::Cell(Cell::Number(i)) => *i,
            _ => rhs
        }
    }
    fn get_f64(&self, rhs: f64) -> f64 {
        match self {
            Value::Cell(Cell::Number(i)) => *i as f64,
            _ => rhs
        }
    }
    fn get_bool(&self, rhs: bool) -> bool {
        match self {
            Value::Cell(Cell::Number(i)) => *i != 0,
            _ => rhs
        }
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Value::Cell(Cell::Number(i))
    }
}

impl<'a> From<&'a str> for Value {
    fn from(s: &str) -> Self {
        Value::Cell(Cell::Text(s.to_string()))
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Value::Cell(Cell::Text(s))
    }
}

impl ValueWriter for Value {
    fn put_value<T>(&mut self, rhs: T) -> &mut Self
        where Self: From<T>, T: ScalarValue
    {
        *self = rhs.into();
        self
    }

    fn push_object<K: ToString, T>(&mut self, key: K, val: T) -> &mut Self
        where Self: From<T>
    {
        if !self.is_table() {
            *self = Value::Table(Table::new());
        }
        match self {
            Value::Table(t) => {
                t.pushk(key.to_string().as_str(), Value::from(val));
            }
            _ => { }
        }
        self
    }

    fn push_array<T>(&mut self, val: T) -> &mut Self
        where Self: From<T>
    {
        if !self.is_table() {
            *self = Value::Table(Table::new());
        }
        match self {
            Value::Table(t) => {
                t.pushi(Value::from(val));
            }
            _ => { }
        }
        self
    }
}

fn main()
{
    let vdef = Value::default();
    println!("{:?}", vdef);

    let i: i64 = vdef.path() | -1;
    println!("vdef | -1: {i}");

    let mut v = Value::default();

    let node = v.path_mut() << 123;
    let i = node | 0;
    println!("v | 0: {i}");

    let _ = v.path_mut() << [123] << [456] << ("key", "val") << ("abc", 789);
    let i = v.path() / 1 | 0;
    println!("v/1 | 0: {i}");
    let s = v.path() / "key" | "";
    println!("v/key | '': {s}");

    println!("v: {:?}", v);
}
