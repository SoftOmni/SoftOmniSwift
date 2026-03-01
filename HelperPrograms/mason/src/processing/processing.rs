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
            "ERROR: PATH ARGUMENT IS REQUIRED FOR PROCESSING.\n\
            ARGUMENT VALIDATION AND FETCHING AND SETTING UP FAILED: PATH IS MISSING."
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
    verbose_log(&format!("RETRIEVED ROOT DIRECTORY PATH '{}'", path_str));

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
            eprintln!(
                "WARNING: A FILE WHO'S NAME IS NOT VALID UTF-8: {:?} WAS SKIPPED \
                DUE TO NOT MATCHING THE GLOB OR REGEX PATTERN",
                file_name
            );
            return false;
        }

        let full_path = full_path_buffer.to_str().unwrap();
        eprintln!(
            "WARNING: THE FILE '{}' WAS SKIPPED DUE TO NOT MATCHING THE GLOB OR REGEX PATTERN",
            full_path
        );
    }

    verbose_log(&format!("FILE '{}' MATCHES FILTER", file_name));
    verbose_log("PROCESSING FILE AND FILE WILL BE INCLUDED");
    result
}
