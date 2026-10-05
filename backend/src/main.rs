use actix_cors::Cors;
use actix_web::{delete, get, http::header, middleware::Logger, post, put, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

pub struct DbState {
    pub db: Mutex<Connection>,
}

// --- Estruturas de Dados ---
#[derive(Serialize, Deserialize, Clone)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub color: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category_id: String,
    pub tags: String,
    pub link: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BlogPost {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub category_id: String,
    pub read_time: String,
    pub date: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SiteInfo {
    pub bio: String,
    pub location: String,
    pub email: String,
    pub github: String,
    pub linkedin: String,
    pub cv_summary: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub status: String,
}

// --- Inicialização do Banco SQLite ---
fn init_db() -> Connection {
    let conn = Connection::open("portfolio.db").expect("Falha ao abrir SQLite");

    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS categories (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            color TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            category_id TEXT NOT NULL,
            tags TEXT NOT NULL,
            link TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS blog_posts (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            summary TEXT NOT NULL,
            category_id TEXT NOT NULL,
            read_time TEXT NOT NULL,
            date TEXT NOT NULL
        );

        CREATE TABLE IF NOT EXISTS site_info (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            bio TEXT NOT NULL,
            location TEXT NOT NULL,
            email TEXT NOT NULL,
            github TEXT NOT NULL,
            linkedin TEXT NOT NULL,
            cv_summary TEXT NOT NULL
        );

        INSERT OR IGNORE INTO site_info (id, bio, location, email, github, linkedin, cv_summary)
        VALUES (1, 
            'Engenheiro de Software focado em sistemas de alta performance com Rust e Vue.js.', 
            'Brasil', 
            'leandro.dev@exemplo.com', 
            'github.com/leandro-dev', 
            'linkedin.com/in/leandro-dev',
            'Experiência em Engenharia de Software, Cloud, Banco de Dados, IA e GameDev.'
        );

        INSERT OR IGNORE INTO categories (id, name, color) VALUES ('web', 'Web Dev', '#3B82F6');
        INSERT OR IGNORE INTO categories (id, name, color) VALUES ('jogo', 'Jogos / Engine', '#EAB308');
        "
    ).expect("Falha ao criar tabelas no SQLite");

    conn
}

// Helper Auth
fn check_auth(req: &HttpRequest) -> bool {
    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(str_val) = auth_header.to_str() {
            return str_val == "Bearer token-secreto-admin-leandro";
        }
    }
    false
}

#[post("/api/v1/auth/login")]
async fn login(payload: web::Json<LoginPayload>) -> impl Responder {
    if payload.username == "admin" && payload.password == "admin123" {
        HttpResponse::Ok().json(LoginResponse {
            token: "token-secreto-admin-leandro".to_string(),
            status: "success".to_string(),
        })
    } else {
        HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Credenciais inválidas" }))
    }
}

#[get("/api/v1/site-info")]
async fn get_site_info(data: web::Data<DbState>) -> impl Responder {
    let conn = data.db.lock().unwrap();
    let mut stmt = conn.prepare("SELECT bio, location, email, github, linkedin, cv_summary FROM site_info WHERE id = 1").unwrap();
    
    let info = stmt.query_row([], |row| {
        Ok(SiteInfo {
            bio: row.get(0)?,
            location: row.get(1)?,
            email: row.get(2)?,
            github: row.get(3)?,
            linkedin: row.get(4)?,
            cv_summary: row.get(5)?,
        })
    });

    match info {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/api/v1/site-info")]
async fn update_site_info(req: HttpRequest, data: web::Data<DbState>, item: web::Json<SiteInfo>) -> impl Responder {
    if !check_auth(&req) {
        return HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Não autorizado" }));
    }

    let conn = data.db.lock().unwrap();
    conn.execute(
        "UPDATE site_info SET bio = ?1, location = ?2, email = ?3, github = ?4, linkedin = ?5, cv_summary = ?6 WHERE id = 1",
        params![item.bio, item.location, item.email, item.github, item.linkedin, item.cv_summary],
    ).unwrap();

    HttpResponse::Ok().json(serde_json::json!({ "status": "updated" }))
}

#[get("/api/v1/categories")]
async fn get_categories(data: web::Data<DbState>) -> impl Responder {
    let conn = data.db.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, name, color FROM categories").unwrap();
    let rows = stmt.query_map([], |row| {
        Ok(Category { id: row.get(0)?, name: row.get(1)?, color: row.get(2)? })
    }).unwrap();

    let mut list = Vec::new();
    for item in rows {
        if let Ok(c) = item { list.push(c); }
    }
    HttpResponse::Ok().json(list)
}

#[post("/api/v1/categories")]
async fn create_category(req: HttpRequest, data: web::Data<DbState>, item: web::Json<Category>) -> impl Responder {
    if !check_auth(&req) {
        return HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Não autorizado" }));
    }
    let conn = data.db.lock().unwrap();
    let mut new_cat = item.into_inner();
    if new_cat.id.is_empty() {
        new_cat.id = new_cat.name.to_lowercase().replace(' ', "-");
    }
    conn.execute(
        "INSERT INTO categories (id, name, color) VALUES (?1, ?2, ?3)",
        params![new_cat.id, new_cat.name, new_cat.color],
    ).ok();

    HttpResponse::Created().json(new_cat)
}

#[delete("/api/v1/categories/{id}")]
async fn delete_category(req: HttpRequest, data: web::Data<DbState>, id: web::Path<String>) -> impl Responder {
    if !check_auth(&req) {
        return HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Não autorizado" }));
    }
    let conn = data.db.lock().unwrap();
    let target_id = id.into_inner();
    conn.execute("DELETE FROM categories WHERE id = ?1", params![target_id]).ok();
    HttpResponse::Ok().json(serde_json::json!({ "status": "deleted", "id": target_id }))
}

#[get("/api/v1/projects")]
async fn get_projects(data: web::Data<DbState>) -> impl Responder {
    let conn = data.db.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, title, description, category_id, tags, link FROM projects").unwrap();
    let rows = stmt.query_map([], |row| {
        Ok(Project {
            id: row.get(0)?,
            title: row.get(1)?,
            description: row.get(2)?,
            category_id: row.get(3)?,
            tags: row.get(4)?,
            link: row.get(5)?,
        })
    }).unwrap();

    let mut list = Vec::new();
    for item in rows {
        if let Ok(p) = item { list.push(p); }
    }
    HttpResponse::Ok().json(list)
}

#[post("/api/v1/projects")]
async fn create_project(req: HttpRequest, data: web::Data<DbState>, item: web::Json<Project>) -> impl Responder {
    if !check_auth(&req) {
        return HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Não autorizado" }));
    }
    let conn = data.db.lock().unwrap();
    let mut p = item.into_inner();
    if p.id.is_empty() { p.id = uuid::Uuid::new_v4().to_string(); }

    conn.execute(
        "INSERT INTO projects (id, title, description, category_id, tags, link) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![p.id, p.title, p.description, p.category_id, p.tags, p.link],
    ).ok();

    HttpResponse::Created().json(p)
}

#[delete("/api/v1/projects/{id}")]
async fn delete_project(req: HttpRequest, data: web::Data<DbState>, id: web::Path<String>) -> impl Responder {
    if !check_auth(&req) {
        return HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Não autorizado" }));
    }
    let conn = data.db.lock().unwrap();
    let target_id = id.into_inner();
    conn.execute("DELETE FROM projects WHERE id = ?1", params![target_id]).ok();
    HttpResponse::Ok().json(serde_json::json!({ "status": "deleted", "id": target_id }))
}

#[get("/api/v1/blog")]
async fn get_blog_posts(data: web::Data<DbState>) -> impl Responder {
    let conn = data.db.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, title, summary, category_id, read_time, date FROM blog_posts").unwrap();
    let rows = stmt.query_map([], |row| {
        Ok(BlogPost {
            id: row.get(0)?,
            title: row.get(1)?,
            summary: row.get(2)?,
            category_id: row.get(3)?,
            read_time: row.get(4)?,
            date: row.get(5)?,
        })
    }).unwrap();

    let mut list = Vec::new();
    for item in rows {
        if let Ok(b) = item { list.push(b); }
    }
    HttpResponse::Ok().json(list)
}

#[post("/api/v1/blog")]
async fn create_blog_post(req: HttpRequest, data: web::Data<DbState>, item: web::Json<BlogPost>) -> impl Responder {
    if !check_auth(&req) {
        return HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Não autorizado" }));
    }
    let conn = data.db.lock().unwrap();
    let mut b = item.into_inner();
    if b.id.is_empty() { b.id = uuid::Uuid::new_v4().to_string(); }

    conn.execute(
        "INSERT INTO blog_posts (id, title, summary, category_id, read_time, date) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        params![b.id, b.title, b.summary, b.category_id, b.read_time, b.date],
    ).ok();

    HttpResponse::Created().json(b)
}

#[delete("/api/v1/blog/{id}")]
async fn delete_blog_post(req: HttpRequest, data: web::Data<DbState>, id: web::Path<String>) -> impl Responder {
    if !check_auth(&req) {
        return HttpResponse::Unauthorized().json(serde_json::json!({ "error": "Não autorizado" }));
    }
    let conn = data.db.lock().unwrap();
    let target_id = id.into_inner();
    conn.execute("DELETE FROM blog_posts WHERE id = ?1", params![target_id]).ok();
    HttpResponse::Ok().json(serde_json::json!({ "status": "deleted", "id": target_id }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Backend em Rust com SQLite rodando em http://localhost:8080");

    let db_conn = init_db();
    let db_state = web::Data::new(DbState { db: Mutex::new(db_conn) });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(db_state.clone())
            .service(login)
            .service(get_site_info)
            .service(update_site_info)
            .service(get_categories)
            .service(create_category)
            .service(delete_category)
            .service(get_projects)
            .service(create_project)
            .service(delete_project)
            .service(get_blog_posts)
            .service(create_blog_post)
            .service(delete_blog_post)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
