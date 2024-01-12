use eframe::egui::{CollapsingHeader, Label, Sense, Ui};
use rfd::{FileDialog, FileHandle};
use std::{
    fs::{self},
    path::PathBuf,
};

// a function that changes the name of the file being edited and returns the new name
pub fn change_title(old_title: String, new_title: String) -> String {
    let new_name = format!("{}.md", new_title);

    match fs::rename(old_title.clone(), new_name.clone()) {
        Ok(()) => println!(
            "File renamed from {} to {} successfully",
            old_title, new_name
        ),
        Err(e) => eprintln!("Error renaming file: {}", e),
    }
    new_name
}

// a function that saves all the text in the body text box to a file with the name from the title text box
pub fn save_file(text: String, title: String, working_dir: PathBuf) {
    let file_path = format!("{}/{}.md", working_dir.display(), title);
    match fs::write(file_path.clone(), text) {
        Ok(_) => println!("Text saved to {}", file_path),
        Err(e) => eprintln!("Error saving text to {}: {}", file_path, e),
    }
}

pub struct FileData {
    pub new_body_text: String,
    pub new_title_text_short: String,
    pub new_old_title_text_short: String,
    pub new_working_dir: PathBuf,
}

// a function that handles the menu file > open file, and saves it to the working directory
pub fn open_file(working_dir: PathBuf) -> FileData {
    // open a native file dialog and select a text file
    // println!("the working directory is '{}'", path.display());
    let files = FileDialog::new()
        .add_filter("markdown", &["md", "txt"])
        .set_directory(format!("{}", working_dir.display()))
        .set_title("open file");
    let selected_file = files.pick_file();

    // set output variables
    let new_title_text = FileHandle::from(selected_file.clone().unwrap()).file_name();
    //println!("the working directory is '{}'", new_title_text);
    let new_file_path = FileHandle::from(selected_file.clone().unwrap()); //.path();
    let new_file_path = new_file_path.path();
    //println!("the working directory is '{}'", new_file_path.display());

    // copy the chosen file into working directory
    match std::fs::copy(
        format!("{}", new_file_path.display()),
        format!("{}/{}", working_dir.display(), new_title_text),
    ) {
        Ok(_) => {}
        Err(e) => eprintln!(
            "Error copying {} to {}: {}",
            new_file_path.display(),
            format!("{}/{}", working_dir.display(), new_title_text),
            e
        ),
    }

    // pull the title and contents of file and set to output variables
    let new_old_title_text_short = new_title_text
        .chars()
        .take(new_title_text.len() - 3)
        .collect();
    let new_title_text_short = new_title_text
        .chars()
        .take(new_title_text.len() - 3)
        .collect();
    let new_body_text = match fs::read_to_string(&new_file_path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file {}: {}", new_file_path.display(), e);
            String::new() // or handle the error in a different way, e.g., return an error variant
        }
    };
    let new_working_dir = working_dir;

    FileData {
        new_body_text,
        new_title_text_short,
        new_old_title_text_short,
        new_working_dir,
    }
}

#[derive(Debug)]
pub enum FileType {
    File,
    Directory,
    Markdown,
    // Add more file types as needed
}

#[derive(Debug)]
pub struct FileEntry {
    pub name: String,
    pub file_type: FileType,
}

// list all files and folders, and output as a vector
pub fn list_files_in_directory(working_dir: &PathBuf) -> Result<Vec<FileEntry>, std::io::Error> {
    let entries = fs::read_dir(working_dir)?;

    let mut result = Vec::new();

    for entry in entries {
        let entry = entry?;
        // println!("{:?}", entry);
        let file_name = entry.file_name();
        let file_type = if entry.file_type()?.is_dir() {
            FileType::Directory
        } else if file_name.to_string_lossy().ends_with(".md") {
            FileType::Markdown
        } else {
            FileType::File
        };
        // println!("type: {:?}, name: {:?}", file_type, file_name);
        result.push(FileEntry {
            name: file_name.to_string_lossy().to_string(),
            file_type,
        });
    }

    Ok(result)
}

pub fn explorer_open_file(working_dir: &PathBuf, file: String) -> FileData {
    let new_old_title_text_short = file.chars().take(file.len() - 3).collect();
    let new_title_text_short = file.chars().take(file.len() - 3).collect();

    let new_file_path = format!("{}/{}", working_dir.display(), file);
    println!("{}", new_file_path);
    let new_body_text = match fs::read_to_string(&new_file_path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file, e);
            String::new() // or handle the error in a different way, e.g., return an error variant
        }
    };

    let new_working_dir = working_dir.clone();

    FileData {
        new_body_text,
        new_title_text_short,
        new_old_title_text_short,
        new_working_dir,
    }
}

pub fn display_explorer(working_dir: &PathBuf, ui: &mut Ui) -> Option<FileData> {
    //println!("{}", working_dir.display());

    let mut file_data_output = None;

    match list_files_in_directory(working_dir) {
        Ok(entries) => {
            for entry in entries {
                match entry.file_type {
                    FileType::Directory => {
                        //println!("{}", entry.name);
                        CollapsingHeader::new(&entry.name).show(ui, |ui| {
                            //Add UI components specific to directories
                            //ui.label("This is a directory!");
                            let mut new_working_dir = working_dir.clone();
                            new_working_dir.push(&entry.name);
                            file_data_output = display_explorer(&new_working_dir, ui);
                        });
                    }

                    FileType::Markdown => {
                        //println!("{}", entry.name);
                        if ui
                            .add(Label::new(&entry.name).sense(Sense::click()))
                            .clicked()
                        {
                            file_data_output =
                                Some(explorer_open_file(&working_dir.clone(), entry.name));
                            // return file_data_output;
                        }
                    }

                    FileType::File => {
                        //println!("Other file: {}", entry.name);
                    }
                }
            }
        }

        Err(err) => {
            eprintln!("Error: {}", err);
            //return None;
        }
    }
    return file_data_output;
}
