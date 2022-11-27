use itertools::Itertools; 
pub fn to_pig_latin(s: &str) -> String {
    s.split_whitespace().map(pigify_word).join(" ")
}

fn pigify_word(word: &str) -> String {
    let vovwels = String::from("aioue");
    if vovwels.contains(word.chars().next().unwrap()) {
        format!("{}-hay", word)
    } else {
        format!("{}-{}ay",word.chars().skip(1).collect::<String>(), word.chars().next().unwrap())
    }
}