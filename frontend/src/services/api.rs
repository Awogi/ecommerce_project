use gloo_net::http::{Request, Response};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use crate::models::{
    User, School, Grade, UniformCategory, Uniform, CartItem, Order, 
    LoginRequest, RegisterRequest, ApiResponse
};

// During development the backend runs on http://127.0.0.1:8080 by default (see backend/.env)
// You can change this or wire it to an env/config value as needed.
const API_BASE: &str = "http://127.0.0.1:8080";

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
    let response = Request::post(&format!("{}/users/login", API_BASE))
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
    let response = Request::post(&format!("{}/users/register", API_BASE))
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
        let response = Request::get(&format!("{}/schools", API_BASE))
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
        let response = Request::get(&format!("{}/grades", API_BASE))
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
        let response = Request::get(&format!("{}/uniform-categories", API_BASE))
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
        let response = Request::get(&format!("{}/uniforms", API_BASE))
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
        let response = Request::get(&format!("{}/uniforms/{}", API_BASE, id))
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
        let mut req = Request::get(&format!("{}/cart/{}", API_BASE, user_id));
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

        let response = Request::post(&format!("{}/cart/{}", API_BASE, user_id))
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
        let mut req = Request::delete(&format!("{}/cart/{}/items/{}", API_BASE, user_id, uniform_id));
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
        let mut req = Request::get(&format!("{}/orders/user/{}", API_BASE, user_id));
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

        let response = Request::post(&format!("{}/orders", API_BASE))
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
