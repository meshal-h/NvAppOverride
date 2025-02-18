use std::env;
use std::fs::{copy, metadata, read_to_string, set_permissions, write};
use std::process::Command;

fn main() {
    println!("-------------------");
    println!("NVIDIA App Override");
    println!("-------------------");

    // Get paths
    let data_path = env::var("LOCALAPPDATA").unwrap();
    let xml_path = format!(
        "{}\\NVIDIA Corporation\\NVIDIA app\\NvBackend\\ApplicationOntology\\data\\fingerprint.db",
        data_path
    );
    let json_path = format!(
        "{}\\NVIDIA Corporation\\NVIDIA app\\NvBackend\\ApplicationStorage.json",
        data_path
    );

    // Backup files
    println!("Backing up files...");

    let xml_path_bak = format!("{}.bak", &xml_path);
    let json_path_bak = format!("{}.bak", &json_path);

    if metadata(&xml_path_bak).is_ok() {
        println!("...XML backup already exists");
    } else {
        copy(&xml_path, &xml_path_bak).unwrap();
    }

    if metadata(&json_path_bak).is_ok() {
        println!("...JSON backup already exists");
    } else {
        copy(&json_path, &json_path_bak).unwrap();
    }

    // XML override
    println!("Overriding XML file...");
    let xml_file = read_to_string(&xml_path).unwrap();
    let xml_new_file = override_xml(&xml_file);
    let mut xml_perms = metadata(&xml_path).unwrap().permissions();
    xml_perms.set_readonly(false);
    set_permissions(&xml_path, xml_perms.clone()).unwrap();
    write(&xml_path, xml_new_file).unwrap();
    xml_perms.set_readonly(true);
    set_permissions(&xml_path, xml_perms.clone()).unwrap();

    // JSON override
    println!("Overriding JSON file...");
    let json_file = read_to_string(&json_path).unwrap();
    let json_new_file: String = override_json(&json_file);
    write(&json_path, json_new_file).unwrap();

    // Restart Services
    println!("Restarting services...");
    restart_windows_service("NvContainerLocalSystem");
    restart_windows_service("NVDisplay.ContainerLocalSystem");

    // Pause
    println!("Done! Closing in 5 seconds...");
    std::thread::sleep(std::time::Duration::from_secs(5));
}

fn override_xml(file: &str) -> String {
    // Super Resolution
    let file = file.replace(
        "<Disable_SR_Override>1</Disable_SR_Override>",
        "<Disable_SR_Override>0</Disable_SR_Override>",
    );
    let file = file.replace(
        "<Disable_SR_Model_Override>1</Disable_SR_Model_Override>",
        "<Disable_SR_Model_Override>0</Disable_SR_Model_Override>",
    );

    // Frame Generation
    let file = file.replace(
        "<Disable_FG_Override>1</Disable_FG_Override>",
        "<Disable_FG_Override>0</Disable_FG_Override>",
    );

    // Ray Reconstruction
    let file = file.replace(
        "<Disable_RR_Override>1</Disable_RR_Override>",
        "<Disable_RR_Override>0</Disable_RR_Override>",
    );
    let file = file.replace(
        "<Disable_RR_Model_Override>1</Disable_RR_Model_Override>",
        "<Disable_RR_Model_Override>0</Disable_RR_Model_Override>",
    );

    return file;
}

fn override_json(file: &str) -> String {
    // Super Resolution
    let file = file.replace(
        "Disable_SR_Override\":true", 
        "Disable_SR_Override\":false");
    let file = file.replace(
        "Disable_SR_Model_Override\":true",
        "Disable_SR_Model_Override\":false",
    );

    // Frame Generation
    let file = file.replace(
        "Disable_FG_Override\":true", 
        "Disable_FG_Override\":false");

    // Ray Reconstruction
    let file = file.replace(
        "Disable_RR_Override\":true", 
        "Disable_RR_Override\":false");
    let file = file.replace(
        "Disable_RR_Model_Override\":true",
        "Disable_RR_Model_Override\":false",
    );

    return file;
}

fn restart_windows_service(service_name: &str) {
    let output = Command::new("cmd")
        .args(&[
            "/C",
            "net",
            "stop",
            service_name,
            "&&",
            "net",
            "start",
            service_name,
        ])
        .output()
        .unwrap();
    
    if !output.status.success() {
        println!("...Failed to restart service: {}.", service_name);
        println!("......Likely forget to run as an Admin.");
    }
}
