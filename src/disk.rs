#[derive(Debug, PartialEq, Copy, Clone)]
enum DiskLocation {
    File(u64),
    Blank,
}

#[derive(Debug)]
pub struct Disk {
    disk_locations: Vec<DiskLocation>,
}

impl Disk {
    pub fn from(input: &str) -> Disk {
        let encoding = input.trim().to_string();
        let mut disk_locations = Vec::new();
        let mut next_file_id = 0;
        let mut next_location = DiskLocation::File(next_file_id);

        for encode in encoding.chars() {
            let length = encode.to_digit(10).unwrap() as u64;

            for _ in 0..length {
                disk_locations.push(next_location);
            }

            match next_location {
                DiskLocation::File(_) => {
                    next_file_id += 1;
                    next_location = DiskLocation::Blank;
                }
                DiskLocation::Blank => {
                    next_location = DiskLocation::File(next_file_id);
                }
            }
        }
        Disk { disk_locations }
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
