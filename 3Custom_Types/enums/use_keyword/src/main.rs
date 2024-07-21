// An attribute to hide warnings for unused code.
#![allow(dead_code)]

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

fn main() {
    // Explicity `use` each name so they are available without
    // manually scoping.
    use crate::Stage::{Beginner, Advanced};
    // Automatically `use` ecah name inside `Role`.
    use crate::Role::*;

    // Equivalent to `Stage::Beginner`
    let stage = Beginner;
    // Equivalent to `Role::Student`
    let role = Student;

    match stage {
        // Note the lack of scoping because of the explicit `use` above.
        Beginner => println!("Beginners are strarting their learning journey!"),
        Advanced => println!("Advanced learners are mastering their subjects"),
    }

    match role {
        // Note again the lack of scoping.
        Student => println!("Students are acquiring knowledge!"),
        Teacher => println!("Teachers are spreading knowledge!"),
    }
}
