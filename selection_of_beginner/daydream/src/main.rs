use std::io::stdin;

const DREAM: [char; 4]  = ['a', 'e', 'r', 'd',];
const ERASE: [char; 4]  = ['s', 'a', 'r', 'e',];

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input);
    let mut vec: Vec<char> = input.trim().chars().collect();
    let mut chr: char = 'a';
    // println!("{}", vec.pop().unwrap());
    // let DREAM   = ['d', 'r', 'e', 'a', 'm'];
    // let DREAMER = ['d', 'r', 'e', 'a', 'm', 'e', 'r'];
    // let ERASE   = ['e', 'r', 'a', 's'];
    // const ERASER: [char; 4]  = ['e', 'r', 'a', 's', 'e', 'r'];
    loop {
        chr = match vec.pop() {
            Some(chr) => chr,
            None => { println!("YES"); break;},
        };
        if !match chr {
            'r' => check_which_dreamer_or_erase(&mut vec),
            'e' => is_erase(&mut vec),
            'm' => is_dream(&mut vec),
             _   => false,
        } {
            println!("NO");
            break;
        }
    }
}

fn is_erase(vec: &mut Vec<char>) -> bool {
    // println!("in erase");
    check_chars(&ERASE, vec)
}
fn is_dream(vec: &mut Vec<char>) -> bool {
    // println!("in dream");
    check_chars(&DREAM, vec)
}
fn check_which_dreamer_or_erase(vec: &mut Vec<char>) -> bool {
    if vec.pop().unwrap() != 'e' {
        return false
    }
    // println!("e");
    // let v = vec.pop().unwrap();
    // println!("v:{}",v );
    // match v {
    match vec.pop().unwrap() {
        's' => {vec.push('s'); check_chars(&ERASE, vec)},
        'm' => check_chars(&DREAM, vec),
         _  => false,
    }
}
fn check_chars(&chars: &[char; 4], vec: &mut Vec<char>) -> bool {
    for i in 0..4 {
        if chars[i] != vec.pop().unwrap() {
        // let v = vec.pop().unwrap();
        // println!("v:{}, vec.len:{}, chras[i]:{}",v, vec.len(), chars[i]);
        // if chars[i] != v {
            return false
        }
    }
    true
}
