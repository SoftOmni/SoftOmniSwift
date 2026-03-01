use crate::arguments::arguments;
use crate::processing::processing::LoopSubCallReturnBehavior;
use crate::processing::sorting::sort_entries;

use crate::arguments::arguments::FileFolderItemLocation;
use crate::processing::child_element_processors::{
    FileWriteResult, gather_consecutive_file_group, process_directory, write_buffer_to_file,
    write_file_group,
};
use crate::processing::verbose_logging::verbose_log;
use std::fs::{DirEntry, ReadDir};
use std::path::Path;

pub(super) fn process_directory_in_order_of_sorting_algorithm(
    arguments: &arguments::Arguments,
    read_dir: &mut ReadDir,
    recursion_depth_limit: usize,
    path: &Path,
) -> usize {
    let initial_directory_setup = directory_processor_setup(
        arguments,
        path,
        recursion_depth_limit,
        "IN-ORDER OF SORTING ALGORITHM",
    );

    if matches!(initial_directory_setup, DirectoryProcessSetupResult::Exit) {
        return 0;
    }

    verbose_log("READING DIRECTORY ENTRIES AND STORING THEM IN BUFFER TO BE SORTED");
    let mut entries: Vec<DirEntry> = Vec::new();

    get_all_entries(read_dir, &mut entries);

    let mut buffer = String::new();
    let number_of_files_inserted_in_children: usize = 0;
    let mut number_of_files_inserted_in_current_directory: usize = 0;

    verbose_log("SORTING DIRECTORY ENTRIES");
    sort_entries(&mut entries, &arguments.sorting_order());
    verbose_log("DIRECTORY ENTRIES SORTED SUCCESSFULLY");

    verbose_log("ITERATING THROUGH SORTED DIRECTORY ENTRIES");
    let mut entry_index: usize = 0;

    while entry_index < entries.len() {
        verbose_log(&format!(
            "PROCESSING DIRECTORY ENTRY AT INDEX {} BY RETRIEVING IT",
            entry_index
        ));
        let entry = &entries[entry_index];
        verbose_log(&format!(
            "RETRIEVED DIRECTORY ENTRY AT INDEX {}",
            entry_index
        ));
        verbose_log(&format!(
            "PROCESSING DIRECTORY ENTRY AT INDEX {} WITH FILE NAME {} BY RETRIEVING FILE TYPE TO DETERMINE PROCESSING TO BE DONE",
            entry_index,
            entry.file_name().to_string_lossy()
        ));
        let potential_file_type = entry.file_type();

        if potential_file_type.is_err() {
            eprintln!(
                "ERROR DETERMINING FILE TYPE FOR ENTRY: {:?}. SKIPPING...",
                entry.file_name()
            );
            continue;
        }

        let file_type = potential_file_type.unwrap();
        verbose_log(&format!(
            "FILE TYPE FOR DIRECTORY ENTRY AT INDEX {} WITH FILE NAME {} IS {:?}",
            entry_index,
            entry.file_name().to_string_lossy(),
            file_type
        ));
        verbose_log("CHECK WHETHER FILE TYPE IS DIRECTORY");
        if file_type.is_dir() {
            verbose_log(&format!(
                "FILE TYPE IS DIRECTORY. PROCESSING DIRECTORY ENTRY AT INDEX {} WITH FILE NAME {}",
                entry_index,
                entry.file_name().to_string_lossy()
            ));
            process_directory(
                arguments,
                &mut buffer,
                entry,
                process_directory_in_order_of_sorting_algorithm,
                recursion_depth_limit,
                path.join(entry.file_name()).as_path(),
            );

            verbose_log(&format!(
                "PROCESSING DIRECTORY ENTRY AT INDEX {} WITH FILE NAME {} COMPLETED",
                entry_index,
                entry.file_name().to_string_lossy()
            ));
            entry_index += 1;
        } else {
            let mut file_group: Vec<String> = Vec::new();
            let mut file_group_length: usize =
                arguments.source_variable_name().len() + " += files(".len();

            verbose_log(&format!(
                "GATHERING CONSECUTIVE FILE GROUP FOR FILE ENTRY STARTING AT INDEX {} WITH FILE NAME {}",
                entry_index,
                entry.file_name().to_string_lossy()
            ));
            let result = gather_consecutive_file_group(
                arguments,
                &mut entry_index,
                &entries,
                &mut file_group,
                &mut file_group_length,
                path.join(entry.file_name()).as_path(),
            );

            verbose_log(&format!(
                "GATHERING CONSECUTIVE FILE GROUP FOR FILE ENTRY ENDING NOW AT INDEX {} WITH FILE NAME {} COMPLETED",
                entry_index,
                entry.file_name().to_string_lossy()
            ));
            verbose_log(
                "CHECKING DIRECTIVES ON WEATHER TO BREAK THE LOOP, GO TO THE NEXT ENTRY OR PROCEED AS NORMAL AND WRITE THE FILE GROUP TO THE BUFFER",
            );
            if matches!(result, LoopSubCallReturnBehavior::Break) {
                verbose_log(
                    "BREAKING THE LOOP...LAST ENTRY PROCESSED ACCORDING TO FILE GROUP GATHERER DIRECTIVES",
                );
                break;
            } else if matches!(result, LoopSubCallReturnBehavior::Continue) {
                verbose_log(
                    "RESTARTING PROCESSING OF ENTRY AS IT IS A DIRECTORY (FILES FAILED TO BE READ) IN ACCORDANCE WITH FILE GROUP GATHERER DIRECTIVES",
                );
                continue;
            }

            verbose_log("EVERYTHING IS OKAY, PROCEEDING TO WRITE THE FILE GROUP TO THE BUFFER");

            write_file_group(arguments, &mut buffer, &file_group, file_group_length);

            verbose_log("WRITING FILE GROUP TO BUFFER COMPLETED");

            verbose_log(&format!(
                "NUMBER OF FILES INSERTED IN BUFFER: {}",
                file_group_length
            ));
            number_of_files_inserted_in_current_directory += file_group_length;
        }
    }

    verbose_log(&format!(
        "TOTAL NUMBER OF FILES INSERTED IN BUFFER: {}",
        number_of_files_inserted_in_current_directory
    ));
    verbose_log(&format!(
        "TOTAL NUMBER OF FILES PROCESSED IN CHILDREN: {}",
        number_of_files_inserted_in_children
    ));
    verbose_log(&format!(
        "TOTAL FOR FOLDER AND CHILDREN: {}",
        number_of_files_inserted_in_children + number_of_files_inserted_in_current_directory
    ));

    verbose_log("ATTEMPTING TO WRITE OUT BUFFER TO meson.build FILE");

    let result = write_buffer_to_file(&mut buffer, path);
    if matches!(result, FileWriteResult::Failure) {
        verbose_log(&format!(
            "AS WRITING BUFFER TO FILE FAILED, \
                THE NUMBER OF FILES FOR THIS DIRECTORY IS REDUCED TO THOSE INSERTED IN CHILDREN: {}",
            number_of_files_inserted_in_children
        ));

        return number_of_files_inserted_in_children;
    }

    verbose_log(&format!(
        "WRITING TO BUFFER SUCCEEDED. EXITING PROCESSING FOR DIRECTORY {}\n\
        RETURNING TOTAL NUMBER OF PROCESSED FILES {}",
        path.display(),
        number_of_files_inserted_in_children + number_of_files_inserted_in_current_directory
    ));
    number_of_files_inserted_in_children + number_of_files_inserted_in_current_directory
}

pub(super) fn process_directory_as_chunks(
    arguments: &arguments::Arguments,
    read_dir: &mut ReadDir,
    recursion_depth_limit: usize,
    path: &Path,
) -> usize {
    let initial_directory_setup = directory_processor_setup(
        arguments,
        path,
        recursion_depth_limit,
        "FILES AND DIRECTORIES GROUPED TOGETHER",
    );
    if matches!(initial_directory_setup, DirectoryProcessSetupResult::Exit) {
        return 0;
    }

    verbose_log("INITIALIZING FILE AND DIRECTORY STORAGE AHEAD OF SORTING");
    let mut buffer = String::new();
    let mut file_entries: Vec<DirEntry> = Vec::new();
    let mut directory_entries: Vec<DirEntry> = Vec::new();

    gather_directory_and_file_entries(read_dir, &mut directory_entries, &mut file_entries);

    verbose_log("SORTING FILE AND DIRECTORY ENTRIES");

    verbose_log("SORTING FILE ENTRIES");
    sort_entries(&mut file_entries, &arguments.sorting_order());
    verbose_log("SORTING FILE ENTRIES FINISHED");

    verbose_log("SORTING DIRECTORY ENTRIES");
    sort_entries(&mut directory_entries, &arguments.sorting_order());
    verbose_log("SORTING DIRECTORY ENTRIES FINISHED");

    let mut number_of_files_changed: usize = 0;

    verbose_log("DETERMINING FILE AND DIRECTORY PROCESSING ORDER");

    if matches!(
        arguments.file_folder_item_location(),
        FileFolderItemLocation::FolderFirst
    ) {
        verbose_log("PROCESSING DIRECTORY GROUP FIRST");
        number_of_files_changed += directory_group_processor(
            arguments,
            &mut directory_entries,
            &mut buffer,
            recursion_depth_limit,
            path,
        );

        number_of_files_changed += file_group_processor(arguments, &mut file_entries, &mut buffer);
    } else {
        verbose_log("PROCESSING FILE GROUP FIRST");
        number_of_files_changed += file_group_processor(arguments, &mut file_entries, &mut buffer);

        number_of_files_changed += directory_group_processor(
            arguments,
            &mut directory_entries,
            &mut buffer,
            recursion_depth_limit,
            path,
        );
    }

    verbose_log(&format!(
        "PROCESSING DIRECTORY '{}' COMPLETE",
        path.display()
    ));
    verbose_log(&format!(
        "NUMBER OF FILES PROCESSED: {}",
        number_of_files_changed
    ));
    number_of_files_changed
}

enum DirectoryProcessSetupResult {
    Exit,
    Continue,
}

fn directory_processor_setup(
    arguments: &arguments::Arguments,
    path: &Path,
    recursion_depth_limit: usize,
    algorithm: &str,
) -> DirectoryProcessSetupResult {
    verbose_log(&format!(
        "PROCESSING DIRECTORY '{}' WITH {}",
        path.display(),
        algorithm
    ));

    verbose_log(&format!(
        "CHECKING RECURSION DEPTH-LIMIT TO SEE IF SHOULD STOP (IF RECURSION DEPTH-LIMIT IS 0)\n\
             CURRENT RECURSION DEPTH-LIMIT IS {}",
        recursion_depth_limit
    ));

    if recursion_depth_limit == 0 {
        verbose_log(&format!(
            "RECURSION DEPTH-LIMIT REACHED. SKIPPING '{}' DIRECTORY PROCESSING",
            path.display()
        ));
        verbose_log("CHECKING IF WAS FILE TARGET AND SHOULD SHOW THIS AS A WARNING");

        if arguments.is_file_target() {
            verbose_log("WAS FILE TARGET. SKIPPING FILE WARNING");
            verbose_log(&format!(
                "EXITING PROCESSING OF DIRECTORY BECAUSE RECURSIVE DEPT-LIMIT REACHED'{}'",
                path.display()
            ));

            return DirectoryProcessSetupResult::Exit;
        }

        verbose_log("PRINTING WARNING MESSAGE");
        eprintln!(
            "Warning: Recursion depth limit reached. Skipping directory processing of {:?}",
            path
        );

        verbose_log(&format!(
            "EXITING PROCESSING OF DIRECTORY BECAUSE RECURSIVE DEPT-LIMIT REACHED'{}'",
            path.display()
        ));

        return DirectoryProcessSetupResult::Exit;
    }

    DirectoryProcessSetupResult::Continue
}

fn get_all_entries(read_dir: &mut ReadDir, entries: &mut Vec<DirEntry>) {
    loop {
        verbose_log("READING POTENTIAL NEXT DIRECTORY ENTRY (COULD BE NONE IF END REACHED)");
        let current_entry = read_dir.next();
        if current_entry.is_none() {
            verbose_log("REACHED END OF DIRECTORY ENTRIES");
            break;
        }

        match current_entry.unwrap() {
            Ok(entry) => {
                verbose_log(&format!(
                    "ADDING VALID ENTRY '{}' TO LIST OF DIRECTORY ENTRIES",
                    entry.path().display()
                ));
                entries.push(entry);
                verbose_log("ENTRY ADDED SUCCESSFULLY");
            }
            Err(error) => eprintln!(
                "ERROR: ERROR READING DIRECTORY ENTRY:\n{:?}.\nSKIPPING",
                error
            ),
        }
    }
}

fn gather_directory_and_file_entries(
    read_dir: &mut ReadDir,
    directory_entries: &mut Vec<DirEntry>,
    file_entries: &mut Vec<DirEntry>,
) {
    verbose_log(
        "LOOPING THROUGH DIRECTORY ENTRIES AND ADDING FILE AND DIRECTORY ENTRIES TO THEIR RESPECTIVE STORAGE",
    );
    loop {
        verbose_log("PROCESSING DIRECTORY ENTRY");
        let current_entry = read_dir.next();
        if current_entry.is_none() {
            verbose_log("REACHED END OF DIRECTORY ENTRIES");
            break;
        }

        verbose_log("PROCESSING DIRECTORY ENTRY (NAME TO BE DETERMINED)");
        verbose_log(
            "CHECKING IF DIRECTORY ENTRY IS VALID AND DETERMINING FILE TYPE TO TAKE APPROPRIATE ACTION",
        );
        match current_entry.unwrap() {
            Ok(entry) => {
                verbose_log(&format!(
                    "DIRECTORY ENTRY IS VALID. DIRECTORY ENTRY IS {:?}",
                    entry.file_name()
                ));
                verbose_log(&format!(
                    "DETERMINING FILE TYPE FOR ENTRY {:?}",
                    entry.file_name()
                ));
                let potential_file_type = entry.file_type();

                if potential_file_type.is_err() {
                    eprintln!(
                        "Error determining file type for entry: {:?}. Skipping",
                        entry.file_name()
                    );
                    continue;
                }

                let file_type = potential_file_type.unwrap();
                verbose_log(&format!(
                    "FILE TYPE FOR ENTRY {:?} IS {:?}",
                    entry.file_name(),
                    file_type
                ));

                if file_type.is_file() {
                    verbose_log(&format!(
                        "ADDING FILE ENTRY {:?} TO FILE ENTRIES",
                        entry.file_name()
                    ));
                    file_entries.push(entry);
                } else if file_type.is_dir() {
                    verbose_log(&format!(
                        "ADDING DIRECTORY ENTRY {:?} TO DIRECTORY ENTRIES",
                        entry.file_name()
                    ));
                    directory_entries.push(entry);
                }
            }
            Err(error) => eprintln!("Error reading directory entry: {:?}. Skipping", error),
        }
    }

    verbose_log(&format!(
        "TOTAL NUMBER OF DIRECTORIES IN STORE IS {}",
        directory_entries.len()
    ));
    verbose_log(&format!(
        "TOTAL NUMBER OF FILES IN STORE IS {}",
        file_entries.len()
    ));
}

fn directory_group_processor(
    arguments: &arguments::Arguments,
    directory_entries: &Vec<DirEntry>,
    buffer: &mut String,
    recursion_depth_limit: usize,
    path: &Path,
) -> usize {
    let mut number_of_files_changed: usize = 0;

    verbose_log("PROCESSING DIRECTORY ENTRIES IN STORE");
    for directory_entry in directory_entries {
        verbose_log(&format!(
            "PROCESSING DIRECTORY ENTRY {:?}",
            directory_entry.file_name()
        ));

        number_of_files_changed += process_directory(
            arguments,
            buffer,
            directory_entry,
            process_directory_as_chunks,
            recursion_depth_limit,
            path.join(directory_entry.file_name()).as_path(),
        );

        verbose_log(&format!(
            "PROCESSING DIRECTORY ENTRY {:?} FINISHED",
            directory_entry.file_name()
        ));
    }

    verbose_log("PROCESSING DIRECTORY ENTRIES IN STORE FINISHED");

    number_of_files_changed
}

fn file_group_processor(
    arguments: &arguments::Arguments,
    file_entries: &Vec<DirEntry>,
    buffer: &mut String,
) -> usize {
    verbose_log("PROCESSING FILE ENTRIES IN STORE");

    verbose_log(
        "CALCULATING FILE GROUP CODE LENGTH AND FILTERING OUT ENTRIES WHO'S NAME IS AN INVALID STRING",
    );
    let mut file_group_length: usize = arguments.source_variable_name().len() + " += files(".len();
    let file_entries_shortened: Vec<String> = file_entries
        .iter()
        .filter_map(|entry| {
            let os_file_name = entry.file_name();
            verbose_log(&format!("PROCESSING FILE ENTRY: {:?}", entry.file_name()));

            let potential_file_name = os_file_name.to_str();
            if potential_file_name.is_none() {
                eprintln!(
                    "ERROR: COULD NOT CONVERT FILE NAME TO STRING FOR ENTRY: {:?}. SKIPPING...",
                    entry
                );
                return None;
            }

            file_group_length += potential_file_name.unwrap().len();
            verbose_log(&format!(
                "FILE NAME DECODED AND ENTRY SAVED FOR LATER PROCESSING\n\
                 LENGTH OF FILE NAME: {} TOTAL CODE LENGTH {}",
                potential_file_name.unwrap().len(),
                file_group_length
            ));
            Some(potential_file_name.unwrap().to_string())
        })
        .collect();

    verbose_log(&format!(
        "FILE GROUP CODE LENGTH CALCULATED AND INVALID ENTRIES FILTERED OUT\n\
         FILE GROUP CODE LENGTH IS {} FOR {} FILES",
        file_group_length,
        file_entries_shortened.len()
    ));

    verbose_log("WRITING FILE GROUP TO BUFFER");
    write_file_group(
        arguments,
        buffer,
        &file_entries_shortened,
        file_group_length,
    );

    verbose_log("FILE GROUP WRITTEN TO BUFFER");
    file_entries_shortened.len()
}
