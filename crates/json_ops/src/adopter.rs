#![allow(unused_variables)] //< many defalut implement for trait

use crate::valueptr::ValuePtr;
use crate::valueptr::ValuePtrMut;

/// Yield json (or more generic value) pointer to support operator `/` overload.
/// All methods have defualt implementation, only override as needed.
/// Usually the `get_*` methods are required to implement for specific type,
/// while the `path*` methods are good enough to use directly.
pub trait ValuePath {
    /// Get value pointer from array by index, for opertor `/ usize`.
    fn get_index<'tr>(&'tr self, i: usize) -> Option<&'tr Self>
    {
        None
    }

    /// Get value pointer from map by key, for operator `/ &str`.
    fn get_key<'tr>(&'tr self, k: &str) -> Option<&'tr Self>
    {
        None
    }

    /// Get mutable value pointer from array by index, for opertor `/ usize`.
    fn get_index_mut<'tr>(&'tr mut self, i: usize) -> Option<&'tr mut Self>
    {
        None
    }

    /// Get mutable value pointer from map by key, for operator `/ &str`.
    fn get_key_mut<'tr>(&'tr mut self, k: &str) -> Option<&'tr mut Self>
    {
        None
    }

    /// Construct immutable value pointer to some initial node.
    /// Used to begin operator `/` chain.
    fn path<'tr>(&'tr self) -> ValuePtr<'tr, Self>
        where Self: ValueReader + Sized
    {
        ValuePtr::new(Some(self))
    }

    /// Construct immutable value pointer and move it follwoing sub path.
    /// Can also begin operator `/` chain, or got target within one call.
    fn pathto<'tr>(&'tr self, p: &str) -> ValuePtr<'tr, Self>
        where Self: ValueReader + Sized
    {
        self.path().pathto(p)
    }

    /// Construct mutable value pointer to some initial node.
    /// Used to begin operator `/` chain.
    fn path_mut<'tr>(&'tr mut self) -> ValuePtrMut<'tr, Self>
        where Self: ValueReader + ValueWriter + Sized
    {
        ValuePtrMut::new(Some(self))
    }

    /// Construct mutable value pointer and move it follwoing sub path.
    /// Can also begin operator `/` chain, or got target within one call.
    fn pathto_mut<'tr>(&'tr mut self, p: &str) -> ValuePtrMut<'tr, Self>
        where Self: ValueReader + ValueWriter + Sized
    {
        self.path_mut().pathto(p)
    }
}


/// The rust type for scalar json node, which can used after operator `|` to read,
/// or/and operator `<<` to write. Only support `i64` for integer, to make use literal
/// number more convenient.
pub trait ScalarValue {}
impl ScalarValue for String {}
impl ScalarValue for &str {}
impl ScalarValue for i64 {}
impl ScalarValue for f64 {}
impl ScalarValue for bool {}
impl ScalarValue for () {}

/// Extend method to read Value, and support operator `| rhs_default`.
/// The default implementation just return `rhs` without any treatment.
/// It is dependent for concrete `Value` type how extract value from node.
pub trait ValueReader {
    /// Support operator `| ""` or some default `&str`. 
    /// Usually return slice refers to string held in node if possible.
    fn get_str<'tr>(&'tr self, rhs: &'tr str) -> &'tr str { rhs }

    /// Support operator `| String`. 
    /// For json pointer it would return stringfy of node except already string.
    fn get_string(&self, rhs: String) -> String { rhs }

    /// Support operator `| 0` or some default `i64`. 
    /// For json pointer, it will also try to parse from string node.
    fn get_i64(&self, rhs: i64) -> i64 { rhs }

    /// Support operator `| 0.0` or some default `f64`. 
    /// For json pointer, it will also try to parse from string node.
    fn get_f64(&self, rhs: f64) -> f64 { rhs }

    /// Support operator `| false`. 
    /// For json pointer, it will also try to parse from string node.
    fn get_bool(&self, rhs: bool) -> bool { rhs }
}

/// Extend method to read Value, and support operator `<< rhs`.
/// The default implementation just return `self` without any modification.
/// It is dependent for concrete `Value` type how modify the `lhs` node.
pub trait ValueWriter {
    /// Put scalar to usually leaf onde, overwrite its value and may type.
    /// For json pointer, it is also possible to overwrite any node type 
    /// include object and array.
    fn put_value<T>(&mut self, rhs: T) -> &mut Self
        where Self: From<T>, T: ScalarValue
    {
        self
    }

    /// Push a key-value pair to usually object-like node.
    /// For json pointer, it would change `lhs` to object if it is not.
    fn push_object<K: ToString, T>(&mut self, key: K, val: T) -> &mut Self
        where Self: From<T>
    {
        self
    }

    /// Push a item to usually array-like node.
    /// For json pointer, it would change `lhs` to array if it is not.
    fn push_array<T>(&mut self, val: T) -> &mut Self
        where Self: From<T>
    {
        self
    }
}

