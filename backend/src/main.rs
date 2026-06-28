use axum::{extract::Path, routing::get, Router, Json};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize, Deserialize)]
struct Language { slug: String, name: String, category: String, icon: String }

#[derive(Serialize, Deserialize)]
struct LanguageDetail { slug: String, name: String, category: String, icon: String, content: String }

async fn health() -> &'static str { "OK" }
async fn root() -> &'static str { "LearnXInMinutes API - Any language, fast" }

async fn list_languages() -> Json<Vec<Language>> {
    Json(vec![
        Language { slug: "rust".into(), name: "Rust".into(), category: "Systems".into(), icon: "🦀".into() },
        Language { slug: "python".into(), name: "Python".into(), category: "Scripting".into(), icon: "🐍".into() },
        Language { slug: "javascript".into(), name: "JavaScript".into(), category: "Web".into(), icon: "⚡".into() },
        Language { slug: "go".into(), name: "Go".into(), category: "Systems".into(), icon: "🐹".into() },
        Language { slug: "typescript".into(), name: "TypeScript".into(), category: "Web".into(), icon: "📘".into() },
        Language { slug: "java".into(), name: "Java".into(), category: "Enterprise".into(), icon: "☕".into() },
        Language { slug: "cpp".into(), name: "C++".into(), category: "Systems".into(), icon: "⚙️".into() },
        Language { slug: "ruby".into(), name: "Ruby".into(), category: "Scripting".into(), icon: "💎".into() },
        Language { slug: "swift".into(), name: "Swift".into(), category: "Mobile".into(), icon: "🐦".into() },
        Language { slug: "kotlin".into(), name: "Kotlin".into(), category: "Mobile".into(), icon: "🟣".into() },
        Language { slug: "haskell".into(), name: "Haskell".into(), category: "Functional".into(), icon: "λ".into() },
        Language { slug: "elixir".into(), name: "Elixir".into(), category: "Functional".into(), icon: "💧".into() },
    ])
}

async fn get_language(Path(slug): Path<String>) -> Json<Option<LanguageDetail>> {
    let content = match slug.as_str() {
        "rust" => "fn main() {\n    println!(\"Hello, world!\");\n}\n\n// Variables\nlet x = 5;\nlet mut y = 10;\ny += 1;\n\n// Functions\nfn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n\n// Structs\nstruct Point {\n    x: f64,\n    y: f64,\n}\n\n// Enums\nenum Direction {\n    North, South, East, West,\n}",
        "python" => "# Python basics\n\n# Variables\nname = \"World\"\nprint(f\"Hello, {name}!\")\n\n# Lists\nnums = [1, 2, 3, 4, 5]\n\n# Functions\ndef add(a, b):\n    return a + b\n\n# Classes\nclass Dog:\n    def __init__(self, name):\n        self.name = name\n    def bark(self):\n        return f\"{self.name} says woof!\"",
        "javascript" => "// JavaScript basics\n\n// Variables\nlet name = \"World\";\nconsole.log(`Hello, ${name}!`);\n\n// Functions\nconst add = (a, b) => a + b;\n\n// Classes\nclass Dog {\n    constructor(name) {\n        this.name = name;\n    }\n    bark() {\n        return `${this.name} says woof!`;\n    }\n}",
        _ => "// Language content coming soon!\n// Check back for a complete guide.",
    };
    let languages = list_languages().await.0;
    let lang = languages.into_iter().find(|l| l.slug == slug)?;
    Json(Some(LanguageDetail { slug: lang.slug, name: lang.name, category: lang.category, icon: lang.icon, content: content.into() }))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::init();
    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health))
        .route("/languages", get(list_languages))
        .route("/languages/:name", get(get_language));
    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("LearnXInMinutes running on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}
