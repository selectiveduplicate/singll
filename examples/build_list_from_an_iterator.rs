use singll::SinglyLinkedList;

fn main() {
    // Let's create an iterator over even numbers from 0 to 100
    let iter = (0..101).filter(|n| n % 2 == 0);
    // Now build a `SinglyLinkedList` with the even numbers
    let list_of_odds = SinglyLinkedList::from_iter(iter);

    list_of_odds.iter().for_each(|n| println!("{}", n));
}
