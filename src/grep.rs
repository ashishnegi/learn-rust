use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

struct Res<'a> {
    line_no: u32,
    line: String
}

fn search_contents<'a>(contents: &'a str, search: &str) -> Vec<Res<'a>> {
    let mut res = vec![];
    let mut line_no = 0;
    for line in contents.lines() {
        line_no = line_no + 1;
        if line.contains(search) {
            res.push(Res{
                line_no: line_no,
                line: line
            });
        }
    }
    res
}

fn grep<'a>(filename: &str, search: &str) -> Result<Vec<Res<'a>>, Box<Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(search_contents(contents.as_str(), search))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grep_1() {
        match grep("src/grep.rs", "use std::fs::") {
            Ok(res) => {
                assert_eq!(2, res.len());
                assert_eq!(1, res[0].line_no);
            },
            _ => assert!(false)
        }
    }
}
