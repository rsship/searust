pub mod utils {
    use serde_json;
    use std::collections::HashMap;
    use std::error::Error;
    use std::fs;
    use std::fs::{DirEntry, File};
    use std::io::{self, BufReader};
    use std::path::{Path, PathBuf};
    use xml::reader::{EventReader, XmlEvent};

    const JSON_FILE_PATH: &str = "index.json";

    pub type TermFreq = HashMap<String, usize>;
    pub type TermFreqIndex = HashMap<PathBuf, TermFreq>;

    pub fn read_entire_file<P: AsRef<Path>>(file_path: P) -> Result<String, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let er = EventReader::new(BufReader::new(file));
        let mut content = String::new();

        for event in er.into_iter() {
            if let XmlEvent::Characters(event) = event? {
                content.push_str(&event);
            }
        }
        Ok(content)
    }

    pub fn write_tf_to_file(
        tf_index: TermFreqIndex,
        pretty: Option<bool>,
    ) -> Result<(), Box<dyn Error>> {
        println!("Saving to JSON");

        let file = File::create(JSON_FILE_PATH)?;
        let pretty = pretty.unwrap_or(false);
        if pretty {
            serde_json::to_writer_pretty(file, &tf_index)?;
        } else {
            serde_json::to_writer(file, &tf_index)?;
        }
        Ok(())
    }

    pub fn walk_dir(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                let first_char = path
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap();

                if path.is_dir() {
                    if first_char == '.' {
                        continue;
                    }
                    walk_dir(&path, cb)?;
                } else {
                    cb(&entry);
                }
            }
        }
        Ok(())
    }
}
