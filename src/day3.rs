use std::collections::HashSet;

fn count_bit_balance<'a, I: Iterator<Item = &'a &'a str> + Clone>(
    line_iter: I,
    single_bit_idx: Option<usize>,
) -> Vec<isize> {
    let bit_count = match single_bit_idx {
        Some(_) => 1,
        None => line_iter.clone().next().unwrap().len(),
    };
    let mut bit_counters = vec![0isize; bit_count];

    for line in line_iter {
        for (i, ch) in line.char_indices() {
            let counter_idx = match single_bit_idx {
                Some(single_bit_idx) => {
                    if single_bit_idx != i {
                        continue;
                    }
                    0
                }
                None => i,
            };

            match ch {
                '0' => bit_counters[counter_idx] -= 1,
                '1' => bit_counters[counter_idx] += 1,
                _ => unreachable!(),
            }
        }
    }

    bit_counters
}

pub fn part1(input: &str) -> usize {
    let all_lines = input.split_whitespace().collect::<HashSet<&str>>();
    let bit_counters = count_bit_balance(all_lines.iter(), None);

    let mut gamma_rate = 0usize;
    let mut mask = 0usize;

    for (i, bit) in bit_counters.into_iter().rev().enumerate() {
        let bit = if bit > 0 { 1 } else { 0 };

        gamma_rate |= bit << i;
        mask |= 1 << i;
    }

    gamma_rate * (gamma_rate ^ mask)
}

fn find_rating<F: Fn(isize, char) -> bool>(
    mut candidates: HashSet<&str>,
    filter: F,
) -> Option<usize> {
    let bit_count = candidates.iter().next().unwrap().len();

    for bit_idx in 0..bit_count {
        let bit_balances = count_bit_balance(candidates.iter(), Some(bit_idx));
        let bit_balance = bit_balances[0];

        for candidate in candidates.clone() {
            let bit_char = candidate.chars().nth(bit_idx)?;

            if !filter(bit_balance, bit_char) {
                candidates.remove(candidate);

                if candidates.len() == 1 {
                    let rating_str = candidates.iter().next()?;
                    return usize::from_str_radix(rating_str, 2).ok();
                }
            }
        }
    }

    None
}

pub fn part2(input: &str) -> usize {
    let all_candidates = input.split_whitespace().collect::<HashSet<&str>>();

    let o2_rating = find_rating(all_candidates.clone(), |bit_balance, bit_char| {
        bit_char == if bit_balance >= 0 { '1' } else { '0' }
    })
    .unwrap();

    let co2_rating = find_rating(all_candidates, |bit_balance, bit_char| {
        bit_char == if bit_balance >= 0 { '0' } else { '1' }
    })
    .unwrap();

    o2_rating * co2_rating
}
