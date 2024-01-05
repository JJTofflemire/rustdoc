use std::{fs, path::PathBuf};
use rfd::{FileDialog, FileHandle};

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
pub fn open_file(working_dir: PathBuf) {
    // open a native file dialog and select a text file
    // println!("the working directory is '{}'", path.display());
    let files = FileDialog::new()
    .add_filter("markdown", &["md", "txt"])
    .set_directory(format!("{}", working_dir.display()))
    .set_title("open file");
    let selected_file = files.pick_file();

    // set output variables
    let title_text = FileHandle::from(selected_file.clone().unwrap()).file_name();
    println!("the working directory is '{}'", title_text);
    let new_file_path = FileHandle::from(selected_file.clone().unwrap());//.path();
    let new_file_path = new_file_path.path();
    println!("the working directory is '{}'", new_file_path.display());

    match std::fs::copy(format!("{}", new_file_path.display()), format!("{}/{}", working_dir.display(), title_text)) {
        Ok(_) => {},
        Err(e) => eprintln!("Error copying {} to {}: {}", new_file_path.display(), format!("{}/{}", working_dir.display(), title_text), e),
    }
}