# type-change

It can be converted to another type with the same name field.

## Usage
```rust
use type_change::From;

#[derive(Clone)]
struct Foo {
    id: i64,
    name: String,
}

#[derive(From)]
#[from(Foo)]
struct Bar {
    id: i64,
    name: String,
}

// equal to follows
//
// impl From<Foo> for Bar {
//     fn from(foo: Foo) -> Bar {
//         Bar { id: foo.id, name: foo.name }
//     }
// }
//

let foo = Foo { id: 1, name: "foo".to_string() };
let bar = Bar { name: "bar".to_string(), ..foo.clone().into() };
assert_eq!(foo.id, bar.id);

```

## Notes
- Only struct with the same field name can be converted.
- All field names must match and be accessible.

## Contributing
**Thanks!**

