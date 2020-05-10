use walkdir::WalkDir;

/// Создание списка "events.xml" файлов.
fn get_list(root: &str) -> Vec<String> {
    let mut array: Vec<String> = vec![];

    for entry in WalkDir::new(root) {
        let path = entry.unwrap().path().display().to_string();

        if path.contains("events.xml") {
            array.push(path);
        }
    }

    return array;
}

fn main() {
    let root = "tests";
    let list = get_list(root);
    dbg!(list);
}
