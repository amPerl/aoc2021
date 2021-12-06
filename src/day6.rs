const FISH_AGE_BIRTH: usize = 8;
const FISH_AGE_RESET: usize = 6;

fn parse_input(input: &str) -> [usize; 9] {
    let mut result = vec![0usize; 9];

    for input_age in input.trim().split(',').map(|x| x.parse::<usize>().unwrap()) {
        result[input_age] += 1;
    }

    result.try_into().unwrap()
}

fn simulate_step(state: &mut [usize; 9]) {
    let fish_to_create = state[0];
    state[0] = 0;

    for i in 1..9 {
        state[i - 1] = state[i];
    }

    state[FISH_AGE_BIRTH] = fish_to_create;
    state[FISH_AGE_RESET] += fish_to_create;
}

pub fn part1(input: &str) -> usize {
    let mut fish_age_counts = parse_input(input);
    for _ in 0..80 {
        simulate_step(&mut fish_age_counts);
    }
    fish_age_counts.iter().sum()
}

pub fn part2(input: &str) -> usize {
    let mut fish_age_counts = parse_input(input);
    for _ in 0..256 {
        simulate_step(&mut fish_age_counts);
    }
    fish_age_counts.iter().sum()
}
