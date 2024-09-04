use std::rc::Rc;

// Stack memory'yi göstermek için fonksiyon
fn stack_memory_example() {
    let x = 42;
    let y = 3.14; //

    println!("Stack değişkeni x: {}", x);
    println!("Stack değişkeni y: {}", y);
}

// Heap memory'yi göstermek için fonksiyon
fn heap_memory_example() {
    let heap_value = Box::new(123);  // heap_value, heap üzerinde bir tam sayıya işaret eder

    println!("Heap değeri: {}", heap_value);

    // Heap üzerinde ayrılmış veriyi paylaşımlı sahiplikle yönetmek için bir rc oluştur
    let rc_value = Rc::new(456);
    let rc_value_clone = Rc::clone(&rc_value);

    println!("Rc değeri: {}", rc_value);
    println!("Rc klonlanmış değeri: {}", rc_value_clone);
}

fn main() {
    // Stack memory
    println!("Stack memory'yi gösterme:");
    stack_memory_example();

    // Heap memory
    println!("\nHeap memory'yi gösterme:");
    heap_memory_example();
}
