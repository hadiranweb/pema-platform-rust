use axum::{
    routing::get,
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
struct Expense {
    id: Uuid,
    date: String, // For simplicity, using String. In a real app, use a proper date type.
    category: String,
    amount: f64,
    description: String,
}

// In-memory database for simplicity
type Db = Arc<Mutex<Vec<Expense>>>;

#[tokio::main]
async fn main() {
    let db = Db::default();

    // بسیار مهم: این تنظیمات CORS برای محیط توسعه مناسب است.
    // در محیط پروداکشن باید آن را به دامنه‌ی مشخصی محدود کنید.
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/expenses", get(get_expenses).post(create_expense))
        .with_state(db)
        .layer(cors);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Backend server listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// Handler for GET /expenses
async fn get_expenses(db: axum::extract::State<Db>) -> impl IntoResponse {
    let expenses = db.lock().unwrap().clone();
    (StatusCode::OK, Json(expenses))
}

#[derive(Deserialize)]
struct CreateExpense {
    date: String,
    category: String,
    amount: f64,
    description: String,
}

// Handler for POST /expenses
async fn create_expense(
    db: axum::extract::State<Db>,
    Json(payload): Json<CreateExpense>,
) -> impl IntoResponse {
    let mut db_lock = db.lock().unwrap();

    let new_expense = Expense {
        id: Uuid::new_v4(),
        date: payload.date,
        category: payload.category,
        amount: payload.amount,
        description: payload.description,
    };

    db_lock.push(new_expense.clone());

    (StatusCode::CREATED, Json(new_expense))
}

