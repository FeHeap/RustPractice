use std::collections::LinkedList;
fn main() {
    let mut list1 = LinkedList::new();
    list1.push_back('a');
    let mut list2 = LinkedList::new();
    list2.push_back('b');
    list2.push_back('c');
    list1.append(&mut list2);
    println!("{:?}", list1);    //['a', 'b', 'c']
    println!("{:?}", list2);    //[]
    list1.pop_front();
    println!("{:?}", list1);    //['b', 'c']
    list1.push_front('e');
    println!("{:?}", list1);    //['e', 'b', 'c']
    list2.push_front('f');
    println!("{:?}", list2);    //['f']
}