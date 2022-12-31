use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    username: String,
    email: String,
    address: Address,
    phone: String,
    website: String,
    company: Company,
}

#[derive(Debug, Serialize, Deserialize)]
struct Company {
    name: String,
    #[serde(rename = "catchPhrase")]
    catch_phrase: String,
    bs: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Address {
    street: String,
    suite: String,
    city: String,
    zipcode: String,
    geo: Geo,
}

#[derive(Debug, Serialize, Deserialize)]
struct Geo {
    lat: String,
    lng: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let response: Vec<User> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/users")
        .send()
        .await?
        .json()
        .await?;

    let result: Vec<_> = response.iter().map(|user| &user.username).collect();
    println!("{:?}", result);

    Ok(())
}
