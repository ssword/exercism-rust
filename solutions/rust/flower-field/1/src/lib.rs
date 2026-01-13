pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return vec![];
    }

    let width = garden[0].len();
    
    if width == 0 {
        return vec![String::new(); garden.len()];
    }

    let flat = garden.concat();
    let bytes = flat.as_bytes();

    let annotated: String = bytes.iter().enumerate().map(|(i, &byte)| {
        if byte == b'*' { return '*' }

        let (row, col) = (i / width, i % width);

        let count = (-1..=1).flat_map(|dy| (-1..=1).map(move |dx| (dy, dx)))
            .filter(|&(dy, dx)| {
                if dy == 0 && dx == 0 { return false; }
                
                let (r, c) = (row as isize + dy, col as isize + dx);
                
                r >= 0 && c >= 0 && (c as usize) < width &&
                bytes.get((r as usize) * width + (c as usize)) == Some(&b'*')
            })
            .count();

        match count {
            0 => ' ',
            n => char::from_digit(n as u32, 10).unwrap(),
        }
    }).collect();

    annotated.as_bytes()
        .chunks(width)
        .map(|chunk| std::str::from_utf8(chunk).unwrap().to_string())
        .collect()
}