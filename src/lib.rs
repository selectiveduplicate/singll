use std::iter::FromIterator;
/// A generic singly linked list.
///
/// The `head` of the list is a `Link` type. A link is a `Box`ed `Node`.
///
/// Each `Node` consists of two items: the data, and another `Link`
pub struct SinglyLinkedList<T> {
    head: Link<T>,
}

/// `Link` is a generic `Node` type wrapped in a `Box`.
type Link<T> = Option<Box<Node<T>>>;

/// Represents the node of a singly linked list.
struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> Default for SinglyLinkedList<T> {
    fn default() -> Self {
        SinglyLinkedList { head: None }
    }
}

impl<T: Copy + Clone> SinglyLinkedList<T> {
    /// Creates a new `SinglyLinkedList`.
    pub fn new() -> Self {
        Self::default()
    }
    /// Pushes an item to a `SinglyLinkedList`.
    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.head.take(),
        });
        self.head = Some(new_node);
    }
    /// Pops (removes) an item from the list and returns it as an `Option` enum.
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    /// Gets the current head immutably.
    pub fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }

    /// Gets the current head mutably.
    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.data)
    }

    /// Length of the list.
    pub fn length(self) -> usize {
        self.into_iter().count()
    }

    /// Gets the last element immutably.
    pub fn back(&self) -> Option<&T> {
        self.iter().last()
    }

    /// Gets the last element mutably.
    pub fn back_mut(&mut self) -> Option<&mut T> {
        self.iter_mut().last()
    }
}

/// An owning iterator over the items of a linked list.
pub struct IntoIter<T>(SinglyLinkedList<T>);

impl<T> SinglyLinkedList<T> {
    /// Consumes the linked list into an iterator and yields
    /// the elements by value.
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T: Copy + Clone> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

/// An iterator yielding immutable references over the items of a linked list.
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> SinglyLinkedList<T> {
    /// Creates an iterator that yields the elements by reference.
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.data
        })
    }
}

/// An iterator yielding mutable references over the items of a linked list.
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> SinglyLinkedList<T> {
    /// Creates an iterator that yields mutable references to the elements.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.data
        })
    }
}

impl FromIterator<i32> for SinglyLinkedList<i32> {
    fn from_iter<T: IntoIterator<Item = i32>>(iter: T) -> Self {
        let mut list = SinglyLinkedList::<i32>::new();

        for i in iter {
            list.push(i);
        }

        list
    }
}

impl FromIterator<u32> for SinglyLinkedList<u32> {
    fn from_iter<T: IntoIterator<Item = u32>>(iter: T) -> Self {
        let mut list = SinglyLinkedList::<u32>::new();

        for i in iter {
            list.push(i);
        }

        list
    }
}

#[cfg(test)]
mod tests {
    use std::collections::LinkedList;

    use super::*;

    #[test]
    fn linked_list_of_strings() {
        let mut list = SinglyLinkedList::new();
        list.push("rust");
        list.push("bytes");
        list.push("compiler");
        assert_eq!(list.pop(), Some("compiler"));
    }

    #[test]
    fn push_an_element_to_a_singly_linked_list() {
        let mut list = SinglyLinkedList::new();
        list.push(-337);
        assert_eq!(list.pop(), Some(-337));
    }

    #[test]
    fn pop_from_empty_singly_linked_list() {
        let mut list = SinglyLinkedList::<u32>::new();
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn pop_from_a_list_of_one_item() {
        let mut list = SinglyLinkedList::new();
        list.push(100);
        assert_eq!(list.pop(), Some(100));
    }

    #[test]
    fn pop_from_a_list_of_three_items() {
        let mut list = SinglyLinkedList::new();

        list.push(100);
        list.push(230);
        list.push(-56);
        assert_eq!(list.pop(), Some(-56));
    }

    #[test]
    fn current_head_immutably() {
        let mut list = SinglyLinkedList::new();

        list.push(100);
        list.push(20);
        list.push(-56);

        assert_eq!(list.front(), Some(&-56));
    }
    #[test]
    fn current_head_mutably() {
        let mut list = SinglyLinkedList::new();

        list.push(100);
        list.push(20);
        list.push(-56);
        assert_eq!(list.front_mut(), Some(&mut -56));

        list.front_mut().map(|node_data| *node_data = 9);
        assert_eq!(list.front(), Some(&9));
    }

    #[test]
    fn into_iter_on_list() {
        let mut list = SinglyLinkedList::new();

        list.push(100);
        list.push(20);
        list.push(-56);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(-56));
        assert_eq!(iter.next(), Some(20));
        assert_eq!(iter.next(), Some(100));
    }

    #[test]
    fn iter_on_list() {
        let mut list = SinglyLinkedList::new();

        list.push(100);
        list.push(20);
        list.push(-56);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&-56));
        assert_eq!(iter.next(), Some(&20));
        assert_eq!(iter.next(), Some(&100));
    }

    #[test]
    fn mutable_iter_on_list() {
        let mut list = SinglyLinkedList::new();

        list.push(100);
        list.push(20);
        list.push(-56);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut -56));
        assert_eq!(iter.next(), Some(&mut 20));
        assert_eq!(iter.next(), Some(&mut 100));
    }

    #[test]
    fn length_of_list() {
        let mut list = SinglyLinkedList::new();

        list.push(100);
        list.push(20);
        list.push(-56);
        assert_eq!(list.length(), 3);
    }

    #[test]
    fn last_element_immutably() {
        let mut list = SinglyLinkedList::new();

        list.push(100);
        list.push(20);
        list.push(-56);
        assert_eq!(list.back(), Some(&100));

        let mut list = SinglyLinkedList::new();
        list.push(23);
        list.push(-49);
        assert_eq!(list.back(), Some(&23));
        assert_eq!(list.front(), Some(&-49));
    }

    #[test]
    fn last_element_mutably() {
        let mut list = SinglyLinkedList::new();

        list.push(100);
        list.push(20);
        list.push(-56);
        assert_eq!(list.back_mut(), Some(&mut 100));

        list.back_mut().map(|node| *node = 780);
        assert_eq!(list.back(), Some(&780));
    }

    #[test]
    fn collect_list_elements_into_vec() {
        let mut list = SinglyLinkedList::new();

        list.push(10);
        list.push(20);
        list.push(30);

        let elements: Vec<i32> = list.iter().map(|&member| member * 2).collect();

        assert_eq!(elements, vec![60, 40, 20]);
    }

    #[test]
    fn collect_list_elements_into_another_list() {
        let mut source = SinglyLinkedList::new();

        source.push(10);
        source.push(20);
        source.push(30);

        let dest: SinglyLinkedList<u32> = source.iter().map(|&member| member * 2).collect();

        let mut dest_iter = dest.iter();
        assert_eq!(dest_iter.next(), Some(&20));
        assert_eq!(dest_iter.next(), Some(&40));
        assert_eq!(dest_iter.next(), Some(&60));
    }

    #[test]
    fn collect_list_elements_into_builtin_linked_list() {
        let mut source = SinglyLinkedList::new();

        source.push(10);
        source.push(20);
        source.push(30);

        let dest: LinkedList<u32> = source.iter().map(|&member| member * 2).collect();

        let mut dest_iter = dest.iter();
        assert_eq!(dest_iter.next(), Some(&60));
        assert_eq!(dest_iter.next(), Some(&40));
        assert_eq!(dest_iter.next(), Some(&20));
    }

    #[test]
    fn make_list_from_an_iterator() {
        let iter = (10..15).into_iter();

        let list = SinglyLinkedList::from_iter(iter);

        let mut list_iter = list.iter();
        assert_eq!(list_iter.next(), Some(&14));
        assert_eq!(list_iter.next(), Some(&13));
        assert_eq!(list_iter.next(), Some(&12));
        assert_eq!(list_iter.next(), Some(&11));
        assert_eq!(list_iter.next(), Some(&10));
    }
}
