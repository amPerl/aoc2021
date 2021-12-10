fn is_opening_char(ch: char) -> bool {
    matches!(ch, '(' | '[' | '{' | '<')
}

fn is_closing_char(ch: char) -> bool {
    matches!(ch, ')' | ']' | '}' | '>')
}

fn get_opening_char(ch: char) -> char {
    match ch {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => '?',
    }
}

fn get_score(ch: char) -> usize {
    match ch {
        // part 1
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        // part 2
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => 0,
    }
}

pub fn part1(input: &str) -> usize {
    let mut score = 0;

    for line in input.split_whitespace() {
        let mut stack = Vec::new();

        for ch in line.chars() {
            if is_opening_char(ch) {
                stack.push(ch);
                continue;
            }

            if !is_closing_char(ch) {
                continue;
            }

            let last_in_stack = stack[stack.len() - 1];

            let expected = get_opening_char(ch);

            if last_in_stack != expected {
                score += get_score(ch);
                break;
            }

            stack.pop();
        }
    }

    score
}

pub fn part2(input: &str) -> usize {
    let mut scores = Vec::new();

    'lines: for line in input.split_whitespace() {
        let mut stack = Vec::new();
        let mut score = 0;

        for ch in line.chars() {
            if is_opening_char(ch) {
                stack.push(ch);
                continue;
            }

            if !is_closing_char(ch) {
                continue;
            }

            let last_in_stack = stack[stack.len() - 1];

            let expected = get_opening_char(ch);

            if last_in_stack != expected {
                continue 'lines;
            }

            stack.pop();
        }

        for stack_ch in stack.into_iter().rev() {
            score *= 5;
            score += get_score(stack_ch)
        }

        scores.push(score);
    }

    scores.sort_unstable();
    scores[scores.len() / 2]
}
