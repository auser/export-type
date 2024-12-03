# export-type

A Rust proc-macro crate for automatically generating TypeScript type definitions from Rust structs and enums.

## Features

- Export Rust structs and enums to TypeScript
- Support for generics
- Field renaming with `rename` and `rename_all` attributes
- Support for common Rust types:
  - Basic types (numbers, strings, booleans)
  - Collections (Vec, HashMap)
  - Optional values (Option<T>)
  - Custom types
- Generates a single TypeScript file with all types

## Installation

Add this to your `Cargo.toml`:

```toml
export-type = { version = "0.1.1", optional = true }
```

## Usage

Use the `#[export_type]` attribute on structs and enums.

Example:

```rust
#[derive(ExportType)]
#[export_type(rename_all = "camelCase", path = "frontend/src/types")]
struct MyStruct {
    field: String,
}

#[derive(ExportType)]
#[export_type(rename_all = "camelCase", path = "frontend/src/types")]
enum MyEnum {
    Variant1,
    Variant2,
}
```

And you'll get a `index.ts` file in the specified path with the following contents:

```typescript
export type MyStruct = {
    field: string;
};

export type MyEnum = "Variant1" | "Variant2";
```

## Build at build-time

You'll need to add a `build.rs` file to your project that builds the TypeScript file at build-time. Here's an example of how to do this:

```rust
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // Skip file copying during cargo publish
    if env::var("CARGO_PUBLISH").is_ok() {
        return;
    }

    // Get the output directory from the proc macro
    if let Ok(types_dir) = env::var("TYPES_OUT_DIR") {
        let types_path = Path::new(&types_dir).join("types");
        
        // Get the target directory for types (customize this path as needed)
        let target_dir = Path::new("frontend/src/types");
        
        // Only copy if the types were generated
        if types_path.exists() {
            // Create target directory if it doesn't exist
            fs::create_dir_all(target_dir).expect("Failed to create target directory");
            
            // Copy the generated index.ts file
            if let Ok(entries) = fs::read_dir(&types_path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() && path.extension().map_or(false, |ext| ext == "ts") {
                        let file_name = path.file_name().unwrap();
                        let target_path = target_dir.join(file_name);
                        
                        fs::copy(&path, &target_path)
                            .unwrap_or_else(|e| panic!("Failed to copy {:?}: {}", file_name, e));
                    }
                }
            }
        }
    }

    // Tell Cargo to rerun this if any of these change
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-env-changed=CARGO_PUBLISH");
    println!("cargo:rerun-if-env-changed=TYPES_OUT_DIR");
}
```

Also add the following to your `Cargo.toml`:

```toml
[package]
build = "build.rs"
```
