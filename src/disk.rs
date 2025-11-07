#[derive(Debug, PartialEq, Copy, Clone)]
enum DiskLocation {
    File(u64),
    Blank,
}

#[derive(Debug)]
pub struct Disk {
    file_lengths: Vec<u64>,
    disk_locations: Vec<DiskLocation>,
    blank_swap_count: u64,
}

impl Disk {
    pub fn from(input: &str) -> Disk {
        let encoding = input.trim().to_string();
        let mut file_lengths = Vec::new();
        let mut disk_locations = Vec::new();
        let mut blank_swap_count = 0;
        let mut staged_blank_swaps = 0;
        let mut next_file_id = 0;
        let mut next_location = DiskLocation::File(next_file_id);

        for encode in encoding.chars() {
            let length = encode.to_digit(10).unwrap() as u64;

            for _ in 0..length {
                disk_locations.push(next_location);
            }

            match next_location {
                DiskLocation::File(_) => {
                    file_lengths.push(length);
                    next_file_id += 1;
                    next_location = DiskLocation::Blank;
                    blank_swap_count += staged_blank_swaps;
                    staged_blank_swaps = 0;
                }
                DiskLocation::Blank => {
                    staged_blank_swaps += length;
                    next_location = DiskLocation::File(next_file_id);
                }
            }
        }
        Disk {
            file_lengths,
            disk_locations,
            blank_swap_count,
        }
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

    pub fn compute_checksum(&mut self) -> u64 {
        let mut start_index = 0;
        let mut end_index = self.get_last_file_block();

        while start_index <= end_index {
            match self.disk_locations[start_index] {
                DiskLocation::File(_) => (),
                DiskLocation::Blank => {
                    self.disk_locations.swap(start_index, end_index);
                    end_index = self.get_last_file_block();
                }
            }
            start_index += 1;
        }

        self.disk_locations
            .iter()
            .enumerate()
            .map(|(index, loc)| match loc {
                DiskLocation::File(file_id) => index as u64 * file_id,
                DiskLocation::Blank => panic!("Unexpected blank disk block"),
            })
            .sum()
    }
}
