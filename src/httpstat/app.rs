use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Write;
use clap::ArgMatches;
use curl;
use rand;
use rand::Rng;

use super::{Body, Header, Time};

pub fn run(args: &ArgMatches) -> Result<(), Box<Error>> {
  let url = args.value_of("url").unwrap();

  let client = try!(curl::easy::Easy::new());
  try!(client.set_url(url));

  let response = try!(client.perform());

  // print header
  println!("{}", Header(response.header));

  // print body
  let mut tempfile_path = env::temp_dir();
  tempfile_path.set_file_name(rand::thread_rng().gen_ascii_chars().take(20).collect::<String>());
  let mut tempfile = try!(File::create(&tempfile_path));
  try!(tempfile.write_all(response.body.as_bytes()));
  println!("{}", Body(tempfile_path.to_string_lossy().into_owned()));

  // print status
  let time = try!(client.get_time());
  println!("{}", Time(url.starts_with("https"), time));

  Ok(())
}