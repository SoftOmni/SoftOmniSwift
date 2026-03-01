use crate::arguments::arguments;
use crate::processing::processing::LoopSubCallReturnBehavior;
use crate::processing::sorting::sort_entries;

use crate::arguments::arguments::FileFolderItemLocation;
use crate::processing::child_element_processors::{
    gather_consecutive_file_group, process_directory, write_buffer_to_file, write_file_group,
    FileWriteResult,
};
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
    );

    if matches!(initial_directory_setup, DirectoryProcessSetupResult::Exit) {
        return 0;
    }

    let mut entries: Vec<DirEntry> = Vec::new();

    get_all_entries(read_dir, &mut entries);

    let mut buffer = String::new();
    let number_of_files_inserted_in_children: usize = 0;
    let mut number_of_files_inserted_in_current_directory: usize = 0;

    sort_entries(&mut entries, &arguments.sorting_order());

    let mut entry_index: usize = 0;

    while entry_index < entries.len() {
        let entry = &entries[entry_index];
        let potential_file_type = entry.file_type();

        if potential_file_type.is_err() {
            eprintln!(
                "Error determining file type for entry: {:?}. Skipping...",
                entry.file_name()
            );
            continue;
        }

        let file_type = potential_file_type.unwrap();
        if file_type.is_dir() {
            process_directory(
                arguments,
                &mut buffer,
                entry,
                process_directory_in_order_of_sorting_algorithm,
                recursion_depth_limit,
                path.join(entry.file_name()).as_path(),
            );

            entry_index += 1;
        } else {
            let mut file_group: Vec<String> = Vec::new();
            let mut file_group_length: usize =
                arguments.source_variable_name().len() + " += files(".len();

            let result = gather_consecutive_file_group(
                arguments,
                &mut entry_index,
                &entries,
                &mut file_group,
                &mut file_group_length,
                path.join(entry.file_name()).as_path(),
            );

            if matches!(result, LoopSubCallReturnBehavior::Break) {
                break;
            } else if matches!(result, LoopSubCallReturnBehavior::Continue) {
                continue;
            }

            write_file_group(arguments, &mut buffer, &file_group, file_group_length);

            number_of_files_inserted_in_current_directory += file_group_length;
        }
    }

    let result = write_buffer_to_file(&mut buffer, path);
    if matches!(result, FileWriteResult::Failure) {
        return number_of_files_inserted_in_children;
    }

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
    );
    if matches!(initial_directory_setup, DirectoryProcessSetupResult::Exit) {
        return 0;
    }

    let mut buffer = String::new();
    let mut file_entries: Vec<DirEntry> = Vec::new();
    let mut directory_entries: Vec<DirEntry> = Vec::new();

    gather_directory_and_file_entries(read_dir, &mut directory_entries, &mut file_entries);

    sort_entries(&mut file_entries, &arguments.sorting_order());

    sort_entries(&mut directory_entries, &arguments.sorting_order());

    let mut number_of_files_changed: usize = 0;

    if matches!(
        arguments.file_folder_item_location(),
        FileFolderItemLocation::FolderFirst
    ) {
        number_of_files_changed += directory_group_processor(
            arguments,
            &mut directory_entries,
            &mut buffer,
            recursion_depth_limit,
            path,
        );

        number_of_files_changed += file_group_processor(arguments, &mut file_entries, &mut buffer);
    } else {
        number_of_files_changed += file_group_processor(arguments, &mut file_entries, &mut buffer);

        number_of_files_changed += directory_group_processor(
            arguments,
            &mut directory_entries,
            &mut buffer,
            recursion_depth_limit,
            path,
        );
    }

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
) -> DirectoryProcessSetupResult {
    if recursion_depth_limit == 0 {
        if arguments.is_file_target() {
            return DirectoryProcessSetupResult::Exit;
        }

        eprintln!(
            "Warning: recursion depth limit reached. Skipping directory processing of {:?}",
            path
        );

        return DirectoryProcessSetupResult::Exit;
    }

    DirectoryProcessSetupResult::Continue
}

fn get_all_entries(read_dir: &mut ReadDir, entries: &mut Vec<DirEntry>) {
    loop {
        let current_entry = read_dir.next();
        if current_entry.is_none() {
            break;
        }

        match current_entry.unwrap() {
            Ok(entry) => {
                entries.push(entry);
            }
            Err(error) => eprintln!(
                "Error: error reading directory entry:\n{:?}.\nSkipping",
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
    loop {
        let current_entry = read_dir.next();
        if current_entry.is_none() {
            break;
        }

        match current_entry.unwrap() {
            Ok(entry) => {
                let potential_file_type = entry.file_type();

                if potential_file_type.is_err() {
                    eprintln!(
                        "Error determining file type for entry: {:?}. Skipping",
                        entry.file_name()
                    );
                    continue;
                }

                let file_type = potential_file_type.unwrap();

                if file_type.is_file() {
                    file_entries.push(entry);
                } else if file_type.is_dir() {
                    directory_entries.push(entry);
                }
            }
            Err(error) => eprintln!("Error reading directory entry: {:?}. Skipping", error),
        }
    }
}

fn directory_group_processor(
    arguments: &arguments::Arguments,
    directory_entries: &Vec<DirEntry>,
    buffer: &mut String,
    recursion_depth_limit: usize,
    path: &Path,
) -> usize {
    let mut number_of_files_changed: usize = 0;

    for directory_entry in directory_entries {
        number_of_files_changed += process_directory(
            arguments,
            buffer,
            directory_entry,
            process_directory_as_chunks,
            recursion_depth_limit,
            path.join(directory_entry.file_name()).as_path(),
        );
    }

    number_of_files_changed
}

fn file_group_processor(
    arguments: &arguments::Arguments,
    file_entries: &Vec<DirEntry>,
    buffer: &mut String,
) -> usize {
    let mut file_group_length: usize = arguments.source_variable_name().len() + " += files(".len();
    let file_entries_shortened: Vec<String> = file_entries
        .iter()
        .filter_map(|entry| {
            let os_file_name = entry.file_name();

            let potential_file_name = os_file_name.to_str();
            if potential_file_name.is_none() {
                eprintln!(
                    "Error: could not convert file name to string for entry: {:?}. Skipping...",
                    entry
                );
                return None;
            }

            file_group_length += potential_file_name.unwrap().len();
            Some(potential_file_name.unwrap().to_string())
        })
        .collect();

    write_file_group(
        arguments,
        buffer,
        &file_entries_shortened,
        file_group_length,
    );

    file_entries_shortened.len()
}
