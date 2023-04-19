# Json Pointer and Operator Overload

## Overview

Though it is wonderful to use `serde` crate to serialize and deserialize json
between native rust type, there is still case to use untyped json tree
directely, may because the json tree is too freely or too loose to map to some
strong type well, for example the json schema structure, or any other reason.

This crate focus to operate on a single node in json tree, based on json
pointer, mainly followed the standard json pointer syntax, and with operator
overload to make it more convenient and intuitive.

* Use `path()` or `path_mut()` method to create an json pointer or mutable
  one, we need new type to support operator overlaod.
* Use path opeator `/` to point to further deeper sub node.
* Use pipe operator `|` to read scalar primitive value from pointed node, may
  finallize the `/` operator chian, which would read as `get_or`.
* Use operator `<<` to put a new scalar value to node, or push more item to
  array or object node.
* Also overralod operator `*` and so the pointer can implicitly used as
  `Opion<&Value>`.

## Dependency

* `serde_json`, the json pointer is pointing to `serde_json::Value`.
* `toml`, if enable feature `toml`, as the data structure of toml is very 
  similar to json.

## Extension

The json or toml pointer is actually concrete type of more generic `Value`
pointer. So it is possible to implement pointer and operator overload for
other json-like data structure, or common value node origanized in a tree,
provided implement the following trait:

* `ValuePath`, for opertor `/`;
* `ValueReader`, for opertor `|`;
* `ValueWriter`, for opertor `<<`.

If you implemnt some `Value` struct from begining youself, then it is further
possible to overload operator for `&Value` directely, no need to use `path()`
method to begin a `/` operator chain.

## Example View

```rust
let v: Value = "{...}".parse().unwrap(); # got a Value tree from anywhere

let node = v.path() / "sub" / 0 / "subdeep";
let node = v.path() / "sub/0/subdeep";
let node = v.pathto("sub/0/subdeep");
let scalar = node | 0;
let scalar = v.path() / "path" / "to" / "string-node" | "";

let mut v: Value = ...;
let _ = v.path_mut() / "path" / "to" / "leaf" << "some new value";
let _ = v.path_mut() / "path" / "to" / "object" << ("key", "val") << ("k2", "v2");
let _ = v.path_mut() / "path" / "to" / "array" << ("val",) << ["v2"];
```

Please refer to documatation for more detailed and runable examples and tests.
