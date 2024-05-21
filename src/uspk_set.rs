use std::{sync::Arc, time::Instant};

use num_format::{Locale, ToFormattedString};
use tracing::info;

use crate::error::RetrieverError;

#[derive(Debug)]
pub struct UnspentScriptPupKeysSet {
    set: Arc<hashbrown::HashSet<Vec<u8>>>,
}

impl UnspentScriptPupKeysSet {
    pub fn new() -> Self {
        let set: hashbrown::HashSet<Vec<u8>> = hashbrown::HashSet::new();
        UnspentScriptPupKeysSet { set: Arc::new(set) }
    }
    pub fn populate_with_dump_file(&mut self, dump_file_path: &str) -> Result<(), RetrieverError> {
        let creation_start = Instant::now();
        let mut set = hashbrown::HashSet::new();
        let mut dump = txoutset::Dump::new(dump_file_path, txoutset::ComputeAddresses::No)?;
        // Loop information.
        let step_size = 100u64;
        let mut average_step_time_in_micros = 0u128;
        let total_loops = dump.utxo_set_size;
        let mut loops_done = 0u64;
        let mut steps_done = 0u128;
        let mut steps_remaining = (total_loops / step_size) as u128;
        let mut step_start_time = Instant::now();
        // Loop.
        while let Some(txout) = dump.next() {
            set.insert(txout.script_pubkey.as_bytes().to_vec());
            // Loop info stuff.
            loops_done += 1;
            if loops_done % step_size == 0 {
                steps_done += 1;
                steps_remaining -= 1;
                average_step_time_in_micros = (step_start_time.elapsed().as_micros()
                    + (steps_done - 1) * average_step_time_in_micros)
                    / steps_done as u128;
                let remaining_time_in_milis = average_step_time_in_micros * steps_remaining;
                info!(
                    "Utxos moved to database: {} of {}",
                    loops_done.to_formatted_string(&Locale::en),
                    total_loops.to_formatted_string(&Locale::en)
                );
                info!("Estimated time to completion: ~{} minutes.", (1 + remaining_time_in_milis / 60_000_000).to_formatted_string(&Locale::en));
                step_start_time = Instant::now();
            }
        }
        info!(
            "UTXO database of {} unspent scripts populated in ~{} mins.",
            total_loops.to_formatted_string(&Locale::en),
            1 + creation_start.elapsed().as_secs() / 60
        );
        self.set = Arc::new(set);
        Ok(())
    }

    // pub fn search_for_path_descriptor_pairs_and_return_those_present(
    //     &self,
    //     path_descriptor_pairs_vec: &Vec<PathDescriptorPair>,
    // ) -> Vec<PathDescriptorPair> {
    //     // Loop information.
    //     let creation_start = Instant::now();
    //     let step_size = 1000u64;
    //     let mut average_step_time_in_milis = 0u128;
    //     let total_loops = path_descriptor_pairs_vec.len() as u64;
    //     let mut loops_done = 0u64;
    //     let mut steps_done = 0u128;
    //     let mut steps_remaining = (total_loops / step_size) as u128;
    //     let pb = indicatif::ProgressBar::new(total_loops as u64)
    //         .with_prefix("Searching in-memory UTXO database: ");
    //     pb.set_style(
    //         ProgressStyle::with_template(&format!("{{prefix}}▕{{bar:.{}}}▏ {{msg}}", "╢▌▌░╟"))
    //             .unwrap(),
    //     );
    //     let mut step_start_time = Instant::now();
    //     // Loop.
    //     let mut finds = vec![];
    //     for path_descriptor_pair in path_descriptor_pairs_vec.iter() {
    //         if self
    //             .set
    //             .contains(&path_descriptor_pair.1.script_pubkey().to_bytes())
    //         {
    //             finds.push(path_descriptor_pair.to_owned())
    //         };
    //         loops_done += 1;
    //         if loops_done % step_size == 0 {
    //             steps_done += 1;
    //             steps_remaining -= 1;
    //             average_step_time_in_milis = (step_start_time.elapsed().as_millis()
    //                 + (steps_done - 1) * average_step_time_in_milis)
    //                 / steps_done as u128;
    //             let remaining_time_in_milis = average_step_time_in_milis * steps_remaining;
    //             pb.inc(step_size);
    //             pb.clone().with_message(format!(
    //                 "{} / {}\nEstimated time to completion: ~{} minutes.",
    //                 loops_done.to_formatted_string(&Locale::en),
    //                 total_loops.to_formatted_string(&Locale::en),
    //                 1 + remaining_time_in_milis / 60_000,
    //             ));
    //             step_start_time = Instant::now();
    //         };
    //     }
    //     pb.finish_with_message(format!(
    //         "UTXO database searched for {} descriptors in ~{} mins.",
    //         total_loops.to_formatted_string(&Locale::en),
    //         1 + creation_start.elapsed().as_secs() / 60
    //     ));
    //     finds
    // }

    pub fn get_inner_set(&self) -> Arc<hashbrown::HashSet<Vec<u8>>> {
        self.set.clone()
    }
}
