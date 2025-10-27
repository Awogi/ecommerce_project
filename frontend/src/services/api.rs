use gloo_net::http::Request;
use serde::Serialize;
use crate::models::{
    User, School, Grade, UniformCategory, Uniform, CartItem, Order, 
    LoginRequest, RegisterRequest, ApiResponse
};

// Determine API base URL at runtime using this precedence:
// 1. localStorage['api_base'] (dev override)
// 2. <meta name="api-base" content="..."> in index.html
// 3. fallback to "http://127.0.0.1:8080"
fn api_base() -> String {
    // 1) localStorage override
    if let Some(win) = web_sys::window() {
        if let Ok(Some(storage)) = win.local_storage() {
            if let Ok(Some(override_val)) = storage.get_item("api_base") {
                if !override_val.is_empty() {
                    // Normalize overrides so they always target the API scope.
                    // If user stored `http://127.0.0.1:8080` we want `http://127.0.0.1:8080/api`.
                    let mut base = override_val.trim_end_matches('/').to_string();
                    if !base.ends_with("/api") {
                        base.push_str("/api");
                    }
                    return base;
                }
            }
        }

        // 2) meta tag in index.html
        if let Some(doc) = win.document() {
            if let Ok(Some(el)) = doc.query_selector("meta[name=\\\"api-base\\\"]") {
                if let Some(content) = el.get_attribute("content") {
                    // If meta is set to a relative "/api" (production default) but we're running
                    // the frontend on a different dev port (e.g. trunk at 8082), try the local
                    // backend URL at 127.0.0.1:8080 for convenience so developers don't need to
                    // set localStorage manually.
                    if content == "/api" {
                        // Inspect current location port. If it's not 8080, prefer 127.0.0.1:8080
                        if let Some(loc) = win.location().port().ok().filter(|p| !p.is_empty()) {
                            if loc != "8080" {
                                return format!("http://127.0.0.1:8080{}", content);
                            }
                        }
                    }
                    return content;
                }
            }
        }
    }

    // 3) default fallback
    "http://127.0.0.1:8080".to_string()
}

// Return base URL for static assets. If the api_base() returns a full URL (e.g. http://127.0.0.1:8081/api)
// we convert that into the corresponding static base (http://127.0.0.1:8081/static). If api_base()
// is a relative "/api" value, we return the relative "/static" so assets are requested from the
// same origin as the page.
pub fn static_base() -> String {
    let base = api_base();
    if base.starts_with("http://") || base.starts_with("https://") {
        // strip trailing slash and the '/api' suffix if present
        let no_trail = base.trim_end_matches('/');
        if no_trail.ends_with("/api") {
            let host = no_trail.trim_end_matches("/api");
            return format!("{}/static", host);
        }
        return format!("{}/static", no_trail);
    }

    // Relative api_base (e.g. "/api") -> use relative /static
    "/static".to_string()
}

pub struct ApiService;

impl ApiService {
    // Helper: read token from localStorage and return an Authorization header value
    fn auth_header() -> Option<String> {
        use web_sys::window;
        if let Some(win) = window() {
            // local_storage() -> Result<Option<Storage>, JsValue>
            if let Ok(Some(storage)) = win.local_storage() {
                // storage.get_item -> Result<Option<String>, JsValue>
                if let Ok(Some(token)) = storage.get_item("auth_token") {
                    return Some(format!("Bearer {}", token));
                }
            }
        }
        None
    }

    // Authentication endpoints
    pub async fn login(credentials: LoginRequest) -> Result<User, String> {
    // Backend exposes user auth under /users
    let response = Request::post(&format!("{}/users/login", api_base()))
            .header("Content-Type", "application/json")
            .json(&credentials)
            .map_err(|e| format!("Failed to serialize request: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            let api_response: ApiResponse<User> = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            
            match api_response.data {
                Some(user) => Ok(user),
                None => Err(api_response.message.unwrap_or("Login failed".to_string())),
            }
        } else {
            Err(format!("Login failed with status: {}", response.status()))
        }
    }

    pub async fn register(user_data: RegisterRequest) -> Result<User, String> {
    let response = Request::post(&format!("{}/users/register", api_base()))
            .header("Content-Type", "application/json")
            .json(&user_data)
            .map_err(|e| format!("Failed to serialize request: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            let api_response: ApiResponse<User> = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            
            match api_response.data {
                Some(user) => Ok(user),
                None => Err(api_response.message.unwrap_or("Registration failed".to_string())),
            }
        } else {
            Err(format!("Registration failed with status: {}", response.status()))
        }
    }

    // Schools endpoints
    pub async fn get_schools() -> Result<Vec<School>, String> {
        let response = Request::get(&format!("{}/schools", api_base()))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            let schools: Vec<School> = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(schools)
        } else {
            Err(format!("Failed to fetch schools: {}", response.status()))
        }
    }

    // Grades endpoints
    pub async fn get_grades() -> Result<Vec<Grade>, String> {
        let response = Request::get(&format!("{}/grades", api_base()))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            let grades: Vec<Grade> = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(grades)
        } else {
            Err(format!("Failed to fetch grades: {}", response.status()))
        }
    }

    // Categories endpoints
    pub async fn get_categories() -> Result<Vec<UniformCategory>, String> {
        let response = Request::get(&format!("{}/uniform-categories", api_base()))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            let categories: Vec<UniformCategory> = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(categories)
        } else {
            Err(format!("Failed to fetch categories: {}", response.status()))
        }
    }

    // Uniforms endpoints
    pub async fn get_uniforms() -> Result<Vec<Uniform>, String> {
        let response = Request::get(&format!("{}/uniforms", api_base()))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            let uniforms: Vec<Uniform> = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(uniforms)
        } else {
            Err(format!("Failed to fetch uniforms: {}", response.status()))
        }
    }

    pub async fn get_uniform(id: i32) -> Result<Uniform, String> {
        let response = Request::get(&format!("{}/uniforms/{}", api_base(), id))
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            let uniform: Uniform = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(uniform)
        } else {
            Err(format!("Failed to fetch uniform: {}", response.status()))
        }
    }

    // Cart endpoints
    pub async fn get_cart(user_id: i32) -> Result<Vec<CartItem>, String> {
    let mut req = Request::get(&format!("{}/cart/{}", api_base(), user_id));
        if let Some(auth) = ApiService::auth_header() {
            req = req.header("Authorization", &auth);
        }
        let response = req.send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            let cart_items: Vec<CartItem> = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(cart_items)
        } else {
            Err(format!("Failed to fetch cart: {}", response.status()))
        }
    }

    pub async fn add_to_cart(user_id: i32, uniform_id: i32, quantity: i32) -> Result<CartItem, String> {
        #[derive(Serialize)]
        struct AddToCartRequest {
            uniform_id: i32,
            quantity: i32,
        }

        let request_data = AddToCartRequest { uniform_id, quantity };

        let response = Request::post(&format!("{}/cart/{}", api_base(), user_id))
            .header("Content-Type", "application/json")
            .json(&request_data)
            .map_err(|e| format!("Failed to serialize request: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            let cart_item: CartItem = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(cart_item)
        } else {
            Err(format!("Failed to add to cart: {}", response.status()))
        }
    }

    pub async fn remove_from_cart(user_id: i32, uniform_id: i32) -> Result<(), String> {
    let mut req = Request::delete(&format!("{}/cart/{}/items/{}", api_base(), user_id, uniform_id));
        if let Some(auth) = ApiService::auth_header() {
            req = req.header("Authorization", &auth);
        }
        let response = req.send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            Ok(())
        } else {
            Err(format!("Failed to remove from cart: {}", response.status()))
        }
    }

    // Orders endpoints
    pub async fn get_orders(user_id: i32) -> Result<Vec<Order>, String> {
    let mut req = Request::get(&format!("{}/orders/user/{}", api_base(), user_id));
        if let Some(auth) = ApiService::auth_header() {
            req = req.header("Authorization", &auth);
        }
        let response = req.send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            let orders: Vec<Order> = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(orders)
        } else {
            Err(format!("Failed to fetch orders: {}", response.status()))
        }
    }

    pub async fn create_order(user_id: i32) -> Result<Order, String> {
        #[derive(Serialize)]
        struct CreateOrderRequest {
            user_id: i32,
        }

        let request_data = CreateOrderRequest { user_id };

        let response = Request::post(&format!("{}/orders", api_base()))
            .header("Content-Type", "application/json")
            .json(&request_data)
            .map_err(|e| format!("Failed to serialize request: {}", e))?
            .send()
            .await
            .map_err(|e| format!("Network error: {}", e))?;

        if response.ok() {
            let order: Order = response
                .json()
                .await
                .map_err(|e| format!("Failed to parse response: {}", e))?;
            Ok(order)
        } else {
            Err(format!("Failed to create order: {}", response.status()))
        }
    }
}
