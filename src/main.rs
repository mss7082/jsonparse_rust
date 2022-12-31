use serde::{de, Deserialize, Deserializer, Serialize};
use std::str::FromStr;

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
    #[serde(deserialize_with = "de_from_str")]
    lat: f64,
    #[serde(deserialize_with = "de_from_str")]
    lng: f64,
}

fn de_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s = <&str>::deserialize(deserializer)?;
    f64::from_str(&s).map_err(de::Error::custom)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let response: Vec<User> = reqwest::Client::new()
        .get("https://jsonplaceholder.typicode.com/users")
        .send()
        .await?
        .json()
        .await?;

    println!("{:#?}", response);
    // let result: Vec<_> = response.iter().map(|user| &user.username).collect();
    // println!("{:?}", result);

    Ok(())
}
