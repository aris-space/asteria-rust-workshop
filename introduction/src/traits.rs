// =============================================
// Section 4: Traits & generics
// =============================================

/// We have already seen a trait, namely [`Clone`]
#[test]
fn cloning() {
    let x = [1, 2, 3];
    let _y = Clone::clone(&x);
}

// A trait is something that a type can do/have.
// It can be something simple as: "this type can be cloned"
// to complex logic such as:
// "this type can be used as the logic for receiving a HTTP request, whose body is of type `ABC` and which returns HTML"

/// The most interesting for beginners are:
/// - [`Clone`] & [`Copy`], to crate copies
/// - [`std::fmt::Debug`] & [`std::fmt::Display`], to print
/// - [`PartialOrd`], [`Ord`], [`PartialEq`] and [`Eq`] to compare
/// - [`Into`] and [`From`] to convert between types
/// - [`std::ops::Add`] and similar to do maths
#[test]
fn common_traits() {}

// Traits must be implemented.

struct MyStruct {
    a: String,
    b: (f32, f32),
}

impl Clone for MyStruct {
    fn clone(&self) -> Self {
        Self {
            a: self.a.clone(),
            b: self.b,
        }
    }
}

// But some can also automatically be implemented using `derive`

#[derive(Debug, Clone, PartialEq)]
enum MyEnum {
    VariantA(String),
    VariantB(f32, f32),
}

#[test]
fn my_enum_is_clone() {
    let x = MyEnum::VariantB(1., 1.);
    let _y = x.clone();
}

// You can also write your own traits

trait Talk {
    fn say_hello(&self);
    fn say_hello_to(&self, other: &str);
}

struct Dog {
    name: String,
}

impl Talk for Dog {
    fn say_hello(&self) {
        println!("Woof!");
    }

    fn say_hello_to(&self, other: &str) {
        println!("Woof, hello {other}!");
    }
}

#[test]
fn test_animal_talk() {
    let dog = Dog {
        name: String::from("Rex"),
    };

    dog.say_hello(); // Should print "Woof!"
    dog.say_hello_to("Bessie"); // Should print "Woof, hello Bessie!"
}

// Traits are particularly powerful when used as bounds in generics:
fn print_if_different<T: PartialEq<T> + std::fmt::Debug>(a: &T, b: &T) {
    if a != b {
        println!("{a:?} != {b:?}");
    }
}

#[allow(unused_variables)]
#[test]
fn test_print_if_different() {
    let not_printable_or_comparable = MyStruct {
        a: "hi".into(),
        b: (1., 1.),
    };
    //X print_if_different(&not_printable_or_comparable, &not_printable_or_comparable); // Won't work

    let printable_and_comparable1 = MyEnum::VariantB(0., 1.0);
    let printable_and_comparable2 = MyEnum::VariantB(0., 2.0);
    print_if_different(&printable_and_comparable1, &printable_and_comparable1);
    print_if_different(&printable_and_comparable1, &printable_and_comparable2);
}

// Generics can also be used for types, but commonly, we don't place bounds on them until in the impl block.

struct MyBox<T>(T);

/// Note that you need to specify `<T>` twice: once to say we are implementing for every T,
/// and once to say that we implement it for this [`MyBox`].
impl<T> MyBox<T>
where
    T: Clone, // This is equilvant to saying `impl<T: Clone>`, but sometimes more readable
{
    fn duplicate(&self) -> (Self, Self) {
        let MyBox(x) = self;
        (MyBox(x.clone()), MyBox(x.clone()))
    }
}

// With this, we have seen the bread and butter of traits in Rust.
// There are many advanced uses, and virtually every crate defines its own traits,
// but we can't covery everything.
// You at least now know they exists and how they look.
