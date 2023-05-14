/*
In this lesson, we explore the basic ideas of references in Rust.

Key questions answered:
- How do we pass references to other functions to mutate an object?
- How does rust help enforce safety for references?
*/

fn main() {
    // By default, things created are considered non-mutable.
    // To declare something as mutable, use `let mut` instead of `let`.
    let mut p = Person{name: String::from("Simon"), age: 19};
    
    let original: String = name_of(&p);
    println!("Person named {}", original);

    // Passing `&` is not sufficient.
    // Passing `&mut` indicates a mutable reference.
    change(&mut p, String::from("Simon Says"), 23);

    // Passing `p` directly moves `p` into the function.
    let after = name_of_consumed(p);
    println!("Changed person named {}", after);
    // println!("Changed person named {}", p.name); // p is no longer valid.

    // What are rules around mutable references? Why does it matter?
    // Let's restore person `p`.
    let mut p = Person{name: String::from("Simon"), age: 19};
    println!("Person named {}", p.name);

    let write1 = &mut p; // first mutable reference
    // let write2 = &mut p; // second mutable reference -- not allowed
    // Only a single mutable reference for a given thing is allowed 
    // at a given time.

    // However, the compiler is also smart enough to understand:
    // 1. When a mutable reference is in-use, and thus it isn't safe to use have
    //    other references.
    // 2. When a mutable reference goes out of scope, and thus it is safe again.
    // See below.

    // let read1 = &p; // a reference -- still not allowed, because write1 is used below.

    change(write1, String::from("Simon Says"), 23);
    
    drop(write1); // This makes write1 go out of scope, i.e., dropped

    let read1: &Person = &p; // allowed. write1 is no longer used.
    println!("Changed person named {}", read1.name);

    // Final thoughts:
    // If you have worked with pointers, it is also important to note that
    // rust guarantees references to always be **valid**. Thus, preventing
    // you from creating dangling pointers as references.
    // 
    // If you are familiar with concurrency patterns, you'll also note that
    // from the example above, by default, rust prevents you from unsafe
    // concurrency patterns by default. For example, having multiple writers,
    // or single writer with multiple readers, acessing a shared resource
    // without synchronization involved is simply not allowed.
    //
    // When a resource is truly shared, you are required to use the proper
    // synchronization primitives to ensure defined and safe behavior.
    //
    // Read https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html
    // for a more in-depth coverage.
}

struct Person {
    age: u32,
    name: String,
}

fn change(p: &mut Person, name: String, age:u32) {
    // to mutate a person, we need `&mut` and not `&`.
    // this indicates to the compiler that the function modifies the
    // passed in person struct.
    p.name = name;
    p.age = age;
}

fn name_of(p: &Person) -> String {
    // Here, we take a reference to `p` with `&Person`.
    // But what happens to `p` after this function returns?
    // Since we only borrow `p`, `p` remains valid even after this function returns.
    // In Rust terminology, we say that `p` is not dropped (see trait [`Drop`]) after
    // returning.
    p.name.clone()
}

fn name_of_consumed(p: Person) -> String {
    // Here, we take ownership of `p` with `Person`.
    // `p` is dropped after this function returns, and no longer can be used.
    p.name
}
