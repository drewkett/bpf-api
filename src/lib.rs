pub mod collections;
pub mod error;
pub mod probes;
pub mod prog;

mod platform;

#[cfg(test)]
mod tests {
    use crate::collections::{Array, HashMap, Queue};

    #[test]
    fn hashmap_set_get() {
        let list = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let map = HashMap::<u32, [u8; 16]>::with_capacity(10).unwrap();
        assert!(matches!(map.set(300, list), Ok(_)));
        assert!(matches!(map.get(300), Ok(_)));
        assert_eq!(map.get(300).unwrap(), list);
    }

    #[test]
    fn queue_push_pop() {
        const QUEUE_SIZE: u32 = 10;
        let queue = Queue::<u32>::with_capacity(QUEUE_SIZE).unwrap();

        /* Fill the queue to the max */
        for i in 0..10 {
            assert!(matches!(queue.push(i + 100), Ok(_)));
        }

        /* make sure the next push fails */
        assert!(matches!(queue.push(1000), Err(_)));

        /* test front and make sure it doesn't consume */
        assert!(matches!(queue.front(), Ok(100)));
        assert!(matches!(queue.front(), Ok(100)));

        /* pop all items and check values */
        for i in 0..10 {
            match queue.pop() {
                Ok(val) => assert_eq!(val, i + 100),
                Err(e) => panic!("queue.pop() failed: {}", e),
            }
        }

        /* make sure queue is now empty */
        assert!(matches!(queue.pop(), Err(_)));
    }

    #[test]
    fn array_set_get() {
        const ARRAY_SIZE: u32 = 10;
        let array = Array::<u32>::with_capacity(ARRAY_SIZE).unwrap();

        for i in 0..ARRAY_SIZE {
            let val = i + 100;
            assert!(matches!(array.get(i), Ok(0)));
            assert!(matches!(array.set(i, val), Ok(_)));
            match array.get(i) {
                Ok(v) => assert_eq!(v, val),
                Err(e) => panic!("array.get() failed: {}", e),
            }
        }
    }
}
