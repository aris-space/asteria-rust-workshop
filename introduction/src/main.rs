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
    //X let _bad = my_int + my_float;
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

        fn equal(&self, other: &Point) -> bool {
            self.x == other.x && self.y == other.y
        }
    }
    assert!(zero.equal(&Point::ZERO));

    // =============================================
    // Section 1: Interlude - Get to know the tooling
    // =============================================

    // The compiler & rust-analizer help you:
    // it shows you where erros occur
    //X let _x = 2 + "hi";

    // it shows you inferred types
    let _x: i32 = 2;
    let _x = 2;

    // it can be used to automatically import / refactor
    {
        fn foo(_: &str) {}
        foo("Hello World"); //X you can extract this into variables, ...

        //X let arc = Arc::new(());
    }

    //X it shows you possiblities:
    let _x = "lkj".len();

    // Rust format helps you stay consistent
    #[rustfmt::skip] //X can be removed
    let   _x =   2+ 2 ;
    let _x = 1; // this looks way nicer

    // Clippy is even smarter about helping you:
    let my_string = "hiii";
    #[allow(clippy::comparison_to_empty)] //X
    if my_string == "" {
        println!(":(");
    }

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
