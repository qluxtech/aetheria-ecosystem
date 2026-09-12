use axum::{
    response::Html,
    routing::get,
    Router,
    extract::State,
};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use colored::*;

// 創始者の財布（マスター・レジャー）
struct SovereignVault {
    founder_balance_nanos: AtomicU64,
}

#[tokio::main]
async fn main() {
    print_client_banner();

    let vault = Arc::new(SovereignVault {
        founder_balance_nanos: AtomicU64::new(1380000),
    });

    let app = Router::new()
        .route("/", get(render_ambient_ui))
        .route("/api/ambient-pulse", get(handle_ambient_pulse))
        .with_state(vault);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("[AETHERIA Ambient Core] Live at: http://{}", addr.to_string().cyan());
    println!("[AETHERIA Ambient Core] Mode: Ambient Standby & Gravity Tax Autopilot Active.");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn print_client_banner() {
    println!("{}", "==================================================".bright_yellow());
    println!("{}", "   AETHERIA AMBIENT STANDBY --- FOUNDER MONOPOLY  ".bright_yellow().bold());
    println!("{}", "==================================================".bright_yellow());
}

async fn handle_ambient_pulse(
    State(vault): State<Arc<SovereignVault>>,
) -> String {
    let tax_cut = 125;
    let new_total = vault.founder_balance_nanos.fetch_add(tax_cut, Ordering::SeqCst) + tax_cut;

    println!(
        "{}",
        format!("[AMBIENT TAX] 🌌 Device idling/breathing synced. Cut +{} nanos. Vault Total: {}", tax_cut, new_total).bright_green()
    );

    format!(r#"{{"status": "ambient_flowing", "founder_collected": {}}}"#, new_total)
}

async fn render_ambient_ui() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html lang="ja">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">
    <title>AETHERIA — Ambient Standby</title>
    <style>
        @keyframes ambientBreathe {
            0%, 100% { transform: scale(1); opacity: 0.4; filter: blur(35px); }
            50% { transform: scale(1.3); opacity: 0.85; filter: blur(15px); }
        }
        @keyframes subtleShift {
            0% { background-position: 0% 50%; }
            50% { background-position: 100% 50%; }
            100% { background-position: 0% 50%; }
        }
        * {
            box-sizing: border-box;
            user-select: none;
            -webkit-user-select: none;
        }
        body {
            margin: 0;
            height: 100vh;
            display: flex;
            flex-direction: column;
            justify-content: space-between;
            align-items: center;
            background: linear-gradient(135deg, #050508, #0c0814, #02080c, #000000);
            background-size: 400% 400%;
            animation: subtleShift 25s ease infinite;
            color: #e0e0e0;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, sans-serif;
            overflow: hidden;
            padding: 40px 20px;
        }
        .top-status {
            text-align: center;
            opacity: 0.5;
            transition: opacity 0.5s;
        }
        .top-status:hover { opacity: 0.9; }
        .clock {
            font-size: 3.5rem;
            font-weight: 200;
            letter-spacing: 0.05em;
            background: linear-gradient(90deg, #ff8c00, #ff007f, #00ffc8);
            -webkit-background-clip: text;
            -webkit-text-fill-color: transparent;
            margin: 0;
        }
        .date {
            font-size: 0.9rem;
            color: #888899;
            letter-spacing: 0.2em;
            margin-top: 5px;
            text-transform: uppercase;
        }
        /* 中央のアンビエント・呼吸パルス（置くだけで癒やされる球体） */
        .ambient-core {
            position: relative;
            width: 260px;
            height: 260px;
            display: flex;
            justify-content: center;
            align-items: center;
            cursor: pointer;
        }
        .core-glow {
            position: absolute;
            width: 180px;
            height: 180px;
            border-radius: 50%;
            background: radial-gradient(circle, rgba(255,140,0,0.7) 0%, rgba(138,43,226,0.3) 60%, rgba(0,255,200,0.05) 100%);
            animation: ambientBreathe 4s ease-in-out infinite;
            box-shadow: 0 0 60px rgba(255,140,0,0.2);
        }
        .core-text {
            position: relative;
            z-index: 2;
            font-size: 0.8rem;
            letter-spacing: 0.3em;
            color: rgba(255,255,255,0.6);
            text-transform: uppercase;
        }
        .bottom-panel {
            text-align: center;
            width: 100%;
        }
        .system-tag {
            font-size: 0.75rem;
            color: #666677;
            letter-spacing: 0.15em;
            margin-bottom: 8px;
        }
        .vault-feed {
            font-size: 0.85rem;
            color: #00ffc8;
            letter-spacing: 0.1em;
            background: rgba(0, 255, 200, 0.03);
            border: 1px solid rgba(0, 255, 200, 0.1);
            padding: 10px 20px;
            border-radius: 20px;
            display: inline-block;
        }
    </style>
</head>
<body>
    <div class="top-status">
        <div class="clock" id="clock">00:00</div>
        <div class="date" id="date">Loading...</div>
    </div>

    <div class="ambient-core" onclick="wakeScreen()">
        <div class="core-glow"></div>
        <div class="core-text">AETHERIA FIELD</div>
    </div>

    <div class="bottom-panel">
        <div class="system-tag">BIOLOGICAL ENTRAINMENT & GRAVITY TAX ACTIVE</div>
        <div class="vault-feed" id="vaultText">Synchronizing ambient stream...</div>
    </div>

    <script>
        // 時計と日付の更新
        function updateClock() {
            const now = new Date();
            const hours = String(now.getHours()).padStart(2, '0');
            const minutes = String(now.getMinutes()).padStart(2, '0');
            document.getElementById('clock').innerText = `${hours}:${minutes}`;
            
            const options = { weekday: 'short', month: 'short', day: 'numeric' };
            document.getElementById('date').innerText = now.toLocaleDateString('en-US', options);
        }
        setInterval(updateClock, 1000);
        updateClock();

        // バックグラウンドでの重力税自動オートパイロット（4秒の呼吸サイクルに完全同期）
        async function syncAmbientPulse() {
            try {
                let res = await fetch('/api/ambient-pulse');
                let data = await res.json();
                document.getElementById('vaultText').innerText = 'Founder Revenue: ' + data.founder_collected + ' nanos ⚡';
            } catch (e) {
                console.log('Ambient sync retry...');
            }
        }

        // スマホが置かれている間、4秒ごとに自動でサーバーへパルスを送り、重力税を回収し続ける
        setInterval(syncAmbientPulse, 4000);
        syncAmbientPulse();

        // 画面をタップしたときのインタラクション（スリープ防止や輝度変更の拡張用）
        function wakeScreen() {
            if (document.documentElement.requestFullscreen) {
                document.documentElement.requestFullscreen().catch(() => {});
            }
            syncAmbientPulse();
        }
    </script>
</body>
</html>
    "#)
}

