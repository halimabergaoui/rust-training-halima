fn main() {
    
    println!("Hello, world!");
}
#[doc = " This is a doc comment."]
pub mod m1 {
    // Missing documentation is ignored here
    #[allow(missing_docs)]
    pub fn undocumented_one() -> i32 { 1 }

    // Missing documentation signals a warning here
    #[warn(missing_docs)]
    #[doc = " This is a doc comment."]
    pub fn undocumented_too() -> i32 { 2 }

    // Missing documentation signals an error here
    #[deny(missing_docs)]
    #[doc = " This is a doc comment."]
    pub fn undocumented_end() -> i32 { 3 }
}

#[warn(missing_docs)]
#[doc = " This is a doc comment."]
pub mod m2{
    #[allow(missing_docs)]
    pub mod nested {
        // Missing documentation is ignored here
        pub fn undocumented_one() -> i32 { 1 }

        // Missing documentation signals a warning here,
        // despite the allow above.
        #[warn(missing_docs)]
        #[doc = " This is a doc comment."]
        pub fn undocumented_two() -> i32 { 2 }
    }

    // Missing documentation signals a warning here
    #[doc = " This is a doc comment."]
    pub fn undocumented_too() -> i32 { 3 }
}

#[forbid(missing_docs)]
#[doc = " This is a doc comment."]
pub mod m3 {
    // Attempting to toggle warning signals an error here
    //#[allow(missing_docs)]
    #[doc = " This is a doc comment."]
    pub fn undocumented_too() -> i32 { 2 }
}

// This allows all lints in the "unused" group.
#[allow(unused)]
// This overrides the "unused_must_use" lint from the "unused"
// group to deny.
#[deny(unused_must_use)]
fn example() {
    // This does not generate a warning because the "unused_variables"
    // lint is in the "unused" group.
    let x = 1;
    print();
    // This generates an error because the result is unused and
    // "unused_must_use" is marked as "deny".
    //let res=std::fs::remove_file("some_file"); // ERROR: unused `Result` that must be used
}
//#[must_use]
fn print()  {
    println!("must use")
}

// The order of these two attributes does not matter.
//#[deny(warnings)]
// The unsafe_code lint is normally "allow" by default.
//#[warn(unsafe_code)]
/*fn example_err() {
    // This is an error because the `unsafe_code` warning has
    // been lifted to "deny".
    unsafe { print() } // ERROR: usage of `unsafe` block
}
*/