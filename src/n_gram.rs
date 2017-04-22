#[allow(dead_code)]
pub fn by_word(n: i32, sentence: &str) -> Vec<Vec<String>> {
    let words = sentence.split(' ')
        .map(|word| word.to_string().to_owned())
        .filter(|word| !word.is_empty())
        .collect::<Vec<String>>();
    let len = words.len();
    let mut done = false;
    let mut begin = 0 as usize;
    let mut end = n as usize;
    let step = 1 as usize;
    let mut res: Vec<Vec<String>> = Vec::new();
    while !done {
        res.push(words[begin..end].to_vec());
        begin += step;
        end += step;
        if end > len {
            done = true;
        }
    }
    res
}

#[allow(dead_code)]
pub fn by_str(n: i32, sentence: &str) -> Vec<String> {
    let strs = sentence.replace(" ", "").to_string();
    let len = strs.len();
    let mut done = false;
    let mut begin = 0 as usize;
    let mut end = n as usize;
    let step = 1 as usize;
    let mut res: Vec<String> = Vec::new();
    while !done {
        res.push(strs[begin..end].to_string());
        begin += step;
        end += step;
        if end > len {
            done = true;
        }
    }
    res
}
