use std::str::FromStr;

use bitcoin::bip32::DerivationPath;
use getset::Getters;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::error::RetrieverError;

use super::exploration_step::{ExplorationStep, ExplorationStepHardness};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Hash, Getters, Default)]
#[get = "pub with_prefix"]
pub struct ExplorationPath {
    base_paths: Vec<DerivationPath>,
    explore: Vec<ExplorationStep>,
    depth: u32,
    sweep: bool,
}

impl ExplorationPath {
    pub fn new(
        base_paths: Option<Vec<String>>,
        explore_str: &str,
        exploration_depth: u32,
        sweep: bool,
    ) -> Result<Self, RetrieverError> {
        info!("Creation of exploration path started.");
        let base_paths = match base_paths {
            Some(base_paths) => base_paths
                .iter()
                .map(|base_path_string| {
                    DerivationPath::from_str(base_path_string)
                        .map_err(RetrieverError::from)
                        .unwrap()
                })
                .collect::<Vec<DerivationPath>>(),
            None => vec![DerivationPath::from_str("m").unwrap()],
        };
        if !check_input_chars(explore_str) {
            error!("Encountered invalid exploration path.");
            return Err(RetrieverError::InvalidExplorationPath);
        }
        let explore_path_split = split_path_steps(explore_str);
        if explore_path_split
            .iter()
            .any(|step| !check_step_sanity(step.clone()))
        {
            error!("Encountered invalid exploration path.");
            return Err(RetrieverError::InvalidExplorationPath);
        }

        let mut explore = vec![];
        for step in explore_path_split {
            explore.push(translate_step_string_to_exploration_step(
                step,
                exploration_depth,
            )?)
        }
        info!("Creation of exploration path finished successfully.");
        Ok(ExplorationPath {
            base_paths,
            explore,
            depth: exploration_depth,
            sweep,
        })
    }

    pub fn num_of_paths(&self) -> usize {
        info!("Calculating the number of paths in exploration path.");
        if self.explore.is_empty() {
            0
        } else {
            self.base_paths.len()
                * self
                    .explore
                    .iter()
                    .fold(1usize, |acc, step| acc * step.num_children() as usize)
        }
    }

    pub fn num_of_paths_sweep(&self) -> usize {
        info!("Calculating the number of sweep paths in exploration path.");
        let mut num_paths = 1;
        let sweep_exploration_paths = self.generate_sweep_exploration_paths();
        for path in sweep_exploration_paths {
            num_paths += path.num_of_paths()
        }
        num_paths
    }

    pub fn size(&self) -> usize {
        if self.sweep {
            self.num_of_paths_sweep()
        } else {
            self.num_of_paths()
        }
    }

    pub fn generate_sweep_exploration_paths(&self) -> Vec<ExplorationPath> {
        info!("Creating sweep exploration paths.");
        let mut sweep_paths = vec![];
        for i in 0..self.explore.len() + 1 {
            sweep_paths.push(ExplorationPath {
                explore: self.explore[..i].to_vec(),
                depth: self.depth,
                base_paths: self.base_paths.clone(),
                sweep: self.sweep,
            });
        }
        sweep_paths
    }
}

impl Zeroize for ExplorationPath {
    fn zeroize(&mut self) {
        self.base_paths =
            vec![DerivationPath::from_str("m/1024/1024/1204/1024").unwrap(); self.base_paths.len()];
        self.explore.zeroize();
        self.depth.zeroize();
        self.sweep.zeroize();
    }
}

impl ZeroizeOnDrop for ExplorationPath {}

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
    range_regex.is_match(step)
}

pub fn step_is_wildcard(step: &str) -> bool {
    let wildcard_regex = Regex::new(r"^\*[h'a]?$").unwrap();
    wildcard_regex.is_match(step)
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
    ExplorationStep::new(start_inclusive, end_inclusive, hardness)
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

    Ok(ExplorationStep::new(
        start_inclusive,
        end_inclusive,
        hardness,
    ))
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
        let expected = ExplorationStep::new(0, 10, ExplorationStepHardness::Hardened);
        assert_eq!(result, expected);

        let result = translate_wildcard_step_string_to_exploration_step("*'".to_string(), 10);
        let expected = ExplorationStep::new(0, 10, ExplorationStepHardness::Hardened);
        assert_eq!(result, expected);

        let result = translate_wildcard_step_string_to_exploration_step("*a".to_string(), 10);
        let expected = ExplorationStep::new(0, 10, ExplorationStepHardness::HardenedAndNormal);
        assert_eq!(result, expected);

        let result = translate_wildcard_step_string_to_exploration_step("*".to_string(), 10);
        let expected = ExplorationStep::new(0, 10, ExplorationStepHardness::Normal);
        assert_eq!(result, expected);
    }

    #[test]
    fn translate_range_step_string_to_exploration_step_works_01() {
        let result = translate_range_step_string_to_exploration_step("0".to_string()).unwrap();
        let expected = ExplorationStep::new(0, 0, ExplorationStepHardness::Normal);
        assert_eq!(result, expected);

        let result = translate_range_step_string_to_exploration_step("9..78h".to_string()).unwrap();
        let expected = ExplorationStep::new(9, 78, ExplorationStepHardness::Hardened);
        assert_eq!(result, expected);

        let result =
            translate_range_step_string_to_exploration_step("100..120a".to_string()).unwrap();
        let expected = ExplorationStep::new(100, 120, ExplorationStepHardness::HardenedAndNormal);
        assert_eq!(result, expected);

        let result = translate_range_step_string_to_exploration_step("..10".to_string()).unwrap();
        let expected = ExplorationStep::new(0, 10, ExplorationStepHardness::Normal);
        assert_eq!(result, expected);

        let result = translate_range_step_string_to_exploration_step("9..7".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn new_works_01() {
        let exploration_str = "0/..8/*h/6..9a/*'/40a";
        let result = ExplorationPath::new(None, exploration_str, 5, false).unwrap();
        let expected = ExplorationPath {
            base_paths: vec![DerivationPath::from_str("m").unwrap()],
            explore: vec![
                ExplorationStep::new(0, 0, ExplorationStepHardness::Normal),
                ExplorationStep::new(0, 8, ExplorationStepHardness::Normal),
                ExplorationStep::new(0, 5, ExplorationStepHardness::Hardened),
                ExplorationStep::new(6, 9, ExplorationStepHardness::HardenedAndNormal),
                ExplorationStep::new(0, 5, ExplorationStepHardness::Hardened),
                ExplorationStep::new(40, 40, ExplorationStepHardness::HardenedAndNormal),
            ],
            depth: 5,
            sweep: false,
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn new_works_02() {
        let exploration_str = "..9a";
        let result = ExplorationPath::new(None, exploration_str, 5, false).unwrap();
        let expected = ExplorationPath {
            base_paths: vec![DerivationPath::from_str("m").unwrap()],
            explore: vec![ExplorationStep::new(
                0,
                9,
                ExplorationStepHardness::HardenedAndNormal,
            )],
            depth: 5,
            sweep: false,
        };
        assert_eq!(result, expected);
    }

    #[test]
    fn new_works_03() {
        let exploration_str = "0u/..8/*h/6..9a/*'/40a";
        let result = ExplorationPath::new(None, exploration_str, 5, false);
        assert!(result.is_err())
    }

    #[test]
    fn new_works_04() {
        let exploration_str = "./.8";
        let result = ExplorationPath::new(None, exploration_str, 5, false);
        assert!(result.is_err())
    }

    #[test]
    fn new_works_05() {
        let exploration_str = "";
        let result = ExplorationPath::new(None, exploration_str, 5, false);
        assert!(result.is_err());
    }

    #[test]
    fn num_of_paths_works_01() {
        let exploration_path = ExplorationPath::new(None, "..8", 5, false).unwrap();
        assert_eq!(exploration_path.num_of_paths(), 9);

        let exploration_path = ExplorationPath::new(None, "4..8h", 5, false).unwrap();
        assert_eq!(exploration_path.num_of_paths(), 5);

        let exploration_path = ExplorationPath::new(None, "8'", 5, false).unwrap();
        assert_eq!(exploration_path.num_of_paths(), 1);

        let exploration_path = ExplorationPath::new(None, "*a", 5, false).unwrap();
        assert_eq!(exploration_path.num_of_paths(), 12);

        let exploration_path = ExplorationPath::new(None, "..8/*a", 5, false).unwrap();
        assert_eq!(exploration_path.num_of_paths(), 108);

        let exploration_path = ExplorationPath::new(None, "3..9h/*'/9a/*/*h", 5, false).unwrap();
        assert_eq!(exploration_path.num_of_paths(), 3024);

        let exploration_path = ExplorationPath::new(None, "/8/*a/..90'/0", 5, false).unwrap();
        assert_eq!(exploration_path.num_of_paths(), 1092);
    }

    #[test]
    fn num_of_paths_sweep_from_root_works_01() {
        let exploration_path = ExplorationPath::new(None, "*a/..2h/4", 1, false).unwrap();
        assert_eq!(exploration_path.num_of_paths_sweep(), 29);
    }

    #[test]
    fn num_of_paths_sweep_from_root_works_02() {
        let exploration_path = ExplorationPath::new(None, "*a/..2h/4", 3, false).unwrap();
        assert_eq!(exploration_path.num_of_paths_sweep(), 57);
    }
}
