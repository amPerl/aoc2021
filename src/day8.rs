use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<HashSet<char>>> {
    input
        .split_whitespace()
        .filter(|&x| x != "|")
        .map(|number| number.chars().collect::<HashSet<char>>())
        .chunks(14)
        .into_iter()
        .map(|line_ch| line_ch.collect::<Vec<HashSet<char>>>())
        .collect()
}

pub fn part1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|line| {
            line.iter()
                .skip(10)
                .filter(|x| [2, 3, 4, 7].contains(&x.len()))
                .count()
        })
        .sum()
}

fn count_frequencies(collection: &[HashSet<char>]) -> HashMap<char, usize> {
    let mut char_counts: HashMap<char, usize> = HashMap::new();
    for &ch in collection.iter().flat_map(|pattern| pattern.iter()) {
        *char_counts.entry(ch).or_default() += 1;
    }
    char_counts
}

pub fn part2(input: &str) -> usize {
    let input_lines = parse_input(input);

    let digit_map = {
        let mut tmp = HashMap::new();
        tmp.insert("abcefg", '0');
        tmp.insert("cf", '1');
        tmp.insert("acdeg", '2');
        tmp.insert("acdfg", '3');
        tmp.insert("bcdf", '4');
        tmp.insert("abdfg", '5');
        tmp.insert("abdefg", '6');
        tmp.insert("acf", '7');
        tmp.insert("abcdefg", '8');
        tmp.insert("abcdfg", '9');
        tmp
    };

    let mut output_sum = 0;

    for line in input_lines {
        let seed = &line[0..10];
        let output = &line[10..14];

        let patterns_by_length = {
            let mut tmp: HashMap<usize, Vec<HashSet<char>>> = HashMap::new();
            for pattern in seed.iter() {
                let entry = tmp.entry(pattern.len()).or_default();
                entry.push(pattern.clone());
            }
            tmp
        };

        // 1 =   c  f 	[len 2, cf]
        // 7 = a c  f 	[len 3, acf]
        // 4 =  bcd f 	[len 4, bcdf]
        // 2 = a cde g	[len 5, acdeg]
        // 3 = a cd fg	[len 5, acdfg]
        // 5 = ab d fg	[len 5, abdfg]
        // 0 = abc efg	[len 6, abcefg]
        // 6 = ab defg	[len 6, abdefg]
        // 9 = abcd fg	[len 6, abcdfg]
        // 8 = abcdefg	[len 7, abcdefg]

        // a: 8 times
        // b: 6 times, unique count!
        // c: 8 times
        // d: 7 times
        // e: 4 times, unique count!
        // f: 9 times
        // g: 7 times

        let char_counts = count_frequencies(seed);

        let &scrambled_e = char_counts
            .iter()
            .find(|(_ch, &count)| count == 4)
            .unwrap()
            .0;

        let &scrambled_b = char_counts
            .iter()
            .find(|(_ch, &count)| count == 6)
            .unwrap()
            .0;

        let one_cf = patterns_by_length.get(&2).unwrap()[0].clone();
        let seven_acf = patterns_by_length.get(&3).unwrap()[0].clone();

        let &scrambled_a = seven_acf.difference(&one_cf).next().unwrap();

        let four_bcdf = patterns_by_length.get(&4).unwrap()[0].clone();
        let scrambled_bd = four_bcdf
            .difference(&one_cf)
            .into_iter()
            .cloned()
            .collect::<HashSet<_>>();

        let scrambled_d = {
            let mut tmp = scrambled_bd.clone();
            tmp.remove(&scrambled_b);
            *tmp.iter().next().unwrap()
        };

        let scrambled_aebd = {
            let mut set = HashSet::new();
            set.insert(scrambled_a);
            set.insert(scrambled_e);
            set.insert(scrambled_b);
            set.insert(scrambled_d);
            set
        };

        let without_aebd = seed
            .iter()
            .filter(|x| [5, 6].contains(&x.len()))
            .map(|x| {
                x.difference(&scrambled_aebd)
                    .into_iter()
                    .cloned()
                    .collect::<HashSet<_>>()
            })
            .collect::<Vec<_>>();

        // without aebd:
        // 2 =   c   g	[len 5, acdeg]
        // 3 =   c  fg	[len 5, acdfg]
        // 5 =      fg	[len 5, abdfg]
        //
        // 0 =   c  fg	[len 6, abcefg]
        // 6 =      fg	[len 6, abdefg]
        // 9 =   c  fg	[len 6, abcdfg]
        //
        // c: 4 times, unique count!
        // f: 5 times, unique count!
        // g: 6 times, unique count!

        let without_aebd_counts = count_frequencies(&without_aebd);

        let &scrambled_c = without_aebd_counts
            .iter()
            .find(|(_ch, &count)| count == 4)
            .unwrap()
            .0;

        let &scrambled_f = without_aebd_counts
            .iter()
            .find(|(_ch, &count)| count == 5)
            .unwrap()
            .0;

        let &scrambled_g = without_aebd_counts
            .iter()
            .find(|(_ch, &count)| count == 6)
            .unwrap()
            .0;

        let translation_map = {
            let mut tmp = HashMap::new();
            tmp.insert(scrambled_a, 'a');
            tmp.insert(scrambled_b, 'b');
            tmp.insert(scrambled_c, 'c');
            tmp.insert(scrambled_d, 'd');
            tmp.insert(scrambled_e, 'e');
            tmp.insert(scrambled_f, 'f');
            tmp.insert(scrambled_g, 'g');
            tmp
        };

        let output_value = output
            .iter()
            .map(|set| {
                set.iter()
                    .map(|x| translation_map.get(x).unwrap())
                    .sorted()
                    .collect::<String>()
            })
            .map(|translated_pattern| digit_map.get(translated_pattern.as_str()).unwrap())
            .collect::<String>()
            .parse::<usize>()
            .unwrap();

        output_sum += output_value;
    }

    output_sum
}
