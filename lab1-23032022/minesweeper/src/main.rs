use minesweeper::annotate;

fn main() {
    let res = annotate(&[
        ".*..*.",
        "..*...",
        "....*.",
        "...*.*",
        ".*..*.",
        "......",
    ]);
    println!("{:?}", res);
}