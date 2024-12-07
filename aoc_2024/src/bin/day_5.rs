use aoc_2024::get_single_path_as_arg;

struct Rule {
    first: u32,
    second: u32,
}

struct PageList {
    pages: Vec<u32>,
}

impl PageList {
    fn check_rule(&self, rule: &Rule) -> bool {
        let first = self.pages.iter().position(|&page| page == rule.first);
        let second = self.pages.iter().position(|&page| page == rule.second);

        if first.is_none() || second.is_none() {
            return true;
        }
        first.unwrap() < second.unwrap()
    }

    fn check_rules(&self, rules: &[Rule]) -> bool {
        rules.iter().all(|rule| self.check_rule(rule))
    }

    fn middle_page(&self) -> u32 {
        let middle_index = self.pages.len() / 2;
        *self.pages.get(middle_index).unwrap()
    }
}

struct Input {
    rules: Vec<Rule>,
    page_lists: Vec<PageList>,
}

fn read_input(path: &std::path::Path) -> Input {
    let input = std::fs::read_to_string(path).expect("Failed to read input as string.");

    let mut rules: Vec<Rule> = Vec::new();
    let mut page_lists: Vec<PageList> = Vec::new();

    let mut reading_rules = true;

    for line in input.lines() {
        if line.is_empty() {
            // Empty line signifies end of rules and start of page lists
            reading_rules = false;
        } else if reading_rules {
            // Parse rule
            let numbers: Vec<u32> = line.split('|').map(|ch| ch.parse().unwrap()).collect();
            if numbers.len() != 2 {
                panic!("Found rule with something different than two numbers.")
            }
            rules.push(Rule {
                first: *numbers.first().unwrap(),
                second: *numbers.get(1).unwrap(),
            });
        } else {
            // Parse page list
            let numbers: Vec<u32> = line.split(',').map(|ch| ch.parse().unwrap()).collect();
            if numbers.len() % 2 == 0 {
                panic!("Found even number of pages in a page list.")
            }
            page_lists.push(PageList { pages: numbers });
        }
    }

    Input { rules, page_lists }
}

fn main() {
    // Read input
    let path = get_single_path_as_arg();
    let input = read_input(&path);

    println!(
        "Found {} rules and {} page lists",
        input.rules.len(),
        input.page_lists.len()
    );

    let sum_middle: u32 = input
        .page_lists
        .iter()
        .filter(|&page_list| page_list.check_rules(&input.rules))
        .map(|page_list| page_list.middle_page())
        .sum();

    println!("The answer to the first half is: {}", sum_middle);
}
