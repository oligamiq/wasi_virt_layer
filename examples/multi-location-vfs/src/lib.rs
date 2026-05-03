#![cfg_attr(target_arch = "wasm32", no_main)]
#![allow(non_snake_case)]

use wasi_virt_layer::prelude::*;

// Import the multiple target modules using the same names as the package names
// (with hyphens converted to underscores per Rust naming conventions)
import_wasm!(mock_tool_one);
import_wasm!(mock_tool_two);
import_wasm!(mock_tool_three);
import_wasm!(mock_tool_four);

#[cfg(target_arch = "wasm32")]
#[unsafe(no_mangle)]
pub fn main() {
    println!("Starting Multi-Location VFS Example");

    // Call each tool's main function
    println!("Running mock-tool-one...");
    mock_tool_one::_main();

    println!("Running mock-tool-two...");
    mock_tool_two::_main();

    println!("Running mock-tool-three...");
    mock_tool_three::_main();

    println!("Running mock-tool-four...");
    mock_tool_four::_main();

    println!("All tools completed!");
}
