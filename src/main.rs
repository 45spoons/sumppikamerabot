use std::fs;

mod telegram {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct User {
        id: u32,
        is_bot: bool,
        first_name: String,
        last_name: Option<String>,
        username: Option<String>,
        language_code: Option<String>,
        is_premium: Option<bool>,
        added_to_attachment_menu: Option<bool>,
        can_join_groups: Option<bool>,
        can_read_all_group_messages: Option<bool>,
        supports_guest_queries: Option<bool>,
        supports_inline_queries: Option<bool>,
        can_connect_to_business: Option<bool>,
        has_main_web_app: Option<bool>,
        has_topics_enabled: Option<bool>,
        allows_users_to_create_topics: Option<bool>,
        can_manage_bots: Option<bool>,
        supports_join_request_queries: Option<bool>,
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tgapikey_bytes = fs::read("./apikey.txt").expect("apikey should be in a readable file called apikey.txt");
    let tgapikey = String::from_utf8(tgapikey_bytes).expect("tgapikey should be valid utf8");

    let _ = get_updates(tgapikey);
    Ok(())
}

fn get_updates(apikey: String) -> Result<(), Box<dyn std::error::Error>> {
    let api_result = reqwest::blocking::get(format!("https://api.telegram.org/bot{apikey}/getUpdates"));
    match api_result {
        Ok(api_response) => {
            println!("api_result = {api_response:?}");
            let body_result = api_response.text();
            match body_result {
                Ok(body) => {
                    println!("yeah dog! we got body: {body}")
                },
                Err(message) => println!("ehh... body parse no worky: {message}"),
            }
        },
        Err(message) => println!("ehh... api no worky: {message}"),
    }
    Ok(())
}