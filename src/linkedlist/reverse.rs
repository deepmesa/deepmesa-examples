use core::fmt::Debug;
use deepmesa::lists::linkedlist::Node;
use deepmesa::lists::FastLinkedList;
use std::collections::LinkedList as StdLinkedList;

//In place reverse of deepmesa::lists::FastLinkedList
pub fn reverse_fll<T: Debug>(list: &mut FastLinkedList<T>) {
    if list.is_empty() || list.len() == 1 {
        return;
    }

    let mut front_p: Node<T> = list.head_node().unwrap();
    let mut back_p: Node<T> = list.tail_node().unwrap();

    for _ in 0..list.len() / 2 {
        let front_next = front_p.next_node(&list).unwrap();
        let back_prev = back_p.prev_node(&list).unwrap();
        front_p.swap_node(&back_p, list);
        front_p = front_next;
        back_p = back_prev;
    }
}

pub fn reverse_std<T>(list: &mut StdLinkedList<T>) {
    if list.is_empty() || list.len() == 1 {
        return;
    }

    let mut tmp_list = StdLinkedList::<T>::new();
    let mut cur: Option<T> = list.pop_front();
    while cur.is_some() {
        tmp_list.push_front(cur.unwrap());
        cur = list.pop_front();
    }

    cur = tmp_list.pop_front();
    while cur.is_some() {
        list.push_back(cur.unwrap());
        cur = tmp_list.pop_front();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse_fll() {
        let mut list = FastLinkedList::<u8>::with_capacity(5);
        for i in 0..6 {
            list.push_front(i);
        }

        reverse_fll(&mut list);

        let mut val = 0;
        for v in list.iter() {
            assert_eq!(v, &val);
            val += 1;
        }
    }

    #[test]
    fn test_reverse_std() {
        let mut list = StdLinkedList::<u8>::new();
        for i in 0..5 {
            list.push_front(i);
        }

        reverse_std(&mut list);
        let mut val = 0;
        for v in list.iter() {
            assert_eq!(v, &val);
            val += 1;
        }
    }
}
