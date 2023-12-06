use std::fs;
use std::process::Command;
use toml::Value;

fn get_workspace_packages() -> Vec<String> {
    // Read the contents of Cargo.toml
    let cargo_toml_content = fs::read_to_string("Cargo.toml").expect("Failed to read Cargo.toml");

    // Parse the contents as TOML
    let toml_value: Value = cargo_toml_content.parse().expect("Failed to parse Cargo.toml as TOML");

    // Get the list of workspace members
    if let Some(members) = toml_value.get("workspace").and_then(|t| t.get("members")) {
        if let Some(members_array) = members.as_array() {
            // Extract package names as strings
            return members_array
                .iter()
                .filter_map(|member| member.as_str().map(String::from))
                .collect();
        }
    }

    // Return an empty vector if workspace members are not found
    Vec::new()
}

fn main() {
    // Get the list of workspace packages from Cargo.toml
    let workspace_packages = get_workspace_packages();

    for package in workspace_packages {
        // Create a Command to run "cargo run -p %package_name%"
        let mut cmd = Command::new("cargo");
        cmd.arg("run").arg("-p").arg(&package);

        // Execute the command and capture both stdout and stderr
        let output = cmd.output().expect("Failed to execute command");

        // Print stdout
        if !output.stdout.is_empty() {
            let stdout_message = String::from_utf8_lossy(&output.stdout);
            println!("cargo run -p {} stdout:\n{}", package, stdout_message);
        }

        // Print stderr
        if !output.stderr.is_empty() {
            let stderr_message = String::from_utf8_lossy(&output.stderr);
            eprintln!("cargo run -p {} stderr:\n{}", package, stderr_message);
        }

        // Check if the command was successful
        if output.status.success() {
            println!("cargo run -p {} executed successfully", package);
        } else {
            eprintln!("cargo run -p {} failed", package);
        }
    }
}
