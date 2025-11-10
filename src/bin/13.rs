advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u64> {
    let mut claw = advent_of_code::claw::ClawBuilder::default()
        .machines(input)
        .build();
    let result = claw.min_cost();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let prize_offset = 10000000000000;
    let mut claw = advent_of_code::claw::ClawBuilder::default()
        .prize_offset(prize_offset)
        .machines(input)
        .build();
    return None;
    let result = claw.min_cost();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_math_proof_of_concept() {
        let final_x = 8400;
        let final_y = 5400;
        let final_x_f = final_x as f64;
        let final_y_f = final_y as f64;
        let b_cost: f64 = 1.0;
        let b_x = 22;
        let b_y = 67;
        let b_x_f = b_x as f64;
        let b_y_f = b_y as f64;

        let a_cost: f64 = 3.0;
        let a_x = 94;
        let a_y = 34;
        let a_x_f = a_x as f64;
        let a_y_f = a_y as f64;

        let prize_distance = (final_x_f.powi(2) + final_y_f.powi(2)).sqrt();

        let b_distance = (b_x_f.powi(2) + b_y_f.powi(2)).sqrt();

        let prize_b_angle = (final_y_f / final_x_f).atan() - (b_y_f / b_x_f).atan();
        let b_distance_on_prize = b_distance / prize_b_angle.cos();
        let b_normal_cost = b_cost * (prize_distance / b_distance_on_prize);

        let a_distance = (a_x_f.powi(2) + a_y_f.powi(2)).sqrt();

        let prize_a_angle = (final_y_f / final_x_f).atan() - (a_y_f / a_x_f).atan();
        let a_distance_on_prize = a_distance / prize_a_angle.cos();
        let a_normal_cost = a_cost * (prize_distance / a_distance_on_prize);

        let mut b_press = 0;
        let mut a_press = 0;

        let b_cheaper = b_normal_cost < a_normal_cost;

        if b_cheaper {
            b_press = b_normal_cost.round() as i32;
        } else {
            a_press = a_normal_cost.round() as i32;
        }

        loop {
            if b_press < 0 || a_press < 0 {
                break;
            }
            let cur_x = b_x * b_press + a_x * a_press;
            let cur_y = b_y * b_press + a_y * a_press;

            if cur_x == final_x && cur_y == final_y {
                break;
            } else if cur_x > final_x || cur_y > final_y {
                if b_cheaper {
                    b_press -= 1;
                    a_press = 0;
                } else {
                    a_press -= 1;
                    b_press = 0;
                }
                continue;
            } else {
                if b_cheaper {
                    a_press += 1;
                } else {
                    b_press += 1;
                }
                continue;
            }
        }
        assert_eq!(b_press, 40);
        assert_eq!(a_press, 80);
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_one_solution() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(36954));
    }
}
