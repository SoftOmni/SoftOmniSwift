use crate::arguments::arguments;
use crate::processing::processing::{is_ok_name_according_to_filter, LoopSubCallReturnBehavior};
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
    let potential_child_read_dir = directory_entry.path().read_dir();
    if potential_child_read_dir.is_err() {
        eprintln!(
            "Error opening child directory: {:?}. Skipping",
            directory_entry.file_name()
        );
        return 0;
    }

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

    if recursion_depth_limit == 0 {
        return 0;
    }

    let mut child_read_dir = potential_child_read_dir.unwrap();
    let number_of_files_changed: usize = recursive_call(
        arguments,
        &mut child_read_dir,
        recursion_depth_limit - 1,
        path,
    );

    buffer.push_str(&format!("\nsubdir('{}')\n", child_dir_name));

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

    while *index < entries.len() && is_file_name_str_none && !is_directory {
        let entry = &entries[*index];

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

        if !is_ok_name_according_to_filter(
            arguments,
            potential_file_name_str.unwrap(),
            directory_path,
        ) {
            *index += 1;
            continue;
        }

        file_name = potential_file_name_str.unwrap().to_string();
        *index += 1;
    }

    if *index >= entries.len() {
        return LoopSubCallReturnBehavior::Break;
    }

    if is_directory {
        return LoopSubCallReturnBehavior::Continue;
    }

    *file_group_length += file_name.len();
    file_group.push(file_name.clone());

    *index += 1;
    while *index < entries.len() {
        let entry = &entries[*index];

        let file_type = entry.file_type();
        if file_type.is_err() {
            eprintln!(
                "Error getting file type for entry: {:?}. Skipping...",
                entry
            );
            *index += 1;
            continue;
        };

        if file_type.unwrap().is_dir() {
            break;
        }

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

        if !is_ok_name_according_to_filter(
            arguments,
            potential_file_name_str.unwrap(),
            directory_path,
        ) {
            *index += 1;
            continue;
        }

        file_name = potential_file_name_str.unwrap().to_string();
        file_group.push(file_name.clone());
        *file_group_length += file_name.len();
    }

    *file_group_length += 1;

    LoopSubCallReturnBehavior::NormalProcessing
}

pub(super) fn write_file_group(
    arguments: &arguments::Arguments,
    buffer: &mut String,
    file_group: &Vec<String>,
    file_group_length: usize,
) {
    buffer.push_str(arguments.source_variable_name());
    buffer.push_str("+= files(");

    if arguments.forcefully_put_on_distinct_lines()
        && file_group_length <= arguments.max_line_length()
    {
        for (i, file_name) in file_group.iter().enumerate() {
            buffer.push_str(file_name);
            if i < file_group.len() - 1 {
                buffer.push_str(", ");
            }
        }

        buffer.push_str(")\n");
        return;
    }

    buffer.push_str("\n");
    for (index, file_name) in file_group.iter().enumerate() {
        buffer.push_str("    ");
        buffer.push_str(file_name);
        if index < file_group.len() - 1 {
            buffer.push_str(",\n");
        }
    }

    buffer.push_str("\n)\n");
}

pub(super) enum FileWriteResult {
    Success,
    Failure,
}

pub(super) fn write_buffer_to_file(buffer: &mut String, path: &Path) -> FileWriteResult {
    let meson_path = path.join("meson.build");

    let potential_file = File::create(meson_path.as_path());
    if potential_file.is_err() {
        eprintln!(
            "Error failed to create meson.build file at path for directory {}:\n{}",
            path.display(),
            meson_path.to_str().unwrap_or("UNKNOWN PATH")
        );

        return FileWriteResult::Failure;
    }

    let mut file = potential_file.unwrap();

    let write_result = file.write_all(buffer.as_bytes());

    if write_result.is_err() {
        eprintln!(
            "Error failed to write buffer to meson.build file at path: {}",
            path.display()
        );
        return FileWriteResult::Failure;
    }

    FileWriteResult::Success
}
