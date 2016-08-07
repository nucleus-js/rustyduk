// stdlib imports
use std::env;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write, Read};
use std::path::Path;
use std::process;
use std::os::unix::fs::PermissionsExt;
//


// crate imports
extern crate zip;
//

// pub enum BuildType {
//     ZIP,
//     // TODO(Fishrock123): impl these
//     // LINKED,
//     // EMBEDDED,
// }

// bundle a directory into a zip file, with options to embed nucleus or not
pub fn build_zip(base_dir: String, output: String, zip_only: bool) {
    println!("Creating {}", &output);

    // Create a new file
    let bundle_file = match File::create(&output) {
        Ok(f) => f,
        Err(err) => {
            println!("Error: could not open file {:?} - {:?}", output, err.kind());
            process::exit(1);
        }
    };

    let mut writer = BufWriter::new(&bundle_file);

    if zip_only {
        // Nothing to do here
        println!("Creating plain zip");
    } else {
        // embed the nucleus executable by prepending it
        // to the beginning of the file

        // this is the path to the nucleus executable
        let exe = env::current_exe().unwrap();

        println!("Embedding nucleus from {}", exe.to_str().unwrap());
        let mut file = File::open(exe).unwrap();

        // read all of the exe
        let mut buf: Vec<u8> = Vec::new();
        match file.read_to_end(&mut buf) {
            Err(err) => panic!(err.to_string()),
            _ => {}
        }

        // write all of the exe to the new file
        match writer.write_all(buf.as_slice()) {
            Err(err) => panic!(err.to_string()),
            _ => {}
        }
    }

    // start up the zip writer
    let mut archive = zip::ZipWriter::new(writer);

    // make a copy of the base dir we should correct the file paths to
    let zip_base = Path::new(&base_dir);

    // write a directory to a zip file
    match write_zip_from_dir(zip_base, zip_base, &mut archive) {
        Err(err) => panic!(err),
        _ => {}
    }

    // make the resulting file executable on most platforms
    // get the existing perms
    let metadata = bundle_file.metadata().unwrap();
    let mut perms = metadata.permissions();
    // change them to 777 and write them
    perms.set_mode(0o777);
    match fs::set_permissions(&output, perms) {
        Err(err) => panic!(err.to_string()),
        _ => {}
    }
}

// write a directory to a zip file
fn write_zip_from_dir(zip_base: &Path,
                      path: &Path,
                      archive: &mut zip::ZipWriter<BufWriter<&File>>)
                      -> io::Result<()> {

    // iterate over directory entries
    for entry in try!(fs::read_dir(path)) {
        let file = try!(entry);

        // recurse if an entry is a directory
        if try!(fs::metadata(file.path())).is_dir() {
            try!(write_zip_from_dir(zip_base, &file.path(), archive));
            continue;
        }

        // read an an actual file and write it to the zip
        match write_zip_file(zip_base, &file.path(), archive) {
            Err(err) => panic!(err.to_string()),
            _ => {}
        }
    }

    Ok(())
}

// read an an actual file and write it to a zip
fn write_zip_file(zip_base: &Path,
                  filename: &Path,
                  archive: &mut zip::ZipWriter<BufWriter<&File>>)
                  -> zip::result::ZipResult<()> {

    // figure out the base dir we should correct the file paths to so that they
    // are accessible from within the zip file by relative file paths
    let mut corrected_base = zip_base.to_str().unwrap().to_owned();
    corrected_base.push('/');
    let local_path = filename.to_str().unwrap().replace(&corrected_base, "");

    println!("Adding {} as {}", filename.to_str().unwrap(), &local_path);

    // start a new file in the zip
    try!(archive.start_file(local_path, zip::CompressionMethod::Bzip2));

    let mut file = File::open(filename).unwrap();

    // read the file from disk
    let mut buf: Vec<u8> = Vec::new();
    match file.read_to_end(&mut buf) {
        Err(err) => panic!(err.to_string()),
        _ => {}
    }

    // write it to the zip
    try!(archive.write_all(buf.as_slice()));

    Ok(())
}
