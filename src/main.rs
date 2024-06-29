use clap::Parser;
// use glob::glob;
use std::fs;
use walkdir::WalkDir;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input folder to search for files
    #[arg(short, long)]
    input_folder: String,

    /// String to search for in filenames
    #[arg(short, long)]
    search_string: String,

    /// Destination folder for copied files
    #[arg(short, long)]
    dest_folder: String,

    /// Case sensitive search (default is case insensitive)
    #[arg(short, long, default_value = "false")]
    case_sensitive: bool,
}

// struct Args {
//     /// Pattern to match files
//     #[arg(short, long)]
//     pattern: String,

//     /// Destination folder for copied files
//     #[arg(short, long)]
//     dest_folder: String,
// }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Create destination folder if it doesn't exist
    fs::create_dir_all(&args.dest_folder)?;

    // // Use glob to find matching files
    // for entry in glob(&args.pattern)? {
    //     match entry {
    //         Ok(path) => {
    //             if let Err(e) = process_file(&path, &args.dest_folder) {
    //                 eprintln!("Error processing file {:?}: {}", path, e);
    //             }
    //         }
    //         Err(e) => eprintln!("Error in glob pattern: {}", e),
    //     }
    // }

    // iteration 002

    // Read the directory and process matching files
    // let entries = fs::read_dir(&args.input_folder)?;
    // for entry in entries {
    //     let entry = entry?;
    //     let path = entry.path();
    //     dbg!(&path);
    //     if path.is_file() {
    //         if let Some(file_name) = path.file_name() {
    //             let file_name_str = file_name.to_string_lossy();
    //             let matches = if args.case_sensitive {
    //                 file_name_str.contains(&args.search_string)
    //             } else {
    //                 file_name_str
    //                     .to_lowercase()
    //                     .contains(&args.search_string.to_lowercase())
    //             };

    //             if matches {
    //                 eprintln!("Matched filename {:?}", &file_name );
    //                 // if let Err(e) = process_file(&path, &args.dest_folder) {
    //                 //     eprintln!("Error processing file {:?}: {}", path, e);
    //                 // }
    //             }
    //         }
    //     }
    // }

    // Walk through the directory and process matching files
    for entry in WalkDir::new(&args.input_folder) {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if let Some(file_name) = path.file_name() {
                let file_name_str = file_name.to_string_lossy();
                let matches = if args.case_sensitive {
                    file_name_str.contains(&args.search_string)
                } else {
                    file_name_str
                        .to_lowercase()
                        .contains(&args.search_string.to_lowercase())
                };

                if matches {
                    // eprint!("matched: {:?} >>>> ", &file_name);
                    // eprintln!("\tpath: {:?}", &path);
                    if let Err(e) = process_file(path, &args.dest_folder) {
                        eprintln!("Error processing file {:?}: {}", path, e);
                    }
                }
            }
        }
    }

    Ok(())
}

fn process_file(file_path: &Path, dest_folder: &str) -> Result<(), Box<dyn std::error::Error>> {
    let full_path = fs::canonicalize(file_path)?;
    let _file_name = full_path.file_name().ok_or("Unable to get file name")?;

    // let new_file_name = format!("{}", full_path.to_string_lossy().replace("/", "_"));
    let new_file_name = format!(
        "{}",
        full_path.to_string_lossy()
            .split('/')
            .skip(7)
            .collect::<Vec<&str>>()
            .join("_")
    );
    
    let dest_path: PathBuf = [dest_folder, &new_file_name].iter().collect();

    // dbg!(&dest_path);

    println!("Copying {:?} to {:?}", file_path, dest_path);
    fs::copy(file_path, dest_path)?;

    Ok(())
}
