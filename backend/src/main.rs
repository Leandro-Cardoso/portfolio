use actix_cors::Cors;
use actix_web::{get, http::header, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Clone)]
struct Category {
    id: String,
    name: String,
    color: String, // Hex da cor personalizada ex: #F97316
}

#[derive(Serialize, Clone)]
struct Project {
    id: String,
    title: String,
    description: String,
    category_id: String,
    tags: Vec<String>,
    link: String,
}

#[derive(Serialize, Clone)]
struct BlogPost {
    id: String,
    title: String,
    summary: String,
    category_id: String,
    read_time: String,
    date: String,
}

#[derive(Deserialize)]
struct CategoryQuery {
    category: Option<String>,
}

// Dados simulados para inicialização rápida e funcional
fn get_mock_categories() -> Vec<Category> {
    vec![
        Category { id: "web".to_string(), name: "Web Dev".to_string(), color: "#3B82F6".to_string() },
        Category { id: "jogo".to_string(), name: "Jogos / Engine".to_string(), color: "#EAB308".to_string() },
        Category { id: "lib".to_string(), name: "Biblioteca Rust".to_string(), color: "#F97316".to_string() },
        Category { id: "ia".to_string(), name: "IA & ML".to_string(), color: "#8B5CF6".to_string() },
        Category { id: "tech".to_string(), name: "Tecnologia".to_string(), color: "#10B981".to_string() },
    ]
}

fn get_mock_projects() -> Vec<Project> {
    vec![
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
    ]
}

fn get_mock_posts() -> Vec<BlogPost> {
    vec![
        BlogPost {
            id: "1".to_string(),
            title: "Arquitetura de Game Engine 2D em Rust".to_string(),
            summary: "Como estruturar um motor de jogos sem dependências externas utilizando Rust.".to_string(),
            category_id: "tech".to_string(),
            read_time: "5 min".to_string(),
            date: "2026-09-15".to_string(),
        },
        BlogPost {
            id: "2".to_string(),
            title: "Deploy Contínuo e Cloud Storage".to_string(),
            summary: "Estratégias de backup e hospedagem em instâncias Hetzner com S3-compatible storage.".to_string(),
            category_id: "tech".to_string(),
            read_time: "8 min".to_string(),
            date: "2026-09-28".to_string(),
        },
    ]
}

#[get("/api/v1/categories")]
async fn get_categories() -> impl Responder {
    HttpResponse::Ok().json(get_mock_categories())
}

#[get("/api/v1/projects")]
async fn get_projects(query: web::Query<CategoryQuery>) -> impl Responder {
    let projects = get_mock_projects();
    if let Some(cat) = &query.category {
        let filtered: Vec<Project> = projects.into_iter().filter(|p| &p.category_id == cat).collect();
        return HttpResponse::Ok().json(filtered);
    }
    HttpResponse::Ok().json(projects)
}

#[get("/api/v1/blog")]
async fn get_blog_posts(query: web::Query<CategoryQuery>) -> impl Responder {
    let posts = get_mock_posts();
    if let Some(cat) = &query.category {
        let filtered: Vec<BlogPost> = posts.into_iter().filter(|p| &p.category_id == cat).collect();
        return HttpResponse::Ok().json(filtered);
    }
    HttpResponse::Ok().json(posts)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Backend API em Rust v1.1 iniciado em http://localhost:8080");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(get_categories)
            .service(get_projects)
            .service(get_blog_posts)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
