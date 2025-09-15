use crate::{
    cookie_crypto::{decrypt_cookie_value, encrypt_cookie_value},
    database::Database,
};
use axum::{
    Router,
    extract::{Query, State},
    http::{HeaderMap, StatusCode, header::SET_COOKIE},
    response::{IntoResponse, Redirect},
    routing::get,
};
use cookie::{Cookie, SameSite};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, TokenResponse,
    TokenUrl, basic::BasicClient, reqwest::async_http_client,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GitHubUser {
    pub id: u64,
    pub login: String,
    pub avatar_url: String,
}

#[derive(Clone)]
pub struct AuthState {
    pub github_client: BasicClient,
    pub database: Database,
}

impl AuthState {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let github_client_id = std::env::var("GITHUB_CLIENT_ID")
            .expect("GITHUB_CLIENT_ID environment variable not set");
        let github_client_secret = std::env::var("GITHUB_CLIENT_SECRET")
            .expect("GITHUB_CLIENT_SECRET environment variable not set");

        let redirect_url = format!("{}/auth/github/callback", crate::config::middleware::host());

        let github_client = BasicClient::new(
            ClientId::new(github_client_id),
            Some(ClientSecret::new(github_client_secret)),
            AuthUrl::new("https://github.com/login/oauth/authorize".to_string())?,
            Some(TokenUrl::new(
                "https://github.com/login/oauth/access_token".to_string(),
            )?),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url)?);

        // Initialize database
        let database_url =
            std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
        let database = Database::new(&database_url)
            .await
            .expect("Failed to initialize database");

        Ok(Self {
            github_client,
            database,
        })
    }
}

pub fn auth_routes() -> Router<AuthState> {
    Router::new()
        .route("/auth/github/login", get(login))
        .route("/auth/github/callback", get(callback))
        .route("/auth/github/logout", get(logout))
}

#[derive(Deserialize)]
pub struct CallbackQuery {
    code: String,
    state: String,
}

async fn login(State(auth_state): State<AuthState>) -> impl IntoResponse {
    let (auth_url, csrf_token) = auth_state
        .github_client
        .authorize_url(CsrfToken::new_random)
        .url();

    // Store CSRF token in cookie
    let csrf_cookie = Cookie::build("csrf_token", csrf_token.secret().clone())
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, csrf_cookie.to_string().parse().unwrap());

    (headers, Redirect::to(auth_url.as_ref())).into_response()
}

async fn callback(
    State(auth_state): State<AuthState>,
    Query(query): Query<CallbackQuery>,
    headers: HeaderMap,
) -> impl IntoResponse {
    // Extract and verify CSRF token from cookies
    let cookies = headers
        .get("cookie")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");

    let mut csrf_token: Option<&str> = None;
    for cookie_str in cookies.split(';') {
        let cookie_str = cookie_str.trim();
        if cookie_str.starts_with("csrf_token=") {
            csrf_token = cookie_str.strip_prefix("csrf_token=");
            break;
        }
    }

    if csrf_token != Some(&query.state) {
        tracing::error!(
            "CSRF token mismatch! Found: {:?}, Expected: {}",
            csrf_token,
            query.state
        );
        return StatusCode::BAD_REQUEST.into_response();
    }

    // Exchange the code for an access token
    let token_result = auth_state
        .github_client
        .exchange_code(AuthorizationCode::new(query.code))
        .request_async(async_http_client)
        .await;

    let token = match token_result {
        Ok(token) => token,
        Err(e) => {
            tracing::error!("Failed to exchange code for token: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    // Fetch user info from GitHub
    let client = reqwest::Client::new();
    let user_response = client
        .get("https://api.github.com/user")
        .bearer_auth(token.access_token().secret())
        .header("User-Agent", "mevlog-rs/backend")
        .send()
        .await;

    let user_response = match user_response {
        Ok(response) => response,
        Err(e) => {
            tracing::error!("Failed to fetch user info from GitHub: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let response_text = match user_response.text().await {
        Ok(text) => text,
        Err(e) => {
            tracing::error!("Failed to read GitHub response as text: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    let github_user: GitHubUser = match serde_json::from_str(&response_text) {
        Ok(user) => user,
        Err(e) => {
            tracing::error!("Failed to parse GitHub user response: {:?}", e);
            tracing::error!("Raw response was: {}", response_text);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    // Create or get user from database
    let (_user, is_new_user) = match auth_state
        .database
        .get_or_create_user(&github_user.login)
        .await
    {
        Ok(result) => result,
        Err(e) => {
            tracing::error!("Failed to create/update user in database: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    // Send Slack notification for new users
    if is_new_user
        && let Err(e) = crate::slack::send_new_user_notification(&github_user.login).await {
            tracing::error!(
                "Failed to send Slack notification for new user {}: {:?}",
                github_user.login,
                e
            );
            // Don't fail the authentication if Slack notification fails
        }

    // Store user in encrypted cookie
    let user_json = serde_json::to_string(&github_user).unwrap();
    tracing::debug!("User JSON to encrypt: {}", user_json);

    let encrypted_user_data = match encrypt_cookie_value(&user_json) {
        Ok(encrypted) => encrypted,
        Err(e) => {
            tracing::error!("Failed to encrypt cookie data: {:?}", e);
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    };

    tracing::debug!("Encrypted user data for cookie: {}", encrypted_user_data);
    let user_cookie = Cookie::build("github_user", encrypted_user_data)
        .path("/")
        .same_site(SameSite::Lax)
        .http_only(true)
        .finish();

    // Clear CSRF token
    let clear_csrf = Cookie::build("csrf_token", "")
        .path("/")
        .max_age(cookie::time::Duration::seconds(0))
        .finish();

    let mut response_headers = HeaderMap::new();
    response_headers.append(SET_COOKIE, user_cookie.to_string().parse().unwrap());
    response_headers.append(SET_COOKIE, clear_csrf.to_string().parse().unwrap());

    (response_headers, Redirect::to("/")).into_response()
}

async fn logout() -> impl IntoResponse {
    let clear_cookie = Cookie::build("github_user", "")
        .path("/")
        .max_age(cookie::time::Duration::seconds(0))
        .finish();

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, clear_cookie.to_string().parse().unwrap());

    (headers, Redirect::to("/")).into_response()
}

pub fn get_user_from_cookies(headers: &HeaderMap) -> Option<GitHubUser> {
    let cookies = headers
        .get("cookie")
        .and_then(|value| value.to_str().ok())?;

    tracing::debug!("Raw cookies: {}", cookies);

    for cookie_str in cookies.split(';') {
        let cookie_str = cookie_str.trim();
        if cookie_str.starts_with("github_user=") {
            let encrypted_data = cookie_str.strip_prefix("github_user=")?;
            tracing::debug!("Encrypted cookie data: {}", encrypted_data);

            // Decrypt the cookie value
            let decrypted_json = match decrypt_cookie_value(encrypted_data) {
                Ok(decrypted) => {
                    tracing::debug!("Successfully decrypted cookie");
                    decrypted
                }
                Err(e) => {
                    tracing::error!("Failed to decrypt cookie: {:?}", e);
                    return None;
                }
            };

            tracing::debug!("Decrypted user JSON: {}", decrypted_json);

            match serde_json::from_str(&decrypted_json) {
                Ok(user) => {
                    tracing::debug!("Successfully parsed user: {:?}", user);
                    return Some(user);
                }
                Err(e) => {
                    tracing::error!("Failed to parse user JSON: {:?}", e);
                    return None;
                }
            }
        }
    }
    tracing::debug!("No github_user cookie found");
    None
}
