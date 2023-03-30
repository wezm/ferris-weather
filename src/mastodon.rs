const INSTANCE: &str = "https://mastodon.decentralised.social/";
const ACCESS_TOKEN: &str = "YArk1Sx50Gu8BJPN1jyUAw2afz4B3JPo1DNdk0PkCV8"; // Only has read perms

enum MastodonError {
    /// The app response is missing the client ID or secret
    MissingClientIdOrSecret,
}

// /// Perform the OAuth flow to obtain credentials
// pub async fn auth(instance: Url, archive_path: &str) -> Result<(), MastodonError> {
//
//     let client = Client::builder()
//         .user_agent(format!("Toot Classic {}", env!("CARGO_PKG_VERSION")))
//         .build()?;
//
//     // Register application to obtain client id and secret
//     let url = instance.join("/api/v1/apps")?;
//     let resp = client
//         .post(url)
//         .form(&[
//             ("client_name", "MArchive"),
//             ("redirect_uris", "urn:ietf:wg:oauth:2.0:oob"),
//             ("scopes", SCOPES),
//             // ("website", instance.as_str()) // There is no website for this application
//         ])
//         .send()
//         .await?; // TODO: Add context info to error
//     let app: Application = json_or_error(resp).await?;
//
//     let client_id = app
//         .client_id
//         .ok_or_else(|| eyre!("app response is missing client id"))?;
//     let client_secret = app
//         .client_secret
//         .ok_or_else(|| eyre!("app response is missing client secret"))?;
//     debug!("Got application: {}, ID: {}", app.name, client_id);
//
//     // Show the approval page
//     let mut url = instance.join("/oauth/authorize")?;
//     url.query_pairs_mut()
//         .append_pair("response_type", "code")
//         .append_pair("client_id", &client_id)
//         .append_pair("redirect_uri", "urn:ietf:wg:oauth:2.0:oob")
//         .append_pair("scope", SCOPES);
//     println!(
//         "\nOpen this page in your browser and paste the code:\n{}",
//         url
//     );
//     print!("\nCode: ");
//     io::stdout().flush()?;
//     let mut code = String::new();
//     io::stdin().read_line(&mut code)?;
//
//     let code = code.trim();
//     if code.is_empty() {
//         return Err(eyre!("code is required"));
//     }
//
//     // Use client id, secret, and code to get a token
//     let url = instance.join("/oauth/token")?;
//     let resp = client
//         .post(url)
//         .form(&[
//             ("grant_type", "authorization_code"),
//             ("code", code),
//             ("client_id", client_id.as_str()),
//             ("client_secret", &client_secret),
//             ("redirect_uri", "urn:ietf:wg:oauth:2.0:oob"),
//             ("scope", SCOPES),
//         ])
//         .send()
//         .await?; // TODO: Add context info to error
//     let token_resp: TokenResponse = json_or_error(resp).await?;
//     debug!("Got token");
//
//     // Save the token (and client credentials)
//     let config = Config::new(
//         client_id,
//         client_secret,
//         instance.to_string(),
//         token_resp.access_token,
//         archive_path.to_string(),
//     );
//     Config::create(None, config)?; // TODO: Support custom config path
//     debug!("Saved config");
//
//     Ok(())
// }

fn post(msg: &str) -> Result<(), ()> {
    // let instance = config.instance_url()?;
    // let mut url = instance.join("/api/v1/timelines/home")?;
    // let bearer_token = format!("Bearer {}", config.access_token);
    // loop {
    //     url.query_pairs_mut().clear().append_pair("limit", "40");
    //     if let Some(ref last_seen_id) = config.last_seen_id {
    //         info!("Fetching statuses since id: {}", last_seen_id);
    //         url.query_pairs_mut().append_pair("min_id", last_seen_id);
    //     } else {
    //         info!("Fetching new statuses");
    //     }
    //
    //     // Fetch home timeline since the last id we have
    //     let resp = client
    //         .get(url.clone())
    //         .header(AUTHORIZATION, &bearer_token)
    //         .send()
    //         .await?;
    //     let statuses: Vec<Status> = json_or_error(resp).await?;
    //     info!("Read {} statuses", statuses.len());
    //
    //     if statuses.is_empty() {
    //         info!("Finished reading statuses");
    //         break;
    //     }
    todo!()
}
