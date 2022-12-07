use itertools::Itertools;

use super::*;
use std::collections::HashMap;

#[derive(Debug)]
struct File<'a> {
    name: &'a str,
    data: FileData<'a>,
}

impl<'a> File<'a> {
    fn new(name: &'a str, data: FileData<'a>) -> Self {
        Self { name, data }
    }
}

#[derive(Debug)]
enum FileData<'a> {
    Data(u32),
    Directory(Vec<File<'a>>),
}

impl<'a> std::default::Default for FileData<'a> {
    fn default() -> Self {
        Self::Directory(Vec::new())
    }
}

#[derive(Debug)]
struct FileSystem<'a> {
    data: HashMap<String, FileData<'a>>,
    working: Vec<&'a str>,
}

impl<'a> FileSystem<'a> {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
            working: Vec::new(),
        }
    }
    fn do_command(&mut self, line: &'a str) {
        let mut iter = line.split_whitespace().skip(1);
        match iter.next() {
            Some("cd") => {
                match iter.next() {
                    Some("..") => {
                        self.working.pop();
                    }
                    Some(x) => {
                        self.working.push(x); // Assume relative paths only
                    }
                    _ => {
                        unimplemented!()
                    }
                }
            }
            _ => {}
        }
    }
    fn active_dir_mut(&mut self) -> &mut FileData<'a> {
        self.data.entry(self.full_path()).or_default()
    }
    fn full_path(&self) -> String {
        if self.working.len() == 1 {
            "/".to_string()
        } else {
            self.working.join("/")
        }
    }
    fn add_listing(&mut self, line: &'a str) -> Option<()> {
        let (fdata, fname) = line.split_whitespace().collect_tuple()?;
        let fdata = if fdata == "dir" {
            FileData::default()
        } else {
            let size = fdata.parse().ok()?;
            FileData::Data(size)
        };
        let file = File::new(fname, fdata);
        match self.active_dir_mut() {
            FileData::Data(_) => unimplemented!(),
            FileData::Directory(vec) => vec.push(file),
        }
        Some(())
    }
    fn directory_sizes(&self) -> HashMap<String, u32> {
        let mut out = HashMap::new();
        let root_size = self.recursive_walk(&mut out, "/".to_string());
        out.insert("/".to_string(), root_size);
        out
    }
    fn recursive_walk(&self, map: &mut HashMap<String, u32>, goal: String) -> u32 {
        match self.data.get(&goal).unwrap() {
            FileData::Data(_) => unimplemented!(),
            FileData::Directory(vec) => {
                let mut total = 0;
                for f in vec {
                    if let FileData::Data(sz) = f.data {
                        total += sz;
                    } else {
                        let path = format!("{}/{}", goal, f.name);
                        // Try memoized lookup
                        if let Some(cost) = map.get(&path) {
                            total += cost;
                        } else {
                            let cost = self.recursive_walk(map, path.clone());
                            total += cost;
                            map.insert(path, cost);
                        }
                    }
                }
                total
            }
        }
    }
}

pub fn solve(input: &str) -> Answer<u32, u32> {
    const DISK_SPACE: u32 = 70_000_000;
    const REQUIRED: u32 = 30_000_000;
    let mut sys = FileSystem::new();
    for line in input.lines() {
        if line.starts_with("$") {
            sys.do_command(line);
        } else {
            sys.add_listing(line);
        }
    }
    let sizes = sys.directory_sizes();
    let part1 = sizes
        .iter()
        .filter_map(|(_k, &v)| if v <= 100000 { Some(v) } else { None })
        .sum();
    let total_usage = *sizes.get("/").unwrap();
    let remaining = DISK_SPACE - total_usage;
    let minimum_removal = REQUIRED - remaining;
    let part2 = sizes.iter().fold(u32::MAX, |acc, (_k, &sz)| {
        if sz > minimum_removal && sz < acc {
            sz
        } else {
            acc
        }
    });
    Answer::new(part1, part2)
}
