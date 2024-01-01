use std::{fs, path::PathBuf};
use rfd::FileDialog;

// a function that changes the name of the file being edited and returns the new name
pub fn change_title(old_title: String, new_title: String) -> String {

    let new_name = format!("{}.md", new_title);

    match fs::rename(old_title.clone(), new_name.clone()) {
        Ok(()) => println!("File renamed from {} to {} successfully", old_title, new_name),
        Err(e) => eprintln!("Error renaming file: {}", e),
    }
    new_name
}

// a function that saves all the text in the body text box to a file with the name from the title text box
pub fn save_file(text: String, title: String) {
    let file_path = format!("{}.md", title);
    match fs::write(file_path.clone(), text) {
        Ok(_) => println!("Text saved to {}", file_path),
        Err(e) => eprintln!("Error saving text to {}: {}", file_path, e),
    }
}

// a function that handles the menu file > open file, and saves it to the working directory
pub fn open_file(path: PathBuf) {
    println!("the working directory is '{}'", path.display());
    let files = FileDialog::new()
    .add_filter("markdown", &["md", "txt"])
    .set_directory(format!("{}", path.display()))
    .pick_file();
}