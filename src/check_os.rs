use std::process::{Command, ExitStatus};
use std::env;
use std::fs;

fn install_php() -> Result<(), String> {
    let os = env::consts::OS;
    println!("Système d'exploitation: {}", os);

    match os {
        "linux" => {
            if let Err(err) = try_package_manager("apt-get") {
                println!("Erreur lors de l'installation de PHP via apt-get: {}", err);
                if let Err(err) = try_package_manager("yum") {
                    println!("Erreur lors de l'installation de PHP via yum: {}", err);
                    if let Err(err) = download_php_from_site() {
                        println!("Erreur lors de l'installation de PHP depuis le site de PHP: {}", err);
                        return Err(err);
                    }
                }
            }
        }
        "macos" => {
            if let Err(err) = try_package_manager("brew") {
                println!("Erreur lors de l'installation de PHP via Homebrew: {}", err);
                if let Err(err) = download_php_from_site() {
                    println!("Erreur lors de l'installation de PHP depuis le site de PHP: {}", err);
                    return Err(err);
                }
            }
        }
        "windows" => {
            if let Err(err) = try_package_manager("choco") {
                println!("Erreur lors de l'installation de PHP via Chocolatey: {}", err);
                if let Err(err) = download_php_from_site() {
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

fn download_php_from_site() -> Result<(), String> {
    println!("Tentative de téléchargement de PHP depuis le site de PHP...");
    // Téléchargement de PHP depuis le site de PHP
    // (Ceci est un exemple, vous devrez ajuster cette partie en fonction du site de téléchargement réel)
    let php_download_url = "https://www.php.net/distributions/php-8.1.0.tar.gz";
    let output_file = "php.tar.gz";
    let output = Command::new("curl")
        .arg("-o")
        .arg(output_file)
        .arg(php_download_url)
        .output()
        .map_err(|e| format!("Erreur lors de l'exécution de la commande: {}", e))?;
    
    if !output.status.success() {
        return Err(format!("Le téléchargement de PHP a échoué: {}", String::from_utf8_lossy(&output.stderr)));
    }

    println!("PHP téléchargé avec succès depuis le site de PHP");

    // Vous pouvez extraire l'archive et installer PHP depuis ici

    // Nettoyage du fichier téléchargé
    fs::remove_file(output_file).map_err(|e| format!("Impossible de supprimer le fichier téléchargé: {}", e))?;

    Ok(())
}