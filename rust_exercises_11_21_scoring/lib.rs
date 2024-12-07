#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod rust_exercises_11_21_scoring {

    use ink::{prelude::vec::Vec, storage::Lazy};

    #[ink(event)]
    pub struct ScoreAdded {
        #[ink(topic)]
        score: u32,
    }

    #[ink(storage)]
    pub struct Scoring {
        scores: Vec<u32>,
    }

    impl Scoring {
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                scores: Default::default(),
            }
        }

        #[ink(message)]
        pub fn add_score(&mut self, score: u32) {
            self.scores.push(score);
            self.env().emit_event(ScoreAdded { score });
        }

        #[ink(message)]
        pub fn get(&self, index: u32) -> Option<u32> {
            self.scores.get(index as usize).copied()
        }
    }
}
