use std::fs::OpenOptions;
use std::io::{Error, Write};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let args = std::env::args();
    let mut args: Vec<String> = args.collect();

    // remove "cargo-tpl" and "tpl"
    args.remove(0);
    args.remove(0);

    let help = {
        let mut yes = false;
        for (idx, arg) in args.iter().enumerate() {
            if arg == "--help" {
                yes = true;
                args.remove(idx);
                break;
            }
        }
        yes
    };

    if help {
        println!("Usage: cargo tpl [--lib,-l] [--workspace,-w] <project_name>");
        return;
    }

    let workspace = {
        let mut yes = false;
        for (idx, arg) in args.iter().enumerate() {
            if arg == "--workspace" || arg == "-w" {
                yes = true;
                args.remove(idx);
                break;
            }
        }
        yes
    };

    let lib = !workspace && {
        let mut yes = false;
        for (idx, arg) in args.iter().enumerate() {
            if arg == "--lib" || arg == "-l" {
                yes = true;
                args.remove(idx);
                break;
            }
        }
        yes
    };

    if args.len() == 0 {
        println!("Usage: cargo tpl [--lib,-l] [--workspace,-w] <project_name>");
        return;
    }

    let pwd = std::env::current_dir().unwrap();
    let pwd = pwd.to_str().unwrap();

    let project_name = &args[0];

    let ref mut root = PathBuf::from(pwd);
    root.push(project_name);

    if root.exists() {
        println!("{} already exists", project_name);
        return;
    }

    let git_exists = {
        let output = Command::new("git").arg("--version").output();
        match output {
            Ok(_) => true,
            Err(_) => false,
        }
    };

    let author = if git_exists {
        read_gitconfig()
    } else {
        "Unknown".to_string()
    };

    let git_init = git_exists
        && !{
            let mut exists = false;
            let mut iter = root.ancestors();
            while let Some(p) = iter.next() {
                if p.join(".git").exists() {
                    exists = true;
                    break;
                }
            }
            exists
        };

    common::make(root, &author, project_name, lib);

    if workspace {
        workspace::make(root, &author, project_name);
    } else if lib {
        lib::make(root, &author, project_name);
    } else {
        bin::make(root, &author, project_name);
    }

    if git_init {
        Command::new("git")
            .arg("init")
            .current_dir(root)
            .output()
            .unwrap();
    }

    println!("{}", "The project has been initialized. you can:");
    println!("$  cd {}", project_name);
    if workspace {
        println!("  cargo update");
        println!("  cargo test -p package_lib");
        println!("  cargo run -p package_cli");
    } else if lib {
        println!("  cargo update");
        println!("  cargo test");
    } else {
        println!("  cargo update");
        println!("  cargo c");
    }
}

mod workspace {
    use std::{collections::HashMap, path::PathBuf};

    use lazy_static::lazy_static;

    lazy_static! {
        static ref FILES: HashMap<&'static str, &'static str> = HashMap::from([
            (
                "Cargo.toml",
                include_str!("templates-workspace/Cargo.toml.tpl"),
            ),
            (
                "crates/package_lib/Cargo.toml",
                include_str!("templates-workspace/crates/package_lib/Cargo.toml.tpl"),
            ),
            (
                "crates/package_lib/src/lib.rs",
                include_str!("templates-workspace/crates/package_lib/src/lib.rs.tpl"),
            ),
            (
                "crates/package_cli/Cargo.toml",
                include_str!("templates-workspace/crates/package_cli/Cargo.toml.tpl"),
            ),
            (
                "crates/package_cli/src/main.rs",
                include_str!("templates-workspace/crates/package_cli/src/main.rs.tpl"),
            ),
        ]);
    }

    pub(crate) fn make(root: &PathBuf, author: &str, project_name: &str) {
        for (newpath, content) in &*FILES {
            let buf = super::replace(content, author, project_name);
            let buf = buf.as_bytes();
            super::write(root, newpath, buf).unwrap();
        }
    }
}

mod lib {
    use std::{collections::HashMap, path::PathBuf};

    use lazy_static::lazy_static;

    lazy_static! {
        static ref FILES: HashMap<&'static str, &'static str> = HashMap::from([
            ("Cargo.toml", include_str!("templates-lib/Cargo.toml.tpl"),),
            ("src/lib.rs", include_str!("templates-lib/src/lib.rs.tpl"),),
            (
                "benches/benchmark.rs",
                include_str!("templates-lib/benches/benchmark.rs.tpl"),
            ),
        ]);
    }

    pub(crate) fn make(root: &PathBuf, author: &str, project_name: &str) {
        for (newpath, content) in &*FILES {
            let buf = super::replace(content, author, project_name);
            let buf = buf.as_bytes();
            super::write(root, newpath, buf).unwrap();
        }
    }
}

mod bin {
    use std::{collections::HashMap, path::PathBuf};

    use lazy_static::lazy_static;

    lazy_static! {
        static ref FILES: HashMap<&'static str, &'static str> = HashMap::from([
            ("Cargo.toml", include_str!("templates-bin/Cargo.toml.tpl"),),
            ("src/main.rs", include_str!("templates-bin/src/main.rs.tpl"),),
        ]);
    }

    pub(crate) fn make(root: &PathBuf, author: &str, project_name: &str) {
        for (newpath, content) in &*FILES {
            let buf = super::replace(content, author, project_name);
            let buf = buf.as_bytes();
            super::write(root, newpath, buf).unwrap();
        }
    }
}

mod common {
    use std::{collections::HashMap, path::PathBuf};

    use lazy_static::lazy_static;

    lazy_static! {
        static ref FILES: HashMap<&'static str, &'static str> = HashMap::from([
            (
                "LICENSE-APACHE",
                include_str!("templates-common/apache.tpl"),
            ),
            ("LICENSE-MIT", include_str!("templates-common/mit.tpl"),),
            (".gitignore", include_str!("templates-common/gitignore.tpl"),),
            (
                ".editorconfig",
                include_str!("templates-common/editorconfig.tpl"),
            ),
            ("README.md", include_str!("templates-common/readme.md.tpl"),),
        ]);
    }

    pub(crate) fn make(root: &PathBuf, author: &str, project_name: &str, ignore_lock_file: bool) {
        for (newpath, content) in &*FILES {
            let mut buf = super::replace(content, author, project_name);
            if ignore_lock_file && newpath == &".gitignore" {
                buf = buf.replace("# Cargo.lock", "Cargo.lock");
            }
            let buf = buf.as_bytes();
            super::write(root, newpath, buf).unwrap();
        }
    }
}

fn replace(content: &str, author: &str, project_name: &str) -> String {
    content
        .replace("{name}", project_name)
        .replace("{year}", &format!("{}", chrono::Utc::now().format("%Y")))
        .replace("{author}", author)
}

fn write(root: &PathBuf, newpath: &str, buf: &[u8]) -> Result<(), Error> {
    let mut path = root.clone();
    path.push(newpath);

    // never overwrite
    if path.exists() {
        println!("skip {}, the file already exists.", path.display());
        return Ok(());
    }

    // create dir
    match path.parent() {
        Some(dir) => std::fs::create_dir_all(dir)?,
        None => (),
    }

    // write to disk
    let mut oo = OpenOptions::new();
    let mut file = oo.create(true).write(true).open(path)?;
    file.write_all(buf)?;
    file.flush()?;

    Ok(())
}

fn read_gitconfig() -> String {
    // git config user.name
    let mut git_name = String::new();
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("user.name")
        .output();
    match output {
        Ok(output) => {
            if output.status.success() {
                git_name.push_str(String::from_utf8_lossy(&output.stdout).trim());
            }
        }
        Err(_) => {}
    }

    // git config user.email
    let mut git_email = String::new();
    let output = Command::new("git")
        .arg("config")
        .arg("--get")
        .arg("user.email")
        .output();
    match output {
        Ok(output) => {
            if output.status.success() {
                git_email.push_str(String::from_utf8_lossy(&output.stdout).trim());
            }
        }
        Err(_) => {}
    }

    if git_name.len() > 0 && git_email.len() > 0 {
        format!("{} <{}>", git_name, git_email)
    } else {
        "Unknown".to_string()
    }
}
