use std::{fs, str::Lines, collections::HashMap, rc::{Rc, Weak}, iter::Peekable};

#[derive(PartialEq)]
enum StorageType {
    File,
    Dir
}

struct StorageItem<'a> {
    storageType: StorageType,
    name: String,
    parent: Option<Weak<StorageItem<'a>>>,
    content: Option<HashMap<&'a str, Rc<StorageItem<'a>>>>
}

impl<'a> StorageItem<'a> {
    fn add_directory(base: Rc<StorageItem>, dir: &str) {
        if let Some(content) = &base.content {
            if !content.contains_key(dir) {
                let directory = StorageItem {
                    storageType: StorageType::Dir,
                    name: dir.to_string(),
                    parent: Some(Rc::downgrade(&base)),
                    content: Some(HashMap::new())
                };

                content.insert(dir, Rc::new(directory));
            }
        }
    }

    fn add_file(&mut self, file: &str, filesize: &str) {

    }
}

struct StorageReader<'a> {
    loc: Rc<StorageItem<'a>>,
    storage: &'a StorageItem<'a>,
    iter: Peekable<Lines<'a>>
}

impl<'a> StorageReader<'a> {
    fn build_and_read(console: String) -> Rc<StorageItem<'a>> {
        let root = Rc::new(StorageItem {
            storageType: StorageType::Dir,
            name: "/".to_string(),
            parent: None,
            content: Some(HashMap::new())
        }); 

        let mut reader = StorageReader {
            loc: root,
            storage: &root,
            iter: console.lines().peekable()
        };

        reader.read();

        root
    }

    fn read(&mut self)  {
        while let Some(line) = self.iter.next() {
            let parts: Vec<_> = line.split_whitespace().collect();

            // Assuming at this point we are always running a command starting with $
            match parts[1] {
                "ls" => self.handle_ls(parts[2]),
                "cd" => self.handle_cd(parts[2]),
                _ => unreachable!()
            }
        }
    }

    /// Handles the lines after a ls command
    /// Once it reaches another command, it breaks 
    fn handle_ls<'b : 'a>(&mut self, loc: &'b str) {
        while let Some(&line) = self.iter.peek() {
            let parts: Vec<_> = line.split_whitespace().collect();

            match parts[0] {
                "$" => break,
                "dir" => StorageItem::add_directory(self.loc,parts[1]),
                filesize => self.loc.add_file(parts[1], filesize)
            };
        }
    }

    fn handle_cd<'b : 'a>(&mut self, loc: &'b str) {
        if let Some(content) = &self.loc.content {
            if loc == ".." {
                self.loc = self.loc.parent;
            }

            for (key, item) in content {
                if item.storageType == StorageType::Dir && *key == loc {
                    self.loc = &item;
                }
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let root = StorageReader::build_and_read(input);
}
