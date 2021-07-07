mod darwin;
use darwin::defaults::cmd::{export_xml, list_domains};
use darwin::defaults::xml::parse_xml;

use std::io;

fn main() {
    println!("Enter domain if you want to specify.");

    let mut domain = String::new();

    io::stdin()
        .read_line(&mut domain)
        .expect("Failed to read line");

    domain = domain.trim().to_string();

    let domains = list_domains();

    if domain != "" {
        if !domains.iter().any(|d| d == domain.as_str()) {
            println!("domain is not found.");
            return;
        }

        present(domain.as_str(), parse_xml(&export_xml(&domain)));
    } else {
        for d in domains {
            present(d.as_str(), parse_xml(&export_xml(&d)));
        }
    }
}

fn present(
    domain: &str,
    ktv: Vec<(String, String, String)>,
) {
    for (k, _, v) in ktv {
        println!(
            "/usr/libexec/PlistBuddy -c \"set {} {}\" ~/Library/Preferences/{}.plist",
            k,
            v,
            domain,
        );
    }
}
