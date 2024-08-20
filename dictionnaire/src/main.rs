use serde::Deserialize;
const API_ENDPOINT: &str = "https://api.dictionaryapi.dev/api/v2/entries/en/";


#[derive(Deserialize, Debug)]
struct APIResponse {
    meanings: Vec<Meaning>
}

#[derive(Deserialize, Debug)]
struct Meaning {
    definitions: Vec<Definition>
}

#[derive(Deserialize, Debug)]
struct Definition {
    definition: String
}

#[tokio::main]

async fn main() {
    let word = std::env::args().nth(1); // Permet de reprendre l'argument indiqué dans la commande de lancement du programme 
    if let Some(word) = word { 
        println!("{:?}", get_definition(word).await);
    }
}

// Construire l'URL à partir de l'endpoint de l'API
fn build_full_endpoint(word: String) -> String {
    format!("{}{}", API_ENDPOINT, word)
}

async fn get_definition(word: String) -> APIResponse {
    let endpoint_url = build_full_endpoint(word);
    let response = reqwest::get(endpoint_url).await.expect("could not fetch data");
    let api_response: Vec<APIResponse> = response.json().await.expect("Could not read data");
    api_response.into_iter().nth(0).expect("Could not find the first element")
}