use crate::arguments::arguments::SortingOrder;
use crate::processing::verbose_logging::verbose_log;
use std::cmp::Ordering;
use std::fs;
use std::fs::DirEntry;

pub(super) fn sort_entries(entries: &mut Vec<DirEntry>, sorting_order: &SortingOrder) {
    verbose_log(&format!(
        "SORTING ENTRIES (FROM SORTER) ACCORDING TO SORTING ORDER {}",
        sorting_order
    ));
    entries.sort_by(|lhs_entry, rhs_entry| {
        verbose_log(&format!(
            "COMPARING '{}' AND '{}' ENTRIES IN SORTING ACCORDING TO SORTING ORDER {}",
            lhs_entry.file_name().to_string_lossy(),
            rhs_entry.file_name().to_string_lossy(),
            sorting_order
        ));

        match sorting_order {
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
        }
    });

    verbose_log(&format!("SORTING COMPLETE (FROM SORTER). SORTED {} ENTRIES", entries.len()));
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
    verbose_log("DEMANDED SORTING ORDER REQUIRES TIME-BASED METADATA ACQUISITION FOR BOTH ENTRIES");
    verbose_log("ACQUIRING METADATA FOR BOTH ENTRIES");

    let lhs_metadata_result = lhs_entry.metadata();
    let rhs_metadata_result = rhs_entry.metadata();

    match (lhs_metadata_result, rhs_metadata_result) {
        (Ok(lhs_metadata), Ok(rhs_metadata)) => {
            verbose_log("METADATA ACQUIRED FOR BOTH ENTRIES");
            verbose_log("ACQUIRING RELEVANT TIME DATA FOR BOTH ENTRIES");
            let lhs_created_result = time_acquirer(&lhs_metadata);
            let rhs_created_result = time_acquirer(&rhs_metadata);

            match (lhs_created_result, rhs_created_result) {
                (Ok(lhs_time), Ok(rhs_time)) => {
                    verbose_log("TIME DATA ACQUIRED FOR BOTH ENTRIES");
                    verbose_log("RETURNING COMPARISON RESULT");
                    sorter(&lhs_time, &rhs_time)
                }
                (Ok(_), Err(rhs_time_error)) => {
                    eprintln!(
                        "ERROR: FETCHING THE {} FOR THE FILE {} FAILED, THEREFORE {} \
                            WILL BE CONSIDERED LAST IN ORDERING COMPARED TO ENTRIES WITHOUT ISSUES\n\
                            THE ERROR THAT HAPPENED WAS {}",
                        &time_field_name,
                        &rhs_entry.path().display(),
                        &rhs_entry.path().display(),
                        rhs_time_error
                    );
                    Ordering::Greater
                }
                (Err(lhs_time_error), Ok(_)) => {
                    eprintln!(
                        "ERROR: FETCHING THE {} FOR THE FILE {} FAILED, THEREFORE {} \
                            WILL BE CONSIDERED LAST IN ORDERING COMPARED TO ENTRIES WITHOUT ISSUES\n\
                            THE ERROR THAT HAPPENED WAS {}",
                        &time_field_name,
                        &lhs_entry.path().display(),
                        &lhs_entry.path().display(),
                        lhs_time_error
                    );
                    Ordering::Less
                }
                (Err(lhs_time_error), Err(rhs_time_error)) => {
                    eprintln!(
                        "ERROR: FETCHING THE {} FOR THE FILES {} AND {} FAILED, THEREFORE THEY \
                             WILL BE CONSIDERED EQUAL BUT LAST OVERALL IN ORDERING COMPARED TO ENTRIES WITHOUT ISSUES\n\
                             THE ERRORS THAT HAPPENED WERE \n{} AND \n{} RESPECTIVELY.",
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
                "ERROR: FETCHING THE METADATA FOR THE FILE {} FAILED, THEREFORE {} WILL BE CONSIDERED LAST IN ORDERING COMPARED TO ENTRIES WITHOUT ISSUES\n\
                    THE ERROR THAT HAPPENED WAS {}",
                &rhs_entry.path().display(),
                &rhs_entry.path().display(),
                rhs_metadata_error
            );
            Ordering::Greater
        }
        (Err(lhs_metadata_error), Ok(_)) => {
            eprintln!(
                "ERROR: FETCHING THE METADATA FOR THE FILE {} FAILED, THEREFORE {} WILL BE CONSIDERED LAST IN ORDERING COMPARED TO ENTRIES WITHOUT ISSUES\n\
                    THE ERROR THAT HAPPENED WAS {}",
                &lhs_entry.path().display(),
                &lhs_entry.path().display(),
                lhs_metadata_error
            );
            Ordering::Less
        }
        (Err(lhs_metadata_error), Err(rhs_metadata_error)) => {
            eprintln!(
                "ERROR: FETCHING THE METADATA FOR THE FILES {} AND {} FAILED, THEREFORE {} AMD {} WILL BE CONSIDERED EQUAL IN ORDERING COMPARED TO ENTRIES WITHOUT ISSUES\n\
                    THE ERRORS THAT HAPPENED WERE \n{} AND \n{} RESPECTIVELY",
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
