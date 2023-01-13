use std::collections::HashMap;

/// The filesystem is a tree structure using an arena allocator hashmap with the keys as directory
/// paths. Directories hold the key indexes of child directories.
type DirectoryTree = HashMap<String, Directory>;

#[derive(Default, Debug)]
pub struct Directory {
    file_sizes: usize,
    children: Vec<String>,
}

/// Sums the filesystem recursively for a path.
pub fn sum_path(path: &String, filesystem: &DirectoryTree) -> usize {
    let mut sum: usize = filesystem.get(path).unwrap().file_sizes;
    for child in &filesystem.get(path).unwrap().children {
        sum += sum_path(child, filesystem);
    }
    sum
}

/// Executes the `cd ..` command on a filesystem path
#[inline(always)]
fn step_back_dir(input: &str) -> String {
    input
        .split('/')
        .collect::<Vec<&str>>()
        .split_last()
        .unwrap()
        .1
        .join("/")
}

/// Constructs a file system from the input CLI commands
fn build_filesystem(input: &str) -> DirectoryTree {
    let mut filesystem = DirectoryTree::new();
    filesystem.insert("".to_string(), Directory::default());

    let mut current_path = "".to_string();
    input.lines().for_each(|line| {
        let split: Vec<&str> = line.split(' ').collect();
        match split[0] {
            // User input commands
            "$" => {
                if split[1] == "cd" {
                    match split[2] {
                        // Return to the root directory
                        "/" => current_path = "".to_string(),
                        // Step back a single directory
                        ".." => current_path = step_back_dir(&current_path),
                        // Change Directory to the child specified in the command parameter
                        _ => current_path = format!("{}/{}", current_path, split[2]),
                    }
                }
            }
            // Child Directory Listing - Add to the Hashmap and current path's child Vec
            // This does NOT adjust the current path.
            "dir" => {
                let child_path = format!("{}/{}", current_path, split[1]);
                filesystem
                    .get_mut(&*current_path)
                    .unwrap()
                    .children
                    .push(child_path.clone());
                filesystem.insert(child_path, Directory::default());
            }
            // File Listing Entry - Add the file size to the current path's sum.
            _ => {
                filesystem.get_mut(&*current_path).unwrap().file_sizes +=
                    split[0].parse::<usize>().unwrap();
            }
        };
    });
    filesystem
}

/// Find all directories with a size <= 100_000 and sum them all.
pub fn part_one(input: &str) -> Option<u32> {
    let filesystem = build_filesystem(input);
    let answer: usize = filesystem
        .keys()
        .map(|key| sum_path(key, &filesystem))
        .filter(|size| size <= &(100_000_usize))
        .sum();
    Some(answer as u32)
}

/// Find the smallest directory that can be deleted to meet the update space requirements.
pub fn part_two(input: &str) -> Option<u32> {
    let filesystem = build_filesystem(input);
    let total_used = sum_path(&"".to_string(), &filesystem);
    let amount_to_free = 30_000_000 - (70_000_000 - total_used);

    let answer = filesystem
        .keys()
        .map(|key| sum_path(key, &filesystem))
        .filter(|size| size >= { &amount_to_free })
        .min()
        .unwrap();
    Some(answer as u32)
}

fn main() {
    let input = &aoc::read_file("inputs", 7);
    aoc::solve!(1, part_one, input);
    aoc::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = aoc::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
