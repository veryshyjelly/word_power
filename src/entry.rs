// Import the necessary types from the exercise module.
use crate::exercise::{Exercise, Matching, Mcq, Recall, RecognizeRoot, YesNo};
// Derive macros for automatic trait implementations.
use strum_macros;

// Import the inquire crate for interactive CLI prompts.
use inquire;
use inquire::formatter::OptionFormatter;
use inquire::{Confirm, Select, Text};

/// A common trait for types that can be interactively read from user input.
///
/// This trait defines a single method, `read`, that returns a vector of instances
/// by prompting the user for input.
pub trait Entry: Sized {
    /// Reads a list of entries from the user.
    ///
    /// # Returns
    ///
    /// A vector containing all instances that were read from user input.
    fn read() -> Vec<Self>;
}

/// Prompts the user for the number of questions and then reads that many questions.
///
/// # Returns
///
/// A vector of question strings entered by the user.
fn read_questions() -> Vec<String> {
    let n = inquire::CustomType::<usize>::new("How many questions?")
        .with_error_message("Please enter a valid number")
        .prompt()
        .unwrap();

    // Collect each question with an index (starting at 1) as a prompt.
    (0..n)
        .map(|i| Text::new(&format!("{}. ", i + 1)).prompt().unwrap())
        .collect()
}

/// Prompts the user to enter a specified number of options.
///
/// Options are labeled with consecutive letters starting from 'a'.
///
/// # Arguments
///
/// * `n` - The number of options to be read.
///
/// # Returns
///
/// A vector containing the option strings provided by the user.
fn read_options(n: usize) -> Vec<String> {
    ('a'..)
        .take(n)
        .map(|c| Text::new(&format!("({})", c)).prompt().unwrap())
        .collect()
}

/// Formatter for displaying options in the select prompt.
///
/// This formatter takes an index and an option value and returns a string in the
/// format "(letter) option_value", where the letter corresponds to the option's position.
const OPTION_FORMATTER: OptionFormatter<String> =
    &|i| format!("({}) {}", ('a'..).nth(i.index).unwrap(), i.value);

/// Implementation of the `Entry` trait for `Matching` exercises.
///
/// This reads matching-type exercises by:
/// - Prompting for a set of questions.
/// - Prompting for a set of options that correspond to these questions.
/// - Asking the user to select the correct option for each question.
impl Entry for Matching {
    fn read() -> Vec<Self> {
        let questions = read_questions();
        let options = read_options(questions.len());

        questions
            .into_iter()
            .enumerate()
            .map(|(i, question)| {
                let answer = Select::new(
                    &format!("{}. {}", i + 1, question),
                    options.clone(),
                )
                    .with_formatter(OPTION_FORMATTER)
                    .prompt()
                    .unwrap();

                Matching::new(question, answer)
            })
            .collect()
    }
}

/// Implementation of the `Entry` trait for `YesNo` exercises.
///
/// This reads yes/no exercises by prompting the user with each question and recording
/// a boolean response.
impl Entry for YesNo {
    fn read() -> Vec<Self> {
        let questions = read_questions();

        questions
            .into_iter()
            .enumerate()
            .map(|(i, question)| {
                let answer = Confirm::new(&format!("{}. {}", i + 1, question))
                    .prompt()
                    .unwrap();
                YesNo::new(question, answer)
            })
            .collect()
    }
}

/// Implementation of the `Entry` trait for `Recall` exercises.
///
/// This reads recall exercises by prompting the user for questions and capturing
/// free-text answers.
impl Entry for Recall {
    fn read() -> Vec<Self> {
        let questions = read_questions();

        questions
            .into_iter()
            .enumerate()
            .map(|(i, question)| {
                let answer = Text::new(&format!("{}. {}", i + 1, question))
                    .prompt()
                    .unwrap();
                Recall::new(question, answer)
            })
            .collect()
    }
}

/// Implementation of the `Entry` trait for `Mcq` (Multiple Choice Questions) exercises.
///
/// This reads MCQ exercises by:
/// - Asking for the number of questions.
/// - Asking for the number of options for each question.
/// - Prompting for the question text and its options.
/// - Allowing the user to select the correct answer for each question.
impl Entry for Mcq {
    fn read() -> Vec<Self> {
        let n = inquire::CustomType::<usize>::new("How many questions?")
            .with_error_message("Please enter a valid number")
            .prompt()
            .expect("Please enter a valid number");

        let m = inquire::CustomType::<usize>::new("How many options?")
            .with_error_message("Please enter a valid number")
            .prompt()
            .expect("Please enter a valid number");

        (0..n)
            .map(|i| {
                (
                    i,
                    Text::new(&format!("{}. ", i + 1)).prompt().unwrap(),
                    read_options(m),
                )
            })
            .map(|(i, q, opts)| {
                let answer = Select::new(
                    &format!("{}. {}", i + 1, q),
                    opts.clone(),
                )
                    .prompt()
                    .unwrap();
                Mcq::new(q, answer, opts)
            })
            .collect()
    }
}

/// Implementation of the `Entry` trait for `RecognizeRoot` exercises.
///
/// This reads recognize-root exercises by:
/// - Prompting for the number of questions.
/// - For each question, capturing the question text, an example, and the user's answer.
impl Entry for RecognizeRoot {
    fn read() -> Vec<Self> {
        let n = inquire::CustomType::<usize>::new("How many questions?")
            .prompt()
            .unwrap();

        (0..n)
            .map(|i| {
                (
                    i,
                    Text::new(&format!("{}. ", i + 1)).prompt().unwrap(),
                    Text::new("Example").prompt().unwrap(),
                )
            })
            .map(|(i, q, ex)| {
                let answer = Text::new(&format!("{}. {}, Example: {}", i + 1, q, ex))
                    .prompt()
                    .unwrap();
                RecognizeRoot::new(q, answer, ex)
            })
            .collect()
    }
}

/// Enum that represents the different exercise entry types available to the user.
///
/// This enum is used in the interactive prompt to let the user choose the type of exercise
/// they wish to create or enter. The derived `Display` implementation (via `strum_macros`)
/// provides a human-readable representation for each variant.
#[derive(strum_macros::Display)]
enum EntryOptions {
    Matching,
    YesNo,
    Recall,
    Mcq,
    RecognizeRoot,
    SaveAndQuit,
}

impl EntryOptions {
    /// Returns a list of all exercise entry options.
    fn all() -> Vec<EntryOptions> {
        vec![
            Self::Matching,
            Self::YesNo,
            Self::Recall,
            Self::Mcq,
            Self::RecognizeRoot,
            Self::SaveAndQuit,
        ]
    }
}

/// Implementation of the `Entry` trait for the overall `Exercise` enum.
///
/// This method continuously prompts the user to choose an exercise type, reads the
/// corresponding exercise data, and returns a vector of all exercises entered until
/// the user selects "SaveAndQuit".
impl Entry for Exercise {
    fn read() -> Vec<Self> {
        (1..)
            .map_while(|_| {
                let tp = Select::new("Exercise type", EntryOptions::all())
                    .prompt()
                    .unwrap();
                match tp {
                    EntryOptions::Matching => Some(Exercise::Matching(Matching::read())),
                    EntryOptions::YesNo => Some(Exercise::YesNo(YesNo::read())),
                    EntryOptions::Recall => Some(Exercise::Recall(Recall::read())),
                    EntryOptions::Mcq => Some(Exercise::Mcq(Mcq::read())),
                    EntryOptions::RecognizeRoot => Some(Exercise::RecognizeRoot(RecognizeRoot::read())),
                    EntryOptions::SaveAndQuit => None,
                }
            })
            .collect()
    }
}
