// =============================================
// Section 3: Ownership
// =============================================

/*
The rust book goes quite in depth on ownership,
see <https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html>.
But the gist is:
** Each value in Rust has an owner.
** There can only be one owner at a time.
** When the owner goes out of scope, the value will be dropped.

If you need to use the variable outside of the owner,
you have two options:
1. Move it, so give away your ownership (of course only if you have it).
2. Share it, so as long as you live, they can also use it.

This together ensures that variables aren't used after the owner dies,
which can lead to all sorts of funky bugs (this is called memory safety).
*/

#![allow(clippy::no_effect_underscore_binding)]

/// Some opaque thing you can own
struct MyThing;

#[allow(clippy::needless_pass_by_value)]
fn consuming_function(_x: MyThing) {}
fn borrowing_function(_x: &MyThing) {}

#[test]
fn use_consuming_borrowing_function() {
    let x = MyThing; // x is now the owner.
    let y = x; // we can use it by moving e.g. to a new variable.
    //X let z = x; // This won't work, y is now the owner.
    consuming_function(y);
    //X let z = y; // This won't work, we have given it away to the function.

    // However, by only sharing it, we can still use it afterwards
    {
        let x = MyThing; // x is now the owner.
        borrowing_function(&x); // we only borrow it to the function
        let _y = x; // Thus x still owns it
    }
}

// Additionally, there is the distinction between mutable sharing, and immutable sharing.
// For mutable sharing, it is important that we only ever share it to one other at a time,
// and no other immutably at the same time.

/// This function needs to edit [`MyThing`] and thus needs a `&mut`.
fn mutably_borrowing_function(_x: &mut MyThing) {}

#[allow(unused_variables)]
#[test]
fn borrowing_rule() {
    let mut x = MyThing;

    // Sharing it immutably many times is fine, everybody can read at the same time.
    {
        let x1 = &x;
        let x2 = &x;
        let x3 = &x;
        borrowing_function(x1);
    }

    // Sharing it mutably more than once isn't,
    // as there could be problems if two would write at the same time.
    {
        let x1 = &mut x;
        //X let x2 = &mut x;
        mutably_borrowing_function(x1);
    }

    // Sharing it mutably while it is shared immutable also doesn't work,
    // as there could be problems if one writes and the other reads a the same time.
    {
        let x1 = &x;
        //X let x2 = &mut x;
        borrowing_function(x1);
    }
}

// Transfering ownership or sharing is often enough,
// but sometimes you need more, for which you have several options.

/// For some types, you can [`Clone`] the value.
#[test]
fn cloning() {
    let mut x = vec![1, 2, 3];
    let y = x.clone();
    // we now created a copy, such that x & y are both owners of their own.

    x[1] = 42;
    println!("{x:?}");
    println!("{y:?}");
}

/// Some types automatically clone when used, namely onces which are [`Copy`], such as ints.
#[test]
fn copying() {
    let x = 23;
    let y = x;
    // we now created a copy, such that x & y are both owners of their own.

    println!("{x:?}");
    println!("{y:?}");
}

#[allow(unused_imports)]
use std::rc::Rc;
#[allow(unused_imports)]
use std::sync::Arc;

/// If you need multiple owners of the same immutable variable,
/// you can use [`Rc`], or more commonly in multithreaded code: [`Arc`].
///
/// This will not duplicate it in memory.
#[test]
fn reference_counted() {
    let x = [1, 2, 3, 4, 5, 6, 7, 8];
    let arc1 = Arc::new(x);
    let arc2 = arc1.clone();

    println!("{arc1:?}");
    println!("{arc2:?}");
}

#[allow(unused_imports)]
use std::sync::Mutex;

/// And if you need to mutate while having only read only access, you can use a lock [`Mutex`].
/// Note that you can combine this with [`Arc`] to get a very powerful shared variable.
#[test]
fn mutexing() {
    let x = Arc::new(Mutex::new(vec![1, 2, 3]));
    let y = x.clone();

    x.lock().expect("no one died while holding the lock")[1] = 42;
    println!("{x:?}");
    println!("{y:?}");
}

// With this, we've gone through the most important rules for ownership,
// which is enforced by the "Borrow Checker",
// and is a unique concept to Rust.

/// We now move on the another potentially new, but not unique concepts: [`super::traits`].
const _: () = ();
