use std::{fs::File, str::Chars, collections::HashMap};
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Trie {
    root: TrieNode
}

#[derive(Debug)]
struct TrieNode {
    letter: Option<char>,
    next: HashMap<char, TrieNode>
}

impl Trie {
    fn new() -> Trie {
        Trie { root: TrieNode::new_root() }
    }

    fn insert(&mut self, string: String) -> () {
        let mut current_node = &mut self.root;
        let chars = string.chars();

        for char in chars {
            if current_node.next_char_exists(char) {
                current_node = current_node.next.get_mut(&char).unwrap();
            } else {
                current_node.next.insert(char, TrieNode::new(char));
                current_node.next.get_mut(&char).unwrap();
            }
        }
    }

    fn contains(&mut self, string: String) -> bool {
        let mut current_node = &mut self.root;
        let chars = string.chars();

        for char in chars {
            if current_node.next_char_exists(char) {
                current_node = current_node.next.get_mut(&char).unwrap();
            } else {
                return false;
            }
        }

        return true;
    }
}

impl TrieNode {
    fn new_root() -> TrieNode {
        TrieNode { letter: None, next: HashMap::new() }
    }

    fn new(letter: char) -> TrieNode {
        TrieNode { letter: Some(letter), next: HashMap::new() }
    }

    fn check_value(self, letter: char) -> bool {
        self.letter == Some(letter)
    }

    fn next_char_exists(&self, letter: char) -> bool {
        self.next.contains_key(&letter)
    }

    // fn child_for_letter(self, letter: char) -> TrieNode {
    //     self.next.get(&letter).unwrap_or(TrieNode::new_root())
    // }
}

pub fn read_input<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut trie = Trie::new();

    let lines = read_input("english-words/words_alpha.txt");
    for line in lines.unwrap().map(|line| line.unwrap()).filter(|line| line.len() == 5) {
        trie.insert(line);
    }
    
    println!("{:?}", trie.contains("steal".to_string()));
}
