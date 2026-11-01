//! Lightweight policy regression, runnable with rustc --test without building the CLI.
#[path = "../src/optimization_profile.rs"]
mod optimization_profile;
use optimization_profile::OptimizationProfile;

#[test]
fn speed_profile_keeps_larger_outputs_and_uses_o4() {
    let profile = OptimizationProfile::parse(Some("speed-o4")).unwrap();
    assert!(profile.keep_first_output(100, 120, false));
    assert!(profile.single_pass());
    assert_eq!(
        profile.arguments(&["--no-inline=init", "-Oz"], false),
        vec!["--no-inline=init", "-O4"]
    );
    assert!(!profile.features().contains(&"--enable-gc"));
    assert!(!profile.features().contains(&"--enable-shared-everything"));
    assert!(profile.features().contains(&"--enable-multimemory"));
}

#[test]
fn default_size_policy_and_explicit_passes_remain_unchanged() {
    let profile = OptimizationProfile::parse(None).unwrap();
    assert!(!profile.keep_first_output(100, 120, false));
    assert!(profile.keep_first_output(100, 120, true));
    assert!(!profile.single_pass());
    assert_eq!(profile.arguments(&[], false), vec!["-Oz"]);
    assert_eq!(
        profile.arguments(&["-O3", "--no-inline=init"], false),
        vec!["--no-inline=init", "-O3"]
    );
    assert!(OptimizationProfile::parse(Some("unknown")).is_err());
}

#[test]
fn compatible_ceiling_is_explicit_for_try_table_only() {
    let profile = OptimizationProfile::parse(Some("max-compatible")).unwrap();
    assert_eq!(profile.arguments(&[], false), vec!["-O4"]);
    assert_eq!(profile.arguments(&[], true), vec!["-O3"]);
    assert!(profile.keep_first_output(100, 120, false));
    assert!(profile.single_pass());
}
