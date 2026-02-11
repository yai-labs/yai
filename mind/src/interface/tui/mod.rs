use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom, Write, BufRead};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph, Wrap};
use ratatui::Terminal;

use crate::bridge::vault::VaultBridge as EngineVaultBridge;
use crate::rpc::protocol::{AliveStatus, Request};
use crate::rpc::uds_client;
use crate::interface::config::RuntimeConfig;
use crate::interface::proc::{log_path, RunState};
use crate::llm::adapter::build_llm_for_ws;

#[derive(Clone)]
struct VaultInfo {
    status: u32,
    energy_quota: u32,
    energy_used: u32,
    last_command_id: u32,
    last_error: String,
    response: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum AppMode {
    Normal,
    ChatInput,
}

struct App {
    ws: String,
    menu_index: usize,
    input: String,
    mode: AppMode,
    messages: Vec<(String, String)>,
    log_component: usize,
    log_lines: Vec<String>,
    last_refresh: Instant,
    force_refresh: bool,
    poll_rate: Duration,
    tick_rate: Duration,
    run_state: Option<RunState>,
    alive: AliveStatus,
    connected: bool,
    vault_info: Option<VaultInfo>,
    vault_bridge: Option<EngineVaultBridge>,
    last_command_id: Option<u32>,
    build_info: String,
    llm_rx: Receiver<(String, String)>,
    llm_tx: Sender<String>,
    event_rx: Receiver<crate::rpc::protocol::Event>,
    last_event: Instant,
    last_event_label: String,
}

const MENU_ITEMS: [&str; 5] = ["Overview", "Processes", "Vault", "Logs", "Chat"];
const LOG_COMPONENTS: [&str; 3] = ["boot", "engine", "mind"];

pub fn run(ws: &str, cfg: &RuntimeConfig) -> anyhow::Result<()> {

    let (llm_resp_tx, llm_resp_rx) = mpsc::channel::<(String, String)>();
    let (llm_req_tx, llm_req_rx) = mpsc::channel::<String>();
    let (event_tx, event_rx) = mpsc::channel::<crate::rpc::protocol::Event>();

    spawn_llm_worker(cfg.clone(), ws.to_string(), llm_req_rx, llm_resp_tx);
    spawn_event_worker(cfg.clone(), ws.to_string(), event_tx);

    let mut app = App {
        ws: ws.to_string(),
        menu_index: 0,
        input: String::new(),
        mode: AppMode::Normal,
        messages: Vec::new(),
        log_component: 0,
        log_lines: Vec::new(),
        last_refresh: Instant::now(),
        force_refresh: true,
        poll_rate: Duration::from_millis(800),
        tick_rate: Duration::from_millis(100),
        run_state: None,
        alive: AliveStatus::default(),
        connected: false,
        vault_info: None,
        vault_bridge: None,
        last_command_id: None,
        build_info: read_manifest(&cfg).unwrap_or_else(|| "unknown".to_string()),
        llm_rx: llm_resp_rx,
        llm_tx: llm_req_tx,
        event_rx,
        last_event: Instant::now(),
        last_event_label: String::new(),
    };

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, &cfg, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    cfg: &RuntimeConfig,
    app: &mut App,
) -> anyhow::Result<()> {
    loop {
        if app.force_refresh || app.last_refresh.elapsed() > app.poll_rate {
            refresh_state(cfg, app);
            app.last_refresh = Instant::now();
            app.force_refresh = false;
        }

        while let Ok((role, msg)) = app.llm_rx.try_recv() {
            app.messages.push((role, msg));
        }
        while let Ok(ev) = app.event_rx.try_recv() {
            app.last_event = Instant::now();
            app.last_event_label = format!("{} {:?}", ev.kind, ev.data);
            app.connected = true;
            match ev.kind.as_str() {
                "status_changed" => {
                    if let Some(obj) = ev.data.as_object() {
                        if let Some(v) = obj.get("boot").and_then(|v| v.as_bool()) {
                            app.alive.boot = v;
                        }
                        if let Some(v) = obj.get("kernel").and_then(|v| v.as_bool()) {
                            app.alive.kernel = v;
                        }
                        if let Some(v) = obj.get("engine").and_then(|v| v.as_bool()) {
                            app.alive.engine = v;
                        }
                        if let Some(v) = obj.get("mind").and_then(|v| v.as_bool()) {
                            app.alive.mind = v;
                        }
                    }
                }
                _ => {}
            }
        }

        terminal.draw(|f| ui(f, app))?;

        if event::poll(app.tick_rate)? {
            if let Event::Key(key) = event::read()? {
                if handle_key(cfg, app, key)? {
                    return Ok(());
                }
            }
        }
    }
}

fn handle_key(
    _cfg: &RuntimeConfig,
    app: &mut App,
    key: KeyEvent,
) -> anyhow::Result<bool> {
    match key.code {
        KeyCode::Char('q') => return Ok(true),
        KeyCode::Char('1') => app.menu_index = 0,
        KeyCode::Char('2') => app.menu_index = 1,
        KeyCode::Char('3') => app.menu_index = 2,
        KeyCode::Char('4') => app.menu_index = 3,
        KeyCode::Char('5') => app.menu_index = 4,
        KeyCode::Char('i') => {
            if app.menu_index == 4 {
                app.mode = AppMode::ChatInput;
            }
        }
        KeyCode::Esc => {
            app.mode = AppMode::Normal;
        }
        KeyCode::Char('l') => {
            if app.menu_index == 3 {
                app.log_component = (app.log_component + 1) % LOG_COMPONENTS.len();
                app.force_refresh = true;
            }
        }
        KeyCode::Char('r') => app.force_refresh = true,
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return Ok(true),
        _ => {}
    }

    if app.menu_index == 4 && app.mode == AppMode::ChatInput {
        match key.code {
            KeyCode::Char(c) => app.input.push(c),
            KeyCode::Backspace => {
                app.input.pop();
            }
            KeyCode::Enter => {
                let prompt = app.input.trim().to_string();
                app.input.clear();
                if !prompt.is_empty() {
                    app.messages.push(("user".to_string(), prompt.clone()));
                    let _ = app.llm_tx.send(prompt);
                }
            }
            _ => {}
        }
    }

    Ok(false)
}

fn refresh_state(cfg: &RuntimeConfig, app: &mut App) {
    let fallback = app.last_event.elapsed() > Duration::from_millis(1500);
    if fallback {
        if let Ok(resp) = uds_client::send_request(&cfg.run_dir, &app.ws, &Request::Status) {
            if let crate::rpc::protocol::Response::Status { state, alive, .. } = resp {
                app.run_state = state;
                app.alive = alive;
                app.connected = true;
                return;
            }
        }
        app.run_state = None;
        app.connected = false;
    }

    if app.vault_bridge.is_none() {
        if let Ok(v) = EngineVaultBridge::attach(&app.ws) {
            app.vault_bridge = Some(v);
        }
    }
    if let Some(v) = &app.vault_bridge {
            let vault = v.as_mut();
        let last_error = c_string(&vault.last_error);
        let response = v.read_response();
        app.vault_info = Some(VaultInfo {
            status: vault.status,
            energy_quota: vault.energy_quota,
            energy_used: vault.energy_consumed,
            last_command_id: vault.last_command_id,
            last_error,
            response,
        });
        app.last_command_id = Some(vault.last_command_id);
    } else {
        app.vault_info = None;
    }

    if app.menu_index == 3 {
        let log_path = log_path(&cfg.run_dir, &app.ws, LOG_COMPONENTS[app.log_component]);
        app.log_lines = tail_last_kb(&log_path, 128, 80);
    }
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10), Constraint::Length(3)])
        .split(size);

    render_header(f, chunks[0], app);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(18), Constraint::Min(10)])
        .split(chunks[1]);

    render_menu(f, body[0], app);
    render_panel(f, body[1], app);

    render_prompt(f, chunks[2], app);
}

fn render_header(f: &mut ratatui::Frame, area: Rect, app: &App) {
    let status = if !app.connected {
        "connecting"
    } else {
        match &app.run_state {
            None => "down",
            Some(_state) => {
                if app.alive.kernel {
                    if app.alive.engine && app.alive.mind {
                        "up"
                    } else {
                        "degraded"
                    }
                } else {
                    "down"
                }
            }
        }
    };

    let mut title = format!(
        "YAI TUI  | ws={} | status={} | build={} | [1-5] menu | q quit",
        app.ws, status, app.build_info
    );
    if !app.last_event_label.is_empty() {
        title.push_str(" | last=");
        let mut last = app.last_event_label.clone();
        if last.len() > 60 {
            last.truncate(60);
            last.push_str("…");
        }
        title.push_str(&last);
    }

    let block = Block::default().borders(Borders::ALL).title(title);
    f.render_widget(block, area);
}

fn render_menu(f: &mut ratatui::Frame, area: Rect, app: &App) {
    let items: Vec<ListItem> = MENU_ITEMS
        .iter()
        .enumerate()
        .map(|(i, name)| {
            let style = if i == app.menu_index {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(Span::styled(*name, style))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Menu"));
    f.render_widget(list, area);
}

fn render_panel(f: &mut ratatui::Frame, area: Rect, app: &App) {
    match app.menu_index {
        0 => render_overview(f, area, app),
        1 => render_processes(f, area, app),
        2 => render_vault(f, area, app),
        3 => render_logs(f, area, app),
        4 => render_chat(f, area, app),
        _ => {}
    }
}

fn render_overview(f: &mut ratatui::Frame, area: Rect, app: &App) {
    let mut lines = vec![
        Line::from(Span::styled("Overview", Style::default().add_modifier(Modifier::BOLD))),
        Line::from(""),
    ];

    if !app.connected {
        lines.push(Line::from("connecting to daemon..."));
    } else if let Some(state) = &app.run_state {
        let boot_status = match state.boot_pid {
            None => "not_started",
            Some(_) if app.alive.boot => "running",
            Some(_) => "completed",
        };
        lines.push(Line::from(format!("boot:       {:?} ({})", state.boot_pid, boot_status)));
        if let Some(pgid) = state.pgid {
            lines.push(Line::from(format!("pgid:       {}", pgid)));
        }
        lines.push(Line::from(format!("kernel pid: {:?}", state.kernel_pid)));
        lines.push(Line::from(format!("engine pid: {:?}", state.engine_pid)));
        lines.push(Line::from(format!("mind pid:   {:?}", state.mind_pid)));
        lines.push(Line::from(format!("socket:     {}", state.socket_path)));
    } else {
        lines.push(Line::from("status: down"));
    }

    let block = Block::default().borders(Borders::ALL).title("Overview");
    let para = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
    f.render_widget(para, area);
}

fn render_processes(f: &mut ratatui::Frame, area: Rect, app: &App) {
    let mut lines = vec![Line::from(Span::styled("Processes", Style::default().add_modifier(Modifier::BOLD)))];

    if !app.connected {
        lines.push(Line::from("connecting to daemon..."));
    } else if let Some(state) = &app.run_state {
        let boot_status = match state.boot_pid {
            None => "not_started",
            Some(_) if app.alive.boot => "running",
            Some(_) => "completed",
        };
        lines.push(Line::from(format!(
            "boot:   pid={:?} status={}",
            state.boot_pid, boot_status
        )));
        lines.push(Line::from(format!(
            "kernel: pid={:?} alive={}",
            state.kernel_pid,
            app.alive.kernel
        )));
        lines.push(Line::from(format!(
            "engine: pid={:?} alive={}",
            state.engine_pid,
            app.alive.engine
        )));
        lines.push(Line::from(format!(
            "mind:   pid={:?} alive={}",
            state.mind_pid,
            app.alive.mind
        )));
    } else {
        lines.push(Line::from("status: down"));
    }

    let block = Block::default().borders(Borders::ALL).title("Processes");
    let para = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
    f.render_widget(para, area);
}

fn render_vault(f: &mut ratatui::Frame, area: Rect, app: &App) {
    let mut lines = vec![Line::from(Span::styled("Vault", Style::default().add_modifier(Modifier::BOLD)))];
    if let Some(v) = &app.vault_info {
        lines.push(Line::from(format!("status: {}", state_name(v.status))));
        lines.push(Line::from(format!("energy: {}/{}", v.energy_used, v.energy_quota)));
        lines.push(Line::from(format!("last_command_id: {}", v.last_command_id)));
        lines.push(Line::from(format!("last_error: {}", v.last_error)));
        lines.push(Line::from(format!("response: {}", v.response)));
    } else {
        lines.push(Line::from("vault unavailable"));
    }

    let block = Block::default().borders(Borders::ALL).title("Vault");
    let para = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
    f.render_widget(para, area);
}

fn render_logs(f: &mut ratatui::Frame, area: Rect, app: &App) {
    let title = format!("Logs ({}) [l to switch]", LOG_COMPONENTS[app.log_component]);
    let lines: Vec<Line> = if app.log_lines.is_empty() {
        vec![Line::from("no logs")]
    } else {
        app.log_lines.iter().map(|l| Line::from(l.clone())).collect()
    };
    let block = Block::default().borders(Borders::ALL).title(title);
    let para = Paragraph::new(lines).block(block).wrap(Wrap { trim: false });
    f.render_widget(para, area);
}

fn render_chat(f: &mut ratatui::Frame, area: Rect, app: &App) {
    let mut lines: Vec<Line> = Vec::new();
    for (role, msg) in &app.messages {
        let label = if role == "user" { "you" } else { "ai" };
        lines.push(Line::from(Span::styled(
            format!("{}:", label),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(msg.clone()));
        lines.push(Line::from(""));
    }

    let block = Block::default().borders(Borders::ALL).title("Chat");
    let para = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
    f.render_widget(para, area);
}

fn render_prompt(f: &mut ratatui::Frame, area: Rect, app: &App) {
    let hint = if app.menu_index == 4 {
        match app.mode {
            AppMode::ChatInput => "prompt (esc to exit)",
            AppMode::Normal => "press i to chat",
        }
    } else {
        ""
    };
    let text = format!("> {}", app.input);
    let block = Block::default().borders(Borders::ALL).title(hint);
    let para = Paragraph::new(text).block(block);
    f.render_widget(para, area);
}

fn state_name(state: u32) -> &'static str {
    match state {
        crate::shared::constants::YAI_STATE_HALT => "HALT",
        crate::shared::constants::YAI_STATE_PREBOOT => "PREBOOT",
        crate::shared::constants::YAI_STATE_READY => "READY",
        crate::shared::constants::YAI_STATE_HANDOFF_COMPLETE => "HANDOFF",
        crate::shared::constants::YAI_STATE_RUNNING => "RUNNING",
        crate::shared::constants::YAI_STATE_SUSPENDED => "SUSPENDED",
        crate::shared::constants::YAI_STATE_ERROR => "ERROR",
        _ => "UNKNOWN",
    }
}

fn c_string(bytes: &[u8]) -> String {
    let nul = bytes.iter().position(|b| *b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..nul]).to_string()
}

fn tail_last_kb(path: &std::path::Path, kb: u64, max_lines: usize) -> Vec<String> {
    let mut f = match File::open(path) {
        Ok(x) => x,
        Err(_) => return vec![],
    };
    let len = f.metadata().map(|m| m.len()).unwrap_or(0);
    let start = len.saturating_sub(kb * 1024);
    if f.seek(SeekFrom::Start(start)).is_err() {
        return vec![];
    }
    let mut buf = String::new();
    if f.read_to_string(&mut buf).is_err() {
        return vec![];
    }
    let mut lines: Vec<String> = buf.lines().map(|s| s.to_string()).collect();
    if lines.len() > max_lines {
        lines = lines.split_off(lines.len() - max_lines);
    }
    lines
}

fn read_manifest(cfg: &RuntimeConfig) -> Option<String> {
    let manifest = cfg
        .artifacts_root
        .join("yai-core")
        .join("dist")
        .join("MANIFEST.json");
    let data = std::fs::read_to_string(manifest).ok()?;
    let v: serde_json::Value = serde_json::from_str(&data).ok()?;
    let git_sha = v.get("git_sha")?.as_str()?.to_string();
    let build_time = v.get("build_time")?.as_str()?.to_string();
    Some(format!("{} {}", git_sha, build_time))
}

fn spawn_llm_worker(cfg: RuntimeConfig, ws: String, rx: Receiver<String>, tx: Sender<(String, String)>) {
    thread::spawn(move || {
        let client = build_llm_for_ws(&cfg, &ws);
        while let Ok(prompt) = rx.recv() {
            let resp = client.complete(&prompt).unwrap_or_else(|e| format!("error: {:?}", e));
            let _ = tx.send(("ai".to_string(), resp));
        }
    });
}

fn spawn_event_worker(cfg: RuntimeConfig, ws: String, tx: Sender<crate::rpc::protocol::Event>) {
    thread::spawn(move || loop {
        let sock = crate::control::workspace::control_socket_path(&cfg.run_dir, &ws);
        let stream = std::os::unix::net::UnixStream::connect(&sock);
        let mut stream = match stream {
            Ok(s) => s,
            Err(_) => {
                thread::sleep(Duration::from_millis(500));
                continue;
            }
        };
        let req = serde_json::to_string(&Request::EventsSubscribe).unwrap();
        let _ = stream.write_all(req.as_bytes());
        let _ = stream.write_all(b"\n");
        let _ = stream.flush();
        let mut reader = std::io::BufReader::new(stream);
        let mut line = String::new();
        loop {
            line.clear();
            let n = match reader.read_line(&mut line) {
                Ok(n) => n,
                Err(_) => break,
            };
            if n == 0 {
                break;
            }
            if let Ok(resp) = serde_json::from_str::<crate::rpc::protocol::Response>(&line) {
                if let crate::rpc::protocol::Response::Event { event } = resp {
                    let _ = tx.send(event);
                }
            }
        }
        thread::sleep(Duration::from_millis(200));
    });
}
