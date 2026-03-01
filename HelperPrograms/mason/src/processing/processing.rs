use crate::arguments::arguments;
use crate::processing::root_element_processors::{process_file, process_root_directory};
use crate::processing::verbose_logging::{VERBOSE_LOGGER, VerboseLogger, verbose_log};
use std::path::Path;

pub fn process(arguments: &arguments::Arguments) -> usize {
    if arguments.path().is_none() {
        eprintln!(
            "Path argument is required for processing.\n\
        Argument validation and fetching and setting up failed: Path is missing."
        );
        return 0;
    }

    if arguments.is_file_target() {
        return process_file(arguments);
    }

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

    if potential_parent_buffer.is_err() {
        let potential_name = path_buffer.to_str();
    let result = process_root_directory(arguments, Path::new(path_str));

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
    let result = arguments.file_check()(arguments, file_name);

    if !result {
        let full_path_buffer = directory_path.join(file_name);
        let full_path = full_path_buffer.to_str();
        if full_path.is_none() {
            eprintln!("Warning: a file who's name is not valid UTF-8: {:?} was skipped due to not matching the glob or regex pattern", file_name);
            return false;
        }

        let full_path = full_path_buffer.to_str().unwrap();
        eprintln!("Warning, the file '{}' was skipped due to not matching the glob or regex pattern", full_path);
    }

    result
}

fn sort_entries(entries: &mut Vec<DirEntry>, sorting_order: &SortingOrder) {
    entries.sort_by(|lhs_entry, rhs_entry| match sorting_order {
        SortingOrder::Alphabetical => lhs_entry.file_name().cmp(&rhs_entry.file_name()),
        SortingOrder::ReverseAlphabetical => {
            lhs_entry.file_name().cmp(&rhs_entry.file_name()).reverse()
        }
        SortingOrder::DateCreatedMostRecent => metadata_time_based_sort(
            lhs_entry,
            rhs_entry,
            "creation time",
            fs::Metadata::created,
            |lhs_time, rhs_time| lhs_time.cmp(rhs_time),
        ),
        SortingOrder::DateCreatedLeastRecent => metadata_time_based_sort(
            lhs_entry,
            rhs_entry,
            "creation time",
            fs::Metadata::created,
            |lhs_time, rhs_time| lhs_time.cmp(rhs_time).reverse(),
        ),
        SortingOrder::DateModifiedMostRecent => metadata_time_based_sort(
            lhs_entry,
            rhs_entry,
            "modification time",
            fs::Metadata::modified,
            |lhs_time, rhs_time| lhs_time.cmp(rhs_time),
        ),
        SortingOrder::DateModifiedLeastRecent => metadata_time_based_sort(
            lhs_entry,
            rhs_entry,
            "modification time",
            fs::Metadata::modified,
            |lhs_time, rhs_time| lhs_time.cmp(rhs_time).reverse(),
        ),
    });

    fn metadata_time_based_sort<TimeAcquirer, Sorter>(
        lhs_entry: &DirEntry,
        rhs_entry: &DirEntry,
        time_field_name: &str,
        time_acquirer: TimeAcquirer,
        sorter: Sorter,
    ) -> Ordering
    where
        TimeAcquirer: Fn(&fs::Metadata) -> std::io::Result<std::time::SystemTime>,
        Sorter: Fn(&std::time::SystemTime, &std::time::SystemTime) -> Ordering,
    {
        let lhs_metadata_result = lhs_entry.metadata();
        let rhs_metadata_result = rhs_entry.metadata();

        match (lhs_metadata_result, rhs_metadata_result) {
            (Ok(lhs_metadata), Ok(rhs_metadata)) => {
                let lhs_created_result = time_acquirer(&lhs_metadata);
                let rhs_created_result = time_acquirer(&rhs_metadata);

                match (lhs_created_result, rhs_created_result) {
                    (Ok(lhs_time), Ok(rhs_time)) => sorter(&lhs_time, &rhs_time),
                    (Ok(_), Err(rhs_time_error)) => {
                        eprintln!(
                            "Error: Fetching the {} for the file {} failed, therefore {} \
                            will be considered last in ordering compared to entries without issues\n\
                            The error that happened was {}",
                            &time_field_name,
                            &rhs_entry.path().display(),
                            &rhs_entry.path().display(),
                            rhs_time_error
                        );
                        Ordering::Greater
                    }
                    (Err(lhs_time_error), Ok(_)) => {
                        eprintln!(
                            "Error: Fetching the {} for the file {} failed, therefore {} \
                            will be considered last in ordering compared to entries without issues\n\
                            The error that happened was {}",
                            &time_field_name,
                            &lhs_entry.path().display(),
                            &lhs_entry.path().display(),
                            lhs_time_error
                        );
                        Ordering::Less
                    }
                    (Err(lhs_time_error), Err(rhs_time_error)) => {
                        eprintln!(
                            "Error: Fetching the {} for the files {} and {} failed, therefore they \
                             will be considered equal but last overall in ordering compared to entries without issues\n\
                             The errors that happened were \n{} and \n{} respectively.",
                            &time_field_name,
                            &lhs_entry.path().display(),
                            &rhs_entry.path().display(),
                            lhs_time_error,
                            rhs_time_error
                        );
                        Ordering::Equal
                    }
                }
            }
            (Ok(_), Err(rhs_metadata_error)) => {
                eprintln!(
                    "Error: Fetching the metadata for the file {} failed, therefore {} will be considered last in ordering compared to entries without issues\n\
                    The error that happened was {}",
                    &rhs_entry.path().display(),
                    &rhs_entry.path().display(),
                    rhs_metadata_error
                );
                Ordering::Greater
            }
            (Err(lhs_metadata_error), Ok(_)) => {
                eprintln!(
                    "Error: Fetching the metadata for the file {} failed, therefore {} will be considered last in ordering compared to entries without issues\n\
                    The error that happened was {}",
                    &lhs_entry.path().display(),
                    &lhs_entry.path().display(),
                    lhs_metadata_error
                );
                Ordering::Less
            }
            (Err(lhs_metadata_error), Err(rhs_metadata_error)) => {
                eprintln!(
                    "Error: Fetching the metadata for the files {} and {} failed, therefore {} amd {} will be considered equal in ordering compared to entries without issues\n\
                    The errors that happened were \n{} and \n{} respectively",
                    &lhs_entry.path().display(),
                    &rhs_entry.path().display(),
                    &lhs_entry.path().display(),
                    &rhs_entry.path().display(),
                    lhs_metadata_error,
                    rhs_metadata_error
                );
                Ordering::Equal
            }
        }
    }
}
