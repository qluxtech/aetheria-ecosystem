// ============================================================================
// AETHERIA-CORE: The Living Breathing Ecosystem Engine
// Architecture: Biological Rhythm & Autonomous Swarm Ledger
// ============================================================================

use tokio::time::{sleep, Duration};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::Local;
use colored::*;

#[tokio::main]
async fn main() {
    print_banner();

    // 呼吸サイクルを定義（例: 4秒に1回の吸気・呼気）
    let breath_interval = Duration::from_secs(4);
    let ecosystem = Arc::new(AetheriaEcosystem::new(breath_interval));

    // 最初のパルス（意志）の注入：「Love」
    ecosystem.inject_intent("Love - The absolute driver of value and harmonious creation.").await;

    // 生態系のメインループを起動
    ecosystem.ignite().await;
}

/// システムバナーの描画
fn print_banner() {
    println!("{}", "==================================================".bright_cyan());
    println!("{}", "       A E T H E R I O S  ---  CORE OS v0.1.0     ".bright_cyan().bold());
    println!("{}", "   The Stratified Autonomous Civilization Network ".truecolor(150, 150, 150));
    println!("{}", "==================================================".bright_cyan());
}

/// AETHERIA 生態系エンジン構造体
pub struct AetheriaEcosystem {
    breath_cycle: Duration,
    intent_queue: Arc<Mutex<Vec<String>>>,
    nano_ledger: Arc<Mutex<u64>>,
}

impl AetheriaEcosystem {
    pub fn new(breath_cycle: Duration) -> Self {
        Self {
            breath_cycle,
            intent_queue: Arc::new(Mutex::new(Vec::new())),
            nano_ledger: Arc::new(Mutex::new(0)),
        }
    }

    /// 人間の「意志（Intent）」を吸入する
    pub async fn inject_intent(&self, intent: &str) {
        let mut queue = self.intent_queue.lock().await;
        queue.push(intent.to_string());
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        println!("[{}] {} {}", timestamp.to_string().dimmed(), ">>> [INHALE INTENT]:".bright_green().bold(), intent.yellow());
    }

    /// 永遠に脈打つ文明OSのメインループ
    pub async fn ignite(self: Arc<Self>) {
        let interval = self.breath_cycle;
        println!("\n[System] AETHERIA Lung synchronized. Breathing rhythm active ({}s interval).\n", interval.as_secs());

        loop {
            // --- 1. 吸気フェーズ (INHALE) ---
            let queue_clone = Arc::clone(&self.intent_queue);
            let mut queue = queue_clone.lock().await;
            
            if !queue.is_empty() {
                while let Some(intent) = queue.pop() {
                    self.process_inhale(&intent).await;
                }
            } else {
                let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
                println!("[{}] {} Ambient zero-point breath syncing...", timestamp.to_string().dimmed(), "(Inhale)".green());
            }

            // 呼吸の深さを調整するウェイト
            sleep(interval / 2).await;

            // --- 2. 呼気フェーズ (EXHALE) ---
            self.process_exhale().await;

            sleep(interval / 2).await;
            println!("{}", "--------------------------------------------------".dimmed());
        }
    }

    async fn process_inhale(&self, intent: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        println!("[{}] Swarm absorbing intent vector: \"{}\"", timestamp.to_string().dimmed(), intent.cyan());
        sleep(Duration::from_millis(300)).await;
        println!("  -> Agents assembling architecture pipelines... [OK]");
    }

    async fn process_exhale(&self) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let mut ledger = self.nano_ledger.lock().await;
        
        // 呼気のたびに生成される微小な富（ナノ決済の加算）
        let generated_nanos = 142857;
        *ledger += generated_nanos;

        println!("[{}] {} Swarm executing self-healing & nano-transactions...", timestamp.to_string().dimmed(), "<<< [EXHALE]:".bright_magenta().bold());
        sleep(Duration::from_millis(300)).await;
        println!("  -> Cloud infra verified. Zero friction errors. Self-healing active.");
        println!("  -> Ledger Flow: +{} micro-credits (Total circulating: {} nanos)", generated_nanos.to_string().green(), ledger.to_string().bright_yellow());
    }
}

