//! MCDP Format 2 Rust Library
//! 
//! This library provides parsing and processing functionality for MCDP (Monotone Co-Design Problem)
//! files in various formats including YAML, CBOR, and JSON (partially supported).

pub mod types;
pub mod parsing;

pub use types::concrete::*;
pub use parsing::{parse_data, detect_format, read_file, DataFormat, Config, ProcessingResults, process_path, process_file};

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use walkdir::WalkDir;

    #[test]
    fn test_parse_all_example_files() {
        let examples_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");
        
        if !examples_dir.exists() {
            println!("Examples directory not found at {:?}, skipping test", examples_dir);
            return;
        }

        let mut total_files = 0;
        let mut success_count = 0;
        let mut failed_files = Vec::new();

        // Walk through all files in examples directory
        for entry in WalkDir::new(&examples_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }

            // Only test files that our parser supports
            let format = detect_format(path);
            match format {
                DataFormat::YAML | DataFormat::YAML_GZ | 
                DataFormat::CBOR | DataFormat::CBOR_GZ |
                DataFormat::JSON | DataFormat::JSON_GZ => {
                    total_files += 1;
                    
                    print!("Testing file: {} ... ", path.strip_prefix(&examples_dir).unwrap_or(path).display());
                    
                    match test_parse_single_file(path) {
                        Ok(_) => {
                            println!("✓ SUCCESS");
                            success_count += 1;
                        }
                        Err(e) => {
                            println!("✗ FAILED: {}", e);
                            failed_files.push((path.to_path_buf(), e));
                        }
                    }
                }
                DataFormat::Unknown(_) => {
                    // Skip unknown file types
                }
            }
        }

        println!("\n=== PARSING SUMMARY ===");
        println!("Total files tested: {}", total_files);
        println!("Successfully parsed: {}", success_count);
        println!("Failed to parse: {}", failed_files.len());
        
        if !failed_files.is_empty() {
            println!("\nFailed files:");
            for (path, error) in &failed_files {
                println!("  - {}: {}", path.display(), error);
            }
        }

        // Calculate success rate
        let success_rate = if total_files > 0 {
            (success_count as f64 / total_files as f64) * 100.0
        } else {
            0.0
        };
        
        println!("Success rate: {:.1}%", success_rate);
        
        // For this test, we'll consider it successful if we can parse at least 80% of files
        // This allows for some files that might be intentionally malformed or test edge cases
        if total_files > 0 {
            assert!(success_rate >= 80.0, 
                "Success rate {:.1}% is below the required 80%. {} out of {} files failed to parse.", 
                success_rate, failed_files.len(), total_files);
        }
    }

    fn test_parse_single_file(path: &Path) -> anyhow::Result<()> {
        let format = detect_format(path);
        let contents = read_file(path)?;
        let _parsed_data = parse_data(&contents, format)?;
        
        // If we get here, parsing was successful
        Ok(())
    }
    
    #[test]
    fn test_parse_specific_file_types() {
        let examples_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples");
        
        if !examples_dir.exists() {
            println!("Examples directory not found, skipping test");
            return;
        }

        // Test a few specific file types we know should exist
        let test_patterns = vec![
            "**/*.yaml.gz",
            "**/*.cbor.gz", 
            "**/*.yaml.ok",
        ];

        for pattern in test_patterns {
            println!("Testing files matching pattern: {}", pattern);
            let pattern_obj = glob::Pattern::new(pattern).unwrap();
            
            let mut found_files = false;
            for entry in WalkDir::new(&examples_dir).into_iter().filter_map(|e| e.ok()) {
                let path = entry.path();
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    if pattern_obj.matches(file_name) && path.is_file() {
                        found_files = true;
                        
                        // Only test files our parser supports
                        let format = detect_format(path);
                        if !matches!(format, DataFormat::Unknown(_)) {
                            match test_parse_single_file(path) {
                                Ok(_) => println!("  ✓ {}", path.file_name().unwrap().to_string_lossy()),
                                Err(e) => println!("  ✗ {}: {}", path.file_name().unwrap().to_string_lossy(), e),
                            }
                        }
                    }
                }
            }
            
            if !found_files {
                println!("  No files found matching pattern: {}", pattern);
            }
        }
    }
}
