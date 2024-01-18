# rustdoc

- a rust utlity for structured text documentation

- I am using this project to learn rust - clone at your own risk!

# to do

- [x] add output variables from `open_file()` to switch working file

- [x] create left panel with explorer

- [ ] seperate `working_dir` and `explorer_dir` so that the explorer expands from root dir instead of only showing the clicked folder

    - it is difficult to implement this because there are difficulties with managing which directory to write to without messing up the display_explorer function

- [ ] add dialog to choose folder and remember folder

  - use the [directories crate](https://docs.rs/directories/5.0.1/directories/struct.BaseDirs.html) to access a platform-agnostic cache
     
  - to do this, I will need to add a `fn startup()` that runs at when launched that checks for a folder or opens a dialogue to open or create one in a default location

- [ ] add match case for how to handle different files and folders in explorer

- [ ] add case for if the `add file` is in the working directory (if needed)

- [ ] make `add file` / `add folder` remember the path used last time

- [ ] add drag and drop support, including into the relavent folders in the explorer
