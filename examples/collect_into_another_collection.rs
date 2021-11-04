use singll::SinglyLinkedList;

fn main() {
    let input = vec![10, 20, 30, 40];
    let mut source = SinglyLinkedList::new();

    input.into_iter().for_each(|number| source.push(number));

    println!("Elements of list:");
    source.iter().for_each(|member| println!("{}", member));

    let dest: Vec<u32> = source.into_iter().map(|member| member * 2).collect();

    println!("Elements of list multiplied by two and collected into a vector:");
    dest.into_iter().for_each(|number| println!("{}", number));
}
