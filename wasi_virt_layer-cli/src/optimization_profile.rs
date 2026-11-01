//! Explicit speed policy, separate from the historical size-driven optimizer.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum OptimizationProfile {
    Size,
    SpeedO4,
}

const LEVELS: &[&str] = &["-O", "-O0", "-O1", "-O2", "-O3", "-O4", "-Oz", "-Os"];

impl OptimizationProfile {
    pub(crate) fn parse(value: Option<&str>) -> Result<Self, &'static str> {
        match value {
            None | Some("") => Ok(Self::Size),
            Some("speed-o4") => Ok(Self::SpeedO4),
            _ => Err("unsupported WVL_OPTIMIZATION_PROFILE"),
        }
    }

    pub(crate) fn single_pass(self) -> bool {
        self == Self::SpeedO4
    }

    pub(crate) fn keep_first_output(self, before: u64, after: u64, force: bool) -> bool {
        self == Self::SpeedO4 || force || after < before
    }

    pub(crate) fn arguments(self, additional: &[&str]) -> Vec<String> {
        let mut args: Vec<String> = additional
            .iter()
            .filter(|arg| !LEVELS.contains(arg))
            .map(|arg| (*arg).to_owned())
            .collect();
        if self == Self::SpeedO4 {
            args.push("-O4".to_owned());
        } else {
            args.extend(
                additional
                    .iter()
                    .filter(|arg| LEVELS.contains(arg))
                    .map(|arg| (*arg).to_owned()),
            );
            if !additional.iter().any(|arg| LEVELS.contains(arg)) {
                args.push("-Oz".to_owned());
            }
        }
        args
    }

    pub(crate) fn features(self) -> &'static [&'static str] {
        if self == Self::SpeedO4 {
            // Keep intermediate multi-memory and EH support, but do not enable
            // GC/shared-everything transforms unsupported by the translator.
            &[
                "--enable-threads",
                "--enable-bulk-memory",
                "--enable-reference-types",
                "--enable-simd",
                "--enable-exception-handling",
                "--enable-multivalue",
                "--enable-multimemory",
            ]
        } else {
            &[
                "--enable-threads",
                "--enable-bulk-memory",
                "--enable-reference-types",
                "--enable-simd",
                "--enable-exception-handling",
                "--enable-shared-everything",
                "--enable-multivalue",
                "--enable-multimemory",
                "--enable-gc",
            ]
        }
    }
}
