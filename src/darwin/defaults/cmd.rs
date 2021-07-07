use std::process::Command;

pub fn list_domains() -> Vec<String> {
    let output = Command::new("defaults")
        .args(&["domains"])
        .output()
        .expect("failed to list domains");

    let s = String::from_utf8_lossy(&output.stdout).to_string();

    return s.split(",").map(|d| d.trim().to_string()).collect();
}

pub fn export_xml(domain: &str) -> String {
    let output = Command::new("defaults")
        .args(&["export", domain, "-"])
        .output()
        .expect("failed to export xml");

    return String::from_utf8_lossy(&output.stdout)
        .replace("<array/>", "<array></array>")
        .replace("<dict/>", "<dict></dict>")
        .replace("<true/>", "<true></true>")
        .replace("<false/>", "<false></false>")
        .to_string();
}
