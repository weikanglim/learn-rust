/*!
In this lesson, we explore the basic ideas of ownership in Rust.
*/

fn main() {
    let p1 = Person { age: 14, name: String::from("Tosh") };
    let p2 = Person { age: 16, name: String::from("Dendra") };

    // Print person by passing ownership (i.e. move semantics)
    print_person_owned(p1);
    // p1 no longer available after move
    // print_person_owned(p1) will be flagged by compiler

    // Print person by borrow (using references)
    print_person(&p2);
    print_person(&p2); // still works

    // Print person by creating a separate deep-copy of person
    print_person_owned(p2.clone());

    // Simple implements the Copy trait. This changes the default semantics
    // to be copy instead of move.
    let my_simple = Simple { integer: 10 };

    print_simple(my_simple);
    print_simple(my_simple); // still valid
    print_simple(my_simple.clone()); // equivalent to the previous two statements
}

fn print_simple(simple: Simple) {
    println!("Simple: integer: {}", simple.integer);
}

fn print_person(person: &Person) {
    println!("Person with name: {}, age: {}", person.name, person.age);
}

fn print_person_owned(person: Person) {
    println!("Owned person with name: {}, age: {}", person.name, person.age);
}

#[derive(Debug, Copy, Clone)]
struct Simple {
    integer: u32
}

struct Person {
    age: u32,
    name: String,
}

impl Clone for Person {
    fn clone(&self) -> Self {
        Person { age: self.age, name: self.name.clone() }
    }
}