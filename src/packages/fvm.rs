use crate::installer::{git::{GitConfig, GitFileIdentifier}, utils::file_utils::InstallType};

pub fn get_fvm_config() -> GitConfig {
    GitConfig {
        package: String::from("leoafarias/fvm"),
        version: String::from("latest"),
        git_identifiers: vec![
            GitFileIdentifier {
                os_name: String::from("windows"),
                arch: String::from("x86_64"),
                os_identifier: String::from("windows"),
                arch_identifier: String::from("x64")
            },
            GitFileIdentifier {
                os_name: String::from("windows"),
                arch: String::from("x86"),
                os_identifier: String::from("windows"),
                arch_identifier: String::from("ia32")
            },
            GitFileIdentifier {
                os_name: String::from("macos"),
                arch: String::from("x86_64"),
                os_identifier: String::from("macos"),
                arch_identifier: String::from("x64")
            },
            GitFileIdentifier {
                os_name: String::from("linux"),
                arch: String::from("x86_64"),
                os_identifier: String::from("linux"),
                arch_identifier: String::from("x64")
            },
            GitFileIdentifier {
                os_name: String::from("linux"),
                arch: String::from("x86"),
                os_identifier: String::from("linux"),
                arch_identifier: String::from("ia32")
            }
        ],
        install_type: InstallType::Command,
        archive_subfolder: String::from("fvm")
    }
}