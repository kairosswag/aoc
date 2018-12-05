
#[aoc(day5, part1)]
pub fn part1(input: &str) -> usize {
    let mut polymer : Vec<char> = input.chars().collect();
    let max = polymer.len();
    let mut count = polymer.len();
    let mut curr_idx = 0;
    while curr_idx != max {
        if curr_idx > 0 && polymer[curr_idx] == '0' {
            curr_idx -= 1;
            continue;
        }
        if curr_idx == 0 && polymer[curr_idx] == '0' {
            curr_idx = idx_next_char(&polymer, curr_idx).unwrap();
            continue;
        }
        if let Some(next) = idx_next_char(&polymer, curr_idx){

            let curr_char = polymer[curr_idx];
            let next_char = polymer[next];
            if curr_char != next_char && curr_char.to_ascii_uppercase() == next_char.to_ascii_uppercase() {
                polymer[curr_idx] = '0';
                polymer[next] = '0';
                count -= 2;
                continue;
            } 
            curr_idx = next;
        } else {
            break;
        }
    }
    println!("{}", count);
    polymer.iter().filter(|&&c| c != '0').count()
}

fn idx_next_char(array: &[char], curr: usize) -> Option<usize> {
    let mut res_idx = curr + 1;
    let len = array.len();
    while res_idx < len && array[res_idx] == '0' {
        res_idx += 1;
    }
    if res_idx >= len {
        return None;
    } else {
        return Some(res_idx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample1() {
        let sample = "dabAcCaCBAcCcaDA";
        assert_eq!(part1(&sample), 10);
    }

    #[test]
    fn sample2() {
        let sample = "HhrbBWJLljRMmoKkOrL";
        assert_eq!(part1(&sample), 3);
    }

    #[test]
    fn sample3() {
        let sample = "HHRbBWJLljRMmoKkOrL";
        assert_eq!(part1(&sample), 5);
    }

}