use crate::arguments::arguments;
use crate::arguments::arguments::FileFolderItemLocation;
use crate::processing::directory_processors::{
    process_directory_as_chunks, process_directory_in_order_of_sorting_algorithm,
};
use std::fs;
use std::path::Path;

pub(super) fn process_root_directory(arguments: &arguments::Arguments, path: &Path) -> usize {
    let result = path.read_dir();
    if result.is_err() {
        eprintln!(
            "Error: skipping directory {} as an error occurred \
             when attempting to retrieve its children.\n\
             The error was: {}",
            path.to_str().unwrap(),
            result.err().unwrap()
        );

        return 0;
    }

    let mut read_dir = result.unwrap();
    
    if !matches!(
        arguments.file_folder_item_location(),
        FileFolderItemLocation::InOrderOfSortingAlgorithm
    ) {
        process_directory_in_order_of_sorting_algorithm(
            arguments,
            &mut read_dir,
            arguments.max_depth(),
            path,
        )
    } else {
        process_directory_as_chunks(arguments, &mut read_dir, arguments.max_depth(), path)
    }
}

pub(super) fn process_file(arguments: &arguments::Arguments) -> usize {
    let file = arguments.path().as_deref().unwrap();
    let path = Path::new(file);
    let path_buffer = path.to_path_buf();
    let potential_parent_buffer = fs::canonicalize(path_buffer.clone());

    if potential_parent_buffer.is_err() {
        let potential_name = path_buffer.to_str();

        if potential_name.is_none() {
            eprintln!("Error: the parent directory of the file was removed after argument parsing");
            return 0;
        }

        eprintln!(
            "Error: the parent directory of the file {} was removed after argument parsing",
            potential_name.unwrap()
        );
        return 0;
    }

    let parent_buffer = potential_parent_buffer.unwrap();
    let potential_directory_reader = parent_buffer.read_dir();
    if potential_directory_reader.is_err() {
        eprintln!(
            "Error: failed to read directory contents of parent directory {} because of {:?}",
            parent_buffer.display(),
            potential_directory_reader.unwrap_err()
        );
        return 0;
    }

    let mut directory_reader = potential_directory_reader.unwrap();

    let result = if !matches!(
        arguments.file_folder_item_location(),
        FileFolderItemLocation::InOrderOfSortingAlgorithm
    ) {
        process_directory_in_order_of_sorting_algorithm(arguments, &mut directory_reader, 1, path)
    } else {
        process_directory_as_chunks(arguments, &mut directory_reader, 1, path)
    };

    result
}
