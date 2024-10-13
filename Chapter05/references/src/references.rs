// Copied from: https://github.com/jduan/cosmos/blob/main/rust_sandbox/rust_sandbox/src/references.rs
use std::collections::HashMap;

pub fn run() {
    let mut table = create_table();
    show_table(&table);
    assert_eq!(table["Gesualdo"][0], "many madrigals");
    sort_works(&mut table);
    assert_eq!(
        table["Caravaggio"],
        vec![
            "The Calling of St. Matthew".to_string(),
            "The Musicians".to_string(),
        ]
    );

    references_are_explicit();
    dot_operator();
    dot_operator2();
    reference_to_reference();
    compare_references();
    borrow_ref_of_any_expr();
    static WORTH_POINTING_AT: i32 = 1000;
    update_globa_var2(&WORTH_POINTING_AT);
    distinct_lifetimes();
    share_vs_mutate2();
}

// type alias
type Table = HashMap<String, Vec<String>>;

pub fn create_table() -> Table {
    let mut table = Table::new();
    table.insert(
        "Gesualdo".to_string(),
        vec![
            "many madrigals".to_string(),
            "Tenebrae Responsoria".to_string(),
        ],
    );
    table.insert(
        "Caravaggio".to_string(),
        vec![
            "The Musicians".to_string(),
            "The Calling of St. Matthew".to_string(),
        ],
    );
    table.insert(
        "Cellini".to_string(),
        vec![
            "Perseus with the head of Medusa".to_string(),
            "a salt cellar".to_string(),
        ],
    );

    table
}

// Immutable Reference
pub fn show_table(table: &Table) {
    for (artist, works) in table {
        println!("works by {}", artist);
        for work in works {
            println!("   {}", work);
        }
    }
}

// Mutable Reference
pub fn sort_works(table: &mut Table) {
    for works in table.values_mut() {
        works.sort();
    }
}

// In Rust, references are created explicitly with the & operator, and dereferenced explicitly with
// the * operator.
pub fn references_are_explicit() {
    // immutable reference
    let x = 10;
    let r = &x;
    assert_eq!(*r, 10);

    // mutable reference
    let mut y = 32;
    // To borrow y as a mutable reference, y has to be defined as immutable as well
    let m = &mut y;
    *m += 32;
    assert_eq!(*m, 64);
}

// Since references are so widely used in Rust, the . operator implicitly dereferences its left
// operand, if needed.
//
// See: https://stackoverflow.com/questions/28519997/what-are-rusts-exact-auto-dereferencing-rules/28552082#28552082
//
// Other examples:
// 1. The println! macro expands to code that uses the dot operator, so it takes advantage of this
//    implicit dereference as well.
// 2. A "for" loop also expands to code that uses the dot operator. That's why you can iterate over
//    a reference to a collection without dereferencing it.
pub fn dot_operator() {
    struct Anime {
        name: &'static str,
        #[allow(dead_code)]
        bechdel_pass: bool,
    }

    let aria = Anime {
        name: "Aria: The Animation",
        bechdel_pass: true,
    };

    let aria_ref = &aria;

    assert_eq!(aria_ref.name, "Aria: The Animation");

    // Equivalent to the above, but with the dereference written out:
    assert_eq!((*aria_ref).name, "Aria: The Animation");
}

// The . operator can also implicitly borrow a reference (immutable & mutable) to its left operand,
// if needed for a method call.
pub fn dot_operator2() {
    let mut v = vec![1973, 1968];
    v.sort();
    // same as above but much uglier
    (&mut v).sort();
    assert_eq!(v, vec![1968, 1973]);
}

pub struct Point {
    #[allow(dead_code)]
    x: i32,
    y: i32,
}

pub fn reference_to_reference() {
    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    let rr: &&Point = &r;
    // The type can be inferred by Rust.
    let rrr: &&&Point = &rr;

    // The dot operator follows as many references as it takes to find its target!
    assert_eq!(729, rr.y);
    assert_eq!(729, rrr.y);
}

pub fn compare_references() {
    let x = 10;
    let y = 10;

    let rx = &x;
    let ry = &y;

    let rrx = &rx;
    let rry = &ry;

    // This assertion succeeds, even though rrx and rry point at different values
    // (namely, rx and ry), because the == operator follows all the references and performs the
    // comparison on their final targets, x and y.
    assert_eq!(rrx, rry);

    // If you actually want to know whether two references point to the same memory, you can use
    // std::ptr::eq, which compares them as addresses:
    assert!(!std::ptr::eq(rx, ry));

    assert_eq!(rx, ry);
    assert_eq!(*rx, y);
    assert_eq!(*rrx, ry);
    // The line below doesn't compile because you can't compare references of different types.
    // assert_eq!(rrx, ry);
}

// Rust lets you borrow a reference to the value of any sort of expression at all.
// In situations like this, Rust simply creates an anonymous variable to hold the expression’s
// value, and makes the reference point to that. The lifetime of this anonymous variable depends on
// what you do with the reference:
//
// * If you immediately assign the reference to a variable in a let statement (or make it part of
// some struct or array that is being immediately assigned), then Rust makes the anonymous
// variable live as long as the variable the let initializes. In the example below, Rust
// would do this for the referent of r.
//
// * Otherwise, the anonymous variable lives to the end of the enclosing statement. In our
// example, the anonymous variable created to hold 1009 lasts only to the end of the
// assert_eq! statement.
#[allow(clippy::op_ref)]
pub fn borrow_ref_of_any_expr() {
    fn factorial(n: usize) -> usize {
        (1..(n + 1)).product()
    }

    // reference to a function call
    let r = &factorial(6);

    assert_eq!(r + &1009, 1729);
}

// Every variable needs to be initialized before you can use it.
// STASH is a "global" variable.
static mut STASH: &i32 = &128;

// This function doesn't compile!
// Since we must allow 'a to be any lifetime, things had better work out if it’s the smallest
// possible lifetime: one just enclosing the call to f.
// Since STASH lives for the program’s entire execution, the reference type it holds must have a
// lifetime of the same length; Rust calls this the 'static lifetime. But the lifetime of p’s
// reference is some 'a, which could be anything, as long as it encloses the call to f. So, Rust
// rejects our code.
// fn update_globa_var<'a>(p: &'a i32) {
//     // Mutable statics are inherently not thread-safe (after all, any thread can access a static at
//     // any time), and even in single-threaded programs, they can fall prey to other sorts of
//     // reentrancy problems. For these reasons, you may access a mutable static only within an
//     // unsafe block.
//     unsafe {
//         STASH = p;
//     }
// }

// This time, the function's signature spells out that p must be a reference with lifetime 'static,
// so there’s no longer any problem storing that in STASH. We can only apply f to references to
// other statics, but that’s the only thing that’s certain not to leave STASH dangling anyway
//
// Take a step back, though, and notice what happened to f’s signature as we amended our way to
// correctness: the original f(p: &i32) ended up as f(p: &'static i32). In other words, we were
// unable to write a function that stashed a reference in a global variable without reflecting that
// intention in the function’s signature. In Rust, a function’s signature always exposes the body’s
// behavior.
//
// Conversely, if we do see a function with a signature like g(p: &i32) (or with the lifetimes
// written out, g<'a>(p: &'a i32)), we can tell that it does not stash its argument p anywhere that
// will outlive the call. There’s no need to look into g’s definition; the signature alone tells us
// what g can and can’t do with its argument. This fact ends up being very useful when you’re
// trying to establish the safety of a call to the function.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn update_globa_var2(p: &'static i32) {
    // Mutable statics are inherently not thread-safe (after all, any thread can access a static at
    // any time), and even in single-threaded programs, they can fall prey to other sorts of
    // reentrancy problems. For these reasons, you may access a mutable static only within an
    // unsafe block.
    unsafe {
        STASH = p;
    }
}

pub fn ref_in_struct() {
    // This says: the lifetime of any reference you store in r had better enclose 'a, and
    // 'a must outlast the lifetime of whatever you store the S.
    #[derive(Debug)]
    struct S<'a> {
        // Whenever a reference type appears inside another type’s definition, you must write out
        // its lifetime.
        #[allow(dead_code)]
        r: &'a i32,
    }

    let s;
    {
        let x = 10;
        // The expression S { r: &x } creates a fresh S value with some lifetime 'a. When you
        // store &x in the r field, you constrain 'a to lie entirely within x’s lifetime.
        // The assignment s = S { ...  } stores this S in a variable whose lifetime extends to the
        // end of the example, constraining 'a to outlast the lifetime of s. And now Rust has
        // arrived at the same contradictory constraints as before: 'a must not outlive x, yet must
        // live at least as long as s. No satisfactory lifetime exists, and Rust rejects the code.
        s = S { r: &x };
        println!("s is {:?}", s);
    }
    // This line doesn't compile. See the above for now.
    // If you move this line to the inner block right after "s = S {...}", it will compile.
    // assert_eq!(10, *s.r);
}

// This isn't much different from the function above.
pub fn struct_inside_struct() {
    #[allow(dead_code)]
    struct S {
        name: String,
    }

    // We can’t leave off S’s lifetime parameter here: Rust needs to know how a T’s lifetime
    // relates to that of the reference in its S
    #[allow(dead_code)]
    struct T<'a> {
        s: &'a S,
    }
}

// Function signatures can have similar effects. Suppose we have a function like this:
//
// fn f<'a>(r: &'a i32, s: &'a i32) -> &'a i32 { r  } // perhaps too tight
//
// Here, both reference parameters use the same lifetime 'a, which can unnecessarily constrain the
// caller in the same way we’ve shown previously. If this is a problem, you can let parameters’
// lifetimes vary independently:
//
// fn f<'a, 'b>(r: &'a i32, s: &'b i32) -> &'a i32 { r  } // looser
//
// Tactic: try the simplest possible definition first, and then loosen restrictions until the code
// compiles
pub fn distinct_lifetimes() {
    // If you change x and y to have the same lifetime 'a, the code wouldn't compile.
    #[allow(dead_code)]
    struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }

    let x = 10;
    let r;

    {
        let y = 20;
        {
            let s = S { x: &x, y: &y };
            r = s.x;
        }
    }

    println!("r is {}", r);
}

// Throughout its lifetime, a shared reference makes its referent read-only: you may not assign to
// the referent or move its value elsewhere.
pub fn share_vs_mutate() {
    let v = vec![1, 2, 3, 4, 5];
    let r = &v;

    // This line doesn't compile because you can't move "v" while the reference r (pointing to v)
    // is still alive (due to the println! line below.)
    // let aside = v;

    println!("first element of vector is {}", r[0]);
}

// Extend a vector with elements of a slice
pub fn extend(vec: &mut Vec<i32>, slice: &[i32]) {
    for elt in slice {
        vec.push(*elt);
    }
}

// Rust's rules for mutation and sharing:
//
// * Shared access is read-only access. Values borrowed by shared references are read-only. Across
// the lifetime of a shared reference, neither its referent, nor anything reachable from that
// referent, can be changed by anything. There exist no live mutable references to anything in that
// structure; its owner is held read-only; and so on. It’s really frozen.
//
// * Mutable access is exclusive access. A value borrowed by a mutable reference is reachable
// exclusively via that reference. Across the lifetime of a mutable reference, there is no other
// usable path to its referent, or to any value reachable from there. The only references whose
// lifetimes may overlap with a mutable reference are those you borrow from the mutable reference
// itself.
pub fn share_vs_mutate2() {
    let mut wave = Vec::new();
    let head = vec![1, 2];
    let tail = [3, 4];

    extend(&mut wave, &head);
    extend(&mut wave, &tail);

    assert_eq!(wave, vec![1, 2, 3, 4]);

    // this line doesn't compile
    // While extending wave, it may run out of space so Rust needs to allow some new space on the
    // heap. That would render the other "&wave" pointer dangling!
    // extend(&mut wave, &wave);
}

#[cfg(test)]
mod tests {
    #[test]
    fn mutablity_can_be_changed_when_ownership_is_transferred() {
        let immutable_box = Box::new(5);
        assert_eq!(5, *immutable_box);

        let mut mutable_box = immutable_box;
        *mutable_box = 4;
        assert_eq!(4, *mutable_box);
    }
}
