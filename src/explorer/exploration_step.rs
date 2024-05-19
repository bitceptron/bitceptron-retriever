use getset::Getters;
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Debug, Clone, PartialEq, Eq, ZeroizeOnDrop, Zeroize, Serialize, Deserialize, Hash)]
pub enum ExplorationStepHardness {
    Hardened,
    Normal,
    HardenedAndNormal,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Zeroize, ZeroizeOnDrop, Hash, Getters,
)]
#[get = "pub with_prefix"]
pub struct ExplorationStep {
    start_inclusive: u32,
    end_inclusive: u32,
    hardness: ExplorationStepHardness,
    iterator_position: u32,
}

impl ExplorationStep {
    pub fn new(
        start_inclusive: u32,
        end_inclusive: u32,
        hardness: ExplorationStepHardness,
    ) -> Self {
        ExplorationStep {
            start_inclusive,
            end_inclusive,
            hardness,
            iterator_position: 0,
        }
    }
    pub fn num_children(&self) -> u32 {
        if self.hardness == ExplorationStepHardness::HardenedAndNormal {
            2 * (self.end_inclusive - self.start_inclusive + 1)
        } else {
            self.end_inclusive - self.start_inclusive + 1
        }
    }

    pub fn reset_iterator(&mut self) {
        self.iterator_position = 0;
    }
}

impl Iterator for ExplorationStep {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let size = self.num_children();
        let result = if self.iterator_position == size {
            None
        } else {
            match self.hardness {
                ExplorationStepHardness::Hardened => Some(format!(
                    "{}'",
                    self.start_inclusive + self.iterator_position
                )),
                ExplorationStepHardness::Normal => Some(format!(
                    "{}",
                    self.start_inclusive + self.iterator_position as u32
                )),
                ExplorationStepHardness::HardenedAndNormal => {
                    if self.iterator_position < size / 2 {
                        Some(format!(
                            "{}'",
                            self.start_inclusive + self.iterator_position as u32
                        ))
                    } else {
                        Some(format!(
                            "{}",
                            self.start_inclusive + self.iterator_position - (size / 2)
                        ))
                    }
                }
            }
        };
        self.iterator_position += 1;
        result
    }
}
