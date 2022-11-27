use medman::scan::scan;
use medman::markdown::write2md;
use medman::search::search;
use std::io::{Write, Read};
use medman::musicfile::MusicFile;
use std::path::Path;
 
   


fn main() {
    println!("que voulez-vous faire?\n Search ou scan ?");
  
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Can't read input");
  
  
 
    
    
    loop{
    if input.trim() == "scan" {
        println!("quel repertoire voulez vous scanner ?");
        input.clear();
        std::io::stdin()
        .read_line(&mut input)
        .expect("Can't read path");
        let input_str = input.trim();
        let path = Path::new(&input_str);
        let music_files = scan(path);

        for music_file in &music_files {
            println!("{:#?}", music_file);
        }

        println!("voulez-vous sauvegarder ce resultat dans un markdown, un json ou auc?");
        input.clear();
        std::io::stdin()
        .read_line(&mut input)
        .expect("Error reading command");

        match input.trim() {
            "markdown" => write2md(music_files),
            "json" => {
                let mut file = std::fs::File::create("data.json").expect("create failed");
                let save = serde_json::to_string(&music_files).unwrap();
                file.write_all(save.as_bytes()).expect("Write failed");
            },
            _ => println!("d'accord, rien ne sera sauvegargé"),
        };
    }
    
    if input.trim() == "search"{
        println!("voulez-vous chercher  des musiques dans un ficher json ou dans un dossier à l'aide de metadonnées? (Tapez dossier ou json)");
        input.clear();
        std::io::stdin()
        .read_line(&mut input)
        .expect("erreur");
        let input_str = input.trim();

        match input_str {
            "json" => {
                let mut file = std::fs::File::open("recherche.json").expect("Couldn't open file");
                let mut contents = String::new();
                file.read_to_string(&mut contents).unwrap();

                
            let mut    music_files: Vec<MusicFile>  = serde_json::from_str(&contents).expect("impossible de  deserialiser le fichier");

                println!("Suivez l'exemple: artist=Michael Jackon+Year=2000)");
                input.clear();
                std::io::stdin()
                .read_line(&mut input)
                .expect("Couldn't read your input");
                let input_str = input.trim().to_string();

                music_files = search(music_files, input_str);
                write2md(music_files);
            },
            "dossier" => {
                println!("Quel dossier voulez-vous scanner?");
                input.clear();
                std::io::stdin()
                .read_line(&mut input)
                .expect("l'input ne peut pas etre lu");
                let input_str = input.trim();
                let mut music_files = scan(Path::new(input_str));

                println!("Suivez l'exemple: artist=Beyonce+album=2000)");
                input.clear();
                std::io::stdin()
                .read_line(&mut input)
                .expect("Couldn't read your input");
                let input_str = input.trim().to_string();

                music_files = search(music_files, input_str);
                write2md(music_files);
            }
            _ => panic!("Enter read or scan!")
        }
    }
}
    
}