pub struct ArrayLinkedList<V> {
    slot: Vec<Option<Node<V>>>,
    head: Option<usize>,
    tail: Option<usize>,
    free: Vec<usize>,
}

#[derive(Default)]
struct Node<V> {
    val: V,
    prev: Option<usize>,
    next: Option<usize>,
}

impl<V> ArrayLinkedList<V> {
    pub fn with_capacity(capacity: usize) -> Self {
        let mut slot = Vec::with_capacity(capacity);
        let mut free = Vec::with_capacity(capacity);
        for i in (0..capacity).rev() {
            slot.push(None);
            free.push(i);
        }
        Self {
            slot,
            head: None,
            tail: None,
            free,
        }
    }

    // returns error if the linked list is full.
    // returns index on success.
    pub fn push_front(&mut self, val: V) -> Result<usize, ()> {
        let Some(free_slot) = self.free.pop() else {
            return Err(());
        };
        let new_node = Node {
            val,
            prev: None,
            next: self.head,
        };
        self.slot[free_slot] = Some(new_node);
        // update previous head
        if let Some(prev_head) = self.get_ref_mut(self.head) {
            prev_head.prev = Some(free_slot);
        }
        // set as tail if this is the first node
        if self.tail.is_none() {
            self.tail = Some(free_slot);
        }
        // set as head
        self.head = Some(free_slot);
        Ok(free_slot)
    }

    pub fn pop_back(&mut self) -> Option<V> {
        let Some(tail) = self.tail else {
            return None;
        };
        self.take_node(tail).map(|node| node.val)
    }

    pub fn remove(&mut self, index: usize) -> Option<V> {
        if index >= self.capacity() {
            return None;
        }
        self.take_node(index).map(|node| node.val)
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.slot.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.capacity() - self.free.len()
    }

    // panic if index out of range
    fn take_node(&mut self, index: usize) -> Option<Node<V>> {
        let Some(node) = self.slot[index].take() else {
            return None;
        };
        self.free.push(index);
        if let Some(next_node) = self.get_ref_mut(node.next) {
            next_node.prev = node.prev;
        }
        if let Some(prev_node) = self.get_ref_mut(node.prev) {
            prev_node.next = node.next;
        }
        if Some(index) == self.head {
            self.head = node.next;
        }
        if Some(index) == self.tail {
            self.tail = node.prev;
        }
        Some(node)
    }

    // panic if index out of range
    fn get_ref_mut(&mut self, idx: Option<usize>) -> Option<&mut Node<V>> {
        idx.map(|idx| self.slot[idx].as_mut()).flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creation() {
        let list: ArrayLinkedList<i32> = ArrayLinkedList::with_capacity(5);
        assert_eq!(list.capacity(), 5);
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_push_front() {
        let mut list = ArrayLinkedList::with_capacity(3);

        assert_eq!(list.push_front(1), Ok(0)); // Index 0
        assert_eq!(list.len(), 1);
        assert!(!list.is_empty());

        assert_eq!(list.push_front(2), Ok(1)); // Index 1
        assert_eq!(list.len(), 2);

        assert_eq!(list.push_front(3), Ok(2)); // Index 2
        assert_eq!(list.len(), 3);

        // List should be full now
        assert_eq!(list.push_front(4), Err(()));
    }

    #[test]
    fn test_pop_back() {
        let mut list = ArrayLinkedList::with_capacity(3);

        // Empty list
        assert_eq!(list.pop_back(), None);

        // Single element
        list.push_front(1).unwrap();
        assert_eq!(list.pop_back(), Some(1));
        assert!(list.is_empty());

        // Multiple elements - should maintain order
        list.push_front(1).unwrap();
        list.push_front(2).unwrap();
        list.push_front(3).unwrap();

        assert_eq!(list.pop_back(), Some(1)); // First pushed element should be at back
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn test_remove() {
        let mut list = ArrayLinkedList::with_capacity(5);

        let idx1 = list.push_front(1).unwrap();
        let idx2 = list.push_front(2).unwrap();
        let idx3 = list.push_front(3).unwrap();

        // Remove middle element
        assert_eq!(list.remove(idx2), Some(2));
        assert_eq!(list.len(), 2);

        // Remove head
        assert_eq!(list.remove(idx3), Some(3));
        assert_eq!(list.len(), 1);

        // Remove tail (only element)
        assert_eq!(list.remove(idx1), Some(1));
        assert!(list.is_empty());

        // Remove non-existent element
        assert_eq!(list.remove(idx1), None);

        // Remove invalid index
        assert_eq!(list.remove(99), None);
    }

    #[test]
    fn test_linked_list_integrity() {
        let mut list = ArrayLinkedList::with_capacity(3);

        let _ = list.push_front(1).unwrap();
        let idx2 = list.push_front(2).unwrap();
        let _ = list.push_front(3).unwrap();

        // Structure should be: 3 <-> 2 <-> 1
        // Verify links by removing middle and checking connectivity
        assert_eq!(list.remove(idx2), Some(2));

        // After removing middle, head and tail should be properly connected
        assert_eq!(list.pop_back(), Some(1)); // Should still work
        assert_eq!(list.pop_back(), Some(3));
    }

    #[test]
    fn test_free_list_reuse() {
        let mut list = ArrayLinkedList::with_capacity(2);

        let idx1 = list.push_front(1).unwrap();
        let _ = list.push_front(2).unwrap();

        // Remove first element
        assert_eq!(list.remove(idx1), Some(1));

        // Should be able to reuse the freed slot
        let idx3 = list.push_front(3).unwrap();
        assert_eq!(idx3, idx1); // Should reuse the same index

        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_back(), Some(2)); // Original second element
        assert_eq!(list.pop_back(), Some(3)); // New element in reused slot
    }

    #[test]
    fn test_boundary_cases() {
        let mut list: ArrayLinkedList<i32> = ArrayLinkedList::with_capacity(0);
        assert_eq!(list.push_front(1), Err(()));
        assert_eq!(list.pop_back(), None);

        let mut list = ArrayLinkedList::with_capacity(1);
        let idx = list.push_front(42).unwrap();
        assert_eq!(list.remove(idx), Some(42));
        assert!(list.is_empty());

        // Should be able to reuse the single slot
        assert_eq!(list.push_front(100), Ok(idx));
    }

    #[test]
    fn test_string_values() {
        let mut list = ArrayLinkedList::with_capacity(2);

        list.push_front("hello".to_string()).unwrap();
        list.push_front("world".to_string()).unwrap();

        assert_eq!(list.pop_back(), Some("hello".to_string()));
        assert_eq!(list.pop_back(), Some("world".to_string()));
    }

    #[test]
    fn test_len_after_mixed_operations() {
        let mut list = ArrayLinkedList::with_capacity(5);

        assert_eq!(list.len(), 0);

        list.push_front(1).unwrap();
        list.push_front(2).unwrap();
        list.push_front(3).unwrap();
        assert_eq!(list.len(), 3);

        // Remove middle element
        let idx2 = list.push_front(4).unwrap();
        assert_eq!(list.len(), 4);
        list.remove(idx2);
        assert_eq!(list.len(), 3);

        // Pop all
        list.pop_back();
        list.pop_back();
        list.pop_back();
        assert_eq!(list.len(), 0);

        // Repopulate
        list.push_front(10).unwrap();
        list.push_front(20).unwrap();
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_free_list_lifo_order() {
        let mut list = ArrayLinkedList::with_capacity(5);

        let idx1 = list.push_front(1).unwrap();
        let idx2 = list.push_front(2).unwrap();
        let idx3 = list.push_front(3).unwrap();

        // Free slots in LIFO order: remove idx3, then idx2, then idx1
        list.remove(idx3); // Frees index 3
        list.remove(idx2); // Frees index 2
        list.remove(idx1); // Frees index 1

        // Push should reuse indices in LIFO order: 1, 2, 3
        assert_eq!(list.push_front(10).unwrap(), idx1); // Last freed = first reused
        assert_eq!(list.push_front(20).unwrap(), idx2);
        assert_eq!(list.push_front(30).unwrap(), idx3);
    }

    #[test]
    fn test_is_empty_after_operations() {
        let mut list = ArrayLinkedList::with_capacity(3);

        assert!(list.is_empty());

        list.push_front(1).unwrap();
        assert!(!list.is_empty());

        list.pop_back();
        assert!(list.is_empty());

        // Multiple push/pop
        let idx1 = list.push_front(1).unwrap();
        let idx2 = list.push_front(2).unwrap();
        assert!(!list.is_empty());

        list.remove(idx2); // Remove head
        assert!(!list.is_empty());

        list.remove(idx1); // Remove remaining
        assert!(list.is_empty());
    }

    #[test]
    fn test_multiple_sequential_removes() {
        let mut list = ArrayLinkedList::with_capacity(5);

        let idx1 = list.push_front(1).unwrap();
        let idx2 = list.push_front(2).unwrap();
        let idx3 = list.push_front(3).unwrap();
        let idx4 = list.push_front(4).unwrap();
        let idx5 = list.push_front(5).unwrap();

        // Remove all except one, then repopulate
        list.remove(idx2);
        list.remove(idx4);
        list.remove(idx1);
        list.remove(idx5);
        assert_eq!(list.len(), 1);

        // Remove the last one
        list.remove(idx3);
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);

        // Repopulate
        let _ = list.push_front(100).unwrap();
        let _ = list.push_front(200).unwrap();
        let _ = list.push_front(300).unwrap();
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_back(), Some(100));
        assert_eq!(list.pop_back(), Some(200));
        assert_eq!(list.pop_back(), Some(300));
    }
}
