# Ownership and Moves

In managing memory, we want two characteristics from our programming language:

1. Language can free memory promptly, at the time of our choosing. This would give us more control over the memory usage of our program.
1. We never want to use a pointer to an object after it's being freed.
   - This is the undefined behavior.
   - And this is the most common way of introducing vulnerabilities in the programs.

In safety first camps, we have Python, Java, and C# that manage memory for us. In performance first camps, we have C and C++ that give us more control over memory. The performance camp is great, if you never make mistakes. But if you do, you can introduce vulnerabilities in your program.

So, there's something fundamental need to be changed. And the change is **Rust**.

- Rust breaks this deadlock between safety and performance. By making check's at compile time and while running our pointers are simple addresses.
- Rust also make's it easy to write multi-threaded programs, and it's quite easy to handle mutexes, message channels, atomic values and so on..
- Rust provides a sweet spot between safety and performance by using the _ownership_ and _borrowing_ rules.

# Ownership

In c/c++, there are some owning objects that decide whether to free the owned object or not.

For example,

```cpp
std::string s = "hello world";
```

In memory, it looks like this:

```

Stack frame

    Buffer      Capacity    Length
+-----------+------------+------------+
|           |            |            |
+-----------+------------+------------+
        |          |
        v          v
  Heap
+-----------+------------+
| hello     | world      |
+-----------+------------+

```

When the `s` goes out of scope, the destructor of `std::string` will be called and the memory will be freed.

In Rust, when the variables go out of scope, the memory will be freed -- dropped, in rust terminology, the owned value will be dropped too.
For example,

In past some C++ libraries shared a single buffer amoung serveral `std::string` values, and using a reference count to decide when the buffer should be freed. But when the string is destroyed, the pointer becomes invalid. Then it's the job of the programmer(us) to make sure that we are not using it anymore.

```rust
fn print_padovan() {
    let mut padovan = vec![1, 1, 1]; // allocated here
    for i in 3..10 {
        let next = padovan[i - 3] + padovan[i - 2];
        padovan.push(next);
    }
    println!("P(1..10) = {:?}", padovan);
} // dropped here
```

#### Rust Box type:

- A `Box<T>` is a pointer to a value of type `T` that is allocated on the heap.
- When a `Box<T>` goes out of scope, the memory will be freed.

```rust
    {
        let point = Box::new((0.625, 0.5));
        let label = format!("{:?}", point);
        assert_eq!(label, "(0.625, 0.5)");
    }
```

Another example,

```rust

struct Person {
    name: String,
    birth: i32,
}

let mut composers = Vec::new();
composers.push(Person {
    name: "Palestrina".to_string(),
    birth: 1525,
});
composers.push(Person {
    name: "Dowland".to_string(),
    birth: 1563,
});
composer.push(Person {
    name: "Lully".to_string(),
    birth: 1632,
});

for composer in &composers {
    println!("{}, born {}", composer.name, composer.birth);
}

```

Rust has the ideas:

- You can move valus from one owner to another.
- Very simple types like integers, floats, and pointers are excused from the ownership rules.
- The standard library provides the reference-counted pointer types `Rc<T>`, and `Arc<T>` which allows values to have multiple owners, under some conditions.
- You can borrow a reference to a value, references are non-owning pointers with limited lifetimes.

# Moves

The comparison of moves in C++/Rust and python:

- In C++, when you move a value, you are moving the ownership of the value and making a deep copy of the value.
- In Rust, when you move a value, you are moving the ownership of the value and invalidating the old value. (means uninitialized the old value)
- In Python, when you move a value, it only increments the reference count of the value.

# More operations that move:

- Moving value like this sounds like inefficient, but
  - The move appliest to the value proper, not the heap storage they own.
  - In vectors, the value proper is a `three-word header` alone, not the heap storage.
  - Rust compiler is good enough at "seeing through" the moves and optimizing them away.

The moves are not limited to the assignment operator. They also happen in the following cases:

- When passing a value to a function.
- When returning a value from a function.
- When constructing a value.

# Moves and Control flow

```rust
let x = vec![10, 20, 30];
if c { // assume the c condition
  f(x); // It good to move value here
} else {
  g(x); // And it's also ok to move value here
}

h(x); // bad: x is uninitialized here if the flow uses either the path
```

For similiar reasons, moving a value in a loop is forbidden:

```rust
let mut v = vec![10, 20, 30];

while f() {
  g(x); // bad: x would be moved in the first iteration
  // uninitialized in the second iteration

  // But if we give another value to x, it's ok
  x = h();
}
e(x);
```

# Moves and Indsxed Content:

# Copy types: The exception to Moves

# `Rc` and `Arc` types: Shared Ownership

- `Arc` is the atomic reference counted type, which is thread safe.
- `Rc` is the reference counted type, which is not thread safe.
