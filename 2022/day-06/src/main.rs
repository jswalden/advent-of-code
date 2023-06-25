static CONTENTS: &str = include_str!("../input");

static START_OF_PACKET_LEN: usize = 4;
static START_OF_MESSAGE_LEN: usize = 14;

fn to_bit(c: char) -> u32 {
    let n = c as u8 - 'a' as u8;
    assert!(n < 26, "must fit in u32");

    1u32 << n
}

fn start_of_component(component_len: usize) -> usize {
    assert!(
        CONTENTS.len() >= component_len,
        "must have at least a marker of data"
    );

    let mut buffer = vec![0; component_len];
    let mut start = 0;
    let mut len = 0;

    let mut bitset = 0u32;

    for (index, c) in CONTENTS.chars().enumerate() {
        let b = to_bit(c);

        if bitset & b != 0 {
            loop {
                let s = buffer[start];
                bitset &= !s;
                start = (start + 1) % component_len;
                len -= 1;

                if s == b {
                    break;
                }
            }
        }

        buffer[(start + len) % component_len] = b;
        bitset |= b;
        len += 1;

        if len == component_len {
            return index + 1;
        }
    }

    panic!("never found a marker");
}

fn main() {
    let packet_start = start_of_component(START_OF_PACKET_LEN);
    println!("packet starts at {}", packet_start);

    let message_start = start_of_component(START_OF_MESSAGE_LEN);
    println!("message starts at {}", message_start);
}
