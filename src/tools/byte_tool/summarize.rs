const U8ALL: usize = u8::MAX as usize + 1;

fn count(input: &[u8]) -> [usize; U8ALL] {
    let mut buff = [0usize; U8ALL];
    for i in input {
        buff[*i as usize] += 1;
    }
    return buff;
}

fn max(input: &[usize]) -> usize {
    let mut max = 0;
    for i in 0..input.len() {
        if input[i] > input[max] {
            max = i;
        }
    }
    return max;
}

pub fn find_max_used(input: &[u8]) -> u8 {
    let buff = count(input);
    return max(&buff) as u8;
}

pub fn find_num(input: &[u8], b: u8) -> usize {
    let buff = count(input);
    return buff[b as usize];
}

pub fn calc_00(input: &[u8]) -> usize {
    let mut count = 0;
    for i in input {
        if *i == 0 {
            count += 1;
        }
    }
    return count;
}

pub fn calc_ff(input: &[u8]) -> usize {
    let mut count = 0;
    for i in input {
        if *i == 0xff {
            count += 1;
        }
    }
    return count;
}

pub fn xor(input: &[u8], key: &[u8]) -> String {
    let mut buff = Vec::new();
    for i in 0..input.len() {
        buff.push(input[i] ^ key[i % key.len()]);
    }
    return hex::encode(buff);
}

pub fn format_guess(input: &[u8]) -> String {
    let fmt = file_format::FileFormat::from_bytes(input);
    let shortname = match fmt.short_name() {
        Some(name) => name,
        None => "unknown",
    };
    return format!(
        "{}\nshort: {}\nmedia type: {}\nextension: {}",
        fmt,
        shortname,
        fmt.media_type(),
        fmt.extension()
    );
}
