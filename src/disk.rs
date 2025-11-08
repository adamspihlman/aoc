use std::{cmp::Reverse, collections::BTreeMap};

#[derive(Debug, PartialEq, Copy, Clone)]
enum DiskLocation {
    File(u64),
    Blank,
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct FileBlock {
    pub start_index: usize,
    pub length: usize,
}

#[derive(Debug)]
pub struct Disk {
    disk_locations: Vec<DiskLocation>,
    file_blocks: BTreeMap<Reverse<u64>, FileBlock>,
}

impl Disk {
    pub fn from(input: &str) -> Disk {
        let encoding = input.trim().to_string();
        let mut disk_locations = Vec::new();
        let mut file_blocks = BTreeMap::new();
        let mut next_file_id = 0;
        let mut next_location = DiskLocation::File(next_file_id);

        for encode in encoding.chars() {
            let length = encode.to_digit(10).unwrap() as usize;
            let start_index = disk_locations.len();

            for _ in 0..length {
                disk_locations.push(next_location);
            }

            match next_location {
                DiskLocation::File(_) => {
                    file_blocks.insert(
                        Reverse(next_file_id),
                        FileBlock {
                            start_index,
                            length,
                        },
                    );
                    next_file_id += 1;
                    next_location = DiskLocation::Blank;
                }
                DiskLocation::Blank => {
                    next_location = DiskLocation::File(next_file_id);
                }
            }
        }
        Disk {
            disk_locations,
            file_blocks,
        }
    }

    fn jump_to_next_blank_block(&mut self, mut start_index: usize) -> usize {
        while start_index < self.disk_locations.len() {
            match self.disk_locations[start_index] {
                DiskLocation::Blank => {
                    break;
                }
                DiskLocation::File(file_id) => {
                    let file_block = self.file_blocks.remove(&Reverse(file_id)).unwrap();
                    start_index += file_block.length;
                }
            }
        }
        start_index
    }

    fn get_first_blank_block_from(&self, mut start_index: usize) -> usize {
        while start_index < self.disk_locations.len()
            && self.disk_locations[start_index] != DiskLocation::Blank
        {
            start_index += 1;
        }
        start_index
    }

    fn get_blank_block_size(&self, start_index: usize) -> usize {
        let mut end_index = start_index;
        while end_index < self.disk_locations.len()
            && self.disk_locations[end_index] == DiskLocation::Blank
        {
            end_index += 1;
        }
        end_index - start_index
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

    fn find_sized_last_file_block(&mut self, max_length: usize) -> Option<(u64, FileBlock)> {
        for (&Reverse(file_id), &file_block) in &self.file_blocks {
            if file_block.length <= max_length {
                self.file_blocks.remove(&Reverse(file_id));
                return Some((file_id, file_block));
            }
        }
        None
    }

    pub fn compute_block_checksum(&mut self) -> u64 {
        let mut start_index = self.jump_to_next_blank_block(0);
        let mut end_index = self.get_last_file_block();

        while start_index <= end_index {
            let blank_block_size = self.get_blank_block_size(start_index);
            let candidate = self.find_sized_last_file_block(blank_block_size);
            match candidate {
                Some((file_id, file_block)) => {
                    for idx in 0..file_block.length {
                        self.disk_locations[start_index + idx] = DiskLocation::File(file_id);
                        self.disk_locations[file_block.start_index + idx] = DiskLocation::Blank;
                    }
                    start_index += file_block.length;
                    end_index = self.get_last_file_block();
                }
                None => {
                    start_index = self.jump_to_next_blank_block(start_index + blank_block_size);
                }
            }
            println!("{:?}", self.disk_locations);
        }

        self.checksum()
    }

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
}
