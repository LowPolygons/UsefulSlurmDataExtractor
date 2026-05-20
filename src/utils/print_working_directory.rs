use std::path::Path;

pub fn print_working_directory(
    working_directory: &Path,
    suppress_files: bool,
) -> Result<(), String> {
    if working_directory
        .try_exists()
        .map_err(|e| format!("Couldn't determine if working directory exists: {e}"))?
        && working_directory.is_dir()
    {
        let mut elements_in_dir = working_directory
            .read_dir()
            .map_err(|_| String::from("Failed to get elements in directory"))?;

        let mut index = 0;
        elements_in_dir.try_for_each(|elem| -> Result<(), String> {
            if index >= 20 && suppress_files {
                if index == 20 {
                    println!(".. Some files Hidden ..");
                }
                return Ok(());
            }

            let path = elem
                .map_err(|_| String::from("Bad element in directory"))?
                .path();

            if path.is_dir() {
                println!("..Dir.. {:?}", path);
            } else {
                println!("..File.. {:?}", path);
            }

            index = index + 1;
            Ok(())
        })?;
    } else {
        println!("Couldn't find working directory");
    }
    Ok(())
}
