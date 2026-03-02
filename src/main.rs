use std::{
    env,
    error::Error,
    fs,
    io,
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum ChallengeStatus {
    Todo,
    Doing,
    Done,
}

impl ChallengeStatus {
    fn badge(self) -> &'static str {
        match self {
            ChallengeStatus::Todo => "TODO",
            ChallengeStatus::Doing => "DOING",
            ChallengeStatus::Done => "DONE",
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Challenge {
    name: String,
    category: String,
    difficulty: String,
    #[serde(default = "default_status")]
    status: ChallengeStatus,
    description: String,
    workdir: String,
}

fn default_status() -> ChallengeStatus {
    ChallengeStatus::Todo
}

#[derive(Debug, Deserialize, Serialize)]
struct ChallengeFile {
    challenges: Vec<Challenge>,
}

#[derive(Debug)]
struct App {
    challenges: Vec<Challenge>,
    selected: usize,
    status_message: String,
    workspace_root: PathBuf,
    config_path: Option<PathBuf>,
    show_logs: bool,
    log_lines: Vec<String>,
    log_scroll: u16,
}

impl App {
    fn new() -> Self {
        let workspace_root = detect_workspace_root();
        let (challenges, config_path) = load_challenges_with_fallback(&workspace_root);
        Self {
            challenges,
            selected: 0,
            status_message: format!("Ready. Root: {}", workspace_root.display()),
            workspace_root,
            config_path,
            show_logs: false,
            log_lines: vec!["Press l to load docker logs".to_string()],
            log_scroll: 0,
        }
    }

    fn selected_challenge(&self) -> Option<&Challenge> {
        self.challenges.get(self.selected)
    }

    fn challenge_workdir_path(&self, challenge: &Challenge) -> PathBuf {
        let p = Path::new(&challenge.workdir);
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            self.workspace_root.join(p)
        }
    }

    fn next(&mut self) {
        if self.challenges.is_empty() {
            return;
        }
        self.selected = (self.selected + 1) % self.challenges.len();
    }

    fn prev(&mut self) {
        if self.challenges.is_empty() {
            return;
        }
        self.selected = if self.selected == 0 {
            self.challenges.len() - 1
        } else {
            self.selected - 1
        };
    }

    fn run_docker_action(&self, args: &[&str]) -> String {
        let Some(challenge) = self.selected_challenge() else {
            return "No challenge selected".to_string();
        };

        let workdir = self.challenge_workdir_path(challenge);
        if !has_compose_file(&workdir) {
            return format!("❌ {} | compose file not found in {}", challenge.name, workdir.display());
        }

        let output = Command::new("docker")
            .args(["compose"])
            .args(args)
            .current_dir(&workdir)
            .output();

        match output {
            Ok(out) if out.status.success() => {
                format!("✅ docker compose {} ({})", args.join(" "), challenge.name)
            }
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                let brief = stderr.lines().next().unwrap_or("command failed");
                format!("❌ {} | {}", challenge.name, brief)
            }
            Err(e) => format!("❌ {} | {}", challenge.name, e),
        }
    }

    fn refresh_logs(&mut self) {
        let Some(challenge) = self.selected_challenge() else {
            self.log_lines = vec!["No challenge selected".to_string()];
            return;
        };

        let workdir = self.challenge_workdir_path(challenge);
        let output = Command::new("docker")
            .args(["compose", "logs", "--tail", "120", "--no-color"])
            .current_dir(&workdir)
            .output();

        match output {
            Ok(out) if out.status.success() => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let mut lines: Vec<String> = stdout.lines().map(|s| s.to_string()).collect();
                if lines.is_empty() {
                    lines.push("(no logs yet)".to_string());
                }
                self.log_lines = lines;
                self.log_scroll = self.log_lines.len().saturating_sub(1) as u16;
                self.status_message = format!("Logs loaded ({})", challenge.name);
            }
            Ok(out) => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                self.log_lines = stderr
                    .lines()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>();
                if self.log_lines.is_empty() {
                    self.log_lines = vec!["failed to fetch logs".to_string()];
                }
                self.log_scroll = 0;
                self.status_message = format!("❌ logs failed ({})", challenge.name);
            }
            Err(e) => {
                self.log_lines = vec![format!("failed to execute docker compose logs: {e}")];
                self.log_scroll = 0;
                self.status_message = format!("❌ logs failed ({})", challenge.name);
            }
        }
    }

    fn generate_writeup(&self) -> String {
        let Some(challenge) = self.selected_challenge() else {
            return "No challenge selected".to_string();
        };

        let dir = PathBuf::from("writeups");
        if let Err(e) = fs::create_dir_all(&dir) {
            return format!("❌ create writeups dir failed: {e}");
        }

        let path = dir.join(format!("{}.md", challenge.name));
        let tpl = format!(
            "# {name}\n\n## Basic Info\n- Category: {category}\n- Difficulty: {difficulty}\n- Workdir: {workdir}\n\n## Environment\n- Docker compose command:\n  - up: `docker compose up -d`\n  - down: `docker compose down`\n\n## Analysis\n-\n\n## Exploit / Solution\n-\n\n## Reproduction Steps\n1.\n2.\n3.\n\n## Pitfalls\n-\n",
            name = challenge.name,
            category = challenge.category,
            difficulty = challenge.difficulty,
            workdir = challenge.workdir
        );

        match fs::write(&path, tpl) {
            Ok(_) => format!("📝 writeup generated: {}", path.display()),
            Err(e) => format!("❌ writeup failed: {e}"),
        }
    }

    fn cycle_status(&mut self) {
        let Some(ch) = self.challenges.get_mut(self.selected) else {
            self.status_message = "No challenge selected".to_string();
            return;
        };

        ch.status = match ch.status {
            ChallengeStatus::Todo => ChallengeStatus::Doing,
            ChallengeStatus::Doing => ChallengeStatus::Done,
            ChallengeStatus::Done => ChallengeStatus::Todo,
        };

        self.status_message = format!("Status -> {} ({})", ch.status.badge(), ch.name);
        if let Err(e) = self.save_challenges() {
            self.status_message = format!("{} | save failed: {}", self.status_message, e);
        }
    }

    fn save_challenges(&self) -> Result<(), Box<dyn Error>> {
        let Some(path) = &self.config_path else {
            return Ok(());
        };
        let payload = ChallengeFile {
            challenges: self.challenges.clone(),
        };
        let text = toml::to_string_pretty(&payload)?;
        fs::write(path, text)?;
        Ok(())
    }

    fn open_shell(&mut self) {
        let Some(challenge) = self.selected_challenge() else {
            self.status_message = "No challenge selected".to_string();
            return;
        };

        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen);

        let shell = std::env::var("SHELL").unwrap_or_else(|_| "bash".to_string());
        let workdir = self.challenge_workdir_path(challenge);
        let result = Command::new(shell).current_dir(&workdir).status();

        let _ = execute!(io::stdout(), EnterAlternateScreen);
        let _ = enable_raw_mode();

        self.status_message = match result {
            Ok(st) if st.success() => format!("Shell exited ({})", challenge.name),
            Ok(_) => format!("Shell exited with non-zero code ({})", challenge.name),
            Err(e) => format!("Open shell failed: {}", e),
        };
    }

    fn logs_scroll_down(&mut self) {
        let max = self.log_lines.len().saturating_sub(1) as u16;
        self.log_scroll = self.log_scroll.saturating_add(1).min(max);
    }

    fn logs_scroll_up(&mut self) {
        self.log_scroll = self.log_scroll.saturating_sub(1);
    }

    fn reload_challenges(&mut self) {
        let (challenges, config_path) = load_challenges_with_fallback(&self.workspace_root);
        self.challenges = challenges;
        self.config_path = config_path;
        self.selected = 0;
        self.status_message = "Challenges reloaded".to_string();
    }

    fn on_key(&mut self, code: KeyCode) -> bool {
        if self.show_logs {
            match code {
                KeyCode::Esc | KeyCode::Char('l') => {
                    self.show_logs = false;
                    self.status_message = "Close logs panel".to_string();
                }
                KeyCode::Char('r') => self.refresh_logs(),
                KeyCode::Char('j') | KeyCode::Down => self.logs_scroll_down(),
                KeyCode::Char('k') | KeyCode::Up => self.logs_scroll_up(),
                KeyCode::Char('q') => return false,
                _ => {}
            }
            return true;
        }

        match code {
            KeyCode::Char('q') => return false,
            KeyCode::Char('j') | KeyCode::Down => self.next(),
            KeyCode::Char('k') | KeyCode::Up => self.prev(),
            KeyCode::Char('t') => self.cycle_status(),
            KeyCode::Char('r') => self.reload_challenges(),
            KeyCode::Enter => self.status_message = "Open challenge details (next step).".to_string(),
            KeyCode::Char('u') => self.status_message = self.run_docker_action(&["up", "-d"]),
            KeyCode::Char('d') => self.status_message = self.run_docker_action(&["down"]),
            KeyCode::Char('l') => {
                self.show_logs = true;
                self.refresh_logs();
            }
            KeyCode::Char('s') => self.open_shell(),
            KeyCode::Char('w') => self.status_message = self.generate_writeup(),
            _ => {}
        }
        true
    }
}

fn detect_workspace_root() -> PathBuf {
    let mut cur = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    loop {
        let has_toml = cur.join("challenges.toml").exists();
        let has_challenges_dir = cur.join("challenges").is_dir();
        if has_toml || has_challenges_dir {
            return cur;
        }
        if !cur.pop() {
            return env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        }
    }
}

fn load_challenges_with_fallback(root: &Path) -> (Vec<Challenge>, Option<PathBuf>) {
    if let Ok(v) = load_challenges_from_toml(root) {
        return v;
    }

    let discovered = discover_challenges_from_fs(root);
    if !discovered.is_empty() {
        return (discovered, None);
    }

    (default_challenges(), None)
}

fn load_challenges_from_toml(root: &Path) -> Result<(Vec<Challenge>, Option<PathBuf>), Box<dyn Error>> {
    let path = root.join("challenges.toml");
    if !path.exists() {
        return Err("challenges.toml not found".into());
    }

    let content = fs::read_to_string(&path)?;
    let parsed: ChallengeFile = toml::from_str(&content)?;
    if parsed.challenges.is_empty() {
        return Err("no challenges in config".into());
    }
    Ok((parsed.challenges, Some(path)))
}

fn discover_challenges_from_fs(root: &Path) -> Vec<Challenge> {
    let mut out = Vec::new();
    let base = root.join("challenges");
    let Ok(entries) = fs::read_dir(base) else {
        return out;
    };

    for entry in entries.flatten() {
        let p = entry.path();
        if !p.is_dir() {
            continue;
        }

        let docker_dir = p.join("docker");
        if !docker_dir.exists() {
            continue;
        }

        let has_compose = ["docker-compose.yml", "docker-compose.yaml", "compose.yml", "compose.yaml"]
            .iter()
            .any(|f| docker_dir.join(f).exists());
        if !has_compose {
            continue;
        }

        let name = p.file_name().unwrap_or_default().to_string_lossy().to_string();
        out.push(Challenge {
            name,
            category: "Unknown".into(),
            difficulty: "Unknown".into(),
            status: ChallengeStatus::Todo,
            description: "Auto-discovered challenge (edit challenges.toml for details).".into(),
            workdir: docker_dir.display().to_string(),
        });
    }

    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

fn has_compose_file(workdir: &Path) -> bool {
    ["docker-compose.yml", "docker-compose.yaml", "compose.yml", "compose.yaml"]
        .iter()
        .any(|f| workdir.join(f).exists())
}

fn default_challenges() -> Vec<Challenge> {
    vec![
        Challenge {
            name: "rsa-baby".into(),
            category: "Crypto".into(),
            difficulty: "Easy".into(),
            status: ChallengeStatus::Todo,
            description: "Recover plaintext using weak RSA key setup.".into(),
            workdir: "./challenges/rsa-baby/docker".into(),
        },
        Challenge {
            name: "fmt-lab".into(),
            category: "Pwn".into(),
            difficulty: "Medium".into(),
            status: ChallengeStatus::Doing,
            description: "Practice format string leak and arbitrary write.".into(),
            workdir: "./challenges/fmt-lab/docker".into(),
        },
    ]
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(120))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && !app.on_key(key.code) {
                    break;
                }
            }
        }
    }

    Ok(())
}

fn ui(f: &mut Frame, app: &App) {
    if app.show_logs {
        let logs = app
            .log_lines
            .iter()
            .map(|l| Line::raw(l.as_str()))
            .collect::<Vec<_>>();

        let panel = Paragraph::new(logs)
            .block(
                Block::default()
                    .title(" Logs (j/k scroll, r refresh, ESC/l close) ")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Yellow)),
            )
            .scroll((app.log_scroll, 0))
            .wrap(Wrap { trim: false });

        f.render_widget(panel, f.area());
        return;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(3)])
        .split(f.area());

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(chunks[0]);

    let items: Vec<ListItem> = app
        .challenges
        .iter()
        .enumerate()
        .map(|(idx, c)| {
            let selected = idx == app.selected;
            let prefix = if selected { ">" } else { " " };
            let line = Line::from(vec![
                Span::raw(format!("{} {}", prefix, c.name)),
                Span::styled(
                    format!(" [{}]", c.status.badge()),
                    Style::default().fg(match c.status {
                        ChallengeStatus::Todo => Color::Yellow,
                        ChallengeStatus::Doing => Color::Blue,
                        ChallengeStatus::Done => Color::Green,
                    }),
                ),
            ]);
            ListItem::new(line)
        })
        .collect();

    let list = List::new(items).block(
        Block::default()
            .title(" Challenges ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
    );

    f.render_widget(list, main_chunks[0]);

    let detail = if let Some(c) = app.selected_challenge() {
        vec![
            Line::from(vec![
                Span::styled("Name: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&c.name),
            ]),
            Line::from(vec![
                Span::styled("Category: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&c.category),
            ]),
            Line::from(vec![
                Span::styled("Difficulty: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&c.difficulty),
            ]),
            Line::from(vec![
                Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(c.status.badge()),
            ]),
            Line::from(vec![
                Span::styled("Workdir: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(&c.workdir),
            ]),
            Line::from(vec![
                Span::styled("Compose: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(if has_compose_file(&app.challenge_workdir_path(c)) {
                    "found"
                } else {
                    "missing"
                }),
            ]),
            Line::raw(""),
            Line::styled("Description", Style::default().add_modifier(Modifier::BOLD)),
            Line::raw(&c.description),
            Line::raw(""),
            Line::styled("Actions", Style::default().add_modifier(Modifier::BOLD)),
            Line::raw("u: up | d: down | l: logs-panel | s: shell | w: writeup | t: cycle status | r: reload"),
        ]
    } else {
        vec![Line::raw("No challenge loaded.")]
    };

    let detail_panel = Paragraph::new(detail)
        .block(
            Block::default()
                .title(" Details ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Magenta)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(detail_panel, main_chunks[1]);

    let footer = Paragraph::new(format!(
        "[j/k] move [u/d/l/s/w/t/r] actions [q] quit | {}",
        app.status_message
    ))
    .block(
        Block::default()
            .title(" Keymap ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::DarkGray)),
    );

    f.render_widget(footer, chunks[1]);
}
