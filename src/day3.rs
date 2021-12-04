use std::collections::HashSet;

/// Returns a vector of "balance" aggregates representing popularities of '0' vs '1' bits at their respective positions.
///
/// # Arguments
/// * `line_iter` - An iterator of input lines. Assumes the string contains only '0' and '1' characters. Assumes all items are of equal length.
/// * `single_bit_idx` - An index for counting only a single bit for every item. Just to make things a bit faster when all counts are not necessary.
///
/// # Return value
/// For each element of the vector, the value is the sum of all bits at that index - assuming the values -1 and 1 for each '0' and '1' bit respectively.
/// As a result, a positive value indicates that '1' bits were more popular and vice versa, with a balance of 0 meaning equal amounts of '0' and '1' bits were present.
///
/// Given the example input values:
/// ```
/// 000111
/// 001100
/// 111000
/// 010101
/// ```
/// the resulting bit balance vector will be `[-2, 0, 0, 2, -2, 0]`
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

    // Look at bits from the right, so that we could push them into `gamma_rate` and `mask` in order
    for (i, bit) in bit_counters.into_iter().rev().enumerate() {
        let bit = if bit > 0 { 1 } else { 0 };

        // Place bits into gamma_rate, first shifting the bit into the expected position based on loop index
        gamma_rate |= bit << i;
        // Also place 1-bits into `mask`, so that we know which bits to invert later
        mask |= 1 << i;
    }

    // Given that `epsilon_rate` is defined as the opposite popularity to `gamma_rate`,
    // we can just invert the bits we added to `gamma_rate` to get `epsilon_rate`
    let epsilon_rate = gamma_rate ^ mask;

    gamma_rate * epsilon_rate
}

fn find_rating<F: Fn(isize, char) -> bool>(
    mut candidates: HashSet<&str>,
    filter: F,
) -> Option<usize> {
    let bit_count = candidates.iter().next().unwrap().len();

    // For every bit position, "distill" the candidates until only one is left
    for bit_idx in 0..bit_count {
        // Only count balance for this single bit, since we have to recount for every next bit anyway
        let bit_balances = count_bit_balance(candidates.iter(), Some(bit_idx));
        let bit_balance = bit_balances[0];

        for candidate in candidates.clone() {
            let bit_char = candidate.chars().nth(bit_idx)?;

            // Run given filter function with the popularity balance for this specific bit
            // and the candidate's bit at the same position for comparison
            if !filter(bit_balance, bit_char) {
                candidates.remove(candidate);

                if candidates.len() == 1 {
                    // Only one candidate is left - parse it into a number and return
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
