#[derive(Clone, Debug)]
pub enum DiskBlock {
    File(u64),
    FreeSpace,
}

type Input = Vec<DiskBlock>;

pub fn parse(input: &str) -> Input {
    input
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            let size = c.to_digit(10).unwrap();
            let block = if i % 2 == 0 {
                DiskBlock::File(i as u64 / 2)
            } else {
                DiskBlock::FreeSpace
            };

            vec![block; size as usize]
        })
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    let mut compacted_blocks = Vec::with_capacity(input.len());

    let mut i = 0;
    let mut j = input.len() - 1;

    'outer: while i <= j {
        let block = &input[i];

        match block {
            DiskBlock::File(_) => compacted_blocks.push(block.clone()),
            DiskBlock::FreeSpace => {
                let mut new_block = &input[j];
                while matches!(new_block, DiskBlock::FreeSpace) {
                    j -= 1;
                    if j <= i {
                        break 'outer;
                    }

                    new_block = &input[j];
                }

                compacted_blocks.push(new_block.clone());
                j -= 1;
            }
        }

        i += 1;
    }

    compacted_blocks
        .into_iter()
        .enumerate()
        .map(|(i, b)| match b {
            DiskBlock::FreeSpace => 0,
            DiskBlock::File(id) => i as u64 * id,
        })
        .sum()
}

pub fn part2(input: &Input) -> u64 {
    // (pos, size, id)
    let mut files = Vec::new();
    // (pos, size)
    let mut spaces = Vec::new();

    let mut pos = 0;
    let mut blocks = input.iter().peekable();
    while let Some(block) = blocks.next() {
        match block {
            DiskBlock::File(id) => {
                let size = std::iter::from_fn(|| {
                    blocks
                        .by_ref()
                        .next_if(|b| matches!(b, DiskBlock::File(id2) if id2 == id))
                })
                .count()
                    + 1;
                files.push((pos, size, id));
                pos += size;
            }
            DiskBlock::FreeSpace => {
                let size = std::iter::from_fn(|| {
                    blocks
                        .by_ref()
                        .next_if(|b| matches!(b, DiskBlock::FreeSpace))
                })
                .count()
                    + 1;
                spaces.push((pos, size));
                pos += size;
            }
        }
    }

    for (file_pos, file_size, _) in files.iter_mut().rev() {
        for (space_pos, space_size) in spaces.iter_mut() {
            if space_pos < file_pos && space_size >= file_size {
                *file_pos = *space_pos;

                *space_pos += *file_size;
                *space_size -= *file_size;

                break;
            }
        }
    }

    files
        .into_iter()
        .map(|(pos, size, id)| (pos..pos + size).sum::<usize>() as u64 * id)
        .sum()
}
