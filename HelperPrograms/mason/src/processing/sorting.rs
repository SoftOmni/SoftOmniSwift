use crate::arguments::arguments::SortingOrder;
use std::cmp::Ordering;
use std::fs;
use std::fs::DirEntry;

pub(super) fn sort_entries(entries: &mut Vec<DirEntry>, sorting_order: &SortingOrder) {
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
}

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
                        "Error: fetching the {} for the file {} failed, therefore {} \
                            will be considered last in ordering compared to entries without issues\n\
                            the error that happened was {}",
                        &time_field_name,
                        &rhs_entry.path().display(),
                        &rhs_entry.path().display(),
                        rhs_time_error
                    );
                    Ordering::Greater
                }
                (Err(lhs_time_error), Ok(_)) => {
                    eprintln!(
                        "Error: fetching the {} for the file {} failed, therefore {} \
                            will be considered last in ordering compared to entries without issues\n\
                            the error that happened was {}",
                        &time_field_name,
                        &lhs_entry.path().display(),
                        &lhs_entry.path().display(),
                        lhs_time_error
                    );
                    Ordering::Less
                }
                (Err(lhs_time_error), Err(rhs_time_error)) => {
                    eprintln!(
                        "Error: fetching the {} for the files {} and {} failed, therefore they \
                             will be considered equal but last overall in ordering compared to entries without issues\n\
                             the errors that happened were \n{} and \n{} respectively.",
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
                "Error: fetching the metadata for the file {} failed, therefore {} will be considered last in ordering compared to entries without issues\n\
                    the error that happened was {}",
                &rhs_entry.path().display(),
                &rhs_entry.path().display(),
                rhs_metadata_error
            );
            Ordering::Greater
        }
        (Err(lhs_metadata_error), Ok(_)) => {
            eprintln!(
                "Error: fetching the metadata for the file {} failed, therefore {} will be considered last in ordering compared to entries without issues\n\
                    the error that happened was {}",
                &lhs_entry.path().display(),
                &lhs_entry.path().display(),
                lhs_metadata_error
            );
            Ordering::Less
        }
        (Err(lhs_metadata_error), Err(rhs_metadata_error)) => {
            eprintln!(
                "Error: fetching the metadata for the files {} and {} failed, therefore {} and {} will be considered equal in ordering compared to entries without issues\n\
                    the errors that happened were \n{} and \n{} respectively",
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
