use crate::arguments::arguments;
use crate::arguments::arguments::FileFolderItemLocation;
use crate::processing::directory_processors::{
    process_directory_as_chunks, process_directory_in_order_of_sorting_algorithm,
};
use crate::processing::verbose_logging::verbose_log;
use std::fs;
use std::path::Path;

pub(super) fn process_root_directory(arguments: &arguments::Arguments, path: &Path) -> usize {
    verbose_log("BEGINNING PROCESSING OF ROOT DIRECTORY");
    verbose_log("RETRIEVING ITERATOR OF ROOT DIRECTORY'S CHILDREN");

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
    verbose_log("RETRIEVED ITERATOR OF ROOT DIRECTORY'S CHILDREN");

    verbose_log("EXAMINING DIRECTORY PROCESSOR TO USE BASED ON FILE/FOLDER GROUPING DESIRED");
    let result = if !matches!(
        arguments.file_folder_item_location(),
        FileFolderItemLocation::InOrderOfSortingAlgorithm
    ) {
        verbose_log("SELECTED IN ORDER OF SORTING ALGORITHM DIRECTORY PROCESSOR");
        process_directory_in_order_of_sorting_algorithm(
            arguments,
            &mut read_dir,
            arguments.max_depth(),
            path,
        )
    } else {
        verbose_log("SELECTED CHUNKED DIRECTORY PROCESSOR");
        process_directory_as_chunks(arguments, &mut read_dir, arguments.max_depth(), path)
    };

    verbose_log(&format!(
        "PROCESSING FINISHED (FROM ROOT DIRECTORY PROCESSOR). NUMBER OF FILES PROCESSED IS {}",
        result
    ));
    result
}

pub(super) fn process_file(arguments: &arguments::Arguments) -> usize {
    verbose_log("FILE PROCESSOR STARTING");
    verbose_log("RETRIEVING FILE PATH");
    let file = arguments.path().as_deref().unwrap();
    let path = Path::new(file);
    let path_buffer = path.to_path_buf();

    verbose_log(&format!(
        "RETRIEVED FILE PATH AS PATH BUFFER WITH VALUE '{}'",
        file
    ));
    verbose_log("CALCULATING PARENT DIRECTORY");
    verbose_log("CANONICALIZING PATH BUFFER");
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

    verbose_log("CALCULATED CANONICALIZED PATH BUFFER");

    let parent_buffer = potential_parent_buffer.unwrap();
    verbose_log(&format!(
        "CALCULATED PARENT DIRECTORY AS '{}'",
        parent_buffer.display()
    ));

    verbose_log("OBTAINING DIRECTORY READER");
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
    verbose_log("DIRECTORY READER OBTAINED");

    verbose_log(
        "EXAMINING DIRECTORY PROCESSOR TO USE BASED ON FILE/FOLDER GROUPING DESIRED FOR DIRECTORY OF meson.build AND NOT FOR SUB-DIRECTORIES",
    );

    let result = if !matches!(
        arguments.file_folder_item_location(),
        FileFolderItemLocation::InOrderOfSortingAlgorithm
    ) {
        verbose_log(
            "SELECTED IN ORDER OF SORTING ALGORITHM DIRECTORY PROCESSOR FOR DIRECTORY OF meson.build AND NOT FOR SUB-DIRECTORIES",
        );
        process_directory_in_order_of_sorting_algorithm(arguments, &mut directory_reader, 1, path)
    } else {
        verbose_log(
            "SELECTED CHUNKED DIRECTORY PROCESSOR FOR DIRECTORY OF meson.build AND NOT FOR SUB-DIRECTORIES",
        );
        process_directory_as_chunks(arguments, &mut directory_reader, 1, path)
    };

    verbose_log(&format!(
        "DIRECTORY PROCESSING COMPLETE (FROM FILE PROCESSOR). PROCESSED {}",
        result
    ));
    result
}
