use singll::SinglyLinkedList;

fn main() {
    let input = vec![10, 20, 30, 40];
    let mut list = SinglyLinkedList::new();

    // populate the linked list
    input.into_iter().for_each(|number| list.push(number));

    println!("Elements before mutation:");

    list.iter().for_each(|member| println!("{}", member));

    // iterate mutably and multiply each element by 2
    list.iter_mut().for_each(|member| *member *= 2);

    println!("Elements after mutation (multiplied by two):");

    list.into_iter().for_each(|member| println!("{}", member));
}
