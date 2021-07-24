use serde_derive::{Serialize, Deserialize};
use reqwest::header;

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    pub browser_download_url: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GitResponse {
    pub tag_name: String,
    pub prerelease: bool,
    pub draft: bool,
    pub assets: Vec<Asset>
}

pub fn get_git_release_by_version(repo: &str, version: &str) -> Option<GitResponse> {
    let url = format!("https://api.github.com/repos/{}/releases/{}", repo, version);

    let mut headers = header::HeaderMap::new();
    headers.append(header::USER_AGENT, header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36 Edg/92.0.902.55"));

    let data = reqwest::blocking::Client::builder()
                                            .default_headers(headers)
                                            .build()
                                            .ok()?
                                            .get(url)
                                            .send()
                                            .ok()?
                                            .text()
                                            .ok()?;

    let git_response : GitResponse = serde_json::from_str(&data)
                                            .ok()?;

    Some(git_response)
}

pub fn get_git_latest_release(repo: &str) -> Option<GitResponse> {
    Some(get_git_release_by_version(repo, "latest")?)
}
