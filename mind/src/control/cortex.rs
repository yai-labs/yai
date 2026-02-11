use crate::control::events::EventBus;
use crate::interface::config::RuntimeConfig;
use crate::interface::proc::is_pid_alive;
use crate::control::workspace;
use serde_json::json;
use std::sync::Arc;
use sysinfo::System;
use tokio::time::{self, Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LoadClass {
    Low,
    Mid,
    High,
}

pub async fn run(cfg: Arc<RuntimeConfig>, ws: Arc<String>, bus: Arc<EventBus>) {
    let mut sys = System::new();
    let mut interval = time::interval(Duration::from_millis(1000));
    let mut ewma: f32 = 0.0;
    let alpha: f32 = 0.3;
    let mut last_class = LoadClass::Mid;
    let mut last_emit = Instant::now() - Duration::from_secs(10);

    loop {
        interval.tick().await;
        let state = workspace::read_state(&cfg, &ws);
        if state.is_none() {
            continue;
        }
        let st = state.unwrap();
        if let Some(pid) = st.engine_pid {
            if !is_pid_alive(pid) {
                continue;
            }
        } else {
            continue;
        }

        sys.refresh_cpu();
        sys.refresh_memory();
        let cpu = sys.global_cpu_info().cpu_usage();
        ewma = alpha * cpu + (1.0 - alpha) * ewma;

        let total_mem = sys.total_memory();
        let used_mem = sys.used_memory();

        let class = if ewma > 80.0 {
            LoadClass::High
        } else if ewma < 30.0 {
            LoadClass::Low
        } else {
            LoadClass::Mid
        };

        if class != last_class && last_emit.elapsed() > Duration::from_secs(3) {
            match class {
                LoadClass::High => {
                    bus.emit(
                        "engine_scale_up",
                        json!({
                            "ws": ws.as_ref(),
                            "reason": "peak",
                            "cpu_ewma": ewma,
                            "cpu": cpu,
                            "mem_used": used_mem,
                            "mem_total": total_mem,
                            "action": "emit_only"
                        }),
                    );
                }
                LoadClass::Low => {
                    bus.emit(
                        "engine_scale_down",
                        json!({
                            "ws": ws.as_ref(),
                            "reason": "low",
                            "cpu_ewma": ewma,
                            "cpu": cpu,
                            "mem_used": used_mem,
                            "mem_total": total_mem,
                            "action": "emit_only"
                        }),
                    );
                }
                LoadClass::Mid => {}
            }
            last_class = class;
            last_emit = Instant::now();
        }
    }
}
