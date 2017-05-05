use std::fs::File;
use std::io::prelude::*;
use std::io::Error;

struct Res {
    line_no: u32,
    line: String
}

fn search_contents(contents: &str, search: &str) -> Vec<Res> {
    let mut res = vec![];
    let mut line_no = 0;
    for line in contents.lines() {
        line_no = line_no + 1;
        if line.contains(search) {
            res.push(Res{
                line_no: line_no,
                line: String::from(line)
            });
        }
    }
    res
}

fn grep(filename: &str, search: &str) -> Result<Vec<Res>, Error> {
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
                assert_eq!(3, res.len());
                assert_eq!(1, res[0].line_no);
                assert_eq!("use std::fs::File;", res[0].line);
            },
            _ => assert!(false)
        }
    }
}
