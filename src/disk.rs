#[derive(Debug)]
pub struct Disk {
    disk_locations: Vec<DiskLocation>,
    file_blocks: Vec<MemoryBlock>,
    blank_blocks: Vec<MemoryBlock>,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct MemoryBlock {
    pub start_index: usize,
    pub length: usize,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum DiskLocation {
    File(u64),
    Blank,
}

impl From<&str> for Disk {
    fn from(value: &str) -> Self {
        let encoding = value.trim().to_string();
        let mut disk_locations = Vec::new();
        let mut file_blocks = Vec::new();
        let mut blank_blocks = Vec::new();
        let mut next_file_id = 0;
        let mut next_location = DiskLocation::File(next_file_id);

        for encode in encoding.chars() {
            let length = encode
                .to_digit(10)
                .expect("encoding should contain only digits") as usize;
            let start_index = disk_locations.len();
            let block = MemoryBlock {
                start_index,
                length,
            };

            for _ in 0..length {
                disk_locations.push(next_location);
            }

            match next_location {
                DiskLocation::File(_) => {
                    file_blocks.push(block);
                    next_file_id += 1;
                    next_location = DiskLocation::Blank;
                }
                DiskLocation::Blank => {
                    blank_blocks.push(block);
                    next_location = DiskLocation::File(next_file_id);
                }
            }
        }
        Self {
            disk_locations,
            file_blocks,
            blank_blocks,
        }
    }
}

impl Disk {
    pub fn compute_contiguous_checksum(&mut self) -> u64 {
        let mut start_index = self.get_first_blank_block_from(0);
        let mut end_index = self.get_last_file_block();

        while start_index <= end_index {
            match self.disk_locations[start_index] {
                DiskLocation::File(_) => (),
                DiskLocation::Blank => {
                    self.disk_locations.swap(start_index, end_index);
                    end_index = self.get_last_file_block();
                }
            }
            start_index = self.get_first_blank_block_from(start_index);
        }
        self.checksum()
    }

    pub fn compute_block_checksum(&mut self) -> u64 {
        while !self.file_blocks.is_empty() {
            let file_id = (self.file_blocks.len() - 1) as u64;
            let file_block = self.file_blocks.pop().expect("checked non-empty");

            match self.blank_blocks.iter_mut().position(|b| {
                b.length >= file_block.length && b.start_index < file_block.start_index
            }) {
                Some(blank_blocks_index) => {
                    let blank_block = &mut self.blank_blocks[blank_blocks_index];

                    for idx in 0..file_block.length {
                        self.disk_locations[blank_block.start_index + idx] =
                            DiskLocation::File(file_id);
                        self.disk_locations[file_block.start_index + idx] = DiskLocation::Blank;
                    }
                    blank_block.length -= file_block.length;
                    blank_block.start_index += file_block.length;

                    if blank_block.length == 0 {
                        self.blank_blocks.remove(blank_blocks_index);
                    }
                }
                None => {
                    continue;
                }
            }
        }

        self.checksum()
    }

    fn get_first_blank_block_from(&self, mut start_index: usize) -> usize {
        while start_index < self.disk_locations.len()
            && self.disk_locations[start_index] != DiskLocation::Blank
        {
            start_index += 1;
        }
        start_index
    }

    fn get_last_file_block(&mut self) -> usize {
        while self
            .disk_locations
            .pop_if(|loc| *loc == DiskLocation::Blank)
            .is_some()
        {
            continue;
        }
        self.disk_locations.len() - 1
    }

    fn checksum(&self) -> u64 {
        self.disk_locations
            .iter()
            .enumerate()
            .map(|(index, loc)| match loc {
                DiskLocation::File(file_id) => index as u64 * file_id,
                DiskLocation::Blank => 0,
            })
            .sum()
    }

    fn _print_disk(&self) {
        self.disk_locations.iter().for_each(|l| match l {
            DiskLocation::Blank => print!("_"),
            DiskLocation::File(id) => print!("{}", id),
        });
        println!();
    }
}
