extern crate argparse;
extern crate glob;

use argparse::{ArgumentParser, StoreTrue, Store};
use glob::glob;
use std::fs;
use std::path::{PathBuf};

fn main() -> std::io::Result<()> {
    let mut debug = false;
    let mut from = 0;
    let mut to = 0;
    let mut dest_folder_basename = String::new();
    let mut src_folder_basename = String::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Quick util app to move mangas folder to Volume folders. Use ?# in your destination folder and source folder basenames to hint where the numbers should go.");
        ap.refer(&mut from)
            .add_option(&["-f", "--from"], Store, "From which volume to start");
        ap.refer(&mut to)
            .add_option(&["-t", "--to"], Store, "To which volume to end");
        ap.refer(&mut dest_folder_basename)
            .add_option(&["-d", "--dest"], Store, "Destination folder basename");
        ap.refer(&mut src_folder_basename)
            .add_option(&["-s", "--src"], Store, "Source folder basename");
        ap.refer(&mut debug)
            .add_option(&["--debug"], StoreTrue, "Show what will be done without doing it.");
        ap.parse_args_or_exit();
    }

    println!("Move folders matching {} starting from {} to {} to folder {}", src_folder_basename, from, to, dest_folder_basename);

    for i in from..to+1 {
        let i_str = i.to_string();

        let d_folder = dest_folder_basename.replace("?#", &i_str);
        match fs::create_dir(&d_folder) {
            Ok(path) => println!("{:?} created successfully.", path),
            Err(why) => println!("Failed to create path because {:?}", why.kind()),
        };

        let g = src_folder_basename.replace("?#", &i_str) + "*";
        println!("Looking for folders matching {}", g);

        for entry in glob(&(g)).unwrap() {
            match entry{
                Ok(path) => {
                    let orig_folder = path.to_str().unwrap();
                    let mut new_folder = PathBuf::from(&d_folder);
                    new_folder.push(&orig_folder);

                    println!("Moving {} to {}", &orig_folder, new_folder.display());
                    match fs::rename(orig_folder, new_folder.to_str().unwrap()) {
                        Ok(_) => println!("Moved {} to {}", &orig_folder, &d_folder),
                        Err(why) => println!("Failed to move {} to {} because {:?}.", &orig_folder, &d_folder.clone(), why.kind()),
                    }
                },
                Err(e) => println!("{:?}", e),
            }
        }

    }

    Ok(())
}
