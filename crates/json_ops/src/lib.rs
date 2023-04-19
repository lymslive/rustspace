//! An alternative and may convenient way to operate on untyped json is provided
//! through operator overloading based on json pointer. 
//! The core start point is path operator `/` to create a json pointer struct,
//! which is no more than `Option<&Value>` that points to a node in json tree.
//!
//! ```rust
//! use serde_json::json;
//! use json_ops::ValuePath;
//!
//! let v = json!({"root": true, "usr": {
//!     "include": ["a.h", "b.h"],
//!     "lib": ["a.so", "b.so", {"name": "c.so", "version": "0.0.1"}],
//!     "lib64": null,
//!     "local": {"include": [], "lib": null}}
//! });
//!
//! let p1 = v.path() / "usr" / "lib" / 2 / "name";
//! let p2 = v.path() / "usr/lib/2/name";
//! let p3 = v.path() / "/usr/lib/2/name";
//! let p4 = v.pointer("/usr/lib/2/name");
//! assert!(p1 == p2 && p2 == p3 && p3.unwrap() == p4.unwrap());
//! assert!(v.path().unwrap() == &v);
//!
//! let usr = v.path() / "usr";
//! let local = v.path() / "usr" / "local";
//! let lib = "lib64";
//! let p1 = usr / lib;
//! let p2 = local / lib;
//!
//! assert!(p1.is_some());
//! assert!(p1.unwrap().is_null());
//! assert!(p2.is_none());
//! assert!(v["usr"]["local"]["lib64 or any absent"].is_null());
//! ```
//!
//! There is another struct for mutable json pointer as well from `path_mut()`.
//!
//! The difference between `/` operator and `pointer()` method:
//!
//! * operator `/` can be chained and manually split path token in compile time.
//! * each path token can be use variable and modify seperately.
//! * joined path can omit the leading `/` required by json pointer syntax.
//! * split path no need to escpace special char when key contins `/` or `~`.
//! * easy to save middle node pointer as variable and reuse later.
//!
//! The difference between `/` operator and `[]` index:
//!
//! * a bit more consistent and compact.
//! * can distinguish json null node and non-exist node.
//! * mutable pointer won't auto insert key to json object as index does. 
//! * mutable pointer won't panic when beyond range of json array as index does. 
//!
//! The pointer struct can further use operator `|` to read the primitive value
//! held in node with default fallback, and operator `<<` to overwrite leaf node
//! or push new item to array or object node.
//!
//! ```rust
//! use serde_json::json;
//! use json_ops::ValuePath;
//!
//! let mut v = json!({"int":10, "float":3.14, "array":["pi", null, true]});
//!
//! let node = v.path() / "int";
//! let val = node | 0;
//! assert_eq!(val, 10);
//! assert_eq!(v.path() / "float" | 0.0, 3.14);
//!
//! let _ode = v.path_mut() / "float" << 31.4;
//! let node = v.path_mut() / "array" / 2 << "true";
//! assert!(node.as_ref().unwrap().is_string()); // changed node type
//!
//! let _ode = v.path_mut() << ("key", "val");
//! let _ode = v.path_mut() / "array" << ("val",) << ["more"] << [100];
//! let _ode = v.path_mut() / "int" << ();
//! assert!(v["int"].is_null());
//!
//! assert_eq!(v, json!({"int":null, "float":31.4, "key":"val", "array":["pi",null,"true","val","more",100]}));
//! ```
//! When enable `toml` feature, then toml pointer can be used as the same as json.
//!

mod valueptr;
mod adopter;
mod ad_json;

#[cfg(feature = "toml")]
mod ad_toml;

pub use adopter::ValuePath;
pub use adopter::ValueReader;
pub use adopter::ValueWriter;
pub use adopter::ScalarValue;
pub use valueptr::ValuePtr;
pub use valueptr::ValuePtrMut;
