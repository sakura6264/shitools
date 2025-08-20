use std::io::Read;
const PNG_MAGIC: &[u8; 8] = b"\x89PNG\r\n\x1a\n";

pub fn polyglot(png: &[u8], content: &[u8], fix_zip: bool) -> Result<Vec<u8>, String> {
    // Create reader for PNG data
    let mut png_in = std::io::Cursor::new(png);

    // Create writer for output
    let mut out = Vec::new();

    // Check the PNG magic is present in the input file
    let mut png_header = [0; PNG_MAGIC.len()];
    png_in
        .read_exact(&mut png_header)
        .map_err(|e| format!("Failed to read PNG header: {}", e))?;

    if png_header != *PNG_MAGIC {
        return Err("Input file is not a valid PNG (incorrect magic bytes)".to_string());
    }

    // Write PNG header to output
    out.extend_from_slice(&png_header);

    let mut idat_body = Vec::new();

    let mut width = 0;
    let mut height = 0;

    // iterate through the chunks of the PNG file
    loop {
        // parse a chunk
        let mut chunk_len_bytes = [0; 4];
        png_in
            .read_exact(&mut chunk_len_bytes)
            .map_err(|e| format!("Failed to read chunk length: {}", e))?;
        let chunk_len = u32::from_be_bytes(chunk_len_bytes) as usize;
        let mut chunk_type = [0; 4];
        png_in
            .read_exact(&mut chunk_type)
            .map_err(|e| format!("Failed to read chunk type: {}", e))?;
        let mut chunk_body = vec![0; chunk_len];
        png_in
            .read_exact(&mut chunk_body)
            .map_err(|e| format!("Failed to read chunk body: {}", e))?;
        let mut chunk_csum_bytes = [0; 4];
        png_in
            .read_exact(&mut chunk_csum_bytes)
            .map_err(|e| format!("Failed to read chunk checksum: {}", e))?;

        // if it's a non-essential chunk, skip over it
        let essential_chunk_type_bytes = [b"IHDR", b"PLTE", b"IDAT", b"IEND"];
        let is_essential = essential_chunk_type_bytes.iter().any(|&x| *x == chunk_type);
        if !is_essential {
            continue;
        }

        // take note of the image width and height, for future calculations
        if &chunk_type == b"IHDR" {
            width = u32::from_be_bytes([chunk_body[0], chunk_body[1], chunk_body[2], chunk_body[3]])
                as usize;
            height =
                u32::from_be_bytes([chunk_body[4], chunk_body[5], chunk_body[6], chunk_body[7]])
                    as usize;
        }

        // There might be multiple IDAT chunks, we will concatenate their contents
        // and write them into a single chunk later
        if &chunk_type == b"IDAT" {
            idat_body.extend_from_slice(&chunk_body);
            continue;
        }

        // the IEND chunk should be at the end, now is the time to write our IDAT
        // chunk, before we actually write the IEND chunk
        if &chunk_type == b"IEND" {
            let start_offset = out.len() + 8 + idat_body.len();

            // concatenate our content that we want to embed
            idat_body.extend_from_slice(content);

            if idat_body.len() > width * height {
                return Err(format!(
                    "Content too large to embed: {} bytes exceeds capacity of {} bytes ({}x{})",
                    idat_body.len(),
                    width * height,
                    width,
                    height
                ));
            }

            // if its a zip file, fix the offsets
            if fix_zip {
                idat_body = fixup_zip(&idat_body, start_offset)?;
            }

            // write the IDAT chunk
            out.extend_from_slice(&(idat_body.len() as u32).to_be_bytes());
            out.extend_from_slice(b"IDAT");
            out.extend_from_slice(&idat_body);
            let data_2_crc = [&b"IDAT"[..], &idat_body[..]].concat();
            let mut crc32_calc = flate2::Crc::new();
            crc32_calc.update(&data_2_crc);
            let crc = crc32_calc.sum();
            out.extend_from_slice(&crc.to_be_bytes());
        }
        // if we reached here, we're writing the IHDR, PLTE or IEND chunk
        out.extend_from_slice(&chunk_len_bytes);
        out.extend_from_slice(&chunk_type);
        out.extend_from_slice(&chunk_body);
        out.extend_from_slice(&chunk_csum_bytes);
        if &chunk_type == b"IEND" {
            // done
            break;
        }
    }
    Ok(out)
}

fn fixup_zip(data: &[u8], offset: usize) -> Result<Vec<u8>, String> {
    let mut data_copy = data.to_vec();

    // Find the "end of central directory" marker
    let end_central_dir_offset = rfind_index(data, b"PK\x05\x06")
        .ok_or("Could not find end of central directory marker in ZIP data")?;

    // Validate that we have enough data to process
    if end_central_dir_offset + 22 > data.len() {
        return Err(format!(
            "ZIP data is truncated: end of central directory at offset {} but data length is {}",
            end_central_dir_offset,
            data.len()
        ));
    }

    // adjust comment length so that any trailing data (i.e. PNG IEND)
    // is part of the comment
    let comment_length = (data_copy.len() - end_central_dir_offset) - 22 + 0x10;
    let cl_range_start = end_central_dir_offset + 20;
    let cl_range_end = end_central_dir_offset + 22;
    data_copy[cl_range_start..cl_range_end].copy_from_slice(&(comment_length as u16).to_le_bytes());

    // Find the number of central directory entries
    let cdent_count = u16::from_le_bytes([
        data_copy[end_central_dir_offset + 10],
        data_copy[end_central_dir_offset + 11],
    ]) as usize;

    if cdent_count == 0 {
        return Err("ZIP file contains no entries".to_string());
    }

    // find the offset of the central directory entries, and fix it
    let cd_range_start = end_central_dir_offset + 16;
    let mut central_dir_start_offset = u32::from_le_bytes([
        data_copy[cd_range_start],
        data_copy[cd_range_start + 1],
        data_copy[cd_range_start + 2],
        data_copy[cd_range_start + 3],
    ]) as usize;

    // iterate over the central directory entries
    for i in 0..cdent_count {
        central_dir_start_offset =
            find_index_from(&data_copy, b"PK\x01\x02", central_dir_start_offset).ok_or(format!(
                "Could not find central directory entry {} of {}",
                i + 1,
                cdent_count
            ))?;

        // Validate that we have enough data to process this entry
        if central_dir_start_offset + 46 > data_copy.len() {
            return Err(format!(
                "ZIP central directory entry {} is truncated",
                i + 1
            ));
        }

        // fix the offset that points to the local file header
        let off_range_start = central_dir_start_offset + 42;
        let off = u32::from_le_bytes([
            data_copy[off_range_start],
            data_copy[off_range_start + 1],
            data_copy[off_range_start + 2],
            data_copy[off_range_start + 3],
        ]) as usize;
        data_copy[off_range_start..off_range_start + 4]
            .copy_from_slice(&((off + offset) as u32).to_le_bytes());
        central_dir_start_offset += 1;
    }
    Ok(data_copy)
}

/// Finds the first occurrence of a byte pattern in a slice, starting from a given index
///
/// This function uses a sliding window approach to find the pattern, which is more efficient
/// than checking every possible position with starts_with.
///
/// # Arguments
/// * `data` - The data to search in
/// * `pattern` - The pattern to search for
/// * `from_index` - The index to start searching from
///
/// # Returns
/// * `Option<usize>` - The index of the first occurrence, or None if not found
fn find_index_from(data: &[u8], pattern: &[u8], from_index: usize) -> Option<usize> {
    // Handle edge cases
    if pattern.is_empty() {
        return Some(from_index);
    }

    if pattern.len() > data.len() || from_index > data.len() - pattern.len() {
        return None;
    }

    // Use memchr for single-byte patterns for better performance
    if pattern.len() == 1 {
        return memchr::memchr(pattern[0], &data[from_index..]).map(|idx| idx + from_index);
    }

    // For multi-byte patterns, use a more efficient search algorithm
    // that doesn't check every position with starts_with
    let end = data.len() - pattern.len() + 1;
    let mut i = from_index;

    while i < end {
        if data[i] == pattern[0] && data[i..i + pattern.len()] == pattern[..] {
            return Some(i);
        }
        i += 1;
    }

    None
}

/// Finds the last occurrence of a byte pattern in a slice
///
/// This function searches from the end of the data toward the beginning,
/// returning the position of the last occurrence of the pattern.
///
/// # Arguments
/// * `data` - The data to search in
/// * `pattern` - The pattern to search for
///
/// # Returns
/// * `Option<usize>` - The index of the last occurrence, or None if not found
fn rfind_index(data: &[u8], pattern: &[u8]) -> Option<usize> {
    // Handle edge cases
    if pattern.is_empty() {
        return Some(data.len());
    }

    if pattern.len() > data.len() {
        return None;
    }

    // Use memrchr for single-byte patterns for better performance
    if pattern.len() == 1 {
        return memchr::memrchr(pattern[0], data).filter(|&idx| data[idx..].starts_with(pattern));
    }

    // For multi-byte patterns, search from the end
    let mut i = data.len() - pattern.len();

    loop {
        if data[i] == pattern[0] && data[i..i + pattern.len()] == pattern[..] {
            return Some(i);
        }

        if i == 0 {
            break;
        }

        i -= 1;
    }

    None
}
