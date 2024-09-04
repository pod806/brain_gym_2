use std::io;
use std::rc::Rc;
use std::cell::RefCell;
#[derive(Debug)]
struct ListNode {
    value: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}
struct LinkedList {
    head: Option<Rc<RefCell<ListNode>>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, value: i32) {
        let new_node = Rc::new(RefCell::new(ListNode {
            value,
            next: self.head.take(),
        }));
        self.head = Some(new_node);
    }

    // Listenin uzunluğu
    fn length(&self) -> usize {
        let mut count = 0;
        let mut current = self.head.clone();
        while let Some(node) = current {
            count += 1;
            current = node.borrow().next.clone();
        }
        count
    }

    // n. elemanı listeden çıkarır
    fn remove_nth_from_end(&mut self, n: usize) {
        let length = self.length();

        if n > length {
            println!("Geçersiz konum!");
            return;
        }

        let mut dummy = Rc::new(RefCell::new(ListNode {
            value: 0,
            next: self.head.clone(),
        }));
        let mut current = dummy.clone();

        // Silinecek elemanın bir öncesine kadar ilerle
        for _ in 0..(length - n) {
            let next = current.borrow().next.clone().unwrap();
            current = next;
        }

        // n. elemanı atla ve listenin geri kalanı
        let next = current.borrow().next.clone().unwrap();
        current.borrow_mut().next = next.borrow().next.clone();

        // Yeni baş düğümü güncelle
        self.head = dummy.borrow().next.clone();
    }

    fn print_list(&self) {
        let mut current = self.head.clone();
        while let Some(node) = current {
            print!("{} => ", node.borrow().value);
            current = node.borrow().next.clone();
        }
        println!("None");
    }
}

fn main() {
    let mut input = String::new();
    println!("Bağlı liste elemanlarını girin (boşluk ile ayırın):");
    io::stdin().read_line(&mut input).expect("Başarısız okuma");
    let values: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Geçersiz sayı"))
        .collect();

    let mut list = LinkedList::new();
    for value in values.into_iter().rev() {
        list.push(value);
    }

    println!("Silmek istediğiniz elemanı soldan başlayarak konumunu girin:");
    let mut nth = String::new();
    io::stdin().read_line(&mut nth).expect("Başarısız okuma");
    let nth: usize = nth.trim().parse().expect("Geçersiz sayı");

    list.remove_nth_from_end(nth);
    list.print_list();
}