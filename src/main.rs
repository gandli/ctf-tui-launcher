use std::{
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
    config_path: Option<PathBuf>,
}

impl App {
    fn new() -> Self {
        let (challenges, config_path) = match load_challenges() {
            Ok(v) => v,
            Err(_) => (default_challenges(), None),
        };
        Self {
            challenges,
            selected: 0,
            status_message: "Ready. Press q to quit.".to_string(),
            config_path,
        }
    }

    fn selected_challenge(&self) -> Option<&Challenge> {
        self.challenges.get(self.selected)
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

        let output = Command::new("docker")
            .args(["compose"])
            .args(args)
            .current_dir(&challenge.workdir)
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
        let result = Command::new(shell).current_dir(&challenge.workdir).status();

        let _ = execute!(io::stdout(), EnterAlternateScreen);
        let _ = enable_raw_mode();

        self.status_message = match result {
            Ok(st) if st.success() => format!("Shell exited ({})", challenge.name),
            Ok(_) => format!("Shell exited with non-zero code ({})", challenge.name),
            Err(e) => format!("Open shell failed: {}", e),
        };
    }

    fn on_key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Char('q') => return false,
            KeyCode::Char('j') | KeyCode::Down => self.next(),
            KeyCode::Char('k') | KeyCode::Up => self.prev(),
            KeyCode::Char('t') => self.cycle_status(),
            KeyCode::Enter => self.status_message = "Open challenge details (next step).".to_string(),
            KeyCode::Char('u') => self.status_message = self.run_docker_action(&["up", "-d"]),
            KeyCode::Char('d') => self.status_message = self.run_docker_action(&["down"]),
            KeyCode::Char('l') => self.status_message = self.run_docker_action(&["logs", "--tail", "60"]),
            KeyCode::Char('s') => self.open_shell(),
            KeyCode::Char('w') => self.status_message = self.generate_writeup(),
            _ => {}
        }
        true
    }
}

fn load_challenges() -> Result<(Vec<Challenge>, Option<PathBuf>), Box<dyn Error>> {
    let path = PathBuf::from("challenges.toml");
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
            Line::raw(""),
            Line::styled("Description", Style::default().add_modifier(Modifier::BOLD)),
            Line::raw(&c.description),
            Line::raw(""),
            Line::styled("Actions", Style::default().add_modifier(Modifier::BOLD)),
            Line::raw("u: up | d: down | l: logs | s: shell | w: writeup | t: cycle status"),
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
        "[j/k] move [u/d/l/s/w/t] actions [q] quit | {}",
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
