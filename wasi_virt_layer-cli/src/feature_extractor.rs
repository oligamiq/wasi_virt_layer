use clap::ArgMatches;

pub fn extract_features(
    matches: &ArgMatches,
    wasm_count: usize,
) -> (
    crate::args::VfsBuildOptions,
    Box<[crate::args::VfsBuildOptions]>,
) {
    let mut vfs_opts = crate::args::VfsBuildOptions::default();
    let mut target_opts =
        vec![crate::args::VfsBuildOptions::default(); wasm_count].into_boxed_slice();

    let wasm_indices: Vec<usize> = matches
        .indices_of("wasm")
        .map(|i| i.collect())
        .unwrap_or_default();

    let features_indices: Vec<usize> = matches
        .indices_of("features")
        .map(|i| i.collect())
        .unwrap_or_default();

    let features_values: Vec<String> = matches
        .get_many::<String>("features")
        .map(|i| i.cloned().collect())
        .unwrap_or_default();

    let no_default_indices: Vec<usize> = if matches.get_count("no_default_features") > 0 {
        matches
            .indices_of("no_default_features")
            .map(|i| i.collect())
            .unwrap_or_default()
    } else {
        vec![]
    };

    let no_opt_indices: Vec<usize> = if matches.get_count("no_opt") > 0 {
        matches
            .indices_of("no_opt")
            .map(|i| i.collect())
            .unwrap_or_default()
    } else {
        vec![]
    };

    let no_opt_all_indices: Vec<usize> = if matches.get_count("no_opt_all") > 0 {
        matches
            .indices_of("no_opt_all")
            .map(|i| i.collect())
            .unwrap_or_default()
    } else {
        vec![]
    };

    // Distribute features
    for (idx, val) in features_indices
        .into_iter()
        .zip(features_values.into_iter())
    {
        let mut target_wasm_idx = None;
        for (w_i, w_idx) in wasm_indices.iter().enumerate() {
            if *w_idx < idx {
                target_wasm_idx = Some(w_i);
            } else {
                break;
            }
        }

        if let Some(w_i) = target_wasm_idx {
            target_opts[w_i].features.push(val);
        } else {
            vfs_opts.features.push(val);
        }
    }

    // Distribute no_default_features
    for idx in no_default_indices {
        let mut target_wasm_idx = None;
        for (w_i, w_idx) in wasm_indices.iter().enumerate() {
            if *w_idx < idx {
                target_wasm_idx = Some(w_i);
            } else {
                break;
            }
        }

        if let Some(w_i) = target_wasm_idx {
            target_opts[w_i].no_default_features += 1;
        } else {
            vfs_opts.no_default_features += 1;
        }
    }

    // Distribute no_opt
    for idx in no_opt_indices {
        let mut target_wasm_idx = None;
        for (w_i, w_idx) in wasm_indices.iter().enumerate() {
            if *w_idx < idx {
                target_wasm_idx = Some(w_i);
            } else {
                break;
            }
        }

        if let Some(w_i) = target_wasm_idx {
            target_opts[w_i].no_opt += 1;
        } else {
            vfs_opts.no_opt += 1;
        }
    }

    // Distribute no_opt_all
    for idx in no_opt_all_indices {
        let mut target_wasm_idx = None;
        for (w_i, w_idx) in wasm_indices.iter().enumerate() {
            if *w_idx < idx {
                target_wasm_idx = Some(w_i);
            } else {
                break;
            }
        }

        if let Some(w_i) = target_wasm_idx {
            target_opts[w_i].no_opt_all += 1;
        } else {
            vfs_opts.no_opt_all += 1;
        }
    }

    (vfs_opts, target_opts)
}
