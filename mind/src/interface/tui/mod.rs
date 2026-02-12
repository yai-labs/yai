use crate::interface::config::RuntimeConfig;
use crate::interface::tui::actions::Action;
use crate::interface::tui::app::{AppState, FocusZone, ViewKind};
use crate::interface::tui::datasource::chat::ChatSource;
use crate::interface::tui::datasource::contracts::ContractsSource;
use crate::interface::tui::datasource::db::DbSource;
use crate::interface::tui::datasource::events::EventsSource;
use crate::interface::tui::datasource::graph::GraphSource;
use crate::interface::tui::datasource::logs::FileTailSource;
use crate::interface::tui::datasource::providers::ProvidersSource;
use crate::interface::tui::datasource::runtime::RuntimeSource;
use crate::interface::tui::datasource::{tick_all, DataSource};
use crate::interface::tui::keymap::KeyMap;
use crate::interface::tui::reducer::reduce;
use crate::interface::tui::theme::Theme;
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph, Wrap};
use ratatui::Terminal;
use std::io::{self, Stdout};
use std::time::{Duration, Instant};

pub mod app;
pub mod actions;
pub mod reducer;
pub mod datasource;
pub mod views;
pub mod widgets;
pub mod snapshot;
pub mod keymap;
pub mod theme;

pub fn run(ws: &str, cfg: &RuntimeConfig) -> Result<()> {
    let mut state = AppState::new(ws.to_string());
    state.graph.depth = 2;

    let mut sources: Vec<Box<dyn DataSource>> = vec![
        Box::new(RuntimeSource),
        Box::new(EventsSource),
        Box::new(GraphSource),
        Box::new(FileTailSource),
        Box::new(DbSource),
        Box::new(ProvidersSource),
        Box::new(ContractsSource),
        Box::new(ChatSource),
    ];

    let mut terminal = setup_terminal()?;
    let _restore = TerminalRestore;
    let theme = Theme::default();

    let tick_rate = Duration::from_millis(250);
    let mut last_tick = Instant::now();

    loop {
        if last_tick.elapsed() >= tick_rate {
            tick_all(cfg, &mut state, &mut sources);
            reduce(&mut state, Action::Tick);
            last_tick = Instant::now();
        }

        terminal.draw(|f| draw_ui(f, &state, &theme))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if !event::poll(timeout)? {
            continue;
        }
        let Event::Key(key) = event::read()? else {
            continue;
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }

        if state.palette.active {
            match key.code {
                KeyCode::Esc => reduce(&mut state, Action::TogglePalette),
                KeyCode::Backspace => {
                    let mut input = state.palette.input.clone();
                    input.pop();
                    reduce(&mut state, Action::PaletteInput(input));
                }
                KeyCode::Enter => {
                    let cmd = state.palette.input.clone();
                    reduce(&mut state, Action::RunCommand(cmd));
                }
                KeyCode::Char(ch) => {
                    let mut input = state.palette.input.clone();
                    input.push(ch);
                    reduce(&mut state, Action::PaletteInput(input));
                }
                _ => {}
            }
            continue;
        }

        if state.active_view == ViewKind::Chat && state.focus == FocusZone::Composer {
            match key.code {
                KeyCode::Esc => {
                    state.focus = FocusZone::Body;
                }
                KeyCode::Enter => {
                    if key.modifiers.contains(KeyModifiers::SHIFT) {
                        reduce(&mut state, Action::ChatInputNewline);
                    } else {
                        let msg = state.chat.input.clone();
                        reduce(&mut state, Action::ChatSend(msg));
                    }
                }
                KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    reduce(&mut state, Action::ChatClearInput);
                }
                KeyCode::Backspace => {
                    let mut input = state.chat.input.clone();
                    input.pop();
                    reduce(&mut state, Action::ChatSetInput(input));
                }
                KeyCode::Char(ch) => {
                    let mut input = state.chat.input.clone();
                    input.push(ch);
                    reduce(&mut state, Action::ChatSetInput(input));
                }
                _ => {}
            }
            continue;
        }

        match key.code {
            KeyCode::Char(KeyMap::QUIT) => break,
            KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                reduce(&mut state, Action::TogglePalette)
            }
            KeyCode::Tab => reduce(&mut state, Action::FocusNext),
            KeyCode::BackTab => prev_view(&mut state),
            KeyCode::Up => reduce(&mut state, Action::MoveUp),
            KeyCode::Down => reduce(&mut state, Action::MoveDown),
            KeyCode::Enter if !state.palette.active && state.active_view != ViewKind::Chat => {
                reduce(&mut state, Action::Select)
            }
            KeyCode::Char('o') => reduce(&mut state, Action::SwitchView(ViewKind::Overview)),
            KeyCode::Char(KeyMap::GRAPH) => reduce(&mut state, Action::SwitchView(ViewKind::Graph)),
            KeyCode::Char(KeyMap::EVENTS) if state.active_view != ViewKind::Chat => {
                reduce(&mut state, Action::SwitchView(ViewKind::Events))
            }
            KeyCode::Char(KeyMap::LOGS) => reduce(&mut state, Action::SwitchView(ViewKind::Logs)),
            KeyCode::Char(KeyMap::DB) => reduce(&mut state, Action::SwitchView(ViewKind::Db)),
            KeyCode::Char(KeyMap::PROVIDERS) => reduce(&mut state, Action::SwitchView(ViewKind::Providers)),
            KeyCode::Char(KeyMap::CONTRACTS) if state.active_view != ViewKind::Chat => {
                reduce(&mut state, Action::SwitchView(ViewKind::Contracts))
            }
            KeyCode::Char('h') => reduce(&mut state, Action::SwitchView(ViewKind::Chat)),
            KeyCode::Char(KeyMap::PALETTE) => reduce(&mut state, Action::TogglePalette),
            KeyCode::Char(KeyMap::SEARCH) => {
                if !state.palette.active {
                    reduce(&mut state, Action::TogglePalette);
                    reduce(&mut state, Action::PaletteInput("search ".to_string()));
                }
            }
            KeyCode::Char('r') => reduce(&mut state, Action::Refresh),
            KeyCode::Char('?') => reduce(&mut state, Action::ToggleHelp),
            KeyCode::Char('f') if state.active_view == ViewKind::Logs => {
                reduce(&mut state, Action::LogsToggleFollow)
            }
            KeyCode::Char('s') if state.active_view == ViewKind::Logs => {
                reduce(&mut state, Action::LogsCycleSource)
            }
            KeyCode::Char('n') if state.active_view == ViewKind::Graph => {
                reduce(&mut state, Action::GraphToggleDepth)
            }
            KeyCode::Char('a') if state.active_view == ViewKind::Graph => {
                reduce(&mut state, Action::GraphActivateSelected)
            }
            KeyCode::Char('C') if state.active_view == ViewKind::Chat => reduce(&mut state, Action::ChatApplyDraft),
            KeyCode::Char('x') if state.active_view == ViewKind::Chat => reduce(&mut state, Action::ChatDiscardDraft),
            KeyCode::Char('E') if state.active_view == ViewKind::Chat && !state.palette.active => {
                reduce(&mut state, Action::TogglePalette);
                reduce(&mut state, Action::PaletteInput("ask ".to_string()));
            }
            _ => {}
        }
    }

    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    Ok(Terminal::new(backend)?)
}

struct TerminalRestore;

impl Drop for TerminalRestore {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let mut stdout = io::stdout();
        let _ = stdout.execute(LeaveAlternateScreen);
    }
}

fn draw_ui(f: &mut ratatui::Frame, state: &AppState, theme: &Theme) {
    f.render_widget(Block::default().style(theme.root()), f.size());

    let root = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1), Constraint::Length(1)])
        .split(f.size());

    let provider = state
        .providers
        .selected
        .as_ref()
        .and_then(|id| state.providers.list.iter().find(|p| &p.id == id))
        .or_else(|| state.providers.list.first());
    let provider_label = provider.map(|p| p.id.as_str()).unwrap_or("none");
    let trust_label = provider
        .map(|p| p.trust_state.as_str())
        .unwrap_or_else(|| state.providers.trust.as_str());

    let top = Paragraph::new(Line::from(vec![
        Span::styled(format!(" WS:{} ", state.ws), theme.title()),
        Span::raw(" "),
        span_status("K", state.status.kernel_alive, theme),
        Span::raw(" "),
        span_status("E", state.status.engine_alive, theme),
        Span::raw(" "),
        span_status("M", state.status.mind_alive, theme),
        Span::raw(" "),
        span_awareness(state.status.awareness_active, theme),
        Span::raw(" "),
        Span::styled(
            format!(" provider:{} ", short(provider_label, 44)),
            Style::default().fg(theme.accent),
        ),
        Span::raw(" "),
        Span::styled(
            format!(" trust:{} ", trust_label),
            trust_style(trust_label, theme),
        ),
        Span::raw(" "),
        Span::styled(
            format!(" view:{} ", view_name(&state.active_view).to_ascii_lowercase()),
            theme.dim(),
        ),
    ]));
    f.render_widget(top, root[0]);

    match state.active_view {
        ViewKind::Chat => render_chat_cockpit(f, state, theme, root[1]),
        ViewKind::Graph => render_graph_cockpit(f, state, theme, root[1]),
        ViewKind::Events => render_events_cockpit(f, state, theme, root[1]),
        ViewKind::Logs => render_logs_cockpit(f, state, theme, root[1]),
        _ => {
            let content = Paragraph::new(current_view_text(state))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(theme.panel())
                        .border_style(theme.border())
                        .title(Span::styled(
                            format!(" {} ", view_name(&state.active_view)),
                            theme.title(),
                        )),
                )
                .wrap(Wrap { trim: false });
            f.render_widget(content, root[1]);
        }
    }

    let mode = if state.palette.active {
        "palette"
    } else if state.active_view == ViewKind::Chat && state.focus == FocusZone::Composer {
        "chat-compose"
    } else {
        "normal"
    };
    let last_cmd = state
        .palette
        .history
        .last()
        .cloned()
        .unwrap_or_else(|| "-".to_string());
    let last_err = if state.chat.last_error.is_empty() {
        "none".to_string()
    } else {
        state.chat.last_error.clone()
    };
    let status = Paragraph::new(format!(
        " {} | Ctrl+P palette | g/e/l/d/p/c/h views | Tab focus | / search | q quit | cmd:{} | err:{}",
        mode,
        short(&last_cmd, 24),
        short(&last_err, 42)
    ))
    .alignment(Alignment::Left)
    .style(theme.dim())
    .block(Block::default().borders(Borders::TOP).border_style(theme.border()));
    f.render_widget(status, root[2]);

    if state.palette.active {
        let area = centered_rect(70, 20, f.size());
        f.render_widget(Clear, area);
        let palette = Paragraph::new(format!(
            "Command Palette\n\n:{}\n\nExamples: ask <prompt> | apply | node <id> | search <term> | source <name>",
            state.palette.input
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(theme.panel_alt())
                .border_style(theme.border())
                .title(Span::styled("Command", theme.title())),
        );
        f.render_widget(palette, area);
    }

    if state.show_help {
        let area = centered_rect(80, 34, f.size());
        f.render_widget(Clear, area);
        let help = Paragraph::new(
            "Help\n\n\
Header: ws/provider/trust/awareness\n\
Body: view content\n\
Footer: keys\n\n\
Pattern: ↑/↓ move, Enter select, / search, : or Ctrl+P palette, Tab focus, q quit\n\
Graph: n depth, a activate\n\
Logs: s source, f follow\n\
Chat: composer Enter send, Shift+Enter newline, Ctrl+U clear, C commit, x discard, E edit/retry\n",
        )
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(theme.panel_alt())
                .border_style(theme.border())
                .title(Span::styled("Legend", theme.title())),
        );
        f.render_widget(help, area);
    }
}

fn span_status(name: &str, alive: bool, theme: &Theme) -> Span<'static> {
    if alive {
        Span::styled(format!("{name}:up"), theme.badge_ok())
    } else {
        Span::styled(format!("{name}:down"), theme.badge_danger())
    }
}

fn span_awareness(active: bool, theme: &Theme) -> Span<'static> {
    if active {
        Span::styled("awareness:on", theme.badge_ok())
    } else {
        Span::styled("awareness:off", theme.badge_warn())
    }
}

fn trust_style(trust: &str, theme: &Theme) -> Style {
    match trust {
        "trusted" | "attached" => theme.badge_ok(),
        "revoked" => theme.badge_danger(),
        _ => theme.badge_warn(),
    }
}

fn render_chat_cockpit(f: &mut ratatui::Frame, state: &AppState, theme: &Theme, area: Rect) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(74), Constraint::Percentage(26)])
        .split(area);
    let left = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(10), Constraint::Length(5), Constraint::Length(2)])
        .split(columns[0]);

    let inline = Paragraph::new(Line::from(vec![
        Span::styled(" CHAT ", theme.title()),
        Span::raw(" "),
        Span::styled(
            format!(
                "agent={} command={} stream={} state={:?}",
                state.chat.last_agent.as_deref().unwrap_or("n/a"),
                state.chat.last_command.as_deref().unwrap_or("n/a"),
                state.chat.streaming_enabled,
                state.chat.request_state
            ),
            theme.dim(),
        ),
    ]));
    f.render_widget(inline, left[0]);

    let transcript = chat_transcript_lines(state, theme);
    let transcript_widget = Paragraph::new(transcript)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .style(theme.panel())
                .border_style(theme.border())
                .title(Span::styled(" Transcript ", theme.title())),
        )
        .wrap(Wrap { trim: false });
    f.render_widget(transcript_widget, left[1]);

    let composer_title = if state.focus == FocusZone::Composer {
        " Composer (active) "
    } else {
        " Composer "
    };
    let composer_widget = Paragraph::new(if state.chat.input.is_empty() {
        "<type prompt here>".to_string()
    } else {
        state.chat.input.clone()
    })
    .style(if state.chat.input.is_empty() {
        theme.dim()
    } else {
        Style::default().fg(theme.fg)
    })
    .block(
        Block::default()
            .borders(Borders::TOP)
            .style(theme.panel())
            .border_style(if state.focus == FocusZone::Composer {
                theme.border_focused()
            } else {
                theme.border()
            })
            .title(Span::styled(composer_title, theme.title())),
    )
    .wrap(Wrap { trim: false });
    f.render_widget(composer_widget, left[2]);

    let action_widget = Paragraph::new(vec![
        Line::from("Enter send | Shift+Enter newline | Ctrl+U clear"),
        Line::from("Ctrl+P palette | C commit | x discard | s stream"),
        Line::from(format!("commit_target={:?}", state.chat.commit_target)),
    ])
    .style(theme.dim())
    .block(
        Block::default()
            .borders(Borders::TOP)
            .style(theme.panel())
            .border_style(theme.border())
            .title(Span::styled(" Actions ", theme.title())),
    );
    f.render_widget(action_widget, left[3]);

    let selected_provider = state
        .providers
        .selected
        .as_ref()
        .and_then(|id| state.providers.list.iter().find(|p| &p.id == id))
        .or_else(|| state.providers.list.first());
    let provider_id = selected_provider
        .map(|p| p.id.as_str())
        .unwrap_or("none");
    let provider_model = selected_provider
        .map(|p| p.model.as_str())
        .unwrap_or("unknown");
    let provider_trust = selected_provider
        .map(|p| p.trust_state.as_str())
        .unwrap_or("unknown");
    let selected_node = state
        .graph
        .selected_node
        .as_deref()
        .map(|s| short(s, 34))
        .unwrap_or_else(|| "none".to_string());
    let subgraph = state
        .graph
        .last_subgraph
        .as_ref()
        .map(|s| format!("{}n/{}e", s.nodes, s.edges))
        .unwrap_or_else(|| "n/a".to_string());
    let activation = state
        .graph
        .last_activation
        .as_ref()
        .map(|a| format!("{}n/{}e", a.nodes, a.edges))
        .unwrap_or_else(|| "n/a".to_string());
    let side = Paragraph::new(vec![
        Line::from(Span::styled(" SESSION", theme.title())),
        Line::from(""),
        Line::from(format!("ws: {}", state.ws)),
        Line::from(format!("provider: {}", short(provider_id, 34))),
        Line::from(format!("trust: {}", provider_trust)),
        Line::from(format!("model: {}", short(provider_model, 34))),
        Line::from(format!(
            "agent: {}",
            state.chat.last_agent.as_deref().unwrap_or("n/a")
        )),
        Line::from(format!(
            "command: {}",
            state.chat.last_command.as_deref().unwrap_or("n/a")
        )),
        Line::from(format!("streaming: {}", state.chat.streaming_enabled)),
        Line::from(format!("state: {:?}", state.chat.request_state)),
        Line::from(""),
        Line::from("Graph live:"),
        Line::from(format!("nodes/edges: {}/{}", state.graph.stats_nodes, state.graph.stats_edges)),
        Line::from(format!("selected: {}", selected_node)),
        Line::from(format!("neighbors: {}", subgraph)),
        Line::from(format!("activation: {}", activation)),
        Line::from(format!("backend: {}", short(&state.graph.backend, 34))),
        Line::from(""),
        Line::from("Context preview:"),
        Line::from(short(&state.chat.context_preview, 34)),
        Line::from(""),
        Line::from("Last error:"),
        Line::from(short(&state.chat.last_error, 34)),
    ])
    .wrap(Wrap { trim: false })
    .block(
        Block::default()
            .borders(Borders::LEFT)
            .style(theme.panel_alt())
            .border_style(theme.border())
            .title(Span::styled(" ", theme.title())),
    );
    f.render_widget(side, columns[1]);
}

fn render_graph_cockpit(f: &mut ratatui::Frame, state: &AppState, theme: &Theme, area: Rect) {
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(36), Constraint::Percentage(64)])
        .split(area);
    let right = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(52), Constraint::Percentage(48)])
        .split(columns[1]);

    let mut node_lines: Vec<Line> = Vec::new();
    if state.graph.node_list.is_empty() {
        node_lines.push(Line::from(Span::styled("(no nodes)", theme.dim())));
    } else {
        for (i, id) in state.graph.node_list.iter().take(30).enumerate() {
            let marker = if i == state.graph.selected_index { ">" } else { " " };
            let style = if i == state.graph.selected_index {
                theme.selected()
            } else {
                Style::default().fg(theme.fg)
            };
            node_lines.push(Line::from(Span::styled(format!("{marker} {id}"), style)));
        }
    }
    let nodes = Paragraph::new(node_lines)
        .block(
            Block::default()
                .borders(Borders::RIGHT)
                .style(theme.panel())
                .border_style(theme.border())
                .title(Span::styled(" Nodes ", theme.title())),
        )
        .wrap(Wrap { trim: false });
    f.render_widget(nodes, columns[0]);

    let details = Paragraph::new(vec![
        Line::from(format!("backend: {}", short(&state.graph.backend, 72))),
        Line::from(format!("nodes/edges: {}/{}", state.graph.stats_nodes, state.graph.stats_edges)),
        Line::from(format!("depth: {}", state.graph.depth.max(1))),
        Line::from(format!(
            "selected: {}",
            state.graph.selected_node.as_deref().unwrap_or("none")
        )),
        Line::from(format!(
            "kind: {}",
            state
                .graph
                .selected_node_kind
                .as_deref()
                .unwrap_or("n/a")
        )),
        Line::from(format!("last_seen: {}", state.graph.selected_node_last_seen)),
        Line::from(""),
        Line::from("meta:"),
        Line::from(short(&state.graph.selected_node_meta.to_string(), 72)),
    ])
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .style(theme.panel())
            .border_style(theme.border())
            .title(Span::styled(" Details ", theme.title())),
    )
    .wrap(Wrap { trim: false });
    f.render_widget(details, right[0]);

    let mut lower_lines: Vec<Line> = vec![
        Line::from(Span::styled("neighbors", theme.title())),
    ];
    if state.graph.neighbors_preview.is_empty() {
        lower_lines.push(Line::from(Span::styled("  (empty)", theme.dim())));
    } else {
        for n in state.graph.neighbors_preview.iter().take(8) {
            lower_lines.push(Line::from(format!("  {n}")));
        }
    }
    lower_lines.push(Line::from(""));
    lower_lines.push(Line::from(Span::styled("activation", theme.title())));
    if state.graph.activation_top.is_empty() {
        lower_lines.push(Line::from(Span::styled("  (run: a)", theme.dim())));
    } else {
        for n in state.graph.activation_top.iter().take(8) {
            lower_lines.push(Line::from(format!("  {n}")));
        }
    }
    lower_lines.push(Line::from(""));
    lower_lines.push(Line::from(Span::styled(
        "keys: ↑/↓ node | n depth | a activate | : node <id>",
        theme.dim(),
    )));

    let lower = Paragraph::new(lower_lines)
        .block(
            Block::default()
                .borders(Borders::NONE)
                .style(theme.panel())
                .border_style(theme.border())
                .title(Span::styled(" Explore ", theme.title())),
        )
        .wrap(Wrap { trim: false });
    f.render_widget(lower, right[1]);
}

fn render_events_cockpit(f: &mut ratatui::Frame, state: &AppState, theme: &Theme, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)])
        .split(area);

    let header = Paragraph::new(vec![
        Line::from(format!(
            "buffered={}  selected={}  expanded={}",
            state.events.items.len(),
            state.events.selected,
            state.events.expanded
        )),
        Line::from(Span::styled(
            "keys: ↑/↓ select | Enter expand/collapse",
            theme.dim(),
        )),
    ])
    .block(
        Block::default()
            .borders(Borders::TOP)
            .style(theme.panel())
            .border_style(theme.border())
            .title(Span::styled(" Event Stream ", theme.title())),
    );
    f.render_widget(header, chunks[0]);

    let mut lines: Vec<Line> = Vec::new();
    if state.events.items.is_empty() {
        lines.push(Line::from(Span::styled("no events yet", theme.dim())));
    } else {
        for (i, line) in state.events.items.iter().rev().take(120).rev().enumerate() {
            let marker = if i == state.events.selected { ">" } else { " " };
            let is_error = line.contains("error") || line.contains("ERROR");
            let is_warn = line.contains("warn") || line.contains("WARN");
            let style = if i == state.events.selected {
                theme.selected()
            } else if is_error {
                theme.badge_danger()
            } else if is_warn {
                theme.badge_warn()
            } else {
                Style::default().fg(theme.fg)
            };
            let text = if state.events.expanded || i == state.events.selected {
                line.to_string()
            } else {
                short(line, 180)
            };
            lines.push(Line::from(Span::styled(format!("{marker} {text}"), style)));
        }
    }
    let body = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::NONE)
                .style(theme.panel())
                .border_style(theme.border())
                .title(Span::styled(" Timeline ", theme.title())),
        )
        .wrap(Wrap { trim: false });
    f.render_widget(body, chunks[1]);
}

fn render_logs_cockpit(f: &mut ratatui::Frame, state: &AppState, theme: &Theme, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(10)])
        .split(area);
    let header = Paragraph::new(vec![
        Line::from(format!(
            "source={} lines={} follow={} search='{}'",
            state.logs.source_selected, state.logs.lines, state.logs.follow, state.logs.search_term
        )),
        Line::from(Span::styled(
            "keys: s source | f follow | / search | r refresh",
            theme.dim(),
        )),
    ])
    .block(
        Block::default()
            .borders(Borders::TOP)
            .style(theme.panel())
            .border_style(theme.border())
            .title(Span::styled(" Logs ", theme.title())),
    );
    f.render_widget(header, chunks[0]);

    let mut lines: Vec<Line> = Vec::new();
    if state.logs.tail_buffer.is_empty() {
        lines.push(Line::from(Span::styled(
            format!("no logs for source={}", state.logs.source_selected),
            theme.dim(),
        )));
    } else {
        for (i, line) in state.logs.tail_buffer.iter().rev().take(120).rev().enumerate() {
            let marker = if i == state.logs.selected { ">" } else { " " };
            let style = if i == state.logs.selected {
                theme.selected()
            } else if line.contains("ERROR") || line.contains("error") {
                theme.badge_danger()
            } else if line.contains("WARN") || line.contains("warn") {
                theme.badge_warn()
            } else {
                Style::default().fg(theme.fg)
            };
            lines.push(Line::from(Span::styled(
                format!("{marker} {}", short(line, 190)),
                style,
            )));
        }
    }
    let body = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::NONE)
                .style(theme.panel())
                .border_style(theme.border())
                .title(Span::styled(" Tail ", theme.title())),
        )
        .wrap(Wrap { trim: false });
    f.render_widget(body, chunks[1]);
}

fn chat_transcript_lines(state: &AppState, theme: &Theme) -> Vec<Line<'static>> {
    if state.chat.transcript.is_empty() {
        return vec![Line::from(Span::styled("(empty)", theme.dim()))];
    }
    let start = state.chat.scroll.min(state.chat.transcript.len());
    let tail = &state.chat.transcript[start..];
    let mut lines = Vec::new();
    for m in tail
        .iter()
        .rev()
        .take(16)
        .rev()
    {
        let role_style = match m.role.as_str() {
            "USER" => Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
            "AGENT" => Style::default().fg(theme.ok).add_modifier(Modifier::BOLD),
            "SYSTEM" => Style::default().fg(theme.warn).add_modifier(Modifier::BOLD),
            _ => theme.dim(),
        };
        let status_style = match format!("{:?}", m.status).as_str() {
            "Committed" => theme.badge_ok(),
            "Discarded" => theme.badge_danger(),
            "Draft" => theme.badge_warn(),
            _ => theme.dim(),
        };
        lines.push(Line::from(vec![
            Span::styled(format!(" {} ", m.role), role_style),
            Span::raw(" "),
            Span::styled(format!("{:?}", m.status), status_style),
        ]));
        lines.push(Line::from(Span::styled(
            short(&m.text, 180),
            Style::default().fg(theme.fg),
        )));
        lines.push(Line::from(Span::raw("")));
    }
    lines
}

fn short(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    let keep = max.saturating_sub(3);
    format!("{}...", &s[..keep])
}

fn current_view_text(state: &AppState) -> String {
    match state.active_view {
        ViewKind::Overview => views::overview::render(state),
        ViewKind::Graph => views::graph::render(state),
        ViewKind::Events => views::events::render(state),
        ViewKind::Logs => views::logs::render(state),
        ViewKind::Db => views::db::render(state),
        ViewKind::Providers => views::providers::render(state),
        ViewKind::Contracts => views::contracts::render(state),
        ViewKind::Chat => views::chat::render(state),
    }
}

fn view_order() -> &'static [ViewKind] {
    const ORDER: [ViewKind; 8] = [
        ViewKind::Overview,
        ViewKind::Graph,
        ViewKind::Events,
        ViewKind::Logs,
        ViewKind::Db,
        ViewKind::Providers,
        ViewKind::Contracts,
        ViewKind::Chat,
    ];
    &ORDER
}

fn view_name(view: &ViewKind) -> &'static str {
    match view {
        ViewKind::Overview => "Overview",
        ViewKind::Graph => "Graph",
        ViewKind::Events => "Events",
        ViewKind::Logs => "Logs",
        ViewKind::Db => "DB",
        ViewKind::Providers => "Providers",
        ViewKind::Contracts => "Contracts",
        ViewKind::Chat => "Chat",
    }
}

fn prev_view(state: &mut AppState) {
    let order = view_order();
    let current = order.iter().position(|v| *v == state.active_view).unwrap_or(0);
    let prev = if current == 0 { order.len() - 1 } else { current - 1 };
    reduce(state, Action::SwitchView(order[prev].clone()));
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
