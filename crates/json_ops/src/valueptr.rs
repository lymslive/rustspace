use crate::adopter::*;

/// Wrap `Option<&Value>` as pointer to json node for operator overload.
///
/// It can used as `Option` implicitly at most time, as overload `*` Deref trait,
/// where `None` means refer to non-exist node, and `'tr` lifetime refers to 
/// the overall json tree.
#[derive(Eq, PartialEq, Debug)]
pub struct ValuePtr<'tr, Value>
where Value: ValuePath + ValueReader
{
    ptr: Option<&'tr Value>,
}

// atuo dervie(Copy, Clone) failed as `Value: Copy` may not satisfied.
// manually implent Copy instead.
impl<'tr, Value> Copy for ValuePtr<'tr, Value>
where Value: ValuePath + ValueReader
{
}

impl<'tr, Value> Clone for ValuePtr<'tr, Value>
where Value: ValuePath + ValueReader
{
    fn clone(&self) -> Self {
        *self
    }
}

/// Mutable josn pointer wrapper of `Optione<&mut Value>` for operator overload.
///
/// It can used as `Option` implicitly at most time, as overload `*` Deref trait,
/// where `None` means refer to non-exist node, and `'tr` lifetime refers to 
/// the overall json tree.
///
/// Note that mutable reference don't support copy, only use it when you really 
/// need to modify the pointed json node, otherwise use the immutable pointer.
#[derive(Eq, PartialEq, Debug)]
pub struct ValuePtrMut<'tr, Value>
where Value: ValuePath + ValueReader + ValueWriter
{
    ptr: Option<&'tr mut Value>,
}

/// Proxy `get_*` methods of `Value` for json pointer.
macro_rules! scalar_getter {
    ($func_name:ident | $ret:ty) => {
        /// Forward the getter method to pointed node, or return `rhs` by default.
        fn $func_name(&self, rhs: $ret) -> $ret {
            match self.ptr {
                Some(v) => v.$func_name(rhs),
                None => rhs,
            }
        }
    };
}

impl<'tr, Value> ValuePtr<'tr, Value>
where Value: ValuePath + ValueReader
{
    /// Trivial new constructor.
    /// Usually there is no need to create `ValuePtr` instance directly, but yield one
    /// from existed json `Value`, except `None`.
    pub fn new(ptr: Option<&'tr Value>) -> Self {
        Self { ptr }
    }

    /// Resolve to sub path, by single index.
    /// Used in operator `/`.
    fn path_index(&self, i: usize) -> Self {
        match self.ptr {
            Some(v) => Self::new(v.get_index(i)),
            None => Self::new(None)
        }
    }

    /// Resolve to sub path, by single key or joined path.
    /// Used in operator `/`.
    fn path_str(&self, p: &str) -> Self {
        if self.ptr.is_none() {
            return Self::new(None);
        }

        let v = self.ptr.unwrap();
        let target = v.get_key(p);
        if target.is_some() {
            Self::new(target)
        }
        else {
            self.pathto(p)
        }
    }

    /// Resolve to sub path by json pointer syntax but leading '/' is optional.
    pub fn pathto(&self, p: &str) -> Self {
        if self.ptr.is_none() {
            return Self::new(None);
        }

        let mut fixp = p;
        if !p.is_empty() && p.chars().nth(0) == Some('/') {
            fixp = &p[1..];
        }

        let target = fixp.split(&['/', '.'][..])
            .map(|x| x.replace("~1", "/").replace("~0", "~"))
            .try_fold(self.ptr.unwrap(), |value, token| {
                value.get_key(&token).or_else(||
                    token.parse::<usize>().ok().and_then(|x| value.get_index(x))
                    )
            });

        Self::new(target)
    }

    /// Get a str ref if the value type matches, or defalut `rhs`.
    /// Used in operator `| ""` or `| &str`.
    fn get_str(&self, rhs: &'tr str) -> &'tr str {
        match self.ptr {
            Some(v) => v.get_str(rhs),
            None => rhs,
        }
    }

    scalar_getter!(get_string | String);
    scalar_getter!(get_i64 | i64);
    scalar_getter!(get_f64 | f64);
    scalar_getter!(get_bool | bool);

}

impl<'tr, Value> ValuePtrMut<'tr, Value>
where Value: ValuePath + ValueReader + ValueWriter
{
    /// Trivial new constructor.
    /// Usually there is no need to create `ValuePtr` instance directly, but yield one
    /// from existed json `Value`, except `None`.
    pub fn new(ptr: Option<&'tr mut Value>) -> Self {
        Self { ptr }
    }

    /// Convert to immutable pointer, leave self None.
    pub fn immut(&mut self) -> ValuePtr<'tr, Value> {
        if self.ptr.is_none() {
            return ValuePtr::new(None);
        }
        let v = self.ptr.take().unwrap();
        ValuePtr::new(Some(v))
    }

    /// Resolve to sub path, by single index.
    /// Used in operator `/`.
    fn path_index(&mut self, i: usize) -> Self {
        match self.ptr.take() {
            Some(v) => Self::new(v.get_index_mut(i)),
            None => Self::new(None)
        }
    }

    /// Resolve to sub path, by single index or joined path.
    /// Used in operator `/`.
    fn path_str(&mut self, p: &str) -> Self {
        if self.ptr.is_none() {
            return Self::new(None);
        }

        // use immutable get to check first, avoid mutable refer twice
        let v = self.ptr.take().unwrap();
        let target = v.get_key(p);
        if target.is_some() {
            Self::new(v.get_key_mut(p))
        }
        else {
            self.ptr = Some(v); // restore reference had took out to `v`
            self.pathto(p)
        }
    }

    /// Resolve to sub path by json pointer syntax but leading '/' is optional.
    pub fn pathto(&mut self, p: &str) -> Self {
        if self.ptr.is_none() {
            return Self::new(None);
        }

        let mut fixp = p;
        if !p.is_empty() && p.chars().nth(0) == Some('/') {
            fixp = &p[1..];
        }

        let target = fixp.split(&['/', '.'][..])
            .map(|x| x.replace("~1", "/").replace("~0", "~"))
            .try_fold(self.ptr.take().unwrap(), |value, token| {
                let try_key = value.get_key(&token);
                if try_key.is_some() {
                    value.get_key_mut(&token)
                }
                else {
                    token.parse::<usize>().ok().and_then(|x| value.get_index_mut(x))
                }
                //value.get_key_mut(&token).or_else(||
                //    token.parse::<usize>().ok().and_then(|x| value.get_index_mut(x))
                //    )
            });

        Self::new(target)
    }

    /// Put a value to json and return pointer to it, which may change the node type.
    /// Implement for `<< (val)` , usually in scarlar node.
    fn put_value<T>(&mut self, rhs: T) -> Self where Value: From<T>, T: ScalarValue {
        match self.ptr.take() {
            Some(v) => { v.put_value(rhs); Self::new(Some(v)) },
            None => Self::new(None)
        }
    }

    /// Push a pair to object node, would invalidate the pointer if type mismatch.
    /// Implment for `<< (key, val)`.
    fn push_object<K: ToString, T>(&mut self, key: K, val: T) -> Self where Value: From<T> {
        match self.ptr.take() {
            Some(v) => { v.push_object(key, val); Self::new(Some(v)) },
            None => Self::new(None)
        }
    }

    /// Push a item to array node, would invalidate the pointer if type mismatch.
    /// Implment for `<< (val, )` or  `<< [item]` .
    fn push_array<T>(&mut self, val: T) -> Self where Value: From<T> {
        match self.ptr.take() {
            Some(v) => { v.push_array(val); Self::new(Some(v)) },
            None => Self::new(None)
        }
    }

}

/* ------------------------------------------------------------ */
// Split operator overload implemntation to seperate files but not sub mod.
include!("overload.rs");
