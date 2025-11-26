use std::path::Path;

use anyhow::Ok;
use serde::{Deserialize, Serialize};
use tokio::fs::{read, write};

const DISCORD_STABLE_BRANCH: &str = "https://discord.com/api/updates/stable?platform=linux";
const DISCORD_PATH: &str = "/opt/discord/";

#[derive(Serialize, Deserialize)]
struct DiscordUpdates {
    #[serde(rename = "name")]
    version: String,
    pub_date: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let file_path = {
        let path = Path::new(DISCORD_PATH);
        path.join("resources/build_info.json")
    };

    let discord_resources = read_resources(&file_path).await?;
    println!("Discord version is {}", discord_resources.version);

    println!("Fetching last discord version...");
    let discord_updates = fetch_updates().await?;
    println!("Last discord version is {}", discord_updates.version);

    if discord_resources.version != discord_updates.version {
        println!("Versions are different, rewrite");
        rewrite(file_path, discord_updates.version).await?;
        println!("Done, now run discord!");

        return Ok(());
    }

    println!("Versions are the same, do nothing");

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct DiscordResources {
    #[serde(rename = "releaseChannel")]
    release_channel: String,
    version: String,
}

async fn read_resources(path: impl AsRef<Path>) -> anyhow::Result<DiscordResources> {
    let build_info_raw = read(path).await?;
    Ok(serde_json::from_slice(&build_info_raw)?)
}

async fn fetch_updates() -> anyhow::Result<DiscordUpdates> {
    Ok(reqwest::get(DISCORD_STABLE_BRANCH)
        .await?
        .json::<DiscordUpdates>()
        .await?)
}

async fn rewrite(path: impl AsRef<Path>, new_version: String) -> anyhow::Result<()> {
    write(
        path,
        serde_json::to_vec(&DiscordResources {
            release_channel: String::from("stable"),
            version: new_version,
        })?,
    )
    .await?;
    Ok(())
}
