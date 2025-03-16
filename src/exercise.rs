use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Exercise {
    Matching(Vec<Matching>),
    YesNo(Vec<YesNo>),
    Recall(Vec<Recall>),
    Mcq(Vec<Mcq>),
    RecognizeRoot(Vec<RecognizeRoot>),
}

#[derive(Serialize, Deserialize)]
pub struct Matching {
    question: String,
    answer: String,
}

impl Matching {
    pub fn new(question: String, answer: String) -> Matching {
        Matching { question, answer }
    }
}

#[derive(Serialize, Deserialize)]
pub struct YesNo {
    question: String,
    answer: bool,
}

impl YesNo {
    pub fn new(question: String, answer: bool) -> YesNo {
        YesNo { question, answer }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Recall {
    question: String,
    answer: String,
}

impl Recall {
    pub fn new(question: String, answer: String) -> Recall {
        Recall { question, answer }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Mcq {
    question: String,
    answer: String,
    options: Vec<String>,
}

impl Mcq {
    pub fn new(question: String, answer: String, options: Vec<String>) -> Mcq {
        Mcq {
            question,
            answer,
            options,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct RecognizeRoot {
    question: String,
    answer: String,
    example: String,
}

impl RecognizeRoot {
    pub fn new(question: String, answer: String, example: String) -> RecognizeRoot {
        RecognizeRoot {
            question,
            answer,
            example,
        }
    }
}
