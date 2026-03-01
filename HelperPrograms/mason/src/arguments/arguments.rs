use glob;
use regex::Regex;
use std::cmp::PartialEq;
use std::env;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub enum FileFolderItemLocation {
    FolderFirst,
    FileFirst,
    InOrderOfSortingAlgorithm,
}

impl PartialEq for FileFolderItemLocation {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

pub enum SortingOrder {
    Alphabetical,
    ReverseAlphabetical,

    DateCreatedMostRecent,
    DateCreatedLeastRecent,

    DateModifiedMostRecent,
    DateModifiedLeastRecent,
}

impl PartialEq for SortingOrder {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Display for SortingOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SortingOrder::Alphabetical => write!(f, "Alphabetical"),
            SortingOrder::ReverseAlphabetical => write!(f, "Reverse Alphabetical"),
            SortingOrder::DateCreatedMostRecent => write!(f, "Date Created Most Recent"),
            SortingOrder::DateCreatedLeastRecent => write!(f, "Date Created Least Recent"),
            SortingOrder::DateModifiedMostRecent => write!(f, "Date Modified Most Recent"),
            SortingOrder::DateModifiedLeastRecent => write!(f, "Date Modified Least Recent"),
        }
    }
}

pub enum InformationKind {
    Help,
    Version,
    License,
}

impl PartialEq for InformationKind {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

pub enum TargetKind {
    Folder,
    File,
}

pub struct Arguments {
    file_folder_item_location: FileFolderItemLocation,
    path: Option<String>,
    sorting_order: SortingOrder,
    target_kind: TargetKind,
    information_request: Option<InformationKind>,
    max_line_length: usize,
    forcefully_put_on_distinct_lines: bool,
    glob: glob::Pattern,
    pattern: Regex,
    file_check: fn(&Arguments, &str) -> bool,
    source_variable_name: String,
    max_depth: usize,
    verbose_output: bool,
}

impl PartialEq for ArgumentResult {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

impl Arguments {
    fn default_line_length() -> usize {
        80
    }

    fn default_glob() -> glob::Pattern {
        glob::Pattern::new("*.cc").unwrap()
    }

    fn default_pattern() -> Regex {
        Regex::new(r".*\.cc$").unwrap()
    }

    fn default_source_variable_name() -> String {
        "sources".to_string()
    }

    fn new() -> Self {
        Arguments {
            file_folder_item_location: FileFolderItemLocation::InOrderOfSortingAlgorithm,
            path: None,
            sorting_order: SortingOrder::Alphabetical,
            target_kind: TargetKind::Folder,
            information_request: None,
            file_check: Arguments::glob_check,
            glob: Self::default_glob(),
            pattern: Self::default_pattern(),
            max_depth: usize::MAX,
            max_line_length: Self::default_line_length(),
            forcefully_put_on_distinct_lines: false,
            source_variable_name: Self::default_source_variable_name(),
            verbose_output: false,
        }
    }

    fn new_help() -> Self {
        Self::new_informational(InformationKind::Help)
    }

    fn new_version() -> Self {
        Self::new_informational(InformationKind::Version)
    }

    fn new_license() -> Self {
        Self::new_informational(InformationKind::License)
    }

    fn new_informational(information_kind: InformationKind) -> Self {
        Arguments {
            file_folder_item_location: FileFolderItemLocation::InOrderOfSortingAlgorithm,
            path: None,
            sorting_order: SortingOrder::Alphabetical,
            target_kind: TargetKind::Folder,
            information_request: Some(information_kind),
            file_check: Arguments::glob_check,
            glob: Self::default_glob(),
            pattern: Self::default_pattern(),
            max_depth: usize::MAX,
            max_line_length: Self::default_line_length(),
            forcefully_put_on_distinct_lines: false,
            source_variable_name: Self::default_source_variable_name(),
            verbose_output: false,
        }
    }

    pub fn file_folder_item_location(&self) -> &FileFolderItemLocation {
        &self.file_folder_item_location
    }

    pub fn path(&self) -> &Option<String> {
        &self.path
    }

    pub fn sorting_order(&self) -> &SortingOrder {
        &self.sorting_order
    }

    pub fn is_help(&self) -> bool {
        matches!(self.information_request, Some(InformationKind::Help))
    }

    pub fn is_version(&self) -> bool {
        matches!(self.information_request, Some(InformationKind::Version))
    }

    pub fn is_license(&self) -> bool {
        matches!(self.information_request, Some(InformationKind::License))
    }

    pub fn max_line_length(&self) -> usize {
        self.max_line_length
    }

    pub fn forcefully_put_on_distinct_lines(&self) -> bool {
        self.forcefully_put_on_distinct_lines
    }

    pub fn file_check(&self) -> fn(&Arguments, &str) -> bool {
        self.file_check
    }

    pub fn source_variable_name(&self) -> &str {
        &self.source_variable_name
    }

    pub fn max_depth(&self) -> usize {
        self.max_depth
    }

    fn glob_check(&self, file_name: &str) -> bool {
        self.glob.matches(file_name)
    }

    fn pattern_check(&self, file_name: &str) -> bool {
        self.pattern.is_match(file_name)
    }

    pub fn is_file_target(&self) -> bool {
        matches!(self.target_kind, TargetKind::File)
    }

    pub fn parse_from_standard_in() -> Option<Self> {
        let arguments: Vec<String> = env::args().collect();

        if arguments.is_empty() {
            return Some(Self::new());
        }

        let mut state: ArgumentParserState = ArgumentParserState::new();
        let mut index: usize = 0;
        let mut result: ArgumentResult = ArgumentResult::OK;
        while index < arguments.len() && result == ArgumentResult::OK {
            result = Self::parse_argument(&arguments[index], index, &mut state);
            index += 1;
        }

        match result {
            ArgumentResult::OK => Some(state.to_arguments()),
            ArgumentResult::Error => None,
            ArgumentResult::Help => {
                if arguments.len() > 1 {
                    eprintln!("The help argument cannot be specified with other arguments");
                    return None;
                }

                Some(Self::new_help())
            }
            ArgumentResult::Version => {
                if arguments.len() > 1 {
                    eprintln!("The version argument cannot be specified with other arguments");
                    return None;
                }

                Some(Self::new_version())
            }
            ArgumentResult::License => {
                if arguments.len() > 1 {
                    eprintln!("The license argument cannot be specified with other arguments");
                    return None;
                }

                Some(Self::new_license())
            }
        }
    }

    fn parse_argument(
        argument: &str,
        index: usize,
        state: &mut ArgumentParserState,
    ) -> ArgumentResult {
        if !argument.starts_with("-") {
            return Self::process_path_argument(argument, index, state);
        }

        if argument.len() == 1 {
            eprintln!(
                "Error: '-' you cannot simply pass an empty dash as an argument. Please provide a valid argument."
            );

            return ArgumentResult::Error;
        }

        let mut characters = argument.chars();
        let second_character = characters.nth(1).unwrap();
        if second_character != '-' {
            return Self::process_flags(argument, index, state);
        }

        Self::process_named_argument(argument, index, state)
    }

    fn process_path_argument(
        argument: &str,
        index: usize,
        state: &mut ArgumentParserState,
    ) -> ArgumentResult {
        if state.has_path_been_seen() {
            eprintln!(
                "Error: you have already provided a path argument as argument n°${}.\nYou cannot have two path arguments",
                state.path_argument_index()
            );

            return ArgumentResult::Error;
        }

        let potential_path: Option<(PathBuf, TargetKind)> = Self::load_and_validate_path(&argument);

        if potential_path.is_none() {
            return ArgumentResult::Error;
        }

        state.path_seen(index, potential_path.unwrap());
        ArgumentResult::OK
    }

    fn process_named_argument(
        argument: &str,
        index: usize,
        state: &mut ArgumentParserState,
    ) -> ArgumentResult {
        let potential_named_argument_components = Self::extract_named_argument_components(argument);
        if potential_named_argument_components.is_none() {
            return ArgumentResult::Error;
        }

        let (argument_name, argument_value) = potential_named_argument_components.unwrap();

        match argument_name.as_str() {
            "sorting_order" | "sorting-order" | "sorting" => {
                if state.has_sorting_order_been_seen() {
                    eprintln!(
                        "The sorting of elements can only be specified once but it was already specified as argument n°{}",
                        state.sorting_order_index()
                    );
                    return ArgumentResult::Error;
                }

                Self::process_sorting_named_argument_value(&argument_value, index, state)
            }
            "files_and_folder_elements_ordering"
            | "files-and-folder-elements-ordering"
            | "child_ordering"
            | "child-ordering"
            | "ordering" => {
                if state.has_files_or_folders_first_been_seen() {
                    eprintln!(
                        "The files and folders ordering argument can only be specified once but it was already specified as argument n°{}",
                        state.files_or_folder_index()
                    );
                    return ArgumentResult::Error;
                }

                Self::process_ordering_named_argument_value(&argument_value, index, state)
            }
            "max-allowed-single-line-files-declaration"
            | "max_allowed_single_line_files_declaration" => {
                if state.has_max_single_line_length_been_seen() {
                    eprintln!(
                        "The max allowed single line files declaration argument can only be specified once but it was already specified as argument n°{}",
                        state.max_single_line_length_index()
                    );
                    return ArgumentResult::Error;
                }

                Self::process_max_single_line_length_argument_value(&argument_value, index, state)
            }
            "max-depth" | "max_depth" => {
                if state.has_max_depth_been_seen() {
                    eprintln!(
                        "The max depth argument can only be specified once but it was already specified as argument n°{}",
                        state.max_depth_index()
                    );
                    return ArgumentResult::Error;
                }

                Self::process_max_depth_named_argument_value(&argument_value, index, state)
            }
            "glob" => {
                if state.has_file_pattern_been_seen() {
                    eprintln!(
                        "The glob and pattern arguments are mutually exclusive and cannot both be specified. \
                        A file pattern has already been registered as argument n°{}",
                        state.file_pattern_index()
                    );
                    return ArgumentResult::Error;
                }

                if state.has_file_glob_been_seen() {
                    eprintln!(
                        "The glob argument has already been registered as argument n°{}",
                        state.file_glob_index()
                    )
                }

                Self::process_glob_named_argument_value(&argument_value, index, state)
            }
            "pattern" => {
                if state.has_file_glob_been_seen() {
                    eprintln!(
                        "The glob and pattern arguments are mutually exclusive and cannot both be specified. \
                        A glob pattern has already been registered as argument n°{}",
                        state.file_glob_index()
                    );
                    return ArgumentResult::Error;
                }

                if state.has_file_pattern_been_seen() {
                    eprintln!(
                        "The pattern argument has already been registered as argument n°{}",
                        state.file_pattern_index()
                    )
                }

                Self::process_pattern_named_argument_value(&argument_value, index, state)
            }
            "sources-variable-name" | "sources_variable_name" => {
                if state.has_source_variable_name_been_seen() {
                    eprintln!(
                        "The sources variable name argument has already been specified as arg n°{} \
                        It cannot be specified multiple times.",
                        state.source_variable_name_index()
                    );
                    return ArgumentResult::Error;
                }

                Self::process_source_variable_name_argument_value(&argument_value, index, state)
            }
            "forcefully_put_file_declarations_on_separate_lines" => {
                if state.has_forcefully_put_on_distinct_lines_been_seen() {
                    eprintln!(
                        "The forcefully put file declarations on separate lines argument has already been specified as arg n°{}. \
                        It cannot be specified multiple times.",
                        state.forcefully_put_on_distinct_lines_index()
                    );
                    return ArgumentResult::Error;
                }

                state.forcefully_put_on_distinct_lines_seen(index);
                ArgumentResult::OK
            }
            "verbose" => {
                if state.has_verbose_been_seen() {
                    eprintln!(
                        "The verbose argument has already been specified as arg n°{}.\
                        It cannot be specified multiple times.",
                        state.verbose_index()
                    );
                    return ArgumentResult::Error;
                }

                state.verbose_seen(index);
                ArgumentResult::OK
            }
            "help" => ArgumentResult::Help,
            "version" => ArgumentResult::Version,
            "license" => ArgumentResult::License,
            &_ => {
                eprintln!(
                    "Your argument name \"{}\" isn't the name of a valid argument.\n\
                    The valid arguments name are 'sorting_order' (or 'sorting-order' or 'sorting'), \
                    'files_and_folder_elements_ordering' (or 'files-and-folder-elements-ordering' \
                    or 'child_ordering' or 'child-ordering' or 'ordering').\n\
                    Please use --help or consult the man page to see the valid arguments",
                    argument_name
                );
                ArgumentResult::Error
            }
        }
    }

    fn process_flags(
        argument: &str,
        index: usize,
        state: &mut ArgumentParserState,
    ) -> ArgumentResult {
        let mut characters: std::str::Chars = argument.chars();
        let mut character = characters.nth(1).unwrap();
        loop {
            match character {
                'A' | 'a' => {
                    if state.has_sorting_order_been_seen() {
                        eprintln!(
                            "Error: sorting order already specified at argument n°{}",
                            state.sorting_order_index()
                        );
                        return ArgumentResult::Error;
                    }

                    state.sorting_order_seen(index, SortingOrder::Alphabetical)
                }
                'Z' | 'z' => {
                    if state.has_sorting_order_been_seen() {
                        eprintln!(
                            "Error: sorting order already specified at argument n°{}",
                            state.sorting_order_index()
                        );
                        return ArgumentResult::Error;
                    }

                    state.sorting_order_seen(index, SortingOrder::ReverseAlphabetical)
                }
                'C' => {
                    if state.has_sorting_order_been_seen() {
                        eprintln!(
                            "Error: sorting order already specified at argument n°{}",
                            state.sorting_order_index()
                        );
                        return ArgumentResult::Error;
                    }

                    state.sorting_order_seen(index, SortingOrder::DateCreatedMostRecent)
                }
                'c' => {
                    if state.has_sorting_order_been_seen() {
                        eprintln!(
                            "Error: sorting order already specified at argument n°{}",
                            state.sorting_order_index()
                        );
                        return ArgumentResult::Error;
                    }

                    state.sorting_order_seen(index, SortingOrder::DateCreatedLeastRecent)
                }
                'M' => {
                    if state.has_sorting_order_been_seen() {
                        eprintln!(
                            "Error: sorting order already specified at argument n°{}",
                            state.sorting_order_index()
                        );
                        return ArgumentResult::Error;
                    }

                    state.sorting_order_seen(index, SortingOrder::DateModifiedMostRecent)
                }
                'm' => {
                    if state.has_sorting_order_been_seen() {
                        eprintln!(
                            "Error: sorting order already specified at argument n°{}",
                            state.sorting_order_index()
                        );
                        return ArgumentResult::Error;
                    }

                    state.sorting_order_seen(index, SortingOrder::DateModifiedLeastRecent)
                }
                'D' | 'd' => {
                    if state.has_files_or_folders_first_been_seen() {
                        eprintln!(
                            "Error: file/folder order already specified at argument n°{}",
                            state.files_or_folder_index()
                        );
                        return ArgumentResult::Error;
                    }

                    state.files_or_folders_first_seen(index, FileFolderItemLocation::FolderFirst)
                }
                'F' | 'f' => {
                    if state.has_files_or_folders_first_been_seen() {
                        eprintln!(
                            "Error: file/folder order already specified at argument n°{}",
                            state.files_or_folder_index()
                        );
                        return ArgumentResult::Error;
                    }

                    state.files_or_folders_first_seen(index, FileFolderItemLocation::FileFirst)
                }
                'I' | 'i' => {
                    if state.has_files_or_folders_first_been_seen() {
                        eprintln!(
                            "Error: file/folder order already specified at argument n°{}",
                            state.files_or_folder_index()
                        );
                        return ArgumentResult::Error;
                    }

                    state.files_or_folders_first_seen(
                        index,
                        FileFolderItemLocation::InOrderOfSortingAlgorithm,
                    )
                }
                'S' | 's' => {
                    if state.has_forcefully_put_on_distinct_lines_been_seen() {
                        eprintln!(
                            "Error: the forcefully put file declarations on separate lines flag has already been specified at argument n°{}",
                            state.forcefully_put_on_distinct_lines_index()
                        );

                        return ArgumentResult::Error;
                    }

                    state.forcefully_put_on_distinct_lines_seen(index);
                }
                'V' | 'v' => {
                    if state.has_verbose_been_seen() {
                        eprintln!(
                            "Error: the verbose flag has already been specified at argument n°{}",
                            state.verbose_index()
                        );

                        return ArgumentResult::Error;
                    }

                    state.verbose_seen(index);
                }
                _ => {
                    eprintln!("Error: unknown flag '{}'", character);
                    return ArgumentResult::Error;
                }
            }

            if let Some(next_char) = characters.next() {
                character = next_char;
            } else {
                break;
            }
        }

        ArgumentResult::OK
    }

    fn load_and_validate_path(path_argument: &str) -> Option<(PathBuf, TargetKind)> {
        let path: &Path = Path::new(&path_argument);
        if !path.exists() {
            eprintln!(
                "Error: the first argument since it does not start with - is considered a path for the program to operate from.\n\
                Your path ('{path_argument}') does not exist!"
            );

            return None;
        }

        if path.is_dir() {
            Some((path.to_path_buf(), TargetKind::Folder))
        } else if path.is_file() {
            if path
                .file_name()
                .is_some_and(|name| name.to_str().is_some_and(|name| name != "meson.build"))
            {
                eprintln!(
                    "Error: the first argument since it does not start with - is considered a path for the program to operate from.\n\
                    Your path ('{path_argument}') is a file but not a meson.build file!"
                );
                return None;
            }

            Some((path.to_path_buf(), TargetKind::File))
        } else {
            eprintln!(
                "Error: the first argument since it does not start with - is considered a path for the program to operate from.\n\
                Your path ('{path_argument}') is neither a file nor a directory!"
            );
            None
        }
    }

    fn extract_named_argument_components(argument: &str) -> Option<(String, String)> {
        let mut characters = argument.chars();
        let _ = characters.nth(1);

        let mut character = characters.next().unwrap();

        let mut argument_name = "".to_string();
        let mut argument_name_index = 0;
        argument_name.reserve(argument.len());
        while Self::is_valid_argument_character_long_options(character) {
            argument_name.push(character);
            argument_name_index += 1;
            let potential_character = characters.next();
            if potential_character.is_none() {
                break;
            }

            character = potential_character.unwrap();
        }

        if Self::is_valid_argument_character_long_options(character) {
            if argument_name == "help" || argument_name == "version" || argument_name == "license" {
                return Some((argument_name, "".parse().unwrap()));
            }

            eprintln!(
                "The specified argument is malformed. When starting with --, \
                    the argument must only contain alphanumeric characters, hyphens, \
                    and underscores and then an equal sign followed by a value.\
                    Your name is correct but the argument ended without an equal sign and a value"
            );
            return None;
        }

        if character != '=' {
            eprintln!(
                "The specified argument is malformed. When starting with --, \
                    the argument must only contain alphanumeric characters, hyphens, \
                    and underscores and then an equal sign followed by a value.\
                    You did not pass a valid character in the argument name,\
                    instead providing '{}' at index {}",
                character,
                2 + argument_name_index
            );
            return None;
        }

        let mut argument_value = "".to_string();
        argument_value.reserve(argument.len());
        let mut argument_value_index: usize = 0;

        while Self::is_valid_argument_character_long_options(character) {
            argument_value.push(character);
            argument_value_index += 1;
            let potential_character = characters.next();
            if potential_character.is_none() {
                break;
            }

            character = potential_character.unwrap();
        }

        if !Self::is_valid_argument_character_long_options(character) {
            eprintln!(
                "The specified argument is malformed. When starting with --, \
                    the argument must only contain alphanumeric characters, hyphens, \
                    and underscores and then an equal sign followed by a value.\n\
                    Your value had an error at index {} (character '{}', ({} as the argument text index \
                    (includes the -- at the start and the '=' and the name)))",
                argument_value_index,
                character,
                2 + argument_name.len() + 1 + argument_value_index
            );
            return None;
        }

        Some((argument_name, argument_value))
    }

    fn process_sorting_named_argument_value(
        argument_value: &String,
        index: usize,
        state: &mut ArgumentParserState,
    ) -> ArgumentResult {
        match argument_value.as_str() {
            "alphabetical" => {
                state.sorting_order_seen(index, SortingOrder::Alphabetical);
                ArgumentResult::OK
            }
            "reverse_alphabetical" | "reverse-alphabetical" => {
                state.sorting_order_seen(index, SortingOrder::ReverseAlphabetical);
                ArgumentResult::OK
            }
            "by_date_created_most_recent" | "by-date-created-most-recent" => {
                state.sorting_order_seen(index, SortingOrder::DateCreatedMostRecent);
                ArgumentResult::OK
            }
            "by_date_created_oldest" | "by-date-created-oldest" => {
                state.sorting_order_seen(index, SortingOrder::DateCreatedLeastRecent);
                ArgumentResult::OK
            }
            "by_date_modified_most_recent" | "by-date-modified-most-recent" => {
                state.sorting_order_seen(index, SortingOrder::DateModifiedMostRecent);
                ArgumentResult::OK
            }
            "by_date_modified_oldest" | "by-date-modified-oldest" => {
                state.sorting_order_seen(index, SortingOrder::DateModifiedLeastRecent);
                ArgumentResult::OK
            }
            &_ => {
                eprintln!(
                    "Error: Invalid value for 'sorting_order' argument. \
                Valid values are 'alphabetical', 'reverse_alphabetical' (or 'reverse-alphabetical'), \
                'by_date_created_most_recent' (or 'by-date-created-most-recent'), \
                'by_date_created_oldest' (or 'by-date-created-oldest'), \
                'by_date_modified_most_recent' (or 'by-date-modified-most-recent'), \
                'by_date_modified_oldest' (or 'by-date-modified-oldest').\n\
                For more information about possible arguments, do --help or check the man page"
                );
                ArgumentResult::Error
            }
        }
    }

    fn process_ordering_named_argument_value(
        argument_value: &String,
        index: usize,
        state: &mut ArgumentParserState,
    ) -> ArgumentResult {
        match argument_value.as_str() {
            "files_first" | "files-first" => {
                state.files_or_folders_first_seen(index, FileFolderItemLocation::FileFirst);
                ArgumentResult::OK
            }
            "folders_first" | "folders-first" => {
                state.files_or_folders_first_seen(index, FileFolderItemLocation::FolderFirst);
                ArgumentResult::OK
            }
            "in_order" | "in-order" => {
                state.files_or_folders_first_seen(
                    index,
                    FileFolderItemLocation::InOrderOfSortingAlgorithm,
                );
                ArgumentResult::OK
            }
            &_ => {
                eprintln!(
                    "Error: Invalid value for 'files_and_folder_elements_ordering' argument. \
                Valid values are 'files_first' (or 'files-first'), \
                'folders_first' (or 'folders-first'), and \
                'in_order' (or in-order).\n\
                For more information about possible arguments, do --help or check the man page"
                );
                ArgumentResult::Error
            }
        }
    }

    fn process_max_single_line_length_argument_value(
        argument_value: &String,
        index: usize,
        state: &mut ArgumentParserState,
    ) -> ArgumentResult {
        let result = usize::from_str(argument_value);
        if result.is_err() {
            eprintln!(
                "Error: Invalid value for 'max-allowed-single-line-files-declaration' argument at index {index}. \
                Expected a positive integer (0 is ok) (it must be less or equal to the maximum value of {}), \
                got '{argument_value}'",
                usize::MAX
            );
            return ArgumentResult::Error;
        }

        state.max_single_line_length_seen(index, result.unwrap());
        ArgumentResult::OK
    }

    fn process_max_depth_named_argument_value(
        argument_value: &String,
        index: usize,
        state: &mut ArgumentParserState,
    ) -> ArgumentResult {
        let result = usize::from_str(argument_value);
        if result.is_err() {
            eprintln!(
                "Error: Invalid value for 'max_depth' argument at index {index}. \
                Expected a positive integer (0 is ok) (it must be less or equal to the maximum value of {}), \
                got '{argument_value}'",
                usize::MAX
            );
            return ArgumentResult::Error;
        }

        state.max_depth_seen(index, result.unwrap());
        ArgumentResult::OK
    }

    fn process_glob_named_argument_value(
        argument_value: &String,
        index: usize,
        state: &mut ArgumentParserState,
    ) -> ArgumentResult {
        let result = glob::Pattern::new(argument_value);
        if result.is_err() {
            eprintln!(
                "Error: Invalid value for 'glob' argument at index {index}.\n\
                The glob pattern is not a valid glob pattern because:\n{}.",
                result.unwrap_err()
            );
            return ArgumentResult::Error;
        }

        state.file_glob_seen(index, result.unwrap());
        ArgumentResult::OK
    }

    fn process_pattern_named_argument_value(
        argument_value: &String,
        index: usize,
        state: &mut ArgumentParserState,
    ) -> ArgumentResult {
        let result = Regex::new(argument_value);
        if result.is_err() {
            eprintln!(
                "Error: Invalid value for 'pattern' argument at index {index}.\n\
                The regular expression pattern is not a valid regular expression pattern because:\n{}.",
                result.unwrap_err()
            );
            return ArgumentResult::Error;
        }

        state.file_pattern_seen(index, result.unwrap());
        ArgumentResult::OK
    }

    fn process_source_variable_name_argument_value(
        argument_value: &String,
        index: usize,
        state: &mut ArgumentParserState,
    ) -> ArgumentResult {
        const REGEX_PATTERN: &str = r"^[a-zA-Z_][a-zA-Z0-9_]*$";

        let mut characters = argument_value.chars();
        let potential_first_char = characters.next();
        if potential_first_char.is_none() {
            eprintln!(
                "Error: Invalid value for 'source_variable_name' argument at index {index}. \
                The source variable name cannot be empty.\n\
                It must be compliant with meson's rules for valid identifiers which state that it must \
                follow the pattern: {REGEX_PATTERN}"
            );
            return ArgumentResult::Error;
        }

        let first_char = characters.next().unwrap();
        if !first_char.is_ascii_alphabetic() && first_char != '_' {
            eprintln!(
                "Error: Invalid value for 'source_variable_name' argument at index {index}. \
                The source variable name must start with an uppercase or \
                lower ascii letter or an underscore.\n\
                It must be compliant with meson's rules for valid identifiers which state that it must \
                follow the pattern: {REGEX_PATTERN}"
            );
            return ArgumentResult::Error;
        }

        let mut potential_char = characters.next();
        while potential_char.is_some() {
            let character = potential_char.unwrap();

            if !character.is_ascii_alphanumeric() && character != '_' {
                eprintln!(
                    "Error: Invalid value for 'source_variable_name' argument at index {index}. \
                    The source variable name must only contain ascii letters, digits, or underscores.\n\
                    It must be compliant with meson's rules for valid identifiers which state that it must \
                    follow the pattern: {REGEX_PATTERN}"
                );
                return ArgumentResult::Error;
            }

            potential_char = characters.next();
        }

        state.source_variable_name_seen(index, argument_value.clone());
        ArgumentResult::OK
    }

    fn is_valid_argument_character_long_options(character: char) -> bool {
        character.is_ascii_alphabetic() || character == '-' || character == '_'
    }
}

enum ArgumentResult {
    OK,
    Error,
    Help,
    Version,
    License,
}

struct ArgumentParserState {
    path_seen: bool,
    path_argument_index: usize,
    path_object: (PathBuf, TargetKind),

    sorting_order_seen: bool,
    sorting_order_seen_index: usize,
    sorting_order_seen_case: SortingOrder,

    files_or_folders_first_seen: bool,
    files_or_folders_first_seen_index: usize,
    files_or_folders_first_seen_case: FileFolderItemLocation,

    max_line_length_seen: bool,
    max_line_argument_index: usize,
    max_line_length: usize,

    forcefully_put_on_distinct_lines: bool,
    forcefully_put_on_distinct_lines_argument_index: usize,
    forcefully_put_on_distinct_lines_case: bool,

    file_glob_seen: bool,
    file_glob_argument_index: usize,
    file_glob: glob::Pattern,

    file_pattern_seen: bool,
    file_pattern_argument_index: usize,
    file_pattern: Regex,

    max_depth_seen: bool,
    max_depth_argument_index: usize,
    max_depth: usize,

    source_variable_name_seen: bool,
    source_variable_name_argument_index: usize,
    source_variable_name: String,

    verbose_seen: bool,
    verbose_index: usize,
    verbose_case: bool,
}

impl ArgumentParserState {
    fn new() -> Self {
        ArgumentParserState {
            path_seen: false,
            path_argument_index: 0,
            path_object: (Default::default(), TargetKind::Folder),

            sorting_order_seen: false,
            sorting_order_seen_index: 0,
            sorting_order_seen_case: SortingOrder::Alphabetical,

            files_or_folders_first_seen: false,
            files_or_folders_first_seen_index: 0,
            files_or_folders_first_seen_case: FileFolderItemLocation::FolderFirst,

            max_line_length_seen: false,
            max_line_argument_index: 0,
            max_line_length: 0,

            forcefully_put_on_distinct_lines: false,
            forcefully_put_on_distinct_lines_argument_index: 0,
            forcefully_put_on_distinct_lines_case: false,

            file_glob_seen: false,
            file_glob_argument_index: 0,
            file_glob: Arguments::default_glob(),

            file_pattern_seen: false,
            file_pattern_argument_index: 0,
            file_pattern: Regex::new(r"/").unwrap(),

            max_depth_seen: false,
            max_depth_argument_index: 0,
            max_depth: usize::MAX,

            source_variable_name_seen: false,
            source_variable_name_argument_index: 0,
            source_variable_name: "sources".to_string(),

            verbose_seen: false,
            verbose_index: 0,
            verbose_case: false,
        }
    }

    fn has_path_been_seen(&self) -> bool {
        self.path_seen
    }

    fn path_seen(&mut self, index: usize, path_object: (PathBuf, TargetKind)) {
        self.path_seen = true;
        self.path_argument_index = index;
        self.path_object = path_object;
    }

    fn path_argument_index(&self) -> usize {
        self.path_argument_index
    }

    fn has_sorting_order_been_seen(&self) -> bool {
        self.sorting_order_seen
    }

    fn sorting_order_seen(&mut self, index: usize, sorting_order: SortingOrder) {
        self.sorting_order_seen = true;
        self.sorting_order_seen_index = index;
        self.sorting_order_seen_case = sorting_order;
    }

    fn sorting_order_index(&self) -> usize {
        self.sorting_order_seen_index
    }

    fn has_files_or_folders_first_been_seen(&self) -> bool {
        self.files_or_folders_first_seen
    }

    fn files_or_folders_first_seen(
        &mut self,
        index: usize,
        files_or_folders_first: FileFolderItemLocation,
    ) {
        self.files_or_folders_first_seen = true;
        self.files_or_folders_first_seen_index = index;
        self.files_or_folders_first_seen_case = files_or_folders_first;
    }

    fn files_or_folder_index(&self) -> usize {
        self.files_or_folders_first_seen_index
    }

    fn has_max_single_line_length_been_seen(&self) -> bool {
        self.max_line_length_seen
    }

    fn max_single_line_length_seen(&mut self, index: usize, max_line_length: usize) {
        self.max_line_length_seen = true;
        self.max_line_argument_index = index;
        self.max_line_length = max_line_length;
    }

    fn max_single_line_length_index(&self) -> usize {
        self.max_line_argument_index
    }

    fn has_forcefully_put_on_distinct_lines_been_seen(&self) -> bool {
        self.forcefully_put_on_distinct_lines
    }

    fn forcefully_put_on_distinct_lines_seen(&mut self, index: usize) {
        self.forcefully_put_on_distinct_lines = true;
        self.forcefully_put_on_distinct_lines_argument_index = index;
        self.forcefully_put_on_distinct_lines_case = true;
    }

    fn forcefully_put_on_distinct_lines_index(&self) -> usize {
        self.forcefully_put_on_distinct_lines_argument_index
    }

    fn has_file_glob_been_seen(&self) -> bool {
        self.file_glob_seen
    }

    fn file_glob_seen(&mut self, index: usize, file_glob: glob::Pattern) {
        self.file_glob_seen = true;
        self.file_glob_argument_index = index;
        self.file_glob = file_glob;
    }

    fn file_glob_index(&self) -> usize {
        self.file_glob_argument_index
    }

    fn has_file_pattern_been_seen(&self) -> bool {
        self.file_pattern_seen
    }

    fn file_pattern_seen(&mut self, index: usize, file_pattern: Regex) {
        self.file_pattern_seen = true;
        self.file_pattern_argument_index = index;
        self.file_pattern = file_pattern;
    }

    fn file_pattern_index(&self) -> usize {
        self.file_pattern_argument_index
    }

    fn has_max_depth_been_seen(&self) -> bool {
        self.max_depth_seen
    }

    fn max_depth_seen(&mut self, index: usize, max_depth: usize) {
        self.max_depth_seen = true;
        self.max_depth_argument_index = index;
        self.max_depth = max_depth;
    }

    fn max_depth_index(&self) -> usize {
        self.max_depth_argument_index
    }

    fn has_source_variable_name_been_seen(&self) -> bool {
        self.source_variable_name_seen
    }

    fn source_variable_name_seen(&mut self, index: usize, source_variable_name: String) {
        self.source_variable_name_seen = true;
        self.source_variable_name_argument_index = index;
        self.source_variable_name = source_variable_name;
    }

    fn source_variable_name_index(&self) -> usize {
        self.source_variable_name_argument_index
    }

    fn has_verbose_been_seen(&self) -> bool {
        self.verbose_seen
    }

    fn verbose_seen(&mut self, index: usize) {
        self.verbose_seen = true;
        self.verbose_index = index;
        self.verbose_case = true;
    }

    fn verbose_index(&self) -> usize {
        self.verbose_index
    }

    fn to_arguments(&self) -> Arguments {
        let path: Option<String> = if self.path_seen {
            Some(self.path_object.0.to_string_lossy().to_string())
        } else {
            None
        };

        let target_kind: TargetKind = if self.path_seen {
            match self.path_object.1 {
                TargetKind::Folder => TargetKind::Folder,
                TargetKind::File => TargetKind::File,
            }
        } else {
            TargetKind::Folder
        };

        let sorting_order: SortingOrder = if self.sorting_order_seen {
            match self.sorting_order_seen_case {
                SortingOrder::Alphabetical => SortingOrder::Alphabetical,
                SortingOrder::ReverseAlphabetical => SortingOrder::ReverseAlphabetical,
                SortingOrder::DateCreatedMostRecent => SortingOrder::DateCreatedMostRecent,
                SortingOrder::DateCreatedLeastRecent => SortingOrder::DateCreatedLeastRecent,
                SortingOrder::DateModifiedMostRecent => SortingOrder::DateModifiedMostRecent,
                SortingOrder::DateModifiedLeastRecent => SortingOrder::DateModifiedLeastRecent,
            }
        } else {
            SortingOrder::Alphabetical
        };

        let file_folder_item_location: FileFolderItemLocation = if self.files_or_folders_first_seen
        {
            match self.files_or_folders_first_seen_case {
                FileFolderItemLocation::FolderFirst => FileFolderItemLocation::FolderFirst,
                FileFolderItemLocation::FileFirst => FileFolderItemLocation::FileFirst,
                FileFolderItemLocation::InOrderOfSortingAlgorithm => {
                    FileFolderItemLocation::InOrderOfSortingAlgorithm
                }
            }
        } else {
            FileFolderItemLocation::FolderFirst
        };

        let max_line_length: usize = if self.max_line_length_seen {
            self.max_line_length
        } else {
            80
        };

        let forcefully_put_on_distinct_lines: bool = if self.forcefully_put_on_distinct_lines {
            self.forcefully_put_on_distinct_lines_case
        } else {
            false
        };

        let source_variable_name: String = if self.source_variable_name_seen {
            self.source_variable_name.clone()
        } else {
            Arguments::default_source_variable_name()
        };

        let max_depth: usize = if self.max_depth_seen {
            self.max_depth
        } else {
            usize::MAX
        };

        let (glob, pattern, file_check): (glob::Pattern, Regex, fn(&Arguments, &str) -> bool) =
            if self.file_pattern_seen {
                (
                    glob::Pattern::new("c").unwrap(),
                    self.file_pattern.clone(),
                    Arguments::pattern_check,
                )
            } else {
                (
                    self.file_glob.clone(),
                    Regex::new(r"/").unwrap(),
                    Arguments::glob_check,
                )
            };

        let verbose_output: bool = if self.verbose_seen {
            self.verbose_case
        } else {
            false
        };

        let arguments = Arguments {
            path,
            target_kind,
            sorting_order,
            file_folder_item_location,
            information_request: None,
            max_line_length,
            forcefully_put_on_distinct_lines,
            glob,
            pattern,
            file_check,
            source_variable_name,
            max_depth,
            verbose_output,
        };

        arguments
    }
}

impl Default for ArgumentParserState {
    fn default() -> Self {
        Self::new()
    }
}
