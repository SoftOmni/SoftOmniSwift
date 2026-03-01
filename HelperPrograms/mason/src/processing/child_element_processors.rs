use crate::arguments::arguments;
use crate::processing::processing::{is_ok_name_according_to_filter, LoopSubCallReturnBehavior};
use crate::processing::verbose_logging::verbose_log;
use std::fs::{DirEntry, File, ReadDir};
use std::io::Write;
use std::path::Path;

pub(super) fn process_directory<SubFunctionRecursiveCall>(
    arguments: &arguments::Arguments,
    buffer: &mut String,
    directory_entry: &DirEntry,
    recursive_call: SubFunctionRecursiveCall,
    recursion_depth_limit: usize,
    path: &Path,
) -> usize
where
    SubFunctionRecursiveCall: Fn(&arguments::Arguments, &mut ReadDir, usize, &Path) -> usize,
{
    verbose_log("PROCESS_DIRECTORY (SUB-DIRECTORY PROCESSOR) STARTING");
    verbose_log(&format!(
        "PROCESS_DIRECTORY INPUTS:\n\
         - DIRECTORY ENTRY NAME: {:?}\n\
         - RECURSION DEPTH LIMIT (CURRENT): {}\n\
         - PARENT PATH: {}",
        directory_entry.file_name(),
        recursion_depth_limit,
        path.display(),
    ));

    verbose_log("ATTEMPTING TO OPEN CHILD DIRECTORY BY ACQUIRING DIRECTORY READER");
    let potential_child_read_dir = directory_entry.path().read_dir();
    if potential_child_read_dir.is_err() {
        eprintln!(
            "Error opening child directory: {:?}. Skipping",
            directory_entry.file_name()
        );
        return 0;
    }
    verbose_log("CHILD DIRECTORY OPENED (DIRECTORY READER OBTAINED)");

    verbose_log("DECODING DIRECTORY ENTRY FILE NAME AS UTF-8 STRING");
    let os_file_name = directory_entry.file_name();
    let potential_file_name_str = os_file_name.to_str();
    if potential_file_name_str.is_none() {
        // This should never happen
        eprintln!(
            "Error converting file name to string for entry: {:?}. Skipping",
            directory_entry.file_name()
        );
        return 0;
    }

    let child_dir_name = potential_file_name_str.unwrap();
    verbose_log(&format!(
        "DIRECTORY ENTRY NAME DECODED SUCCESSFULLY: '{}'",
        child_dir_name
    ));

    if recursion_depth_limit == 0 {
        verbose_log("RECURSION DEPTH LIMIT REACHED (0). NOT RECURSING INTO CHILD DIRECTORY.");
        return 0;
    }

    verbose_log(&format!(
        "RECURSING INTO CHILD DIRECTORY '{}' WITH DEPTH LIMIT {}",
        child_dir_name,
        recursion_depth_limit - 1
    ));
    let mut child_read_dir = potential_child_read_dir.unwrap();
    let number_of_files_changed: usize = recursive_call(
        arguments,
        &mut child_read_dir,
        recursion_depth_limit - 1,
        path,
    );
    verbose_log(&format!(
        "RECURSIVE CALL FINISHED FOR CHILD DIRECTORY '{}'. NUMBER OF FILES PROCESSED: {}",
        child_dir_name, number_of_files_changed
    ));

    verbose_log(&format!(
        "WRITING subdir() DECLARATION TO BUFFER FOR {}",
        child_dir_name
    ));
    buffer.push_str(&format!("\nsubdir('{}')\n", child_dir_name));
    verbose_log(&format!(
        "subdir() DECLARATION WRITTEN TO BUFFER FOR {}",
        child_dir_name
    ));

    verbose_log("PROCESS_DIRECTORY (SUB-DIRECTORY PROCESSOR) FINISHED");
    number_of_files_changed
}

pub(super) fn gather_consecutive_file_group(
    arguments: &arguments::Arguments,
    index: &mut usize,
    entries: &Vec<DirEntry>,
    file_group: &mut Vec<String>,
    file_group_length: &mut usize,
    directory_path: &Path,
) -> LoopSubCallReturnBehavior {
    let mut potential_file_name_str: Option<&str>;

    let mut is_directory = false;
    let mut file_name: String = String::new();
    let mut is_file_name_str_none = false;

    verbose_log("BEGINNING FILE GROUP GATHERING PROCESS");
    verbose_log("ATTEMPTING TO GATHER FIRST MEMBER OF FILE GROUP WHICH IS NOT A DIRECTORY");
    verbose_log("IF IT IS A DIRECTORY, RESTART PROCESSING OF ENTRY IN DIRECTORY PROCESSOR");
    while *index < entries.len() && is_file_name_str_none && !is_directory {
        verbose_log(&format!("READING ENTRY AT INDEX {}", *index));
        let entry = &entries[*index];

        verbose_log(&format!(
            "ATTEMPTING TO READ FILE NAME AND TYPE FOR ENTRY AT INDEX {}",
            *index
        ));
        let file_type = entry.file_type();

        let file_name_os = entry.file_name();
        let potential_file_name_str = match file_name_os.to_str() {
            None => None,
            Some(s) => Some(s),
        };

        is_file_name_str_none = potential_file_name_str.is_none();

        if file_type.is_err() {
            eprintln!("Error getting file type for entry: {:?}. Skipping", entry);
            *index += 1;
            continue;
        };

        is_directory = file_type.unwrap().is_dir();

        if is_file_name_str_none {
            eprintln!(
                "Error converting file name to string for entry: {:?}. Skipping...",
                entry.file_name()
            );

            *index += 1;
            continue;
        }

        verbose_log(&format!(
            "FILE NAME FOR ENTRY ACQUIRED AND IS '{}'",
            potential_file_name_str.unwrap()
        ));
        verbose_log(&format!(
            "FILE TYPE FOR ENTRY ACQUIRED. IS A DIRECTORY: {}",
            is_directory
        ));

        verbose_log(
            "CHECKING IF FILE IS TO BE INCLUDED ACCORDING TO FILTER (DEFAULT GLOB, CUSTOM GLOB OR REGEX PATTERN)",
        );

        if !is_ok_name_according_to_filter(
            arguments,
            potential_file_name_str.unwrap(),
            directory_path,
        ) {
            verbose_log("FILE IS NOT INCLUDED ACCORDING TO FILTER");
            verbose_log("SKIPPING FILE");
            *index += 1;
            continue;
        }

        file_name = potential_file_name_str.unwrap().to_string();
        *index += 1;
    }

    verbose_log(
        "EXITED FIRST FILE ENTRY GATHERING LOOP... CHECKING WHY AND IF SHOULD STOP FILE GATHERER",
    );
    verbose_log(&format!(
        "FIRST ENTRY WAS AT INDEX {} AND ITS NAME IS {}",
        index, file_name
    ));

    verbose_log(
        "CHECKING IF INDEX IS OUT OF BOUNDS AND THUS NOT MORE ENTRIES ARE TO BE PROCESSED IN DIRECTORY PROCESSOR",
    );
    if *index >= entries.len() {
        verbose_log("INDEX IS OUT OF BOUNDS");
        verbose_log("NO MORE ENTRIES TO PROCESS FOR THIS DIRECTORY");
        return LoopSubCallReturnBehavior::Break;
    }

    verbose_log("INDEX IS NOT OUT OF BOUNDS, THIS IS AN ENTRY");
    verbose_log("CHECKING IF CURRENT ENTRY IS A DIRECTORY");

    if is_directory {
        verbose_log("CURRENT ENTRY IS A DIRECTORY");
        verbose_log("EXITING FILE GATHERER WITH MESSAGE TO USE DIRECTORY CHILD PROCESSOR");
        return LoopSubCallReturnBehavior::Continue;
    }

    verbose_log("CURRENT ENTRY IS NOT A DIRECTORY, PROCESSING AS FILE");
    verbose_log(
        "ADDING FILE NAME AND FILE NAME LENGTH TO FILE GROUP AND CODE LENGTH FOR FILE GROUP TRACKER",
    );
    *file_group_length += file_name.len();
    file_group.push(file_name.clone());
    verbose_log(&format!(
        "CODE LENGTH FOR FILE GROUP TRACKER: {}",
        *file_group_length
    ));

    verbose_log("INCREMENTING INDEX AND CHECKING IF THERE ARE MORE ENTRIES");
    *index += 1;
    while *index < entries.len() {
        verbose_log(&format!(
            "ADDITIONAL ENTRY PRESENT AT INDEX {}...ACQUIRING. WILL STOP IF IS A DIRECTORY",
            *index
        ));

        verbose_log("ACQUIRING ENTRY");
        let entry = &entries[*index];

        verbose_log("ACQUIRING FILE TYPE");
        let file_type = entry.file_type();
        if file_type.is_err() {
            eprintln!(
                "Error getting file type for entry: {:?}. Skipping...",
                entry
            );
            *index += 1;
            continue;
        };

        verbose_log("FILE TYPE ACQUIRED. CHECKING IF DIRECTORY");
        if file_type.unwrap().is_dir() {
            verbose_log("ENTRY IS A DIRECTORY. STOPPING FILE GROUP PROCESSING");
            break;
        }

        verbose_log("ACQUIRING FILE NAME");
        let file_name_temporary = entry.file_name();
        potential_file_name_str = match file_name_temporary.to_str() {
            None => None,
            Some(s) => Some(s),
        };

        if potential_file_name_str.is_none() {
            eprintln!(
                "Error getting file name for entry: {:?}. Skipping...",
                entry
            );
            *index += 1;
            continue;
        }

        verbose_log("FILE NAME ACQUIRED");
        verbose_log(
            "CHECKING IF FILE IS TO BE INCLUDED ACCORDING TO FILTER (DEFAULT GLOB, CUSTOM GLOB OR REGEX PATTERN)",
        );

        if !is_ok_name_according_to_filter(
            arguments,
            potential_file_name_str.unwrap(),
            directory_path,
        ) {
            verbose_log("FILE IS NOT INCLUDED ACCORDING TO FILTER. SKIPPING");
            *index += 1;
            continue;
        }

        verbose_log("FILE IS INCLUDED ACCORDING TO FILTER. PROCESSING");
        verbose_log("ADDING FILE NAME TO FILE GROUP AND FILE GROUP CODE LENGTH");

        file_name = potential_file_name_str.unwrap().to_string();
        file_group.push(file_name.clone());
        *file_group_length += file_name.len();
    }

    verbose_log("FILE GROUP PROCESSING COMPLETE");
    verbose_log("ADDING ONE TO FILE GROUP CODE LENGTH TO REPRESENT CLOSING PARENTHESIS IN CODE LENGTH");
    *file_group_length += 1;

    verbose_log(&format!("TOTAL FILE GROUP CODE LENGTH: {}", file_group_length));
    verbose_log(&format!("NUMBER OF FILES IN FILE GROUP: {}", file_group.len()));
    LoopSubCallReturnBehavior::NormalProcessing
}

pub(super) fn write_file_group(
    arguments: &arguments::Arguments,
    buffer: &mut String,
    file_group: &Vec<String>,
    file_group_length: usize,
) {
    verbose_log("WRITING FILE GROUP TO BUFFER (FROM BUFFER WRITER)");
    verbose_log(&format!(
        "WRITING (SOURCE VARIABLE NAME) {} += files( TO BUFFER",
        arguments.source_variable_name()
    ));

    buffer.push_str(arguments.source_variable_name());
    buffer.push_str("+= files(");

    verbose_log(&format!(
        "DETERMINING WHETHER TO WRITE ON SAME LINE OR NOT BASED ON \n\
         THE FORCEFULLY PUT ON DISTINCT LINES FLAG AND MAX LINE LENGTH ARGUMENT\n\
         THEIR VALUES ARE {} AND {} RESPECTIVELY. THE CURRENT FILE GROUP CODE LENGTH IS {}.",
        arguments.forcefully_put_on_distinct_lines(),
        arguments.max_line_length(),
        file_group_length
    ));

    if arguments.forcefully_put_on_distinct_lines()
        && file_group_length <= arguments.max_line_length()
    {
        verbose_log(&format!(
            "WRITING FILE GROUP TO BUFFER ON SAME LINE BECAUSE FORCEFULLY PUT ON DISTINCT LINES FLAG \
            IS {} AND MAX LINE LENGTH IS {} AND FILE GROUP CODE LENGTH IS {}.",
            arguments.forcefully_put_on_distinct_lines(),
            arguments.max_line_length(),
            file_group_length
        ));

        for (i, file_name) in file_group.iter().enumerate() {
            buffer.push_str(file_name);
            if i < file_group.len() - 1 {
                buffer.push_str(", ");
            }
        }

        verbose_log("FINISHED WRITING FILE ENTRIES TO BUFFER");

        buffer.push_str(")\n");
        verbose_log("FINISHED WRITING FILE GROUP TO BUFFER (FROM BUFFER WRITER)");
        return;
    }

    verbose_log(&format!(
        "WRITING FILE GROUP TO BUFFER ON MULTIPLE LINES BECAUSE FORCEFULLY PUT ON DISTINCT LINES FLAG \
        IS {} AND MAX LINE LENGTH IS {} AND FILE GROUP CODE LENGTH IS {}.",
        arguments.forcefully_put_on_distinct_lines(),
        arguments.max_line_length(),
        file_group_length
    ));

    buffer.push_str("\n");
    for (index, file_name) in file_group.iter().enumerate() {
        buffer.push_str("    ");
        buffer.push_str(file_name);
        if index < file_group.len() - 1 {
            buffer.push_str(",\n");
        }
    }

    verbose_log("FINISHED WRITING FILE GROUP TO BUFFER ON MULTIPLE LINES");
    buffer.push_str("\n)\n");
    verbose_log("FINISHED WRITING FILE GROUP TO BUFFER (FROM BUFFER WRITER)");
}

pub(super) enum FileWriteResult {
    Success,
    Failure,
}

pub(super) fn write_buffer_to_file(buffer: &mut String, path: &Path) -> FileWriteResult {
    verbose_log(&format!(
        "CALCULATING PATH BUFFER FOR meson.build FILE IN DIRECTORY: {}",
        path.display()
    ));
    let meson_path = path.join("meson.build");
    verbose_log("PATH BUFFER FOR meson.build FILE CALCULATED");

    verbose_log(&format!(
        "ATTEMPTING TO CREATE meson.build FILE AT PATH: {}",
        meson_path.display()
    ));
    let potential_file = File::create(meson_path.as_path());
    if potential_file.is_err() {
        eprintln!(
            "Error failed to create meson.build file at path for directory {}:\n{}",
            path.display(),
            meson_path.to_str().unwrap_or("UNKNOWN PATH")
        );

        return FileWriteResult::Failure;
    }

    verbose_log("meson.build FILE CREATED");
    let mut file = potential_file.unwrap();

    verbose_log("ATTEMPTING TO WRITE BUFFER TO meson.build FILE");
    let write_result = file.write_all(buffer.as_bytes());

    if write_result.is_err() {
        eprintln!(
            "Error failed to write buffer to meson.build file at path: {}",
            path.display()
        );
        return FileWriteResult::Failure;
    }

    verbose_log(&format!(
        "BUFFER WRITTEN TO meson.build FILE ({} CHARACTERS)",
        buffer.len()
    ));
    FileWriteResult::Success
}
