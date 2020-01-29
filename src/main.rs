extern crate reqwest;

use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let client : reqwest::Client = reqwest::Client::new();

  authenticate(&client)
    .await?;

  // println!("{:#?}", resp);

  Ok(())
}

fn get_thing<T>(map : &HashMap<String, T>) -> &T {
    return &map["origin"];
}

fn print_thing<T: std::fmt::Display>(map : HashMap<String, T>) -> () {
    println!("{}", map["origin"]);
}

// Authenticate with
// https://auth.tdameritrade.com/oauth?client_id=UTWO0CNUAERNK4OQDM5QC3LED4IAJBMU@AMER.OAUTHAP&response_type=code&redirect_uri=http://localhost:5432
//
async fn authenticate(client : &reqwest::Client) -> Result<(), Box<dyn std::error::Error>> {
                         // "https://api.tdameritrade.com/v1/oauth2/token"
  let resp = client.post("https://api.tdameritrade.com/v1/oauth2/token")
    .body(
      concat!(
        
        "'grant_type': 'authorization_code' ",
        "'code': 'U3M0+y2dVo+lylzRkhUZPJWHdI+uj0d5A4ccOaMz3S6DmnlsUqjZiUBaA7O7CS4GEsBj2cDmrQvXh0VXypHRSiOpO/OOodyviUW2eUr9hOJ+PHw1x6oF5tocku0zQCmfmTNdmU/A21IueSgWq4PNCfp64dXxY8/XaFgZZiBqBAh31Sev1pUTYDxZFXICM04dveUy76z9dWE4kkOZzW6gLjCy+6T7f2nskFKoboTZO0eYeeaxGzmObz4qdG6aAHMQLs3jtFk9M4dLOrALIzrgYea/DFgDPBauoSg3AAQExi2kYs+BIVQn/xY7O+5f1gWC7DOZrKkFA4ZR23EFTOR5lsDlWflR5IhpL7iR4Nf8kI9XysW9oadHgxCW0SVocbs+BaUfBIN1Y7jwfmukctadbQFx1cAIeCNup/sLDLRv4OmZZjDLF42qLa4DMJD100MQuG4LYrgoVi/JHHvlFLz9KB0YwHhouSdq9tGnzY+OmrIumg8aeHMBjWs5YIuEFf2SVbs5JewZ4GCauxXO6IV7yi1V+IqZwW0QSUabUTZ6YwNP3lSBQUZZU4ak/EcbK67c1DeehGLH2gOcagRvM6xrDCqiQQ9ALmn9HzSsD6q6ryCrrL/k9j9mdklgkGh7TBUPD2GRXR71iWpL2vwPiEpSZi7KAesROImP67h/HY/8u9T2+U/TcBikzYl5xOPhC0TrFHtu2zf0fcQuyfvHldaUJQIo9zivnq41Pef3X+lfTdNt9X32Z8uB0gG/ShPB2aehGSWXzVtAguiy1l/suKs7KIzNq/Wp3PqvWfy0PQJH91Flm4ZsB/+ZLFwoTYDtZqpFmAByuH34C0Ox6+Tqk7NKxHrEAya+4fSGzhwMjKQnQGM3ZNMfb40H7Pcn2iC0P+08TGVPXwLlUYQ=212FD3x19z9sWBHDJACbC00B75E' ",
        "'client_id': 'UTWO0CNUAERNK4OQDM5QC3LED4IAJBMU' ",
        "'redirect_url': 'http://localhost/'")
      )
    .send()
    .await?
    ;

  println!("Response: {:#?}", resp);

  Ok(())
}
