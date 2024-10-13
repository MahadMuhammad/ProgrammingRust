// copied from https://github.com/yamen225/programming-rust-book/blob/main/chapter_5/src/main.rs
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    #[test]
    fn references_to_values() {
        use std::collections::HashMap;

        type Table = HashMap<String, Vec<String>>;

        // fn show(table: Table) {
        // now it can use refernces.
        fn show(table: &Table) {
            for (artist, works) in table {
                println!("works by {}:", artist);
                for work in works {
                    println!("  {}", work);
                }
            }
        }

        fn main_fn() {
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
            /*
            show(table);
            assert_eq!(table["Gesualdo"][0], "many madrigals"); // will raise error

            output:
            error: borrow of moved value: `table`
            |
            20 |     let mut table = Table::new();
            |         --------- move occurs because `table` has type `HashMap<String, Vec<String>>`,
            |                   which does not implement the `Copy` trait
            ...
            31 |     show(table);
            |          ----- value moved here
            32 |     assert_eq!(table["Gesualdo"][0], "many madrigals");
            |                ^^^^^ value borrowed here after move
             */

            // solution to the above using shared ref
            show(&table);
            assert_eq!(table["Gesualdo"][0], "many madrigals");

            // to change the values for an object passed by ref we need mut
            fn sort_works(table: &mut Table) {
                for (_artis, works) in table {
                    works.sort();
                }
            }

            sort_works(&mut table);
        }
        main_fn();
    }

    #[test]
    fn rust_references_vs_cpp_references() {
        /*
        in cpp
        int x = 10;
        int &r = x;             // initialization creates reference implicitly
        assert(r == 10);        // implicitly dereference r to see x's value
        r = 20;                 // stores 20 in x, r itself still points to x
         */

        // Rust code
        let x = 10; // &x is a shared reference to x
        let _r = &x; // explicitly dereference r

        // creating a mutable reference using mut
        let mut y = 32;
        let m = &mut y; // &mut y is a mutable reference to y
        *m += 32; // explicitly dereference m to set y's value
        assert!(*m == 64); // and to see y's new value

        // Since references are so widely used in Rust,
        // the . operator implicitly dereferences its left operand, if needed
        struct Anime {
            name: &'static str,
            bechdel_pass: bool,
        };
        let aria = Anime {
            name: "Aria: The Animation",
            bechdel_pass: true,
        };
        let anime_ref = &aria;
        assert_eq!(anime_ref.name, "Aria: The Animation");
        // Equivalent to the above, but with the dereference written out:
        assert_eq!((*anime_ref).name, "Aria: The Animation");

        let mut v = vec![1973, 1968];
        v.sort(); // implicitly borrows a mutable reference to v
        (&mut v).sort(); // equivalent, but more verbose
    }

    #[test]
    fn assigning_references() {
        let x = 10;
        let y = 20;
        let mut r = &x;
        let b = true;
        if b {
            r = &y; // ref is now pointing somewhere else
        }
        assert!(*r == 20);
    }

    #[test]
    fn references_to_references() {
        struct Point {
            x: i32,
            y: i32,
        }
        let point = Point { x: 1000, y: 729 };
        let r: &Point = &point;
        let rr: &&Point = &r;
        let rrr: &&&Point = &rr;

        assert_eq!(rrr.y, 729);
    }

    #[test]
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

    #[test]
    fn borrowing_references_to_arbitrary_expressions() {
        fn factorial(n: usize) -> usize {
            (1..n + 1).product()
        }
        let r = &factorial(6);
        // arithmetic operators can see through one level of references
        assert_eq!(r + &1009, 1729);
    }

    #[test]
    fn borrowing_a_local_variable() {
        /*
        {
            let r;
            {
                let x = 1;
                r = &x;
            }
            assert_eq!(*r, 1) // bad: reads memory `x``used to occupy`
        }

        output:
        error: `x` does not live long enough
        --> references_dangling.rs:8:5
        |
        7  |         r = &x;
        |             ^^ borrowed value does not live long enough
        8  |     }
        |     - `x` dropped here while still borrowed
        9  |     assert_eq!(*r, 1);  // bad: reads memory `x` used to occupy
        10 | }
         */
    }

    #[test]
    fn receiving_references_as_function_arguments() {
        /*
        // this code can't run
        static mut STASH: &i32
        fn f(p: &i32) {STASH = p;}

        // enhance it better as rust won't let us muttate a static
        static mut STASH: &i32 = &128;
        fn f(p: &i32) {
            // still not good enough
            unsafe {
                STASH = p;
            }
        }

        outputs:
        error: lifetime may not live long enough
        --> src/main.rs:236:17
            |
        233 |         fn f(p: &i32) {
            |                 - let's call the lifetime of this reference `'1`
        ...
        236 |                 STASH = p;
            |                 ^^^^^^^^^ assignment requires that `'1` must outlive `'static`

         */

        static mut STASH: &i32 = &128;
        fn f(p: &'static i32) {
            // adding static lifetime to match the lifetime of STASH
            unsafe {
                STASH = p;
            }
        }
        static WORTH_POINTING_AT: i32 = 1000;
        f(&WORTH_POINTING_AT);
    }

    #[test]
    fn returning_references() {
        // v should have at least one element
        // fn smallest<'a>(v: &'a [i32]) -> &'a i32 { // with life time
        fn smallest(v: &[i32]) -> &i32 {
            // ommiting life time
            let mut s = &v[0];
            for r in &v[1..] {
                if *r < *s {
                    s = r;
                }
            }
            s
        }
        let s;
        /*
        {
            let parabola = [9, 4, 1, 0, 1, 4, 9];
            s = smallest(&parabola);
        }
        assert_eq!(*s, 0); // bad: points to element of dropped array

        outputs:
        error[E0597]: `parabola` does not live long enough
        --> src/main.rs:260:26
            |
        259 |             let parabola = [9, 4, 1, 0, 1, 4, 9];
            |                 -------- binding `parabola` declared here
        260 |             s = smallest(&parabola);
            |                          ^^^^^^^^^ borrowed value does not live long enough
        261 |         }
            |         - `parabola` dropped here while still borrowed
        262 |         assert_eq!(*s, 0);
            |         ----------------- borrow later used here
         */
        // moving to the appropriate lifetime
        {
            let parabola = [9, 4, 1, 0, 1, 4, 9];
            s = smallest(&parabola);
            assert_eq!(*s, 0);
        }
    }

    #[test]
    fn structs_containing_references() {
        /* This does not compile
        struct S {
            r: &i32
        }
        let s;
        {
            let x = 10;
            s = S {r: &x};
        }
        assert_eq!(*s.r, 10);
        outputs:
        error[E0106]: missing lifetime specifier
        --> src/main.rs:299:16
            |
        299 |             r: &i32,
            |                ^ expected named lifetime parameter

        // adding a lifetime only
        struct S<'a> {
            r: &'a i32,
        }
        let s;
        {
            let x = 10;
            s = S { r: &x };
        }
        assert_eq!(*s.r, 10); // error: variable s outlives the variable x

        outputs:
        error[E0597]: `x` does not live long enough
        --> src/main.rs:313:24
            |
        312 |             let x = 10;
            |                 - binding `x` declared here
        313 |             s = S { r: &x };
            |                        ^^ borrowed value does not live long enough
        314 |         }
            |         - `x` dropped here while still borrowed
        315 |         assert_eq!(*s.r, 10);
            |         -------------------- borrow later used here

        struct S<'a> {
            r: &'a i32,
        }

        struct D {
            s: S,
        }

        outputs:
        error[E0106]: missing lifetime specifier
        --> src/main.rs:334:16
            |
        334 |             s: S,
            |                ^ expected named lifetime parameter

         */

        struct S<'a> {
            r: &'a i32,
        }
        // This Works
        struct D<'a> {
            s: S<'a>,
        }
    }

    #[test]
    fn distinct_lifetime_parameters() {
        /*
        struct S<'a> {
            x: &'a i32,
            y: &'a i32,
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
        println!("{}", r);

        outputs:
        error[E0597]: `y` does not live long enough
        --> src/main.rs:367:39
            |
        365 |             let y = 20;
            |                 - binding `y` declared here
        366 |             {
        367 |                 let s = S { x: &x, y: &y };
            |                                       ^^ borrowed value does not live long enough
        ...
        370 |         }
            |         - `y` dropped here while still borrowed
        371 |         println!("{}", r);
            |                        - borrow later used here
        */
        //fix this
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
        println!("{}", r);
    }

    #[test]
    fn sharing_versus_mutation() {
        let v = vec![4, 8, 19, 27, 34, 10];

        /*
        let r = &v;
        let aside = v; // moving vector to aside
        r[0]; // bad: uses `v`, which is now uninitialized

        outputs:
        error[E0505]: cannot move out of `v` because it is borrowed
        --> src/main.rs:410:21
            |
        408 |         let v = vec![4, 8, 19, 27, 34, 10];
            |             - binding `v` declared here
        409 |         let r = &v;
            |                 -- borrow of `v` occurs here
        410 |         let aside = v; // moving vector to aside
            |                     ^ move out of `v` occurs here
        411 |         r[0]; // bad: uses `v`, which is now uninitialized
            |         - borrow later used here
        */
        {
            let r = &v;
            r[0]; // ok, vector is still there
        }
        let aside = v;

        fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
            for elt in slice {
                vec.push(*elt);
            }
        }

        let mut wave = Vec::new();
        let head = vec![0.0, 1.0];
        let tail = [0.0, -1.0];
        extend(&mut wave, &head);
        extend(&mut wave, &tail);
        assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);

        /*
        extend(&mut wave, &wave);
        assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0]);

        outputs:
        error[E0502]: cannot borrow `wave` as immutable because it is also borrowed as mutable
        --> src/main.rs:447:27
            |
        447 |         extend(&mut wave, &wave);
            |         ------ ---------  ^^^^^ immutable borrow occurs here
            |         |      |
            |         |      mutable borrow occurs here
            |         mutable borrow later used by call
         */

        /*

        let mut x = 10;
        let r1 = &x;
        let r2 = &x; // ok multiple shared borrows permitted
        x += 10; // error cannot assign to `x` because it is borrowed
        let m = &mut x; //error cannot borrow `x` as mutable because
                        // it is also borrowed as immutable
        println!("{}, {}, {}", r1, r2, m); // the references are used here
                                           // so their lifetimes must lost
                                           // at least this long

        let mut y = 10;
        let m1 = &mut y;
        let m2 = &mut y; // error: cannot borrow as mutable more than once
        let z = y; // error: cannot use `y` because it was mutable borrowed
        println!("{}, {}, {}", m1, m2, z); // references are used here
         */

        let mut w = (107, 109);
        let r = &w;
        let r0 = &r.0; // ok: reborrwing shared as shared

        // let m1 = &mut r.1; // error: can't reborrow shared as mutable
        println!("{}", r0);

        let mut v = (136, 139);
        let m = &mut v;
        let m0 = &mut m.0; // ok: reborrowing mutable from mutable
        *m0 = 137;
        let r1 = &m.1; // ok: reborrowing shared from mutable,
                       // and doesn't overlap with r0

        // v.1; //error: access through other oaths still forbidden
        println!("{}", r1); // r1 gets used here
    }

    #[test]
    fn rust_shared_references_vs_c_pointers_to_const() {
        /* C code
        int x = 42; // int variable, not const
        const int *p = &x; //pointer to onst int
        assert(*p == 42);
        x++; // change variable directly
        assert(*p == 43); // "constant" referent's value has changed
         */

        let mut x = 42; //non-const i32 variable
        let p = &x; // shared reference to i32
        assert_eq!(*p, 42);
        // x += 1; // error: cannot assign to x because it is borrowed
        assert_eq!(*p, 42); // if you take out the assignment, this is true
    }
}
