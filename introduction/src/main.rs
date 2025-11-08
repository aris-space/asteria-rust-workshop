//! This crate runs you through some of the concepts of rust.
//! Read through it top to bottom, as it tries to guide you through various sections.

#![allow(dead_code)]

#[allow(clippy::no_effect_underscore_binding)]
#[allow(clippy::items_after_statements)]
fn main() {
    // =============================================
    // Section 0: Repetition of basics
    // =============================================

    // Create variables
    let my_str = "hii";
    let my_int = 12;
    let my_float = 12.;
    let my_tuple = (my_int, my_float);

    // Do maths, but only on same types
    // let _bad = my_int + my_float; //X Uncomment this line
    let good = f64::from(my_int) + my_float;
    let _nests = good + (good * good);

    // Use blocks
    let block_ret = {
        let my_str = my_tuple;
        my_str.0
    };
    println!("{my_str}"); // still "hii"

    // define and use functions
    fn double(x: i32) -> i32 {
        x + x // equivalent to `return x + x;`
    }
    double(block_ret);

    // define and use composite types
    struct Point {
        x: i32,
        y: i32,
    }
    let zero = Point { x: 0, y: 0 };

    // define methods (functions on objects)
    impl Point {
        const ZERO: Point = Point { x: 0, y: 0 };

        #[allow(clippy::needless_pass_by_value)]
        fn equal(&self, other: Point) -> bool {
            self.x == other.x && self.y == other.y
        }
    }
    assert!(zero.equal(Point::ZERO));

    // =============================================
    // Section 1: Interlude - Get to know the tooling
    // =============================================

    // The compiler & rust-analizer help you:
    // it shows you where erros occur
    // let _x = 2 + "hi"; //X Uncomment this line

    // it shows you inferred types
    let _x: i32 = 2;
    let _x = 2;

    // it can be used to automatically import / refactor
    {
        fn foo(_: &str) {}
        foo("Hello World"); // you can extract this into variables, ... //X Press Cmd+. (Quick Fix)

        // let arc = Arc::new(()); //X Remove this line and press (Quick Fix)
    }

    // to rename variables //X Press F2 (Rename Symbol)
    let baaaad_name = [1, 2, 3];
    let _y = baaaad_name;

    // it shows you possiblities: //X Delete the `.len()` and type .
    let _x = "lkj".len();

    // Rust format helps you stay consistent
    #[rustfmt::skip] // can be removed //X Remove this line and save
    let   _x =   2+ 2 ;
    let _x = 1; // this looks way nicer

    // Clippy is even smarter about helping you:
    let my_string = "hiii";
    #[allow(clippy::comparison_to_empty)] //X Remove this line
    // These attributes can override the warnings
    if my_string == "" {
        println!(":(");
    }

    // rust also has smart documentation comments, using with three slashes

    /// This struct is all my own.
    ///
    /// You can use it like this:
    /// ```
    /// let x = MyStruct {
    ///     foo: 12
    /// };
    /// ```
    ///
    /// and you can also link to other stuff, such as [`str`].
    struct MyStruct {
        /// Fields can have docs too. This is an [`i32`].
        foo: i32,
    }
    // functions can and should also have docs.

    // Cargo has many features, including running your code,
    // but also running automated tests.
    // For this, we have to jump out of the main function
}

// A test can be run in VSCode using a button click.
#[test]
fn check_addition_is_as_expected() {
    assert_eq!(1 + 2, 3);
}

// In conclusion: use the tooling, it the best thing about Rust!

// =============================================
// Section 2: Advanced types
// =============================================

// you might know enums from C or another language,
// where they are just a shortcut for a number.
// you can do that as well in Rust.

enum JustAShortCut {
    ValueA = 1,
    ValueB = 2,
}

#[test]
fn numeric_enum() {
    assert_eq!(JustAShortCut::ValueA as usize, 1);
}

// However, in Rust they are so much more.
// They are in essence a type with is exactly one of some other types, called variants.

enum ExactlyOneOfBelow {
    /// It can either be a u32
    VariantA(u32),
    /// Or a String
    VariantB(String),
    /// Or even another string. They are different variants.
    VariantC(String),
    /// You can also inline a struct or tuple
    VariantD { my_field: u32, another_field: i16 },
}

// Most importantly though, they can be used for some very common constructs,
// such as an optional type, or a something that can fail.

mod this_module_is_already_in_the_standard_library {
    /// Introducing, `Option<T>`
    enum Option<T> {
        None,
        Some(T),
    }

    /// And `Result<T, E>`,
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
}

/// So instead of throwing an error, you return a Result.
fn fallible_function_div_by(x: u32, y: u32) -> Result<u32, ()> {
    if y == 0 { Err(()) } else { Ok(x / y) }
}

/// And you can work with enums, such as results with pattern matching
#[test]
fn use_the_fallible_function() {
    match fallible_function_div_by(10, 2) {
        Ok(value) => println!("Result is {value}"),
        Err(()) => println!("Division by zero!"),
    }
}

/// You can also use if let for simple cases
#[test]
fn use_if_let() {
    if let Ok(value) = fallible_function_div_by(10, 2) {
        println!("Result is {value}");
    }
}

/// And let-else to deal with the happy case
#[test]
fn use_let_else() {
    let Ok(value) = fallible_function_div_by(10, 2) else {
        println!("Division by zero!");
        return;
    };
    println!("Result is {value}");
}

/// Of course, you can nest the types, to get crazy functions
#[allow(clippy::option_option)]
fn option_of_option_flatten<T>(x: Option<Option<T>>) -> Option<T> {
    match x {
        Some(Some(x)) => Some(x),
        _ => None,
    }
}

#[test]
fn test_option_of_option_flatten() {
    assert_eq!(option_of_option_flatten::<()>(None), None);
    assert_eq!(option_of_option_flatten::<()>(Some(None)), None);
    assert_eq!(option_of_option_flatten::<()>(Some(Some(()))), Some(()));
}

/// Given `ExactlyOneOfBelow`, return the inner u32 if it is a `VariantA` or `VariantD`,
/// Otherwise return None.
#[allow(clippy::needless_pass_by_value)]
fn extract_number(_x: ExactlyOneOfBelow) -> Option<u32> {
    //X Exercise: implement this function
    todo!()
}

#[test]
fn test_extract_number() {
    assert_eq!(extract_number(ExactlyOneOfBelow::VariantA(5)), Some(5));
    assert_eq!(
        extract_number(ExactlyOneOfBelow::VariantB("test".to_string())),
        None
    );
    assert_eq!(
        extract_number(ExactlyOneOfBelow::VariantD {
            my_field: 10,
            another_field: 0
        }),
        Some(10)
    );
}

/// Given two Results, return Ok if both are Ok and add their values,
/// otherwise return the first Err encountered
fn combine_results(x: Result<u32, ()>, y: Result<u32, ()>) -> Result<u32, ()> {
    //X Exercise: implement this function
    todo!("{x:?} {y:?}")
}

#[test]
fn test_combine_results() {
    assert_eq!(combine_results(Ok(2), Ok(3)), Ok(5));
    assert_eq!(combine_results(Err(()), Ok(3)), Err(()));
    assert_eq!(combine_results(Ok(2), Err(())), Err(()));
}

/// Given two optional string slices that should represent
/// unsigned integers, return their sum as Some(u32).
/// Return None if either input is missing or invalid.
///
/// Note that you can parse with
/// ```
/// let _: Result<u32, _> = "1".parse();
/// ```
fn sum_string_options(a: Option<&str>, b: Option<&str>) -> Option<u32> {
    // Exercise: implement this function
    todo!("{a:?}, {b:?}")
}

#[test]
fn test_sum_string_options() {
    assert_eq!(sum_string_options(Some("3"), Some("4")), Some(7));
    assert_eq!(sum_string_options(Some("0"), None), None);
    assert_eq!(sum_string_options(Some("x"), Some("4")), None);
    assert_eq!(sum_string_options(None, Some("4")), None);
    assert_eq!(sum_string_options(Some("1"), Some("2")), Some(3));
}

// =============================================
// Section 3: Ownership
// =============================================

// Ok, we have now used soo many lines, it is time to show how to make modules:
// Ctrl/Cmd click on it.
mod ownership;

// =============================================
// Section 4: Traits & generics
// =============================================
mod traits;

// =============================================
// Section 5: async
// =============================================
mod async_code;

// =============================================
// Section 6: Modules
// =============================================
#[test]
fn use_pub_stuff() {
    let x = async_code::MyPublicStruct::new(8);
    println!("{}", x.public_field);
    // println!("{}", x.private_field); //X Uncomment this line
}
