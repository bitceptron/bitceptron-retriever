use std::str::FromStr;

use bitcoin::bip32::{ChildNumber, DerivationPath};
use regex::Regex;
use serde::{Deserialize, Serialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::error::RetrieverError;

#[derive(Debug, Clone, PartialEq, Eq, ZeroizeOnDrop, Zeroize, Serialize, Deserialize, Hash)]
pub enum ExplorationStepHardness {
    Hardened,
    Normal,
    HardenedAndNormal,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Zeroize, ZeroizeOnDrop, Hash)]
pub struct ExplorationStep {
    start_inclusive: u32,
    end_inclusive: u32,
    hardness: ExplorationStepHardness,
}

impl ExplorationStep {
    pub fn num_children(&self) -> u32 {
        if self.hardness == ExplorationStepHardness::HardenedAndNormal {
            2 * (self.end_inclusive - self.start_inclusive + 1)
        } else {
            self.end_inclusive - self.start_inclusive + 1
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Zeroize, ZeroizeOnDrop, Hash)]
pub struct ExplorationPath {
    path: Vec<ExplorationStep>,
    depth: u32,
}

impl ExplorationPath {
    pub fn new(exploration_str: &str, exploration_depth: u32) -> Result<Self, RetrieverError> {
        if check_input_chars(exploration_str) == false {
            return Err(RetrieverError::InvalidExplorationPath);
        }
        let exploration_path_split = split_path_steps(exploration_str);
        if exploration_path_split
            .iter()
            .any(|step| !check_step_sanity(step.clone()))
        {
            return Err(RetrieverError::InvalidExplorationPath);
        }

        let mut path = vec![];
        for step in exploration_path_split {
            path.push(translate_step_string_to_exploration_step(
                step,
                exploration_depth,
            )?)
        }

        Ok(ExplorationPath {
            path,
            depth: exploration_depth,
        })
    }

    pub fn num_of_paths(&self) -> u64 {
        if self.path.is_empty() {
            0
        } else {
            self.path
                .iter()
                .fold(1, |acc, step| acc * step.num_children() as u64)
        }
    }

    pub fn generate_derivation_paths_for_exploration_path(
        &self,
    ) -> Result<Vec<DerivationPath>, RetrieverError> {
        let mut derivation_paths = vec![];
        for step in self.path.clone() {
            if derivation_paths.is_empty() {
                match step.hardness {
                    ExplorationStepHardness::Hardened => {
                        for child_number in step.start_inclusive..step.end_inclusive + 1 {
                            derivation_paths.push(DerivationPath::from_str(
                                format!("m/{}'", child_number).as_str(),
                            )?)
                        }
                    }
                    ExplorationStepHardness::Normal => {
                        for child_number in step.start_inclusive..step.end_inclusive + 1 {
                            derivation_paths.push(DerivationPath::from_str(
                                format!("m/{}", child_number).as_str(),
                            )?)
                        }
                    }
                    ExplorationStepHardness::HardenedAndNormal => {
                        for child_number in step.start_inclusive..step.end_inclusive + 1 {
                            derivation_paths.push(DerivationPath::from_str(
                                format!("m/{}'", child_number).as_str(),
                            )?)
                        }
                        for child_number in step.start_inclusive..step.end_inclusive + 1 {
                            derivation_paths.push(DerivationPath::from_str(
                                format!("m/{}", child_number).as_str(),
                            )?)
                        }
                    }
                }
            } else {
                match step.hardness {
                    ExplorationStepHardness::Hardened => {
                        let mut round_result = vec![];
                        for child_number in step.start_inclusive..step.end_inclusive + 1 {
                            for child in derivation_paths.clone() {
                                round_result.push(
                                    child.extend(ChildNumber::from_hardened_idx(child_number)?),
                                );
                            }
                        }
                        derivation_paths = round_result;
                    }
                    ExplorationStepHardness::Normal => {
                        let mut round_result = vec![];
                        for child_number in step.start_inclusive..step.end_inclusive + 1 {
                            for child in derivation_paths.clone() {
                                round_result.push(
                                    child.extend(ChildNumber::from_normal_idx(child_number)?),
                                );
                            }
                        }
                        derivation_paths = round_result;
                    }
                    ExplorationStepHardness::HardenedAndNormal => {
                        let mut round_result = vec![];
                        for child_number in step.start_inclusive..step.end_inclusive + 1 {
                            for child in derivation_paths.clone() {
                                round_result.push(
                                    child.extend(ChildNumber::from_hardened_idx(child_number)?),
                                );
                            }
                        }
                        for child_number in step.start_inclusive..step.end_inclusive + 1 {
                            for child in derivation_paths.clone() {
                                round_result.push(
                                    child.extend(ChildNumber::from_normal_idx(child_number)?),
                                );
                            }
                        }
                        derivation_paths = round_result;
                    }
                }
            }
        }
        Ok(derivation_paths)
    }

    pub fn generate_sweep_exploration_paths(&self) -> Vec<ExplorationPath> {
        let mut sweep_paths = vec![];
        for i in 0..self.path.len() + 1 {
            sweep_paths.push(ExplorationPath {
                path: self.path[..i].to_vec(),
                depth: self.depth,
            });
        }
        sweep_paths
    }

    pub fn num_of_paths_sweep_from_root(&self) -> u64 {
        let mut num_paths = 1;
        let sweep_exploration_paths = self.generate_sweep_exploration_paths();
        for path in sweep_exploration_paths {
            num_paths += path.num_of_paths()
        }
        num_paths
    }

    pub fn generate_derivation_paths_for_exploration_path_sweep(
        &self,
    ) -> Result<Vec<DerivationPath>, RetrieverError> {
        let mut derivation_paths = vec![];
        derivation_paths.extend(vec![DerivationPath::from_str("m")?]);
        let sweep_paths = self.generate_sweep_exploration_paths();
        for path in sweep_paths {
            derivation_paths.extend(path.generate_derivation_paths_for_exploration_path()?)
        }
        Ok(derivation_paths)
    }
}

pub fn check_input_chars(input: &str) -> bool {
    let regex = Regex::new(r"^[\d./'ha*]+$").unwrap();
    regex.is_match(input)
}

pub fn split_path_steps(input: &str) -> Vec<String> {
    let mut path_split = input.split('/').collect::<Vec<&str>>();
    path_split.retain(|str| !str.is_empty());
    path_split
        .iter()
        .map(|path_str| path_str.to_string())
        .collect()
}

pub fn step_is_range(step: &str) -> bool {
    let range_regex = Regex::new(r"^\d*(\.\.)?\d+[h'a]?$").unwrap();
    range_regex.is_match(&step)
}

pub fn step_is_wildcard(step: &str) -> bool {
    let wildcard_regex = Regex::new(r"^\*[h'a]?$").unwrap();
    wildcard_regex.is_match(&step)
}

pub fn check_step_sanity(step: String) -> bool {
    step_is_wildcard(&step) || step_is_range(&step)
}

pub fn extract_step_hardness(step: &str) -> ExplorationStepHardness {
    match step.chars().last().unwrap() {
        'h' | '\'' => ExplorationStepHardness::Hardened,
        'a' => ExplorationStepHardness::HardenedAndNormal,
        _ => ExplorationStepHardness::Normal,
    }
}

pub fn translate_wildcard_step_string_to_exploration_step(
    step_string: String,
    exploration_depth: u32,
) -> ExplorationStep {
    let hardness = extract_step_hardness(&step_string);
    let start_inclusive = 0;
    let end_inclusive = exploration_depth;
    ExplorationStep {
        start_inclusive,
        end_inclusive,
        hardness,
    }
}

pub fn translate_range_step_string_to_exploration_step(
    step_string: String,
) -> Result<ExplorationStep, RetrieverError> {
    let hardness = extract_step_hardness(&step_string);

    let point_regex = Regex::new(r"^\d+[h'a]?$").unwrap();
    let start_regex = Regex::new(r"^\d+\.\.").unwrap();
    let end_regex = Regex::new(r"\.\.\d+").unwrap();

    let start_inclusive = match point_regex.find(&step_string) {
        Some(start) => start
            .as_str()
            .chars()
            .filter(|char| char.is_ascii_digit())
            .map(|char| char.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<u32>()
            .unwrap(),
        None => match start_regex.find(&step_string) {
            Some(start) => start
                .as_str()
                .chars()
                .filter(|char| *char != '.')
                .map(|char| char.to_string())
                .collect::<Vec<String>>()
                .join("")
                .parse::<u32>()
                .unwrap(),
            None => 0u32,
        },
    };

    let end_inclusive = match point_regex.find(&step_string) {
        Some(end) => end
            .as_str()
            .chars()
            .filter(|char| char.is_ascii_digit())
            .map(|char| char.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<u32>()
            .unwrap(),
        None => match end_regex.find(&step_string) {
            Some(end) => end
                .as_str()
                .chars()
                .filter(|char| *char != '.')
                .map(|char| char.to_string())
                .collect::<Vec<String>>()
                .join("")
                .parse::<u32>()
                .unwrap(),
            None => return Err(RetrieverError::InvalidStepRange),
        },
    };

    if end_inclusive < start_inclusive {
        return Err(RetrieverError::InvalidStepRange);
    }

    Ok(ExplorationStep {
        start_inclusive,
        end_inclusive,
        hardness,
    })
}

pub fn translate_step_string_to_exploration_step(
    step_string: String,
    exploration_depth: u32,
) -> Result<ExplorationStep, RetrieverError> {
    if step_is_range(&step_string) {
        Ok(translate_range_step_string_to_exploration_step(
            step_string,
        )?)
    } else if step_is_wildcard(&step_string) {
        Ok(translate_wildcard_step_string_to_exploration_step(
            step_string,
            exploration_depth,
        ))
    } else {
        Err(RetrieverError::InvalidExplorationPath)
    }
}

#[cfg(test)]
mod tests {
    use hashbrown::HashSet;

    use super::*;

    #[test]
    fn check_input_chars_works_01() {
        assert!(check_input_chars("89/..90'/*"));
        assert!(check_input_chars("*/*a/*'/*h"));
        assert!(!check_input_chars("o"));
        assert!(check_input_chars("*../90//09"));
        assert!(!check_input_chars("+/*h/90/5"));
        assert!(!check_input_chars("+/+"));
        assert!(!check_input_chars("+/7"));
    }

    #[test]
    fn split_path_steps_work_01() {
        let str = "90/..9/*a";
        let result = split_path_steps(str);
        let expected = vec!["90".to_string(), "..9".to_string(), "*a".to_string()];
        assert_eq!(result, expected);

        let str = "90//..9/*a";
        let result = split_path_steps(str);
        let expected = vec!["90".to_string(), "..9".to_string(), "*a".to_string()];
        assert_eq!(result, expected);

        let str = "..89/4..9/*a/*'/*h/*/90..900h";
        let result = split_path_steps(str);
        let expected = vec![
            "..89".to_string(),
            "4..9".to_string(),
            "*a".to_string(),
            "*'".to_string(),
            "*h".to_string(),
            "*".to_string(),
            "90..900h".to_string(),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn step_is_range_works_01() {
        let range_steps = vec!["..90", "8..78", "..4h", "8..9'", "9..9a"];
        range_steps
            .iter()
            .for_each(|step| assert!(step_is_range(step)));

        let not_range_steps = vec![
            "*", "*'", "*h", "*a", "p", "..*", "h..*", "*'ha", "..*h", "*ha", "89'h",
        ];
        not_range_steps
            .iter()
            .for_each(|step| assert!(!step_is_range(step)));
    }

    #[test]
    fn step_is_wildcard_works_01() {
        let not_wildcard_steps = vec![
            "..90", "8..78", "..4h", "8..9'", "9..9a", "**", "..*h", "*ha", "89'h",
        ];
        not_wildcard_steps
            .iter()
            .for_each(|step| assert!(!step_is_wildcard(step)));

        let wildcard_steps = vec!["*", "*'", "*h", "*a"];
        wildcard_steps
            .iter()
            .for_each(|step| assert!(step_is_wildcard(step)));
    }

    #[test]
    fn extract_step_hardness_works_01() {
        assert_eq!(
            extract_step_hardness("9'"),
            ExplorationStepHardness::Hardened
        );
        assert_eq!(
            extract_step_hardness("..9h"),
            ExplorationStepHardness::Hardened
        );
        assert_eq!(extract_step_hardness("9"), ExplorationStepHardness::Normal);
        assert_eq!(
            extract_step_hardness("*a"),
            ExplorationStepHardness::HardenedAndNormal
        );
    }

    #[test]
    fn translate_wildcard_step_string_to_exploration_step_works_01() {
        let result = translate_wildcard_step_string_to_exploration_step("*h".to_string(), 10);
        let expected = ExplorationStep {
            start_inclusive: 0,
            end_inclusive: 10,
            hardness: ExplorationStepHardness::Hardened,
        };
        assert_eq!(result, expected);

        let result = translate_wildcard_step_string_to_exploration_step("*'".to_string(), 10);
        let expected = ExplorationStep {
            start_inclusive: 0,
            end_inclusive: 10,
            hardness: ExplorationStepHardness::Hardened,
        };
        assert_eq!(result, expected);

        let result = translate_wildcard_step_string_to_exploration_step("*a".to_string(), 10);
        let expected = ExplorationStep {
            start_inclusive: 0,
            end_inclusive: 10,
            hardness: ExplorationStepHardness::HardenedAndNormal,
        };
        assert_eq!(result, expected);

        let result = translate_wildcard_step_string_to_exploration_step("*".to_string(), 10);
        let expected = ExplorationStep {
            start_inclusive: 0,
            end_inclusive: 10,
            hardness: ExplorationStepHardness::Normal,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn translate_range_step_string_to_exploration_step_works_01() {
        let result = translate_range_step_string_to_exploration_step("0".to_string()).unwrap();
        let expected = ExplorationStep {
            start_inclusive: 0,
            end_inclusive: 0,
            hardness: ExplorationStepHardness::Normal,
        };
        assert_eq!(result, expected);

        let result = translate_range_step_string_to_exploration_step("9..78h".to_string()).unwrap();
        let expected = ExplorationStep {
            start_inclusive: 9,
            end_inclusive: 78,
            hardness: ExplorationStepHardness::Hardened,
        };
        assert_eq!(result, expected);

        let result =
            translate_range_step_string_to_exploration_step("100..120a".to_string()).unwrap();
        let expected = ExplorationStep {
            start_inclusive: 100,
            end_inclusive: 120,
            hardness: ExplorationStepHardness::HardenedAndNormal,
        };
        assert_eq!(result, expected);

        let result = translate_range_step_string_to_exploration_step("..10".to_string()).unwrap();
        let expected = ExplorationStep {
            start_inclusive: 0,
            end_inclusive: 10,
            hardness: ExplorationStepHardness::Normal,
        };
        assert_eq!(result, expected);

        let result = translate_range_step_string_to_exploration_step("9..7".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn new_works_01() {
        let exploration_str = "0/..8/*h/6..9a/*'/40a";
        let result = ExplorationPath::new(exploration_str, 5).unwrap();
        let expected = ExplorationPath {
            path: vec![
                ExplorationStep {
                    start_inclusive: 0,
                    end_inclusive: 0,
                    hardness: ExplorationStepHardness::Normal,
                },
                ExplorationStep {
                    start_inclusive: 0,
                    end_inclusive: 8,
                    hardness: ExplorationStepHardness::Normal,
                },
                ExplorationStep {
                    start_inclusive: 0,
                    end_inclusive: 5,
                    hardness: ExplorationStepHardness::Hardened,
                },
                ExplorationStep {
                    start_inclusive: 6,
                    end_inclusive: 9,
                    hardness: ExplorationStepHardness::HardenedAndNormal,
                },
                ExplorationStep {
                    start_inclusive: 0,
                    end_inclusive: 5,
                    hardness: ExplorationStepHardness::Hardened,
                },
                ExplorationStep {
                    start_inclusive: 40,
                    end_inclusive: 40,
                    hardness: ExplorationStepHardness::HardenedAndNormal,
                },
            ],
            depth: 5,
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn new_works_02() {
        let exploration_str = "..9a";
        let result = ExplorationPath::new(exploration_str, 5).unwrap();
        let expected = ExplorationPath {
            path: vec![ExplorationStep {
                start_inclusive: 0,
                end_inclusive: 9,
                hardness: ExplorationStepHardness::HardenedAndNormal,
            }],
            depth: 5,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn new_works_03() {
        let exploration_str = "0u/..8/*h/6..9a/*'/40a";
        let result = ExplorationPath::new(exploration_str, 5);
        assert!(result.is_err())
    }

    #[test]
    fn new_works_04() {
        let exploration_str = "./.8";
        let result = ExplorationPath::new(exploration_str, 5);
        assert!(result.is_err())
    }

    #[test]
    fn new_works_05() {
        let exploration_str = "";
        let result = ExplorationPath::new(exploration_str, 5);
        assert!(result.is_err());
    }

    #[test]
    fn num_of_paths_works_01() {
        let exploration_path = ExplorationPath::new("..8", 5).unwrap();
        assert_eq!(exploration_path.num_of_paths(), 9);

        let exploration_path = ExplorationPath::new("4..8h", 5).unwrap();
        assert_eq!(exploration_path.num_of_paths(), 5);

        let exploration_path = ExplorationPath::new("8'", 5).unwrap();
        assert_eq!(exploration_path.num_of_paths(), 1);

        let exploration_path = ExplorationPath::new("*a", 5).unwrap();
        assert_eq!(exploration_path.num_of_paths(), 12);

        let exploration_path = ExplorationPath::new("..8/*a", 5).unwrap();
        assert_eq!(exploration_path.num_of_paths(), 108);

        let exploration_path = ExplorationPath::new("3..9h/*'/9a/*/*h", 5).unwrap();
        assert_eq!(exploration_path.num_of_paths(), 3024);

        let exploration_path = ExplorationPath::new("/8/*a/..90'/0", 5).unwrap();
        assert_eq!(exploration_path.num_of_paths(), 1092);
    }

    #[test]
    fn generate_derivation_paths_for_exploration_path_works_01() {
        let exploration_path = ExplorationPath::new("*/9..10h/4a", 1).unwrap();
        let mut result = HashSet::new();
        result.extend(
            exploration_path
                .generate_derivation_paths_for_exploration_path()
                .unwrap(),
        );

        let expected_vec = vec![
            DerivationPath::from_str("m/0/9h/4").unwrap(),
            DerivationPath::from_str("m/0/9h/4h").unwrap(),
            DerivationPath::from_str("m/0/10h/4").unwrap(),
            DerivationPath::from_str("m/0/10h/4h").unwrap(),
            DerivationPath::from_str("m/1/9h/4").unwrap(),
            DerivationPath::from_str("m/1/9h/4h").unwrap(),
            DerivationPath::from_str("m/1/10h/4").unwrap(),
            DerivationPath::from_str("m/1/10h/4h").unwrap(),
        ];
        let mut expected = HashSet::new();
        expected.extend(expected_vec);
        assert_eq!(expected, result);
    }

    #[test]
    fn generate_derivation_paths_for_exploration_path_works_02() {
        let exploration_path = ExplorationPath::new("*'/..2h/4", 1).unwrap();
        let mut result = HashSet::new();
        result.extend(
            exploration_path
                .generate_derivation_paths_for_exploration_path()
                .unwrap(),
        );

        let expected_vec = vec![
            DerivationPath::from_str("m/0'/0h/4").unwrap(),
            DerivationPath::from_str("m/0'/1h/4").unwrap(),
            DerivationPath::from_str("m/0'/2h/4").unwrap(),
            DerivationPath::from_str("m/1'/0h/4").unwrap(),
            DerivationPath::from_str("m/1'/1h/4").unwrap(),
            DerivationPath::from_str("m/1'/2h/4").unwrap(),
        ];
        let mut expected = HashSet::new();
        expected.extend(expected_vec);
        assert_eq!(expected, result);
    }

    #[test]
    fn generate_sweep_exploration_paths_works_01() {
        let exploration_path = ExplorationPath::new("*a/..2h/4", 1).unwrap();
        let mut result = HashSet::new();
        result.extend(
            exploration_path
                .generate_derivation_paths_for_exploration_path_sweep()
                .unwrap(),
        );

        let expected_vec = vec![
            DerivationPath::from_str("m/0/0h/4").unwrap(),
            DerivationPath::from_str("m/0/1h/4").unwrap(),
            DerivationPath::from_str("m/0/2h/4").unwrap(),
            DerivationPath::from_str("m/0'/0h/4").unwrap(),
            DerivationPath::from_str("m/0'/1h/4").unwrap(),
            DerivationPath::from_str("m/0'/2h/4").unwrap(),
            DerivationPath::from_str("m/1/0h/4").unwrap(),
            DerivationPath::from_str("m/1/1h/4").unwrap(),
            DerivationPath::from_str("m/1/2h/4").unwrap(),
            DerivationPath::from_str("m/1'/0h/4").unwrap(),
            DerivationPath::from_str("m/1'/1h/4").unwrap(),
            DerivationPath::from_str("m/1'/2h/4").unwrap(),
            //
            DerivationPath::from_str("m/0/0h").unwrap(),
            DerivationPath::from_str("m/0/1h").unwrap(),
            DerivationPath::from_str("m/0/2h").unwrap(),
            DerivationPath::from_str("m/0'/0h").unwrap(),
            DerivationPath::from_str("m/0'/1h").unwrap(),
            DerivationPath::from_str("m/0'/2h").unwrap(),
            DerivationPath::from_str("m/1/0h").unwrap(),
            DerivationPath::from_str("m/1/1h").unwrap(),
            DerivationPath::from_str("m/1/2h").unwrap(),
            DerivationPath::from_str("m/1'/0h").unwrap(),
            DerivationPath::from_str("m/1'/1h").unwrap(),
            DerivationPath::from_str("m/1'/2h").unwrap(),
            //
            DerivationPath::from_str("m/0'").unwrap(),
            DerivationPath::from_str("m/1").unwrap(),
            DerivationPath::from_str("m/0").unwrap(),
            DerivationPath::from_str("m/1'").unwrap(),
            //
            DerivationPath::from_str("m").unwrap(),
        ];
        let mut expected = HashSet::new();
        expected.extend(expected_vec);
        assert_eq!(expected, result);
    }

    #[test]
    fn num_of_paths_sweep_from_root_works_01() {
        let exploration_path = ExplorationPath::new("*a/..2h/4", 1).unwrap();
        assert_eq!(exploration_path.num_of_paths_sweep_from_root(), 29);
    }

    #[test]
    fn num_of_paths_sweep_from_root_works_02() {
        let exploration_path = ExplorationPath::new("*a/..2h/4", 3).unwrap();
        assert_eq!(exploration_path.num_of_paths_sweep_from_root(), 57);
    }
}
