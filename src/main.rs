extern crate rusoto_core;
extern crate rusoto_s3;
extern crate chrono;
extern crate clap;

use std::error::Error;
use std::process::exit;

use rusoto_core::{DefaultCredentialsProvider, Region};
use rusoto_core::default_tls_client;
use rusoto_s3::{PutObjectRequest, PutObjectOutput, S3Client, S3};

use chrono::prelude::*;

use clap::{App, Arg};

const BUCKET_KEY: &'static str = "bucket";
const FILE_KEY: &'static str = "file";

fn main() {
    match inner_main() {
        Ok(_) => exit(0),
        Err(e) => {
            println!("Something went horribly wrong: {}", e);
            exit(1)
        }
    }
}

fn inner_main() -> Result<(), Box<Error>> {
    let version = version();
    let app = App::new("taps3")
        .version(version.as_str())
        .author("Lloyd (github.com/lloydmeta)")
        .about(
            "taps3 (tapsy) is a command that just writes the current time to a given \
            S3 bucket and filename",
        )
        .arg(
            Arg::with_name(BUCKET_KEY)
                .short("B")
                .long(BUCKET_KEY)
                .takes_value(true)
                .number_of_values(1)
                .required(true)
                .validator(|s| non_empty_string_validator(&s, BUCKET_KEY))
                .help(
                    "The name of the bucket that you want to write your tap file to.",
                ),
        )
        .arg(
            Arg::with_name(FILE_KEY)
                .short("F")
                .long(FILE_KEY)
                .takes_value(true)
                .number_of_values(1)
                .required(false)
                .default_value("tapped")
                .validator(|s| non_empty_string_validator(&s, FILE_KEY))
                .help("The name of the file that you want to write to."),
        );

    // in case we need to print help
    let mut app_clone = app.clone();
    let matches = app.get_matches();
    match (matches.value_of(BUCKET_KEY), matches.value_of(FILE_KEY)) {
        (Some(bucket_raw), Some(file_raw)) => {
            let bucket = bucket_raw.trim();
            let file = file_raw.trim();
            let write_result = write_time(bucket, file).map(|_| ());
            Ok(write_result?)
        }
        _ => Ok(app_clone.print_help()?),
    }

}

fn write_time(bucket_name: &str, file_name: &str) -> Result<PutObjectOutput, Box<Error>> {
    let provider = DefaultCredentialsProvider::new()?;
    let client = S3Client::new(default_tls_client()?, provider, Region::ApNortheast1);
    let utc: DateTime<Utc> = Utc::now();
    let utc_str = format!("{}", utc);

    let put_req = build_s3_req(bucket_name, file_name, &Some(&utc_str));
    Ok(client.put_object(&put_req)?)
}

/// Create a request for writing to a specified S3 bucket and file name (key) with
/// a given set of contents.
fn build_s3_req(bucket_name: &str, file_name: &str, contents: &Option<&str>) -> PutObjectRequest {
    let mut r = PutObjectRequest::default();
    r.bucket = bucket_name.into();
    r.key = file_name.into();
    r.body = contents.map(|c| c.bytes().collect());
    r
}

/// Return the current crate version
fn version() -> String {
    let (maj, min, pat) = (
        option_env!("CARGO_PKG_VERSION_MAJOR"),
        option_env!("CARGO_PKG_VERSION_MINOR"),
        option_env!("CARGO_PKG_VERSION_PATCH"),
    );
    match (maj, min, pat) {
        (Some(maj), Some(min), Some(pat)) => format!("{}.{}.{}", maj, min, pat),
        _ => "".to_owned(),
    }
}

fn non_empty_string_validator(v: &str, key_name: &str) -> Result<(), String> {
    if v.trim().is_empty() {
        Err(format!("{} should not be empty", key_name))
    } else {
        Ok(())
    }
}
