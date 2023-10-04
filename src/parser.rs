use super::util;
use std::path::Path;

type ParsedResult = Result<(), Box<dyn std::error::Error>>;

pub fn parse_pdf(path: &Path) {
    todo!("not implemented yet")
}

pub fn parse_xml<P>(path: &Path, mut cb: P) -> ParsedResult
where
    P: FnMut(Vec<char>),
{
    match util::read_entire_file(path) {
        Ok(content) => {
            let content = content.chars().collect::<Vec<_>>();
            cb(content);
            Ok(())
        }

        Err(err) => {
            eprintln!("{:?}", err);
            Err(err)
        }
    }
}
