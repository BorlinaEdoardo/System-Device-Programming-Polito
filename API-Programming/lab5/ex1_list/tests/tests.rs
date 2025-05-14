pub use LinkedList::LinkedList::List1::{List as LinkedList1};
use LinkedList::LinkedList::List1::Node;

#[test]
fn push_pop_list(){
    let mut list = LinkedList1::<i32>::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let val = list.pop().unwrap();
    assert_eq!(val, 3);
    assert_eq!(list.pop(), Some(2));
    assert_eq!(list.pop(), Some(1));
}

#[test]
fn take_iter_list(){
    let mut list = LinkedList1::<i32>::new();
    list.push(4);
    list.push(3);
    list.push(2);
    list.push(1);

    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);



    let mut list_taken = list.take(4);
    assert_eq!(1, list.pop().unwrap());
    assert_eq!(2, list.pop().unwrap());
    assert_eq!(3, list.pop().unwrap());
    assert_eq!(4, list.pop().unwrap());

    assert_eq!(4, list_taken.pop().unwrap());
    assert_eq!(3, list_taken.pop().unwrap());
    assert_eq!(2, list_taken.pop().unwrap());
    assert_eq!(1, list_taken.pop().unwrap());
    return;
    /*
        let mut i = 1;
        for el in list_taken.iter(){
            let Node::Cons(val, _) = el else { println!("Panico paura"); break };
            println!("element: {}", val);
            //assert_eq!(val, i);
            i += 1;
        }

     */

}