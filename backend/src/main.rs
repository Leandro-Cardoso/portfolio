use actix_cors::Cors;
use actix_web::{delete, get, http::header, middleware::Logger, post, put, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

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
    pub tags: Vec<String>,
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

pub struct AppState {
    pub categories: Mutex<Vec<Category>>,
    pub projects: Mutex<Vec<Project>>,
    pub posts: Mutex<Vec<BlogPost>>,
}

#[derive(Deserialize)]
struct CategoryQuery {
    category: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ContactMessage {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub message: String,
}

// --- Endpoints Categorias ---
#[get("/api/v1/categories")]
async fn get_categories(data: web::Data<AppState>) -> impl Responder {
    let cats = data.categories.lock().unwrap();
    HttpResponse::Ok().json(&*cats)
}

#[post("/api/v1/categories")]
async fn create_category(data: web::Data<AppState>, item: web::Json<Category>) -> impl Responder {
    let mut cats = data.categories.lock().unwrap();
    let mut new_cat = item.into_inner();
    if new_cat.id.is_empty() {
        new_cat.id = new_cat.name.to_lowercase().replace(' ', "-");
    }
    cats.push(new_cat.clone());
    HttpResponse::Created().json(new_cat)
}

#[delete("/api/v1/categories/{id}")]
async fn delete_category(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let mut cats = data.categories.lock().unwrap();
    let target_id = id.into_inner();
    cats.retain(|c| c.id != target_id);
    HttpResponse::Ok().json(serde_json::json!({ "status": "deleted", "id": target_id }))
}

// --- Endpoints Projetos ---
#[get("/api/v1/projects")]
async fn get_projects(data: web::Data<AppState>, query: web::Query<CategoryQuery>) -> impl Responder {
    let projs = data.projects.lock().unwrap();
    if let Some(cat) = &query.category {
        let filtered: Vec<Project> = projs.iter().filter(|p| &p.category_id == cat).cloned().collect();
        return HttpResponse::Ok().json(filtered);
    }
    HttpResponse::Ok().json(&*projs)
}

#[post("/api/v1/projects")]
async fn create_project(data: web::Data<AppState>, item: web::Json<Project>) -> impl Responder {
    let mut projs = data.projects.lock().unwrap();
    let mut new_proj = item.into_inner();
    if new_proj.id.is_empty() {
        new_proj.id = format!("{}", projs.len() + 1);
    }
    projs.push(new_proj.clone());
    HttpResponse::Created().json(new_proj)
}

#[delete("/api/v1/projects/{id}")]
async fn delete_project(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let mut projs = data.projects.lock().unwrap();
    let target_id = id.into_inner();
    projs.retain(|p| p.id != target_id);
    HttpResponse::Ok().json(serde_json::json!({ "status": "deleted", "id": target_id }))
}

// --- Endpoints Blog ---
#[get("/api/v1/blog")]
async fn get_blog_posts(data: web::Data<AppState>, query: web::Query<CategoryQuery>) -> impl Responder {
    let posts = data.posts.lock().unwrap();
    if let Some(cat) = &query.category {
        let filtered: Vec<BlogPost> = posts.iter().filter(|p| &p.category_id == cat).cloned().collect();
        return HttpResponse::Ok().json(filtered);
    }
    HttpResponse::Ok().json(&*posts)
}

#[post("/api/v1/blog")]
async fn create_blog_post(data: web::Data<AppState>, item: web::Json<BlogPost>) -> impl Responder {
    let mut posts = data.posts.lock().unwrap();
    let mut new_post = item.into_inner();
    if new_post.id.is_empty() {
        new_post.id = format!("{}", posts.len() + 1);
    }
    posts.push(new_post.clone());
    HttpResponse::Created().json(new_post)
}

#[delete("/api/v1/blog/{id}")]
async fn delete_blog_post(data: web::Data<AppState>, id: web::Path<String>) -> impl Responder {
    let mut posts = data.posts.lock().unwrap();
    let target_id = id.into_inner();
    posts.retain(|p| p.id != target_id);
    HttpResponse::Ok().json(serde_json::json!({ "status": "deleted", "id": target_id }))
}

// Endpoint POST para processar mensagens de contato
#[post("/api/v1/contact")]
async fn send_contact_message(item: web::Json<ContactMessage>) -> impl Responder {
    let msg = item.into_inner();
    println!("📩 Nova mensagem de contato recebida de: {} ({})", msg.name, msg.email);
    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": "Mensagem recebida com sucesso!"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Backend API em Rust v1.2 rodando em http://localhost:8080");

    let initial_state = web::Data::new(AppState {
        categories: Mutex::new(vec![
            Category { id: "web".to_string(), name: "Web Dev".to_string(), color: "#3B82F6".to_string() },
            Category { id: "jogo".to_string(), name: "Jogos / Engine".to_string(), color: "#EAB308".to_string() },
            Category { id: "lib".to_string(), name: "Biblioteca Rust".to_string(), color: "#F97316".to_string() },
            Category { id: "ia".to_string(), name: "IA & ML".to_string(), color: "#8B5CF6".to_string() },
            Category { id: "tech".to_string(), name: "Tecnologia".to_string(), color: "#10B981".to_string() },
        ]),
        projects: Mutex::new(vec![
            Project {
                id: "1".to_string(),
                title: "Tiles Gear Engine".to_string(),
                description: "Engine 2D focada em camadas e tiles desenvolvida do zero em Rust puro.".to_string(),
                category_id: "jogo".to_string(),
                tags: vec!["Rust".to_string(), "2D".to_string(), "GameDev".to_string()],
                link: "https://github.com/leandro/tiles-gear".to_string(),
            },
            Project {
                id: "2".to_string(),
                title: "Portfolio Pessoal & Admin".to_string(),
                description: "Sistema web dinâmico com Vue 3, CSS modular e API REST em Rust Actix.".to_string(),
                category_id: "web".to_string(),
                tags: vec!["Vue.js".to_string(), "Rust".to_string(), "REST".to_string()],
                link: "#".to_string(),
            },
        ]),
        posts: Mutex::new(vec![
            BlogPost {
                id: "1".to_string(),
                title: "Arquitetura de Game Engine 2D em Rust".to_string(),
                summary: "Como estruturar um motor de jogos sem dependências externas utilizando Rust.".to_string(),
                category_id: "tech".to_string(),
                read_time: "5 min".to_string(),
                date: "2026-09-15".to_string(),
            },
        ]),
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(initial_state.clone())
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
