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

- When we pass a value to a function in a way that moves it's ownership, we say that we have passed it by **value**.
- When we pass a reference to a value, we say that we have passed it by **reference**.
- The `&` operator is used to create a reference to a value.

# Working with References:

With references, we can access or manipulate the data without changing the ownership of the data.

## Rust References Versus C++ References:

- In C++ and Rust, but references are just addresses at machine level. But rust references are feel different.
- In C++, references are created implicitely by conversion and dereferencing implicitly too.

```cpp
//C++ code
int x = 10;
int& r = x; // r is a reference to x
assert(r == 10);
r = 20; // x is now 20
```

- In rust, the references are created explicitly with the `&` operator.
- And dereferenced explicitly with the `*` operator.

```rust
// error-pattern: cannot assign to `x` because it is borrowed
// error-pattern: cannot borrow `x` as mutable because it is also borrowed as immutable
// error-pattern: cannot borrow `y` as mutable more than once at a time
// error-pattern: cannot use `y` because it was mutably borrowed

fn main() {
    let mut x = 10;
    let r1 = &x;
    let r2 = &x;     // ok: multiple shared borrows permitted
    x += 10;         // error: cannot assign to `x` because it is borrowed
    let m = &mut x;  // error: cannot borrow `x` as mutable because it is
                     // also borrowed as immutable

    let mut y = 20;
    let m1 = &mut y;
    let m2 = &mut y;  // error: cannot borrow as mutable more than once
    let z = y;        // error: cannot use `y` because it was mutably borrowed
}
```

But there's an exception.

- The `.` operator can implicity borrow a reference to its left operand if needed for a method call. For example, vectors `sort` method takes a mutable reference to the vector, so these two calls are equivalent.

```rust
let mut v = vec![1973, 1968];
v.sort();   // implicit borrowing
(&mut v).sort(); // equivalent but more verbose
```

In a nutshell, whereas C++ converts implicitly between references and left values (that is expressions references to locations in memory), With these conversions appearing anywhere they are needed in rust, you use the `&` and `*` operator to create and follow references with the exception of `.` operator, which borrows and dereferences implicitly.

## Assigning references:

```rust
let x = 10;
let y = 20;

let mut r = &x;

if b {r = &y;}

assert(*r == 10 || *r == 20);
```

Assigning a reference to a variable makes that variable point to somewhere new. The reference `r` initially points to the variable X, but if B is true, the code points it at variable Y instead.

This behavior may seem too obvious to be worth mentioning. Of course, variable X are now points to variable Y, since we stored `&Y` in it, but you point this out because C references behave very differently as shown earlier. Assigning a value to our reference in C stores the value to its reference.

- Once the C++ reference has been initialized, there is no way to make it to point at anything else.

## References to References:

- Rust permits references to references.
- The dot expression guided by the type of the variable actually traverse the references to that point before fetching the actual value of the type itself.

## Creating References:

- Like the `.` operator, rust comparisons operator "see through" any number of references.

```rust
fn comparing_references() {
        let x = 10;
        let y = 10;

        let rx = &x;
        let ry = &y;

        let rrx = &rx;
        let rry = &ry;

        assert!(rrx <= rry);
        assert!(rrx == rry);

        assert!(rx == ry); // their references are equql i.e. x == y
        assert!(!(std::ptr::eq(rx, ry))); // but occupy different addresses in memory

        // assert!(rx == rrx); // error: type mismatch `&i32` vs `&&i32`
        assert!(rx == *rrx) // this is okay
    }
```

- The final assertion here succeeds even though hard rrx and rry point at different values. Because the double equal operator follows all the references and to perform the comparison of their final targets.
- This is almost always the behavior you want, especially when writing generic functions.
- If you actually want to know whether two references point the same memory location, you can use `std::ptr::eq`, which compares them as addresses.
- One more thing to note that the opponents of the comparison operator must have exactly the same type, including their references.

## The references are never null:

- Rust references are never null.
- There is no analog to C's Null or C nullptr.
- There is no default initial value for a reference or you cannot use any variable until it initialize, regardless of its type.
- Rust won't convert integer to references(Outside of `unsafe` code), so you can't convert zero to a difference.
- C and C++ code often uses a null pointer to indicate the absence of a value.
- For example, the malloc function Returns either a pointer to a new block of memory or nullptr if there isn't enough memory available to satisfy the request.
- In rust, if you need a value that is either a reference to something or not, you can use the `option<&T>` type.
- At the machine level, Rust represents `None` as null pointer and some. as the non-zero addresses.
- So option is just as efficient as the label pointer in C or cpp. Although it is safer, its type requires you to check whether it's none before you can use it.

## Boring references to arbitrary expressions:

- Whereas C and CPP only lets you apply the ampersand operator to certain kind of expressions.
- Rust lets you borrow a reference to a variable of any sort of expression at all

```rust
    fn borrowing_references_to_arbitrary_expressions() {
        fn factorial(n: usize) -> usize {
            (1..n + 1).product()
        }
        let r = &factorial(6);
        // arithmetic operators can see through one level of references
        assert_eq!(r + &1009, 1729);
    }
```

- In situations like this, rust simply creates an anonymous variable to hold the expressions value and makes the reference point to that
- The lifetime of this anonymous variable depends upon what you do with the reference.

  - If you immediately assign the reference to a variable in the let statement Then just make the anonymous variable live, as long as the variable His active
  - Otherwise, the anonymous variable lives to the end of the enclosing statement

- If you're used to C or C++, this may sound error prone, but Rust will always report the problem to you at compile time So you do not need to worry about this.

# References to slices or trait objects:

- The references we have shown so far are all simple addresses
- However, rust also includes two types of _fat pointers_.
  - Two-word values carrying the addresses of some value along with some further information necessary to put the value to use.
  - A reference to a slice is a fat pointer carrying the starting index of the slice and its length.
- Rust other kind of fat pointer is a trait object. A reference to a value that implements a certain trait, a trait object carries a values address and a pointer to the traits implementation, appropriate to that value for invoking the triats method.
- Apart from carrying this extra data, slice and trait objects references behave just like the other sort of references we have seen so far in this chapter.
- They don't own their referents. They are not allowed to outlive their referents. They may be mutable or shared and so on.
