use std::{error::Error, io, process::Command, time::Duration};

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

#[derive(Clone, Copy, Debug)]
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

#[derive(Debug)]
struct Challenge {
    name: &'static str,
    category: &'static str,
    difficulty: &'static str,
    status: ChallengeStatus,
    description: &'static str,
    workdir: &'static str,
}

#[derive(Debug)]
struct App {
    challenges: Vec<Challenge>,
    selected: usize,
    status_message: String,
}

impl App {
    fn new() -> Self {
        Self {
            challenges: vec![
                Challenge {
                    name: "rsa-baby",
                    category: "Crypto",
                    difficulty: "Easy",
                    status: ChallengeStatus::Todo,
                    description: "Recover plaintext using weak RSA key setup.",
                    workdir: "./challenges/rsa-baby/docker",
                },
                Challenge {
                    name: "fmt-lab",
                    category: "Pwn",
                    difficulty: "Medium",
                    status: ChallengeStatus::Doing,
                    description: "Practice format string leak and arbitrary write.",
                    workdir: "./challenges/fmt-lab/docker",
                },
                Challenge {
                    name: "tiny-note",
                    category: "Web",
                    difficulty: "Easy",
                    status: ChallengeStatus::Done,
                    description: "Reproduce auth bypass via cookie tampering.",
                    workdir: "./challenges/tiny-note/docker",
                },
            ],
            selected: 0,
            status_message: "Ready. Press q to quit.".to_string(),
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
            .current_dir(challenge.workdir)
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

    fn on_key(&mut self, code: KeyCode) -> bool {
        match code {
            KeyCode::Char('q') => return false,
            KeyCode::Char('j') | KeyCode::Down => self.next(),
            KeyCode::Char('k') | KeyCode::Up => self.prev(),
            KeyCode::Enter => self.status_message = "Open challenge details (next step).".to_string(),
            KeyCode::Char('u') => self.status_message = self.run_docker_action(&["up", "-d"]),
            KeyCode::Char('d') => self.status_message = self.run_docker_action(&["down"]),
            KeyCode::Char('l') => self.status_message = self.run_docker_action(&["logs", "--tail", "60"]),
            KeyCode::Char('s') => self.status_message = "TODO: open interactive shell pane".to_string(),
            KeyCode::Char('w') => self.status_message = "TODO: generate writeup scaffold".to_string(),
            _ => {}
        }
        true
    }
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

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Challenges ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));

    f.render_widget(list, main_chunks[0]);

    let detail = if let Some(c) = app.selected_challenge() {
        vec![
            Line::from(vec![
                Span::styled("Name: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(c.name),
            ]),
            Line::from(vec![
                Span::styled("Category: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(c.category),
            ]),
            Line::from(vec![
                Span::styled("Difficulty: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(c.difficulty),
            ]),
            Line::from(vec![
                Span::styled("Status: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(c.status.badge()),
            ]),
            Line::from(vec![
                Span::styled("Workdir: ", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(c.workdir),
            ]),
            Line::raw(""),
            Line::styled("Description", Style::default().add_modifier(Modifier::BOLD)),
            Line::raw(c.description),
            Line::raw(""),
            Line::styled("Actions", Style::default().add_modifier(Modifier::BOLD)),
            Line::raw("u: up | d: down | l: logs | s: shell | w: writeup | Enter: open"),
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
        "[j/k] move  [Enter] open  [u/d/l/s/w] actions  [q] quit  | {}",
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
