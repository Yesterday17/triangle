use std::collections::HashMap;
use std::fs;
use std::path::Path;
use askama_actix::{Template};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Template, Deserialize)]
#[template(path = "quiz.html")]
pub struct Quiz {
    #[serde(default)]
    index: u8,
    name: String,
    description: String,
    #[serde(default)]
    links: Vec<QuizLink>,
}

#[derive(Deserialize)]
pub struct QuizLink {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct QuizLock {
    pub uuid: Uuid,
}

#[derive(Deserialize)]
pub struct Config {
    /// Quiz content
    quiz: Vec<Quiz>,
    /// Quiz uuid lock
    #[serde(skip)]
    lock: Vec<QuizLock>,
}

impl Config {
    pub fn new<P>(config: P, lock: P) -> Self
        where P: AsRef<Path> {
        // read config
        let config = fs::read_to_string(config).expect("Failed to read config");
        let mut config: Config = toml::from_str(&config).expect("Failed to parse config");
        for (i, quiz) in config.quiz.iter_mut().enumerate() {
            quiz.index = (i + 1) as u8;
        }

        // read lock
        let lock_data: Vec<QuizLock> = match fs::read_to_string(lock.as_ref()) {
            Ok(lock) => {
                serde_json::from_str(&lock).expect("Failed to parse lock file")
            }
            Err(_) => {
                config.quiz.iter().map(|_| QuizLock { uuid: Uuid::new_v4() }).collect()
            }
        };
        config.lock = lock_data;

        config.validate();
        fs::write(lock, serde_json::to_string(&config.lock).unwrap()).expect("Failed to write lock file");

        config
    }

    fn validate(&mut self) {
        if self.quiz.is_empty() {
            panic!("No quiz found");
        }

        if self.quiz.len() != self.lock.len() {
            if self.quiz.len() > self.lock.len() {
                // new quiz added
                for _ in 0..(self.quiz.len() - self.lock.len()) {
                    self.lock.push(QuizLock { uuid: Uuid::new_v4() });
                }
            } else {
                // quiz deleted
                for _ in self.quiz.len()..self.lock.len() {
                    let removed = self.lock.pop().unwrap();
                    eprintln!("Removed {} from lock", removed.uuid);
                }
            }
        }
    }

    pub fn into_state(self) -> AppState {
        AppState {
            first: self.lock[0].uuid.clone(),
            quiz: self.quiz.into_iter().enumerate().map(|(i, q)| (self.lock[i].uuid, q)).collect(),
        }
    }
}

pub struct AppState {
    first: Uuid,
    quiz: HashMap<Uuid, Quiz>,
}

impl AppState {
    pub fn first(&self) -> &Uuid {
        &self.first
    }

    pub fn get(&self, uuid: &Uuid) -> Option<&Quiz> {
        self.quiz.get(uuid)
    }
}
