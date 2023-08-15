use std::{cmp::Ordering, collections::HashMap, io::BufRead, ops::Range};

/**
 * B. Memory Manager
 * time limit per test: 1 second
 * memory limit per test: 64 megabytes
 * input: standard input
 * output: standard output
 *
 * There is little time left before the release of the first national operating system BerlOS. Some of its components are
 * not finished yet — the memory manager is among them. According to the developers' plan, in the first release the memory
 * manager will be very simple and rectilinear. It will support three operations:
 *   alloc n — to allocate n bytes of the memory and return the allocated block's identifier x;
 *   erase x — to erase the block with the identifier x;
 *   defragment — to defragment the free memory, bringing all the blocks as close to the beginning of the memory as possible
 * and preserving their respective order;
 *
 * The memory model in this case is very simple. It is a sequence of m bytes, numbered for convenience from the first to the
 * m-th.
 *
 * The first operation alloc n takes as the only parameter the size of the memory block that is to be allocated. While
 * processing this operation, a free block of n successive bytes is being allocated in the memory. If the amount of such
 * blocks is more than one, the block closest to the beginning of the memory (i.e. to the first byte) is prefered. All these
 * bytes are marked as not free, and the memory manager returns a 32-bit integer numerical token that is the identifier of
 * this block. If it is impossible to allocate a free block of this size, the function returns NULL.
 *
 * The second operation erase x takes as its parameter the identifier of some block. This operation frees the system memory,
 * marking the bytes of this block as free for further use. In the case when this identifier does not point to the
 * previously allocated block, which has not been erased yet, the function returns ILLEGAL_ERASE_ARGUMENT.
 *
 * The last operation defragment does not have any arguments and simply brings the occupied memory sections closer to the
 * beginning of the memory without changing their respective order.
 *
 * In the current implementation you are to use successive integers, starting with 1, as identifiers. Each successful alloc
 * operation procession should return following number. Unsuccessful alloc operations do not affect numeration.
 *
 * You are to write the implementation of the memory manager. You should output the returned value for each alloc command.
 * You should also output ILLEGAL_ERASE_ARGUMENT for all the failed erase commands.
 *
 * Input
 * The first line of the input data contains two positive integers t and m (1 ≤ t ≤ 100;1 ≤ m ≤ 100), where t — the amount
 * of operations given to the memory manager for processing, and m — the available memory size in bytes. Then there follow
 * t lines where the operations themselves are given. The first operation is alloc n (1 ≤ n ≤ 100), where n is an integer.
 * The second one is erase x, where x is an arbitrary 32-bit integer numerical token. The third operation is defragment.
 *
 * Output
 * Output the sequence of lines. Each line should contain either the result of alloc operation procession, or
 * ILLEGAL_ERASE_ARGUMENT as a result of failed erase operation procession. Output lines should go in the same order in
 * which the operations are processed. Successful procession of alloc operation should return integers, starting with 1, as
 * the identifiers of the allocated blocks.
 *
 * Examples
 *   Input
 *   6 10
 *   alloc 5
 *   alloc 3
 *   erase 1
 *   alloc 6
 *   defragment
 *   alloc 6
 *   
 *   Output
 *   1
 *   2
 *   NULL
 *   3
 */
fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let (t, m) = first_line.split_once(' ').unwrap();
    let (t, m): (usize, usize) = (t.parse().unwrap(), m.parse().unwrap());

    let mut new_id = 1;
    let mut mappings = HashMap::new();
    let mut memory = vec![MemoryCell::Free(0..m)];
    for line in lines.take(t) {
        let line = line.unwrap();

        if let Some((action, value)) = line.split_once(' ') {
            let value: isize = value.parse().unwrap();

            if action == "alloc" {
                let size = value as usize;
                let free_position = match memory
                    .iter()
                    .position(|cell| cell.is_free() && cell.size() >= size)
                {
                    Some(pos) => pos,
                    None => {
                        println!("NULL");
                        continue;
                    }
                };

                if let Some(remaining) = memory[free_position].allocate(size) {
                    memory.insert(free_position + 1, remaining);

                    // Moving all mappings
                    mappings
                        .values_mut()
                        .filter(|pos| **pos > free_position)
                        .for_each(|pos| *pos += 1);
                }

                let id = new_id;
                new_id += 1;
                mappings.insert(id, free_position);
                println!("{id}");
            } else {
                let id = value;

                if let Some(&pos) = mappings.get(&id) {
                    let mut size = memory[pos].size();
                    memory[pos].free();
                    mappings.remove(&id);

                    // Merging with previous and next, if possible
                    if pos < memory.len() - 1 && memory[pos + 1].is_free() {
                        let next_size = memory[pos + 1].size();
                        memory[pos].append(next_size);
                        memory.remove(pos + 1);
                        size += next_size;

                        // Moving all mappings
                        mappings
                            .values_mut()
                            .filter(|p| **p > pos + 1)
                            .for_each(|p| *p -= 1);
                    }

                    if pos > 0 && memory[pos - 1].is_free() {
                        memory[pos - 1].append(size);
                        memory.remove(pos);

                        // Moving all mappings
                        mappings
                            .values_mut()
                            .filter(|p| **p > pos)
                            .for_each(|p| *p -= 1);
                    }
                } else {
                    println!("ILLEGAL_ERASE_ARGUMENT");
                }
            }
        } else {
            // Defragment
            let mut i = 0;
            while i < memory.len() - 1 {
                let (left, right) = memory.split_at_mut(i + 1);
                let current = &mut left[i];
                let next = &mut right[0];

                if current.is_free() && next.is_free() {
                    current.append(next.size());
                    memory.remove(i + 1);

                    // Moving all mappings
                    mappings
                        .values_mut()
                        .filter(|pos| **pos > i)
                        .for_each(|pos| *pos -= 1);

                    continue;
                } else if current.is_free() && !next.is_free() {
                    next.move_to(current.range().start);
                    current.move_to(next.range().end);
                    memory.swap(i, i + 1);

                    // Moving one mapping
                    *mappings.values_mut().find(|pos| **pos == i + 1).unwrap() = i;
                }

                i += 1;
            }
        }
    }
}

#[derive(Debug)]
enum MemoryCell {
    Free(Range<usize>),
    Allocated(Range<usize>),
}

impl MemoryCell {
    // Returns the rest if it splitted
    fn allocate(&mut self, len: usize) -> Option<Self> {
        if let Self::Free(range) = self {
            match range.len().cmp(&len) {
                Ordering::Less => panic!("Tried to allocate memory on small block"),
                Ordering::Equal => {
                    *self = Self::Allocated(range.clone());
                    None
                }
                Ordering::Greater => {
                    let remaining_range = range.start + len..range.end;
                    let range = range.start..range.start + len;
                    *self = Self::Allocated(range);
                    Some(Self::Free(remaining_range))
                }
            }
        } else {
            panic!("Called allocate on allocated block")
        }
    }

    fn free(&mut self) {
        if let Self::Allocated(range) = self {
            *self = Self::Free(range.clone());
        } else {
            panic!("Called free on free block")
        }
    }

    fn append(&mut self, size: usize) {
        let range = self.range_mut();
        range.end += size;
    }

    fn move_to(&mut self, pos: usize) {
        let range = self.range_mut();
        let size = range.len();
        *range = pos..pos + size;
    }

    fn is_free(&self) -> bool {
        matches!(self, Self::Free(_))
    }

    fn range(&self) -> &Range<usize> {
        match self {
            Self::Free(range) => range,
            Self::Allocated(range) => range,
        }
    }

    fn range_mut(&mut self) -> &mut Range<usize> {
        match self {
            Self::Free(range) => range,
            Self::Allocated(range) => range,
        }
    }

    fn size(&self) -> usize {
        self.range().len()
    }
}
