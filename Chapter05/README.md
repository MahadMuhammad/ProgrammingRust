# Chapter 5 - References

```
Libraries cannot provide new inabilites -- Mark Miller
```

- The simple `Box<T>` heap pointer, and the pointers internal to `String` and `Vec<T>` values are owning pointers. Means when the owning pointer goes out of scope, the referent goes with it.
- Rust also has non-owning pointers, which are called `references`, which have no effect on their referent's lifetimes.
  - In fact, it's rather the opposite: references must never outlive their referents.
  - To make this possible, rust refers to creating a reference to some value as `borrowing` that value.
- This feature in rust, is outside of research programming languages, and is called `borrowing` in rust.
- In rust, every data type incorporate a `lifetime` information to ensure that they are valid and used safely.

# References to Values:

As an example, we are building a table of artists and their works. So, we can use this data type.

```rust
// Passing a non-copyable data structure by value moves it.

// error-pattern: borrow of moved value: `table`
// error-pattern: move occurs because `table` has type `std::collections::HashMap<std::string::String, std::vec::Vec<std::string::String>>`, which does not implement the `Copy` trait

use std::collections::HashMap;

type Table = HashMap<String, Vec<String>>;

fn show(table: Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}

fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),
                 vec!["many madrigals".to_string(),
                      "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(),
                 vec!["The Musicians".to_string(),
                      "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(),
                 vec!["Perseus with the head of Medusa".to_string(),
                      "a salt cellar".to_string()]);

    show(table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");
}
```

- This definition of `show` raises a few concerns:

  - The `table` is passed to function by value, which means that the `table` is moved, it was no longer available.
  - The `table` is not copied, because the `Table` type does not implement the `Copy` trait.
  - But if we try to use it after in the `main` function, we get the error.

- But for simply printing, we need to move the value of a data structure into a completely new function.
- The right way to handle this, is to use `references`.

  - A references lets you access the value without changing it's ownership.

- References are of two types:
  - **Shared References:** lets you read but not write.
    - We can have as many shared references as we want.
    - The expression `&e` yields a reference to `e` value.
    - Shared references are `Copy` types.
  - **Mutable References:** lets you both read and write.
    - We can have only one mutable reference.
    - `&mut e` yields a mutable reference to `e` value.
    - Mutable references are not `Copy` types.
- This rule enforces, the single `writer` and multiple `readers` rule.
- As long as the shared references to a value, the owner cannot modify the value of that data type.
- If we want to give the shared `reference` we need to do this:

```rust
show(&table);
```

- References are non-owning pointers, so the table remains the owner of the entire structure. But it cannot modify the value of the data structure, when the show function is running.

```rust
fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("  {}", work);
        }
    }
}
```

- When we pass a value to a function in a way that moves it's ownershoip, we say that we have passed it by **value**.
- When we pass a reference to a value, we say that we have passed it by **reference**.
- The `&` operator is used to create a reference to a value.
