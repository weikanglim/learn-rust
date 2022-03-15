fn main() {
    let p1 = Person { age: 14, name: "Tosh".to_owned() };
    let p2 = Person { age: 16, name: "Dendra".to_owned() };

    // Print person by passing ownership
    print_person_by_ownership(p1);
    // p1 no longer available after move
    // print_person_by_ownership(p1) will be flagged by compiler

    // Print person by borrow (using references)
    print_person(&p2);
    print_person(&p2); // still works

    // Print person by creating a separate deep-copy of person
    print_person_by_ownership(p2.clone());

    // Illustrates copy semantics using Clone (.clone) vs Copy trait (implciit)
    let my_simple = Simple { integer: 10 };

    print_simple(my_simple.clone());
    print_simple(my_simple);
}

fn print_simple(simple: Simple) {
    println!("Simple: integer: {}", simple.integer);
}

fn print_person(person: &Person) {
    println!("Person with name: {}, age: {}", person.name, person.age);
}

fn print_person_by_ownership(person: Person) {
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