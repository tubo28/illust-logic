type Mask = u64;
type BlockSize = usize;

fn dfs_with_constraints(
    width: usize,
    cur: Mask,
    pos: usize,
    blocks: &[BlockSize],
    empty: Mask,
    filled: Mask,
) {
    if blocks.is_empty() {
        if cur & filled == filled {
            println!("flags: {:010b}", cur);
        }
        return;
    }

    let cur_block = blocks[0];
    let mask = (1u64 << cur_block) - 1;
    for i in pos..(pos + width - cur_block + 1) {
        if i + cur_block <= width {
            let next = cur | mask << i;
            let cur_head = (1u64 << (cur_block + i)) - 1;
            if empty & next == 0
                && (filled >> (i + cur_block)) & 1 == 0
                && next & filled & cur_head == filled & cur_head
            {
                dfs_with_constraints(width, next, i + cur_block + 1, &blocks[1..], empty, filled);
            }
        }
    }
}

fn main() {
    dfs_with_constraints(10, 0, 0, &[1, 2, 3], 0b00000_00000, 0b00000_00000);
    println!("");
    dfs_with_constraints(10, 0, 0, &[1, 2, 3], 0b00000_00000, 0b10000_00100);
    println!("");
}
