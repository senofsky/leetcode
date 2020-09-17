// Design your implementation of the linked list. You can choose to use the
// singly linked list or the doubly linked list. A node in a singly linked list
// should have two attributes: val and next. val is the value of the current
// node, and next is a pointer/reference to the next node. If you want to use
// the doubly linked list, you will need one more attribute prev to indicate the
// previous node in the linked list. Assume all nodes in the linked list are
// 0-indexed.
//
// Implement these functions in your linked list class:
//
//     get(index) : Get the value of the index-th node in the linked list. If
//     the index is invalid, return -1.
//
//     addAtHead(val) : Add a node of value val before the first element of the
//     linked list. After the insertion, the new node will be the first node of
//     the linked list.
//
//     addAtTail(val) : Append a node of value val to the last element of the
//     linked list.
//
//     addAtIndex(index, val) : Add a node of value val before the index-th node
//     in the linked list. If index equals to the length of linked list, the
//     node will be appended to the end of linked list. If index is greater than
//     the length, the node will not be inserted.
//
//     deleteAtIndex(index) : Delete the index-th node in the linked list, if
//     the index is valid.

// I have to match the parameters given by LeetCode, but I personally would use
// unsigned ints for indexes and lengths. In Rust, this is the type usize.

// TODO: Look at how the std library implements a linked list

#[derive(Debug)]
struct MyLinkedList {
    head: Option<Box<Node>>,
    length: i32,
}

#[derive(Debug)]
struct Node {
    val: i32,
    next: Option<Box<Node>>,
}

impl MyLinkedList {
    fn new() -> Self {
        MyLinkedList {
            head: None,
            length: 0,
        }
    }

    fn get(&self, index: i32) -> i32 {
        if self.head.is_none() {
            return -1;
        }
        let mut node = self.head.as_ref().unwrap();
        for _ in 0..index {
            if node.next.is_none() {
                return -1;
            } else {
                node = node.next.as_ref().unwrap();
            }
        }

        node.val
    }

    fn add_at_head(&mut self, val: i32) {
        self.add_at_index(0, val);
    }

    fn add_at_tail(&mut self, val: i32) {
        self.add_at_index(self.length, val);
    }

    fn add_at_index(&mut self, index: i32, val: i32) {
        if index == 0 {
            let new_node = Node {
                val,
                next: self.head.take(),
            };
            self.head = Some(Box::new(new_node));
        } else if index <= self.length {
            let mut node = self.head.as_mut().unwrap();
            for _ in 1..index {
                node = node.next.as_mut().unwrap();
            }
            let new_node = Node {
                val,
                next: node.next.take(),
            };
            node.next = Some(Box::new(new_node));
        } else {
            return;
        }

        self.length += 1;
    }

    fn delete_at_index(&mut self, index: i32) {
        if index >= self.length {
            return;
        } else if index == 0 {
            self.head = self.head.as_mut().unwrap().next.take();
        } else {
            let mut node = self.head.as_mut().unwrap();
            for _ in 1..index {
                node = node.next.as_mut().unwrap();
            }
            node.next = node.next.as_mut().unwrap().next.take();
        }

        self.length -= 1;
    }
}

fn main() {
    println!("Creating Linked List");
    let mut linked_list = MyLinkedList::new();
    println!("{:#?}\n", linked_list);

    println!("Add '1' at head");
    linked_list.add_at_head(1);
    println!("{:#?}\n", linked_list);

    println!("Add '3' at tail");
    linked_list.add_at_tail(3);
    println!("{:#?}\n", linked_list);

    println!("Add '1' at index '2'");
    linked_list.add_at_index(1, 2);
    println!("{:#?}\n", linked_list);

    println!("Get value at index '1'");
    let result = linked_list.get(1);
    println!("result = {}\n", result);

    println!("Delete index '1'");
    linked_list.delete_at_index(1);
    println!("{:#?}\n", linked_list);

    println!("Get value at index '1'");
    let result = linked_list.get(1);
    println!("result = {}", result);
}
