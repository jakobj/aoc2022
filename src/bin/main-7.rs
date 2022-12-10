use std::fs;

struct File {
    name: String,
    size: usize,
}

struct Directory {
    name: String,
    files: Vec<File>,
    children: Vec<Directory>,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            files: Vec::new(),
            children: Vec::new(),
        }
    }

    fn add_subdirectory(&mut self, name: &str) {
        self.children.push(Self::new(name));
    }

    fn add_file(&mut self, name: &str, size: usize) {
        self.files.push(File {
            name: name.to_string(),
            size,
        });
    }

    fn get_subdirectory(&mut self, name: &str) -> &mut Self {
        for dir in self.children.iter_mut() {
            if dir.name == name {
                return dir;
            }
        }
        panic!("Directory not found {} in dir {}", name, self.name);
    }

    fn print(&self, depth: usize) -> String {
        let prefix = (0..depth).map(|_| "  ").collect::<Vec<&str>>().join("");
        let mut s = String::new();
        s.push_str(&format!("{}- {} (dir)\n", prefix, self.name));
        for dir in self.children.iter() {
            s.push_str(&dir.print(depth + 1));
        }
        for file in self.files.iter() {
            s.push_str(&format!(
                "{}  - {} (file, size={})\n",
                prefix, file.name, file.size
            ));
        }
        s
    }
}

fn main() {
    let file_path = "inputs/7.txt";
    let content = fs::read_to_string(file_path).expect("Should be able to read file");
    let root = build_directory_tree(&content);
    println!("{}", root.print(0));

    let mut sizes = Vec::new();
    determine_sizes_of_all_directories(&root, &mut sizes);
    let total_size_small_directories = sizes.iter().filter(|&size| *size < 100_000).sum::<usize>();
    println!("The total size is {}.", total_size_small_directories);

    let total_disk_space = 70_000_000;
    let required_disk_space = 30_000_000;
    let used_disk_space = sizes.iter().max().unwrap();
    let unused_disk_space = total_disk_space - used_disk_space;
    let necessary_to_free = required_disk_space - unused_disk_space;

    let x = sizes
        .iter()
        .filter(|&size| *size > necessary_to_free)
        .min()
        .expect("Should be able to determine directory");
    println!(
        "The small directory that would free up enough space has size {}.",
        x
    );
}

fn build_directory_tree(content: &str) -> Directory {
    let mut lines = content.lines().into_iter();
    let mut root = Directory::new("/");
    let mut current = &mut root;
    let mut path = Vec::new();
    while let Some(l) = lines.next() {
        if is_command(l) {
            let (cmd, arg) = parse_command(l);
            match cmd.as_str() {
                "cd" => {
                    let arg = arg.unwrap();
                    match arg.as_str() {
                        "/" => (),
                        ".." => {
                            path.pop();
                            current = navigate_to_path(&mut root, &path);
                        }
                        _ => {
                            path.push(arg.to_string());
                            current = current.get_subdirectory(&arg);
                        }
                    }
                }
                "ls" => {}
                _ => panic!("Unknown command {}", cmd),
            }
        } else {
            if is_dir(l) {
                let name = parse_directory_name(l);
                current.add_subdirectory(&name);
            } else {
                let (name, size) = parse_filename_and_size(l);
                current.add_file(&name, size);
            }
        }
    }
    root
}

fn is_command(l: &str) -> bool {
    l.chars().next().unwrap() == '$'
}

fn parse_command(l: &str) -> (String, Option<String>) {
    assert!(l.chars().next().unwrap() == '$');
    let l_split = l.split_whitespace().collect::<Vec<&str>>();
    let cmd = l_split[1].to_string();
    let arg;
    if l_split.len() > 2 {
        arg = Some(l_split[2].to_string());
    } else {
        arg = None
    }
    (cmd, arg)
}

fn navigate_to_path<'a>(root: &'a mut Directory, path: &Vec<String>) -> &'a mut Directory {
    let mut dir = root;
    for dir_name in path {
        dir = dir.get_subdirectory(dir_name);
    }
    dir
}

fn is_dir(l: &str) -> bool {
    l.split_whitespace().next().unwrap() == "dir"
}

fn parse_directory_name(l: &str) -> String {
    l.split_whitespace().last().unwrap().to_string()
}

fn parse_filename_and_size(l: &str) -> (String, usize) {
    let l_split = l.split_whitespace().collect::<Vec<&str>>();
    (
        l_split[1].to_string(),
        l_split[0].parse().expect("Should be able to parse size"),
    )
}

fn determine_sizes_of_all_directories(dir: &Directory, sizes: &mut Vec<usize>) -> usize {
    let mut size = 0;
    for subdir in dir.children.iter() {
        size += determine_sizes_of_all_directories(subdir, sizes);
    }
    for file in dir.files.iter() {
        size += file.size;
    }
    sizes.push(size);
    size
}
