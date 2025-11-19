use advent_of_code::lan;

advent_of_code::solution!(23);

fn parse_input(input: &str) -> Vec<(String, String)> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            (parts[0].to_string(), parts[1].to_string())
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let parsed = parse_input(input);
    let lan = lan::Lan::from(parsed);
    let groups = lan.find_groups();

    Some(groups.len() as u64)
}

pub fn part_two(input: &str) -> Option<String> {
    let parsed = parse_input(input);
    let lan = lan::Lan::from(parsed);
    let maximal_clique = lan.find_maximal_clique();

    let mut names: Vec<String> = maximal_clique
        .iter()
        .map(|comp| comp.name().to_string())
        .collect();

    names.sort();

    Some(names.join(","))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_one_solution() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1215));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }

    #[test]
    fn test_part_two_solution() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(
            result,
            Some("bm,by,dv,ep,ia,ja,jb,ks,lv,ol,oy,uz,yt".to_string())
        );
    }
}
