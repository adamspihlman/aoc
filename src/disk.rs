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

    pub fn compute_checksum(&mut self) -> u64 {
        let mut compact: Vec<u64> = Vec::new();
        let mut next_index = 0;
        let mut prev_file_id = 0;

        while prev_file_id < (self.file_lengths.len() - 1) as u64 {
            let location = self.disk_locations[next_index];
            match location {
                DiskLocation::File(file_id) => {
                    prev_file_id = file_id;
                    compact.push(file_id);
                    let index = file_id as usize;
                    if self.file_lengths[index] > 0 {
                        self.file_lengths[index] -= 1;
                    }
                }
                DiskLocation::Blank => {
                    while self.file_lengths[self.file_lengths.len() - 1] == 0 {
                        self.file_lengths.pop();
                    }

                    let file_id = self.file_lengths.len() - 1;
                    compact.push(file_id as u64);

                    if self.file_lengths[file_id] <= 1 {
                        self.file_lengths.pop();
                    } else {
                        self.file_lengths[file_id] -= 1;
                    }
                    self.blank_swap_count -= 1;
                }
            }
            next_index += 1;
        }

        compact
            .iter()
            .enumerate()
            .map(|(index, id)| index as u64 * id)
            .sum()
    }
}
