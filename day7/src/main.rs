use std::{rc::{Rc, Weak}, cell::RefCell, iter::Peekable, str::Lines, vec, fs, borrow::Cow};

use ptree::{TreeItem, print_tree};

#[derive(Clone)]
struct StorageItem {
    parent: Weak<RefCell<StorageItem>>,
    name: String,
    size: usize,
    children: Option<RefCell<Vec<Rc<RefCell<StorageItem>>>>>
}

impl StorageItem {
    fn add_directory(base: &Rc<RefCell<StorageItem>>, dir_name: &str) {
        if base.borrow().children.is_none() {
            panic!("Trying to add directory to file.");
        }

        if let Some(ref children) = base.borrow().children {
            let new_directory = StorageItem {
                parent: Rc::downgrade(&base),
                name: dir_name.to_string(),
                size: 0,
                children: Some(RefCell::new(vec![]))
            };

            children.borrow_mut().push(Rc::new(RefCell::new(new_directory)));
        }
    }

    fn add_file(base: &Rc<RefCell<StorageItem>>, file_name: &str, file_size: &str) {
        if base.borrow().children.is_none() {
            panic!("Trying to add file to file.");
        }
        let mut base_mut_borrowed = base.borrow_mut();
        if let Some(ref children) = base_mut_borrowed.children {
            let file_size = file_size.parse().unwrap();

            let new_directory = StorageItem {
                parent: Rc::downgrade(&base),
                name: file_name.to_string(),
                size: file_size,
                children: None
            };

            children.borrow_mut().push(Rc::new(RefCell::new(new_directory)));

            // Recursively add the size upwards the tree
            base_mut_borrowed.add_size(file_size);
        }
    }

    fn add_size(&mut self, file_size: usize) {
        // Add size to itself
        self.size += file_size;

        // Add size to parent if it exists
        if let Some(parent) = self.parent.upgrade() {
            parent.borrow_mut().add_size(file_size);
        }
    }
}

impl TreeItem for StorageItem {
    type Child = Self;

    fn write_self<W: std::io::Write>(&self, f: &mut W, _: &ptree::Style) -> std::io::Result<()> {
        if self.children.is_some() {
            // Directory
            write!(f, "dir {:?} ({:?})", self.name, self.size)
        } else {
            // File
            write!(f, "file {:?} ({:?})", self.name, self.size)
        }
    }

    fn children(&self) -> std::borrow::Cow<[Self::Child]> {
        if let Some(ref children) = self.children {
            // Directory
            let array: Vec<StorageItem> = children.borrow().iter().map(|c| c.borrow().clone()).collect();
            Cow::from(array)
        } else {
            // File
            Cow::from(vec![])
        }
    } 
}

struct StorageReader<'a> {
    loc: Rc<RefCell<StorageItem>>,
    iter: Peekable<Lines<'a>>
}

impl<'a> StorageReader<'a> {
    fn build_and_read(console: String) -> Rc<RefCell<StorageItem>> {
        let root = Rc::new(RefCell::new(StorageItem {
            parent: Weak::new(),
            name: "/".to_string(),
            size: 0,
            children: Some(RefCell::new(vec![]))
        }));

        let mut reader = StorageReader {
            loc: Rc::clone(&root),
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
                "ls" => self.handle_ls(),
                "cd" => self.handle_cd(parts[2]),
                _ => unreachable!()
            }
        }
    }

    /// Handles the lines after a ls command
    /// Once it reaches another command, it breaks 
    fn handle_ls<'b : 'a>(&mut self) {
        while let Some(line) = self.iter.peek() {

            let parts: Vec<_> = line.split_whitespace().collect();

            // Break out of it upon next command
            if parts[0] == "$" { 
                break; 
            }

            self.iter.next();

            match parts[0] {
                "dir" => StorageItem::add_directory(&self.loc, parts[1]),
                filesize => StorageItem::add_file(&self.loc, parts[1], filesize)
            };
        }
    }

    fn handle_cd(&mut self, loc: &str) {
        if loc == ".." {
            let smth = self.loc.borrow().parent.upgrade().unwrap();
            self.loc = smth;
        } else {
            let current_loc = self.loc.to_owned();
            let current_loc = current_loc.borrow();

            if let Some(ref children) = current_loc.children {
                let children = children.borrow();

                for child in children.iter() {
                    if child.borrow().children.is_some() && child.borrow().name == loc {
                        let new_loc = Rc::clone(child);
                        self.loc = new_loc;
                        break;
                    }
                }
            } 
        }
    }
}


fn check_sum_sub_100_directories(item: &StorageItem, sum: &mut usize) {
    if item.children.is_some() {
        // Directory
        if item.size <= 100000 {
            *sum += item.size;
        }

        for child in item.children.as_ref().unwrap().borrow().iter() {
            check_sum_sub_100_directories(&child.borrow(), sum);
        }
    } 
}

fn check_smallest_super_100(item: &StorageItem, min: &mut usize) {
    if item.children.is_some() {
        // Directory
        if item.size >= 3562874 && item.size < *min {
            *min = item.size;
        }

        for child in item.children.as_ref().unwrap().borrow().iter() {
            check_smallest_super_100(&child.borrow(), min);
        }
    } 
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let root = StorageReader::build_and_read(input);

    let root = root.borrow().clone();

    print_tree(&root).unwrap();

    let mut sum = 0;

    check_sum_sub_100_directories(&root, &mut sum);

    println!("{sum}");

    let mut min = usize::MAX;

    check_smallest_super_100(&root, &mut min);

    println!("{min}");
}
