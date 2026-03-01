use crate::arguments::arguments;
use crate::processing::root_element_processors::{process_file, process_root_directory};
use crate::processing::verbose_logging::{VERBOSE_LOGGER, VerboseLogger, verbose_log};
use std::path::Path;

pub fn process(arguments: &arguments::Arguments) -> usize {
    VERBOSE_LOGGER.set_verbose(arguments);

    verbose_log("BEGINNING PROCESSING");
    verbose_log("RETRIEVING PATH FOR PROCESSING FROM ARGUMENTS DATA");
    if arguments.path().is_none() {
        eprintln!(
            "Path argument is required for processing.\n\
        Argument validation and fetching and setting up failed: Path is missing."
        );
        return 0;
    }

    verbose_log(&format!(
        "RETRIEVED PATH '{}' FOR PROCESSING FROM ARGUMENTS DATA",
        arguments.path().as_deref().unwrap()
    ));

    verbose_log("SELECTING PROCESSOR");
    if arguments.is_file_target() {
        verbose_log("SELECTED FILE PROCESSOR");
        return process_file(arguments);
    }

    verbose_log("SELECTED DIRECTORY PROCESSOR");
    verbose_log("RETRIEVING ROOT DIRECTORY PATH");
    let path_str: &str = arguments.path().as_deref().unwrap();
    process_root_directory(arguments, Path::new(path_str))
}

fn process_root_directory(arguments: &arguments::Arguments, path: &Path) -> usize {
    let result = path.read_dir();
    if result.is_err() {
        eprintln!(
            "Skipping directory {} as an error occurred when attempting to retrieve its children\n\
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
        process_directory_as_chunks(
            arguments,
            &mut read_dir,
            arguments.max_depth(),
            path,
        )
    }
}

fn process_file(arguments: &arguments::Arguments) -> usize {
    let file = arguments.path().as_deref().unwrap();
    let path = Path::new(file);

    let path_buffer = path.to_path_buf();
    let potential_parent_buffer = fs::canonicalize(path_buffer.clone());

    let result = process_root_directory(arguments, Path::new(path_str));
    verbose_log(&format!(
        "PROCESSING COMPLETELY FINISHED (FROM PROCESSOR). NUMBER OF FILES PROCESSED IS {}",
        result
    ));

    result
}

pub(super) enum LoopSubCallReturnBehavior {
    NormalProcessing,
    Break,
    Continue,
}

pub(super) fn is_ok_name_according_to_filter(
    arguments: &arguments::Arguments,
    file_name: &str,
    directory_path: &Path,
) -> bool {
    verbose_log(&format!("CHECKING IF FILE '{}' MATCHES FILTER (DEFAULT GLOB, CUSTOM GLOB OR PATTERN)", file_name));
    verbose_log("TO DECIDE WHETHER TO PROCESS THE FILE OR NOT AND INCLUDE IT");
    let result = arguments.file_check()(arguments, file_name);

    if !result {
        verbose_log(&format!("FILE '{}' DOES NOT MATCH FILTER", file_name));
        verbose_log("SKIPPING FILE");
        let full_path_buffer = directory_path.join(file_name);
        let full_path = full_path_buffer.to_str();
        if full_path.is_none() {
            eprintln!("Warning: a file who's name is not valid UTF-8: {:?} was skipped due to not matching the glob or regex pattern", file_name);
            return false;
        }

        let full_path = full_path_buffer.to_str().unwrap();
        eprintln!("Warning, the file '{}' was skipped due to not matching the glob or regex pattern", full_path);
    }

    verbose_log(&format!("FILE '{}' MATCHES FILTER", file_name));
    verbose_log("PROCESSING FILE AND FILE WILL BE INCLUDED");
    result
}
