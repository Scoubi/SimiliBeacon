use rand::prelude::*;
use std::{thread::sleep, time::Duration};

fn main() {
    
    let m: i8 = 60; // Default time in minutes to wait between each URL Request
    let uri = vec!["CONTRIBUTING.md", "CodeQL.yml", "LICENSE.txt", "README.md", "SECURITY.md", "cglicenses.json", "cgmanifest.json", "gulpfile.js", "package.json", "product.json"];
    let uri_len = uri.len() - 1;
    let mut r_uri: usize = thread_rng().gen_range(1,uri_len); // random uri selector
    let mut x: i8 = 0; // count the number of itteration to the loop
    let mut y: i8 = thread_rng().gen_range(1,11); // Number of loop to do before changing the URI
    let fqdn = "https://github.com/microsoft/vscode/".to_string().to_owned(); // Domain we use for our C2 config
    let mut url = fqdn.clone() + uri.get(r_uri).unwrap(); // Full URL where to grab the C2 instructions

    loop {

        let jit: f32 = thread_rng().gen_range(0.0,2.0);
        let ttw8: u64 = (m as f32 * 60.0 * jit) as u64; // Time to wait in minutes
        let minutes = Duration::from_secs(ttw8.try_into().unwrap());

        println!("Minutes to wait: {} URL to Fetch: {}", ttw8/60, url);
        sleep(minutes);
        reqwest::blocking::get(url.clone()).expect("request failed");
        x+=1;

        if x == y {
            y = thread_rng().gen_range(1,11);
            x = 0;
            r_uri = thread_rng().gen_range(0, uri_len);
            //println!("R_URI: {r_uri}");
            url = fqdn.clone() + uri.get(r_uri).unwrap();
        }
    }
}
