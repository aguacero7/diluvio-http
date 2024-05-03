use std::process::{Command, ExitStatus};
use std::env;
use std::fs;
use reqwest;
use zip;
fn install_php() -> Result<(), String> {
    let os = env::consts::OS;
    println!("Système d'exploitation: {}", os);

    match os {
        "linux" => {
            if let Err(err) = try_package_manager("apt-get") {
                println!("Erreur lors de l'installation de PHP via apt-get: {}", err);
                if let Err(err) = try_package_manager("yum") {
                    println!("Erreur lors de l'installation de PHP via yum: {}", err);
                    if let Err(err) = download_php_from_site(os) {
                        println!("Erreur lors de l'installation de PHP depuis le site de PHP: {}", err);
                        return Err(err);
                    }
                }
            }
        }
        "macos" => {
            if let Err(err) = try_package_manager("brew") {
                println!("Erreur lors de l'installation de PHP via Homebrew: {}", err);
                if let Err(err) = download_php_from_site(os) {
                    println!("Erreur lors de l'installation de PHP depuis le site de PHP: {}", err);
                    return Err(err);
                }
            }
        }
        "windows" => {
            if let Err(err) = try_package_manager("choco") {
                println!("Erreur lors de l'installation de PHP via Chocolatey: {}", err);
                if let Err(err) = download_php_from_site(os) {
                    println!("Erreur lors de l'installation de PHP depuis le site de PHP: {}", err);
                    return Err(err);
                }
            }
        }
        _ => {
            println!("Système d'exploitation non pris en charge");
            return Err("Système d'exploitation non pris en charge".to_string());
        }
    }

    Ok(())
}

fn try_package_manager(manager: &str) -> Result<(), String> {
    println!("Tentative d'installation de PHP via {}", manager);
    let output = Command::new(manager)
        .arg("install")
        .arg("php")
        .output()
        .map_err(|e| format!("Erreur lors de l'exécution de la commande: {}", e))?;

    if output.status.success() {
        println!("PHP installé avec succès via {}", manager);
        Ok(())
    } else {
        Err(format!("L'installation de PHP via {} a échoué", manager))
    }
}
fn download_php_from_site(os: &str) -> Result<(), String> {
    println!("Tentative de téléchargement de PHP depuis le site de PHP...");

    let php_download_url = match os {
        "windows" => "https://windows.php.net/downloads/releases/php-8.3.6-src.zip",
        _ => "https://www.php.net/distributions/php-8.3.6.tar.gz",
    };

    let mut output_file = match os {
        "linux" | "macos" => "/tmp/php.tar.gz".to_string(),
        "windows" => "C:\\Temp\\php.zip".to_string(),
        _ => return Err("Système d'exploitation non pris en charge".to_string()),
    };

    let output_file_path = Path::new(&output_file);
    let mut response = reqwest::blocking::get(php_download_url)
        .map_err(|e| format!("Erreur lors de la requête HTTP: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("Le téléchargement de PHP a échoué avec le code de statut {}", response.status()));
    }

    let mut file = fs::File::create(&output_file_path)
        .map_err(|e| format!("Impossible de créer le fichier de sortie: {}", e))?;

    response.copy_to(&mut file)
        .map_err(|e| format!("Erreur lors de la copie de la réponse HTTP dans le fichier: {}", e))?;

    println!("PHP téléchargé avec succès depuis le site de PHP");

    // Installation de PHP depuis le fichier zip pour Windows
    if os == "windows" {
        if let Err(err) = install_php_from_zip(&output_file_path.to_string_lossy(), "C:\\Temp\\") {
            println!("Erreur lors de l'installation de PHP depuis le fichier zip: {}", err);
            return Err(err);
        }
    }

    Ok(())
}

fn install_php_from_zip(zip_file: &str, destination: &str) -> Result<(), String> {
    println!("Installation de PHP depuis le fichier zip...");

    let file = File::open(zip_file).map_err(|e| format!("Impossible d'ouvrir le fichier zip : {}", e))?;
    let mut archive = ZipArchive::new(file).map_err(|e| format!("Impossible d'ouvrir le fichier zip : {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| format!("Impossible de lire le fichier dans le zip : {}", e))?;
        let mut outpath = Path::new(destination);
        outpath.push(file.sanitized_name());

        if (&*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).map_err(|e| format!("Impossible de créer le répertoire : {}", e))?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).map_err(|e| format!("Impossible de créer le répertoire : {}", e))?;
                }
            }

            let mut outfile = File::create(&outpath).map_err(|e| format!("Impossible de créer le fichier : {}", e))?;
            std::io::copy(&mut file, &mut outfile).map_err(|e| format!("Impossible de copier le fichier : {}", e))?;
        }
    }

    println!("PHP installé avec succès depuis le fichier zip");

    Ok(())
}

pub fn is_php_installed() -> bool {
    match env::consts::OS {
        "linux" => is_php_installed_linux(),
        "macos" => is_php_installed_macos(),
        "windows" => is_php_installed_windows(),
        _ => {
            println!("Système d'exploitation non pris en charge.");
            false
        }
    }
}

fn is_php_installed_linux() -> bool {
    let output = Command::new("which")
        .arg("php")
        .output()
        .expect("La commande a échoué");

    output.status.success()
}

fn is_php_installed_macos() -> bool {
    let output = Command::new("command")
        .arg("-v")
        .arg("php")
        .output()
        .expect("La commande a échoué");

    output.status.success()
}

fn is_php_installed_windows() -> bool {
    let output = Command::new("where")
        .arg("php")
        .output()
        .expect("La commande a échoué");

    output.status.success()
}
