use std::fs::{self, File};

fn create_illustrator_structure(project_name :&str) -> Result<(), std::io::Error>{
    fs::create_dir(project_name)?;

    // Create main folders: source, output, docs and design
    let folders = ["source", "output", "docs", "design"];
    for folder in folders.iter() {
        let path = format!("{}/{}", project_name, folder);
        fs::create_dir(&path)?;
    }

    // Make subfolders for the source folder
    let source_folders = ["fonts", "pictures"];
    for folder in source_folders.iter() {
        let path = format!("{}/source/{}", project_name, folder);
        fs::create_dir(&path)?;
    }

    // Add the readme file in the main folder
    let md_path = format!("{}/README.md", project_name);
    File::create(md_path)?;

    println!("Illustrator folder structure successfully created.");

    Ok(())
}


fn main() {
    let args :Vec<String> = std::env::args().collect();
    
    if args.len() != 3 {
        eprintln!("Usage: mfs <type> <project_name>");
        std::process::exit(1);
    }

    let project_name :&str = &args[2];
    let type_project :&str = &args[1];

    match type_project {  // g for game, 3d for 3d project
        "ai" => {
            match create_illustrator_structure(project_name){
                Ok(_) => (),
                Err(err) => eprintln!("Couldn't make folder: {}", err),
            }
        },

        _ => println!("That is not known"),

    }

}