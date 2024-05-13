# rk-utils

![ci result](https://github.com/rockie/rk-utils/actions/workflows/ci.yml/badge.svg)

A collection of utility functions and data structures for rust.

## Features

- `str`: String utilities
  - `is_quoted`, `substring`, `unquote`, `url_to_nodes`, `ensure_prefix`, `ensure_suffix`, `drop_prefix`, `drop_suffix`, `join_path_segment`, `join_path_segments`
- `topo_sort`: Topological sorting
- `trie`: Trie data structure for longest matching a path of nodes

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rk-utils = "0.1"
```

## Examples

```rust
use rk_utils::StringUtil;

let s = "'Hello, World!'";
assert_eq!(s.is_quoted(), true);

let s = "'\\'Hello, World!\\''";
assert_eq!(s.unquote(true, None), "'Hello, World!'");

let s = "'Hello, World!'";
assert_eq!(s.substring(1, -1), "Hello, World!");
```

```rust
use rk_utils::topo_sort;
use std::collections::{ HashMap, HashSet };

let mut deps = HashMap::new();
deps.insert("b".to_string(), HashSet::from(["a".to_string()]));
deps.insert("c".to_string(), HashSet::from(["b".to_string()]));

let sorted = topo_sort(&deps).unwrap();
assert_eq!(sorted, ["a", "b", "c"]);
```

```rust
use crate::str::StringUtil;
use crate::trie::Trie;

let mut trie = Trie::new();

let route1 = "/cloud/instance/".url_to_nodes();
let route2 = "/cloud/".url_to_nodes();
let route3 = "/builder/instance".url_to_nodes();
let route4 = "/builder".url_to_nodes();
let route5 = "/".url_to_nodes();

trie.insert(route1, 1);
trie.insert(route2, 2);
trie.insert(route3, 3);
trie.insert(route4, 4);
trie.insert(route5, 5);

let input1 = "/cloud/instance/xxx".url_to_nodes();
assert_eq!(trie.find_longest_match(input1), Some(&1));

let input2 = "/cloud/xxx".url_to_nodes();
assert_eq!(trie.find_longest_match(input2), Some(&2));

let input3 = "/builder/instance/".url_to_nodes();
assert_eq!(trie.find_longest_match(input3), Some(&3));

let input4 = "/fjeao".url_to_nodes();
assert_eq!(trie.find_longest_match(input4), Some(&5));
```

## License

MIT