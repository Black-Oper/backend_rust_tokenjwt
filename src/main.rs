// Importa as bibliotecas necessárias

use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use serde_json;

mod database;
use database::db::init_db;
use database::structs::{NewUser, User, HeaderJson};

mod utils;
use utils::codificar::converter_string_base64;
use utils::hmac::{compute_hmac, verify_hmac};

// Define a chave secreta para o HMAC
const KEY: &[u8] = b"jeonjungkook";

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Inicializa o servidor
    println!("Servidor iniciado em http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .service(get_users)
            .service(create_user)
            .service(login)
            .service(autenticar)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

#[get("/listarusuarios")]
async fn get_users() -> impl Responder {

    // Inicializa a conexão com o banco de dados e armazena na variável conn
    let conn = match init_db() {
        Ok(c) => c,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Erro ao conectar: {}", e)),
    };

    // Prepara a consulta SQL
    let mut stmt = match conn.prepare("SELECT id, username, password FROM users") {
        Ok(s) => s,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Erro na consulta: {}", e)),
    };

    let user_iter = match stmt.query_map([], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            password: row.get(2)?,
        })
    }) {
        Ok(iter) => iter,
        Err(e) => return HttpResponse::InternalServerError().body(format!("Erro ao iterar: {}", e)),
    };

    let mut users: Vec<User> = Vec::new();
    for user in user_iter {
        match user {
            Ok(u) => users.push(u),
            Err(e) => return HttpResponse::InternalServerError().body(format!("Erro ao ler usuário: {}", e)),
        }
    }

    HttpResponse::Ok().json(users)
}

#[post("/login")]
async fn login(login_user: actix_web::web::Json<NewUser>) -> impl Responder {
    let conn = match init_db() {
        Ok(c) => c,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Erro ao conectar: {}", e))
        }
    };

    // Recupera a senha (HMAC) armazenada para o usuário
    let stored_password: String = match conn.query_row(
        "SELECT password FROM users WHERE username = ?1",
        &[&login_user.username],
        |row| row.get(0),
    ) {
        Ok(p) => p,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Erro ao obter usuário: {}", e))
        }
    };

    // Verifica se o HMAC da senha informada corresponde à senha armazenada
    let is_valid = match verify_hmac(KEY, login_user.password.as_bytes(), &stored_password) {
        Ok(valid) => valid,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Erro ao verificar senha: {}", e))
        }
    };

    if is_valid {
        let header_json = serde_json::to_string(&HeaderJson {
            alg: "HS256".to_string(),
            typ: "JWT".to_string(),
        }).unwrap();
        let token_header = converter_string_base64(&header_json);

        let payload_json = serde_json::to_string(&User {
            id: match conn.query_row(
                "SELECT id FROM users WHERE username = ?1",
                &[&login_user.username],
                |row| row.get(0),
            ) {
                Ok(id) => id,
                Err(e) => return HttpResponse::InternalServerError().body(format!("Erro ao obter ID do usuário: {}", e)),
            },
            username: login_user.username.clone(),
            password: stored_password.clone(),
        }).unwrap();
        let token_payload = converter_string_base64(&payload_json);

        let token_almost = format!("{}.{}", token_header, token_payload);

        let token_signature = match compute_hmac(KEY, token_almost.as_bytes()) {
            Ok(sig) => sig,
            Err(e) => return HttpResponse::InternalServerError().body(format!("Erro ao gerar assinatura: {}", e)),
        };

        let token = format!("{}.{}", token_almost, token_signature);

        HttpResponse::Ok().body(token)
    } else {
        HttpResponse::Unauthorized().body("Usuário ou senha inválidos!")
    }
}

#[post("/cadastrar")]
async fn create_user(new_user: actix_web::web::Json<NewUser>) -> Result<impl Responder, actix_web::Error> {
    // Inicializa a conexão com o banco de dados
    let conn = match init_db() {
        Ok(c) => c,
        Err(e) => return Ok(HttpResponse::InternalServerError().body(format!("Erro ao conectar: {}", e))),
    };

    // Calcula o HMAC da senha. Observe que convertemos a senha para bytes.
    let hmac_result = compute_hmac(KEY, new_user.password.as_bytes())
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    // Insere o novo usuário no banco utilizando o HMAC (senha criptografada)
    let result = conn.execute(
        "INSERT INTO users (username, password) VALUES (?1, ?2)",
        &[&new_user.username, &hmac_result],
    );

    match result {
        Ok(_) => Ok(HttpResponse::Ok().body("Usuário cadastrado com sucesso!")),
        Err(e) => Ok(HttpResponse::InternalServerError().body(format!("Erro ao cadastrar usuário: {}", e))),
    }
}

#[post("/autenticar")]
async fn autenticar(token: String) -> impl Responder {
    let token = token.trim(); // Remove espaços extras
    match utils::jwttoken::verify_jwt_token(token) {
        Ok(user) => {
            let response = serde_json::json!({
                "message": "token é válido",
                "user": user
            });
            HttpResponse::Ok().json(response)
        },
        Err(_) => HttpResponse::Unauthorized().body("token não é válido"),
    }
}

