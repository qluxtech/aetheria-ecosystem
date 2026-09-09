// ============================================================================
// AETHERIA-CLIENT: Stratified Civilization Web Server & UI Gate
// Renders the 1/f breathing rhythm and civilization strata frontend.
// ============================================================================

use axum::{
    response::Html,
    routing::get,
    Router,
};
use std::net::SocketAddr;
use colored::*;

#[tokio::main]
async fn main() {
    print_client_banner();

    let app = Router::new().route("/", get(render_stratified_ui));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("[Client] Stratified UI online. Access the ecosystem at: http://{}", addr.to_string().cyan());
    println!("[Client] Synchronizing with human biological 1/f rhythm...\n");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn print_client_banner() {
    println!("{}", "==================================================".bright_blue());
    println!("{}", "    AETHERIA CLIENT --- STRATIFIED UI v0.1.0      ".bright_blue().bold());
    println!("{}", "==================================================".bright_blue());
}

async fn render_stratified_ui() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="ja">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>AETHERIA — Civilization OS</title>
    <style>
        @keyframes breathe {
            0%, 100% { transform: scale(1); opacity: 0.6; filter: blur(20px); }
            50% { transform: scale(1.15); opacity: 0.95; filter: blur(10px); }
        }
        @keyframes strataShift {
            0% { background-position: 0% 50%; }
            50% { background-position: 100% 50%; }
            100% { background-position: 0% 50%; }
        }
        body {
            margin: 0;
            height: 100vh;
            display: flex;
            justify-content: center;
            align-items: center;
            background: linear-gradient(135deg, #120802, #1b1224, #041018, #000000);
            background-size: 400% 400%;
            animation: strataShift 16s ease infinite;
            color: #f0f0f0;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            overflow: hidden;
        }
        .strata-container {
            text-align: center;
            position: relative;
            z-index: 2;
        }
        .lung-pulse {
            width: 220px;
            height: 220px;
            border-radius: 50%;
            background: radial-gradient(circle, rgba(255,140,0,0.8) 0%, rgba(138,43,226,0.4) 50%, rgba(0,255,200,0.1) 100%);
            animation: breathe 4s ease-in-out infinite;
            margin: 0 auto 30px auto;
            box-shadow: 0 0 50px rgba(255,140,0,0.3);
        }
        h1 {
            font-size: 2.5rem;
            letter-spacing: 0.2em;
            margin: 0 0 10px 0;
            background: linear-gradient(90deg, #ff8c00, #ff007f, #00ffc8);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
        }
        p {
            color: #a0a0b0;
            font-size: 0.95rem;
            letter-spacing: 0.05em;
        }
        .intent-input {
            margin-top: 30px;
            background: rgba(255, 255, 255, 0.05);
            border: 1px solid rgba(255, 255, 255, 0.15);
            padding: 15px 25px;
            border-radius: 30px;
            color: #fff;
            font-size: 1rem;
            outline: none;
            width: 320px;
            transition: all 0.3s ease;
        }
        .intent-input:focus {
            border-color: #ff8c00;
            box-shadow: 0 0 20px rgba(255,140,0,0.4);
        }
    </style>
</head>
<body>
    <div class="strata-container">
        <div class="lung-pulse"></div>
        <h1>AETHERIA</h1>
        <p>Stratified Civilization OS — Synchronized with 4s Breath</p>
        <input type="text" class="intent-input" value="Love" readonly />
    </div>
</body>
</html>
    "#)
}

