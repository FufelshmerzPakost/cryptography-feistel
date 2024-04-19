
fn permute_word(mut word: Vec<u8>, key: usize) -> Vec<u8>{
    for i in 0..word.len() {
        let ln = word.len();
        word.swap(i, (i + key) % ln);
    }
    word
}

fn keys_gen(key: &str, decrypt: bool, rounds: u8) -> Vec<Vec<u8>>{
    let mut res:Vec<Vec<u8>> = Vec::new();
    for i in 0..rounds {
        res.push(permute_word(key.as_bytes().to_vec(), (i+2) as usize));
    }
    if decrypt {res.reverse()}
    res
}

fn word_slice_gen(word: &str) -> Vec<Vec<u8>> {
    let mut res: Vec<Vec<u8>> = Vec::new();
    let mut format_str = word.as_bytes().to_vec(); 
    while format_str.len() % 4 != 0 {
        format_str.push(0);
    }
    for i in 0..(format_str.len()/4) {
        let cut:Vec<u8>  = format_str[0+4*i..4+4*i].to_vec();
        res.push(cut);
    }
    res
}

fn f(mut right: Vec<u8>, key: Vec<u8>) -> Vec<u8>{
    for i in 0..right.len() {
        right[i] ^= &key[i];
        right[i] = !right[i];
        right[i] <<= 1;
    }
    right
}

fn crypt_round(mut left: Vec<u8>, right: Vec<u8>, key: Vec<u8>) -> Vec<Vec<u8>>{
    for i in 0..left.len() {
        let res_f = f(right.clone(), key.clone());
        left[i] = left[i] ^ res_f[i];
    }
    vec![right, left]
}

fn crypt_tool(word: &str, key: &str, decrypt: bool) {
    let keys = keys_gen(key, decrypt, 12);
    let word_slices = word_slice_gen(word);
    for i in 0..word_slices.len(){
        
    }
}   

fn main() {
    let word = "boykostansilav";
    let key = "lock";
    let res = word_slice_gen(word);
    let k = keys_gen(key, true, 3);
    println!("{:?}", res);
    println!("{:?}", k);
}
