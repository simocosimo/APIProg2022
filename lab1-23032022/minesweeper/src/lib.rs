pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut res = Vec::new();
    for x in 0..minefield.len() {
        let mut to_add = String::new();
        for y in 0..minefield[x].len() {
            let c = minefield[x].as_bytes().iter().nth(y).unwrap();
            if *c as char == '*' { to_add.push(*c as char); }
            else { to_add.push_str(&examine_neighbors(minefield, x as i32, y as i32)); }
        }
        res.push(to_add);
    }
    res
}

fn is_on_map(x: i32, y: i32, cols: i32, rows: i32) -> bool { x >= 0 && y >= 0 && x < cols && y < rows }

fn examine_neighbors(minefield: &[&str], x: i32, y: i32) -> String {
    let mut count: i32 = 0;
    let cols = minefield.len();
    let rows = minefield[0].len();
    for xx in -1i32..2 {
        for yy in -1i32..2 {
            if xx == 0 && yy == 0 { continue; }
            if is_on_map(x + xx , y + yy, cols as i32, rows as i32) &&
                minefield[(x + xx) as usize].as_bytes().iter().nth((y + yy) as usize).unwrap() == &('*' as u8) {
                count += 1;
            }
        }
    }
    if count.to_string() == "0" {" ".to_string()} else {count.to_string()}
}