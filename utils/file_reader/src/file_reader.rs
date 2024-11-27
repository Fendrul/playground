use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::PathBuf;

pub struct FileReader {
    reader: BufReader<File>,
}

impl FileReader {
    /// Constructs a new `FileReader`.
    ///
    /// # Arguments
    ///
    /// * `path_to_file_string` - A string slice that holds the path to the file.
    ///
    /// # Returns
    ///
    /// * `FileReader` - A new `FileReader` instance.
    ///
    /// # Errors
    ///
    /// This function will return an error if the file cannot be opened.
    pub fn new(path_to_file_string: &str) -> Result<FileReader, std::io::Error> {
        let path_to_project: PathBuf = get_path_to_project();
        let path_to_file = path_to_project.join(path_to_file_string);

        let file = File::open(path_to_file)?;

        let buf_reader = BufReader::new(file.try_clone()?);

        Ok(FileReader { reader: buf_reader })
    }

    /// Reads a line from the file.
    ///
    /// # Returns
    ///
    /// * `Option<String>` - The line read from the file, or `None` if the end of the file has been reached or an error occurred.
    pub fn read_line(&mut self) -> Option<String> {
        let mut line = String::new();

        match self.reader.read_line(&mut line) {
            Ok(0) | Err(_) => None,
            Ok(_) => Some(line),
        }
    }
    
    pub fn read_file(path: &str) -> Result<String, std::io::Error> {
        let path_to_project: PathBuf = get_path_to_project();
        let path_to_file = path_to_project.join(path);

        let file = File::open(path_to_file)?;
        let mut buf_reader = BufReader::new(file);

        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;

        Ok(contents)
    }
}

impl Iterator for FileReader {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.read_line()
    }
}

fn get_path_to_project() -> PathBuf {
    let current_dir = std::env::current_dir().expect("Failed to get current directory to the project.");
    let mut new_path = PathBuf::new();

    for component in current_dir.components() {
        match component {
            std::path::Component::Prefix(prefix) => {
                new_path.push(prefix.as_os_str());
            }

            std::path::Component::RootDir => {
                new_path.push(std::path::MAIN_SEPARATOR.to_string());
            }

            std::path::Component::Normal(os_str) => {
                if os_str == "src" {
                    break;
                }

                new_path.push(os_str);
            }

            _ => {}
        }
    }

    new_path
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use tempfile::tempdir;

    use super::*;

    #[test]
    fn reads_lines_from_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello\nWorld").unwrap();

        let mut file_reader = FileReader::new(file_path.to_str().unwrap()).unwrap();
        assert_eq!(file_reader.next(), Some("Hello\n".to_string()));
        assert_eq!(file_reader.next(), Some("World\n".to_string()));
        assert_eq!(file_reader.next(), None);
    }

    #[test]
    fn return_err_when_path_does_not_exist() {
        let file_reader = FileReader::new("non_existent_file.txt");
        
        assert!(file_reader.is_err());
    }

    #[test]
    fn returns_none_when_file_is_empty() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("empty.txt");
        File::create(&file_path).unwrap();

        let mut file_reader = FileReader::new(file_path.to_str().unwrap()).unwrap();
        assert_eq!(file_reader.next(), None);
    }
}