// stdlib imports
use std::env;
use std::fs::{self, File};
use std::io::{self, BufWriter, Write, Read};
use std::path::{Path};
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

pub fn build_zip(base_dir: String, output: String, zip_only: bool) {
    println!("Creating {}", &output);

    let bundle_file = match File::create(&output) {
        Ok(f) => { f }
        Err(err) => {
            println!("Error: could not open file {:?} - {:?}", output, err.kind());
            process::exit(1);
        }
    };

    let mut writer = BufWriter::new(&bundle_file);

    if zip_only {
        println!("Creating plain zip");
    } else {
        let exe = env::current_exe().unwrap();

        println!("Embedding nucleus from {}", exe.to_str().unwrap());
        let mut file = File::open(exe).unwrap();

        let mut buf: Vec<u8> = Vec::new();
        match file.read_to_end(&mut buf) {
            Err(err) => { panic!(err.to_string()) }
            _ => {}
        }

        match writer.write_all(buf.as_slice()) {
            Err(err) => { panic!(err.to_string()) }
            _ => {}
        }
    }

    let mut archive = zip::ZipWriter::new(writer);

    let zip_base = Path::new(&base_dir);
    match write_zip_from_dir(zip_base, zip_base, &mut archive) {
        Err(err) => {
            panic!(err)
        }
        _ => {}
    }

    let metadata = bundle_file.metadata().unwrap();
    let mut perms = metadata.permissions();
    perms.set_mode(0o777);
    match fs::set_permissions(&output, perms) {
        Err(err) => { panic!(err.to_string()) }
        _ => {}
    }
}

fn write_zip_from_dir(zip_base: &Path, path: &Path, archive: &mut zip::ZipWriter<BufWriter<&File>>) -> io::Result<()> {
    for entry in try!(fs::read_dir(path)) {
        let file = try!(entry);

        // recurse
        if try!(fs::metadata(file.path())).is_dir() {
            try!(write_zip_from_dir(zip_base, &file.path(), archive));
            continue;
        }

        match write_zip_file(zip_base, &file.path(), archive) {
            Err(err) => { panic!(err.to_string()) }
            _ => {}
        }
    }

    Ok(())
}

fn write_zip_file(zip_base: &Path, filename: &Path, archive: &mut zip::ZipWriter<BufWriter<&File>>) -> zip::result::ZipResult<()> {
    let mut corrected_base = zip_base.to_str().unwrap().to_owned();
    corrected_base.push('/');
    let local_path = filename.to_str().unwrap().replace(&corrected_base, "");

    println!("Adding {} as {}", filename.to_str().unwrap(), &local_path);

    try!(archive.start_file(local_path, zip::CompressionMethod::Bzip2));

    let mut file = File::open(filename).unwrap();

    let mut buf: Vec<u8> = Vec::new();
    match file.read_to_end(&mut buf) {
        Err(err) => { panic!(err.to_string()) }
        _ => {}
    }

    try!(archive.write_all(buf.as_slice()));

    Ok(())
}
