use std::fmt;
use std::ptr;

struct LinkedListNode {
    value: i64,
    prev: *mut LinkedListNode,
    next: *mut LinkedListNode,
}

const DEBUG: bool = true;

fn if_debug<F>(f: F)
where
    F: Fn() -> (),
{
    if DEBUG {
        f();
    }
}

impl LinkedListNode {
    unsafe fn mix_node(list: &CircularLinkedList, node: *mut LinkedListNode) {
        let amount = (*node).value % (list.len() - 1) as i64;
        if amount == 0 {
            return;
        }

        let (before, after) = if amount > 0 {
            let mut before = (*node).next;
            for _ in 1..amount {
                before = (*before).next;
            }

            (before, (*before).next)
        } else {
            let mut after = (*node).prev;
            for _ in 1..amount.unsigned_abs() {
                after = (*after).prev;
            }

            ((*after).prev, after)
        };

        let (before_node, after_node) = ((*node).prev, (*node).next);
        (*before_node).next = after_node;
        (*after_node).prev = before_node;

        (*before).next = node;
        (*node).next = after;
        (*after).prev = node;
        (*node).prev = before;
    }
}

struct CircularLinkedList {
    head: *mut LinkedListNode,
    zero: *mut LinkedListNode,
    count: usize,
}

impl CircularLinkedList {
    fn new(first_value: i64) -> CircularLinkedList {
        let head = Self::make_node(first_value);
        unsafe {
            (*head).prev = head;
            (*head).next = head;
        }

        CircularLinkedList {
            head,
            zero: if first_value == 0 {
                head
            } else {
                ptr::null_mut()
            },
            count: 1,
        }
    }

    fn len(&self) -> usize {
        self.count
    }

    fn make_node(value: i64) -> *mut LinkedListNode {
        Box::into_raw(Box::new(LinkedListNode {
            value,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }))
    }

    fn append(&mut self, value: i64) {
        let new_node = Self::make_node(value);

        unsafe {
            let begin = self.head;
            let end = (*self.head).prev;

            assert_eq!((*end).next, begin);
            assert_eq!((*begin).prev, end);

            (*end).next = new_node;
            (*new_node).prev = end;
            (*begin).prev = new_node;
            (*new_node).next = begin;

            if value == 0 {
                assert!(self.zero.is_null(), "should only have one zero");
                self.zero = new_node;
            }
        }

        self.count += 1;
    }

    fn zero_iter(&mut self) -> Option<ZeroIter> {
        if self.zero.is_null() {
            None
        } else {
            Some(ZeroIter(self.zero))
        }
    }

    fn nodes_in_order(&self) -> Vec<*mut LinkedListNode> {
        let mut node = self.head;
        let mut in_order = vec![node];
        unsafe {
            while (*node).next != self.head {
                in_order.push((*node).next);
                node = (*node).next;
            }
        }
        in_order
    }

    fn mix(&mut self) {
        for node in CircularLinkedList::nodes_in_order(&self) {
            unsafe {
                LinkedListNode::mix_node(self, node);
            }
        }
    }

    fn mix_ten(&mut self) {
        let in_order = CircularLinkedList::nodes_in_order(&self);
        for _ in 0..10 {
            for node in &in_order {
                unsafe {
                    LinkedListNode::mix_node(self, *node);
                }
            }
        }
    }
}

impl fmt::Display for CircularLinkedList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let head = self.head;
        let mut current = head;
        write!(f, "{}", unsafe { (*current).value })?;
        unsafe {
            while (*current).next != head {
                current = (*current).next;
                write!(f, ", {}", (*current).value)?;
            }
        }
        Ok(())
    }
}

impl Drop for CircularLinkedList {
    fn drop(&mut self) {
        let head = self.head;
        let mut node = head;
        loop {
            unsafe {
                let next = (*node).next;
                drop(Box::from_raw(node));
                if next == head {
                    break;
                }
                node = next;
            }
        }
    }
}

struct ZeroIter(*mut LinkedListNode);

impl Iterator for ZeroIter {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        Some(unsafe {
            let v = (*self.0).value;
            self.0 = (*self.0).next;
            v
        })
    }
}

fn to_circular_linked_list(encrypted: &str, key: i64) -> CircularLinkedList {
    let mut iter = encrypted
        .trim()
        .split('\n')
        .map(|line| line.parse::<i64>().expect("i64"));

    let Some(n) = iter.next() else {
        panic!("empty encrypted list");
    };

    let mut cll = CircularLinkedList::new(n * key);
    while let Some(n) = iter.next() {
        cll.append(n * key);
    }

    cll
}

fn expect_thousands(cll: &mut CircularLinkedList, expected_thousands: [i64; 3]) {
    if_debug(|| {
        if cll.len() < 10 {
            println!("cll: {}", cll);
        }
    });

    let mut thousands = vec![];

    assert_eq!(
        cll.zero_iter().expect("zero in cll").next(),
        Some(0),
        "must start at zero"
    );

    let zit = cll.zero_iter().expect("zero in cll");
    for (i, value) in zit.enumerate().take(3001) {
        if i == 0 {
            assert_eq!(value, 0);
        } else if i % 1000 == 0 {
            thousands.push(value);
        }
    }

    println!(
        "Coordinates: {}",
        thousands
            .iter()
            .map(<_>::to_string)
            .collect::<Vec<_>>()
            .join(", ")
    );
    assert_eq!(thousands, expected_thousands);

    let coord_sum: i64 = thousands.iter().sum();
    let expected_sum = expected_thousands.iter().copied().sum();
    println!("Observed coordinate sum: {coord_sum}");
    println!("Expected coordinate sum: {expected_sum}");
    assert_eq!(coord_sum, expected_sum);
}

const PART1_KEY: i64 = 1;
const PART2_KEY: i64 = 811_589_153;

#[test]
fn given_example() {
    static ENCRYPTED: &str = "1
2
-3
3
-2
0
4";

    let number_count = ENCRYPTED.lines().count();

    let mut cll = to_circular_linked_list(ENCRYPTED, PART1_KEY);
    cll.mix();

    assert_eq!(
        cll.zero_iter()
            .expect("contains zero")
            .take(number_count)
            .collect::<Vec<_>>(),
        [0, 3, -2, 1, 2, -3, 4]
    );

    expect_thousands(&mut cll, [4, -3, 2]);

    let mut cll = to_circular_linked_list(ENCRYPTED, PART2_KEY);

    assert_eq!(
        cll.zero_iter()
            .expect("contains zero")
            .take(number_count)
            .collect::<Vec<_>>(),
        [
            0,
            3246356612,
            811589153,
            1623178306,
            -2434767459,
            2434767459,
            -1623178306,
        ]
    );

    cll.mix_ten();

    assert_eq!(
        cll.zero_iter()
            .expect("contains zero")
            .take(number_count)
            .collect::<Vec<_>>(),
        [
            0,
            -2434767459,
            1623178306,
            3246356612,
            -1623178306,
            2434767459,
            811589153
        ]
    );
}

#[test]
fn adjusted_example_positive() {
    static ENCRYPTED: &str = "0
1
2
3";

    let mut cll = to_circular_linked_list(ENCRYPTED, PART1_KEY);
    cll.mix();

    assert_eq!(
        cll.zero_iter().unwrap().take(cll.len()).collect::<Vec<_>>(),
        vec![0, 1, 3, 2]
    );

    expect_thousands(&mut cll, [0, 0, 0]);
}

#[test]
fn adjusted_example_negative() {
    static ENCRYPTED: &str = "0
1
2
-3";

    let mut cll = to_circular_linked_list(ENCRYPTED, PART1_KEY);
    cll.mix();

    assert_eq!(
        cll.zero_iter().unwrap().take(cll.len()).collect::<Vec<_>>(),
        vec![0, 1, -3, 2]
    );

    expect_thousands(&mut cll, [0, 0, 0]);
}

#[test]
fn two_elems() {
    static ENCRYPTED: &str = "5
0";

    let mut cll = to_circular_linked_list(ENCRYPTED, PART1_KEY);
    cll.mix();

    expect_thousands(&mut cll, [0, 0, 0]);
}

fn main() {
    static ENCRYPTED: &str = include_str!("../input");

    // Part 1.
    let mut cll = to_circular_linked_list(ENCRYPTED, PART1_KEY);
    cll.mix();

    expect_thousands(&mut cll, [9916, 5669, 7736]);

    // Part 2.
    let mut cll = to_circular_linked_list(ENCRYPTED, PART2_KEY);
    cll.mix_ten();

    expect_thousands(&mut cll, [3105140099378, 730430237700, -2407173427798]);
}
