use std::io::Read;
const PNG_MAGIC: &[u8; 8] = b"\x89PNG\r\n\x1a\n";

pub fn polyglot(png: &[u8], content: &[u8], fix_zip: bool) -> Result<Vec<u8>, String> {
    // create reader for data
    let mut png_in = std::io::Cursor::new(png);
    // create writer for output
    let mut out = Vec::new();
    // check the PNG magic is present in the input file, and write it to the output file
    let mut png_header = [0; PNG_MAGIC.len()];
    png_in.read_exact(&mut png_header).map_err(|e| e.to_string())?;
    if png_header != *PNG_MAGIC {
        return Err("Input file is not a PNG".to_string());
    }
    out.extend_from_slice(&png_header);

    let mut idat_body = Vec::new();

    let mut width = 0;
    let mut height = 0;

    // iterate through the chunks of the PNG file
    loop {
        // parse a chunk
        let mut chunk_len_bytes = [0; 4];
        png_in.read_exact(&mut chunk_len_bytes).map_err(|e| e.to_string())?;
        let chunk_len = u32::from_be_bytes(chunk_len_bytes) as usize;
        let mut chunk_type = [0; 4];
        png_in.read_exact(&mut chunk_type).map_err(|e| e.to_string())?;
        let mut chunk_body = vec![0; chunk_len];
        png_in.read_exact(&mut chunk_body).map_err(|e| e.to_string())?;
        let mut chunk_csum_bytes = [0; 4];
        png_in.read_exact(&mut chunk_csum_bytes).map_err(|e| e.to_string())?;

        // if it's a non-essential chunk, skip over it
        let essential_chunk_type_bytes = [b"IHDR", b"PLTE", b"IDAT", b"IEND"];
        let is_essential = essential_chunk_type_bytes.iter().any(|&x| *x == chunk_type);
        if !is_essential {
            continue;
        }

        // take note of the image width and height, for future calculations
        if &chunk_type == b"IHDR" {
            width = u32::from_be_bytes([chunk_body[0], chunk_body[1], chunk_body[2], chunk_body[3]]) as usize;
            height = u32::from_be_bytes([chunk_body[4], chunk_body[5], chunk_body[6], chunk_body[7]]) as usize;
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
                return Err("Content too large to embed".to_string());
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
    // find the "end of central directory" marker
    let end_central_dir_offset =
        rfind_index(data, b"PK\x05\x06").ok_or("Could not find end of central directory")?;

    // adjust comment length so that any trailing data (i.e. PNG IEND)
    // is part of the comment
    let comment_length = (data_copy.len() - end_central_dir_offset) - 22 + 0x10;
    let cl_range_start = end_central_dir_offset + 20;
    let cl_range_end = end_central_dir_offset + 22;
    data_copy[cl_range_start..cl_range_end].copy_from_slice(&(comment_length as u16).to_le_bytes());

    // find the number of central directory entries
    let cdent_count = u16::from_le_bytes([
        data_copy[end_central_dir_offset + 10],
        data_copy[end_central_dir_offset + 11],
    ]) as usize;

    // find the offset of the central directory entries, and fix it
    let cd_range_start = end_central_dir_offset + 16;
    let mut central_dir_start_offset = u32::from_le_bytes([
        data_copy[cd_range_start],
        data_copy[cd_range_start + 1],
        data_copy[cd_range_start + 2],
        data_copy[cd_range_start + 3],
    ]) as usize;

    // iterate over the central directory entries
    for _ in 0..cdent_count {
        central_dir_start_offset = find_index_from(&data_copy, b"PK\x01\x02", central_dir_start_offset)
            .ok_or("Could not find central directory entry")?;

        // fix the offset that points to the local file header
        let off_range_start = central_dir_start_offset + 42;
        let off = u32::from_le_bytes([
            data_copy[off_range_start],
            data_copy[off_range_start + 1],
            data_copy[off_range_start + 2],
            data_copy[off_range_start + 3],
        ]) as usize;
        data_copy[off_range_start..off_range_start + 4].copy_from_slice(&((off + offset) as u32).to_le_bytes());
        central_dir_start_offset += 1;
    }
    Ok(data_copy)
}

fn find_index_from(data: &[u8], pattern: &[u8], from_index: usize) -> Option<usize> {
    for i in from_index..data.len() {
        if data[i..].starts_with(pattern) {
            return Some(i);
        }
    }
    None
}

fn rfind_index(data: &[u8], pattern: &[u8]) -> Option<usize> {
    for i in (0..data.len()).rev() {
        if data[i..].starts_with(pattern) {
            return Some(i);
        }
    }
    None
}
