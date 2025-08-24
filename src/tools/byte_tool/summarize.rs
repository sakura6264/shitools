use serde::{Deserialize, Serialize};
use std::io::Write;
use std::os::windows::process::CommandExt;

const U8ALL: usize = u8::MAX as usize + 1;
const MAGIKA_FILE: &str = "magika.exe";
const TEMP_FILE: &str = "temp.bin";

fn count(input: &[u8]) -> [usize; U8ALL] {
    let mut buff = [0usize; U8ALL];
    for i in input {
        buff[*i as usize] += 1;
    }
    buff
}

fn max(input: &[usize]) -> usize {
    let mut max = 0;
    for i in 0..input.len() {
        if input[i] > input[max] {
            max = i;
        }
    }
    max
}

pub fn find_max_used(input: &[u8]) -> u8 {
    let buff = count(input);
    max(&buff) as u8
}

pub fn find_num(input: &[u8], b: u8) -> usize {
    let buff = count(input);
    buff[b as usize]
}

pub fn calc_00(input: &[u8]) -> usize {
    let mut count = 0;
    for i in input {
        if *i == 0 {
            count += 1;
        }
    }
    count
}

pub fn calc_ff(input: &[u8]) -> usize {
    let mut count = 0;
    for i in input {
        if *i == 0xff {
            count += 1;
        }
    }
    count
}

pub fn xor(input: &[u8], key: &[u8]) -> String {
    let mut buff = Vec::new();
    for i in 0..input.len() {
        buff.push(input[i] ^ key[i % key.len()]);
    }
    hex::encode(buff)
}

pub fn format_guess(input: &[u8]) -> String {
    let return_val = match magika_wrapper(input) {
        Ok(output) => output,
        Err(err) => {
            format!("Failed: {}", err)
        }
    };
    let temp_path = crate::sub_path(TEMP_FILE).unwrap();
    let _ = std::fs::remove_file(&temp_path);
    return_val
}

fn magika_wrapper(input: &[u8]) -> Result<String, String> {
    let magika_path = crate::sub_path(MAGIKA_FILE)?;
    let temp_path = crate::sub_path(TEMP_FILE)?;

    // Create temporary file
    let mut file = std::fs::File::create(&temp_path)
        .map_err(|err| format!("Failed to create temporary file: {}", err))?;
    file.write_all(input)
        .map_err(|err| format!("Failed to write to temporary file: {}", err))?;
    file.flush()
        .map_err(|err| format!("Failed to flush temporary file: {}", err))?;

    // Run magika.exe with temporary file

    let output = std::process::Command::new(magika_path)
        .arg("--jsonl")
        .arg(&temp_path)
        .stdout(std::process::Stdio::piped())
        .creation_flags(0x08000000) // Set creation flags to hide console window
        .output()
        .map_err(|err| format!("Failed to run magika.exe: {}", err))?;

    if !output.status.success() {
        return Err(format!(
            "magika.exe failed with exit code {}",
            output.status.code().unwrap_or(1)
        ));
    }

    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    magika_json_resolve(output_str)
}

#[derive(Clone, Serialize, Deserialize, Default)]
struct MagikaJson {
    path: String,
    result: MagikaResult,
}

#[derive(Clone, Serialize, Deserialize, Default)]
struct MagikaResult {
    status: String,
    value: MagikaResultValue,
}

#[derive(Clone, Serialize, Deserialize, Default)]
struct MagikaResultValue {
    dl: MagikaPredictedValue,
    output: MagikaPredictedValue,
    score: f64,
}

#[derive(Clone, Serialize, Deserialize, Default)]
struct MagikaPredictedValue {
    description: String,
    extensions: Vec<String>,
    group: String,
    is_text: bool,
    label: String,
    mime_type: String,
}

fn magika_json_resolve(magika_json: String) -> Result<String, String> {
    let json: MagikaJson = serde_json::from_str(&magika_json)
        .map_err(|err| format!("Failed to parse magika.exe output: {}", err))?;
    if json.result.status != "ok" {
        return Err(format!("magika.exe returned error: {}", json.result.status));
    }
    let mut output_buffer = String::with_capacity(magika_json.len());
    output_buffer.push_str("label: ");
    output_buffer.push_str(&json.result.value.output.label);
    output_buffer.push('\n');
    output_buffer.push_str("description: ");
    output_buffer.push_str(&json.result.value.output.description);
    output_buffer.push('\n');
    output_buffer.push_str("group: ");
    output_buffer.push_str(&json.result.value.output.group);
    output_buffer.push('\n');
    output_buffer.push_str("is_text: ");
    if json.result.value.output.is_text {
        output_buffer.push_str("true");
    } else {
        output_buffer.push_str("false");
    }
    output_buffer.push('\n');
    output_buffer.push_str("mime_type: ");
    output_buffer.push_str(&json.result.value.output.mime_type);
    output_buffer.push('\n');
    output_buffer.push_str("extensions: ");
    for ext in &json.result.value.output.extensions {
        output_buffer.push_str(ext);
        output_buffer.push(',');
    }
    output_buffer.pop(); // Remove trailing comma
    output_buffer.push('\n');
    output_buffer.push_str("score: ");
    output_buffer.push_str(&json.result.value.score.to_string());
    output_buffer.shrink_to_fit();
    Ok(output_buffer)
}
