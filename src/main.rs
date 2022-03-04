use colored::Colorize;
use std::collections::HashMap;
use std::io::stdin;
use std::io::Write;
use std::process::Output;

fn main() {
    println!("{} {}", "README.md".green().bold(), "Generator");

    // Taking inputs from the user
    let mut project_name = String::new();
    println!("{}", "Enter the project name: ".blue());
    stdin()
        .read_line(&mut project_name)
        .expect("Failed to read line");

    let mut short_description = String::with_capacity(120);
    println!(
        "{}",
        "Enter the short description (It should be short, concise to hook the reader) : ".blue()
    );
    stdin()
        .read_line(&mut short_description)
        .expect("Failed to read line");

    let mut image_url = String::new();
    println!("{}", "Enter the image url (Leave blank if none): ".blue());
    stdin()
        .read_line(&mut image_url)
        .expect("Failed to read line");

    let mut license = String::new();
    println!(
        "{}",
        "Enter the license - (MIT/Apache/GPL / Enter any custom license) (Leave blank if none): "
            .blue()
    );
    stdin()
        .read_line(&mut license)
        .expect("Failed to read line");

    let mut demo = String::new();
    println!(
        "{}",
        "Link for image/gif demonstrating your project ".blue()
    );
    println!("{}", "(Leave blank if none): ".blue());
    stdin().read_line(&mut demo).expect("Failed to read line");

    let mut installation_command = String::new();
    println!("{}","Enter the installation command(s). You can add multiple commands by seperating with && (If left blank, I will generate one for you automatically): ".blue());
    stdin()
        .read_line(&mut installation_command)
        .expect("Failed to read line");

    // Checks
    assert!(
        project_name.trim().len() > 0,
        "Project name cannot be empty"
    );
    assert!(
        short_description.trim().len() > 0,
        "Short description cannot be empty"
    );

    file_factory(
        project_name,
        short_description,
        image_url,
        license,
        demo,
        installation_command,
    );
}

fn file_factory(
    project_name: String,
    short_description: String,
    image_url: String,
    license: String,
    demo: String,
    installation_command: String,
) {
    // Creating the README.md file
    let mut file = std::fs::File::create("README.md").expect("Failed to create file");
    let mut content = String::new();

    content.push_str("<div align=\"center\">\n");
    content.push_str(
        format!(
            "<h1 align=\"center\">Welcome to {}</h1>\n",
            project_name.trim()
        )
        .as_str(),
    );

    let mut licenses: HashMap<&str, &str> = HashMap::new();
    licenses.insert(
        "mit",
        "<img alt=\"License: MIT\" src=\"https://img.shields.io/badge/License-MIT-yellow.svg\" />",
    );
    licenses.insert("apache", "<img alt=\"License: Apache\" src=\"https://img.shields.io/badge/license-Apache%202-blue\" />");
    licenses.insert(
        "gpl",
        "<img alt=\"License: GPL\" src=\"https://img.shields.io/badge/license-GPL-blue\" />",
    );

    content.push_str(
        licenses
            .get(license.trim().to_lowercase().as_str())
            .unwrap_or(
                &format!(
                "<img alt=\"License: {}\" src=\"https://img.shields.io/badge/License-{}-blue\" />",
                license.trim(),
                license.trim()
            )
                .as_str(),
            ),
    );
    content.push_str("<br>\n");

    content.push_str(format!("{}\n", short_description.trim()).as_str());

    if image_url.len() > 5 {
        content.push_str(
            format!(
                "<img src=\"{}\" alt=\"{}\" width=\"500\" height=\"500\">\n",
                image_url.trim(),
                project_name.trim()
            )
            .as_str(),
        );
    }

    content.push_str("</div>\n");

    content.push_str("\n***\n");

    if demo.len() > 5 {
        content.push_str(format!("![{}]({})\n", project_name.trim(), demo.trim()).as_str());
    }

    let mut use_command: &str = "write use command here";
    // Installation part
    if installation_command.len() > 5 {
        content.push_str(
            format!(
                "\n### Installation\n```\n{}\n```\n",
                installation_command.trim().replace("&& ", "\n")
            )
            .as_str(),
        );
    } else {
        // Gets the current repository url from git
        let repo_url: Output = std::process::Command::new("git")
            .arg("remote")
            .arg("get-url")
            .arg("origin")
            .output()
            .expect("Failed to get repo url");

        let mut installation_command = String::new();
        installation_command.push_str("\n### Installation\n```");

        repo_url
            .stdout
            .as_slice()
            .split(|&x| x == b'\n')
            .for_each(|x| {
                installation_command
                    .push_str(format!("\ngit clone {}", String::from_utf8_lossy(x)).as_str());
            });

        // Change directory - Gets the directory from github link or the current directory
        installation_command.push_str("\ncd ");
        installation_command.push_str(project_name.trim());
        installation_command.push_str("\n");

        // Checks the current folder. If it has Cargo.toml, package.json or requirements.txt, then generate the installation command automatically
        if std::fs::read_to_string("Cargo.toml").is_ok() {
            installation_command.push_str("cargo install");
            use_command = "cargo run";
        } else if std::fs::read_to_string("package.json").is_ok() {
            installation_command.push_str("npm install");
            use_command = "npm start";
        } else if std::fs::read_to_string("requirements.txt").is_ok() {
            installation_command.push_str("pip install -r requirements.txt");
            use_command = "python main.py";
        }

        installation_command.push_str("\n```\n");

        content.push_str(installation_command.as_str());
    }

    // Usage part
    content.push_str("\n### Usage\n");
    content.push_str("```\n");
    content.push_str(use_command);
    content.push_str("\n```\n");

    // Contributing part
    content.push_str("\n### Contributing\n");
    content.push_str("");

    // License part
    content.push_str("\n### License\n");
    content.push_str(
        format!(
            "This project is licensed under the {} license",
            license.trim()
        )
        .as_str(),
    );

    // Show support part
    content.push_str("\n### Show your support\n");
    content.push_str("Leave a ⭐ if you like this project\n");

    content.push_str("\n***\n");
    content.push_str("Readme made with 💖 using [README Generator by Dhravya Shah](https://github.com/Dhravya/readme-generator)");

    // Save the file
    file.write_all(content.as_bytes())
        .expect("Failed to write to file");
}
