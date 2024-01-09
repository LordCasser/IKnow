mod services;



use std::string::String;
use std::io::{self};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}
};
use ratatui::{prelude::*, widgets::*};
use crate::services::toutiao::ToutiaoResult;
use crate::services::weibo::WeiboResult;
use crate::services::zhihu::{QuestionResult, SearchResult};
use std::{error::Error};
use std::time::{Duration, Instant};

use ratatui::style::Stylize;




struct App {
    state: TableState,
    toutiao: Vec<ToutiaoResult>,
    weibo: Vec<WeiboResult>,
    zhihu_question: Vec<QuestionResult>,
    zhihu_search: Vec<SearchResult>,
    current_page: usize,
}

impl App {
    fn new() -> App{
        App {
            state: TableState::default(),
            toutiao: services::toutiao::fetch_data(),
            weibo: services::weibo::fetch_data(),
            zhihu_question: services::zhihu::fetch_question(),
            zhihu_search: services::zhihu::fetch_search(),
            current_page: 0,
        }

    }

    fn on_tick(&mut self) {
        self.switch();
    }

    pub fn switch(&mut self) {
        self.current_page  = (self.current_page+1) % 4;
        self.fetch_data();
    }

    // fetch data for next page
    fn fetch_data(&mut self) {
        let next = (self.current_page+1) % 4;

        match next {
            0 => self.toutiao = services::toutiao::fetch_data(),
            1 => self.weibo = services::weibo::fetch_data(),
            2 => self.zhihu_question = services::zhihu::fetch_question(),
            3 => self.zhihu_search = services::zhihu::fetch_search(),
            _ => {}
        }
    }

    pub fn get_data(&self) -> Vec<String> {
        match self.current_page {
            0 => self.toutiao.iter().map(|x| x.title.clone()).collect(),
            1 => self.weibo.iter().map(|x| x.title.clone()).collect(),
            2 => self.zhihu_question.iter().map(|x| x.title.clone()).collect(),
            3 => self.zhihu_search.iter().map(|x| x.title.clone()).collect(),
            _ => Vec::new(),
        }
    }

    pub fn get_page(&self) -> String {
        match self.current_page {
            0 => "toutiao".to_string(),
            1 => "weibo".to_string(),
            2 => "zhihu_question".to_string(),
            3 => "zhihu_search".to_string(),
            _ => "NuLL".to_string(),
        }
    }

}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let tick_rate = Duration::from_secs(60);
    let app = App::new();
    let res = run_app(&mut terminal, app, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|f| ui(f, &app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
}

fn ui(f: &mut Frame, app: &App) {
    let size = f.size();

    let block = Block::default().black();
    f.render_widget(block, size);

    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(size);
    let text = app.get_data();

    let mut lines = vec![];
    let mut  counter = 0;
    for line in text {
        counter = counter + 1;
        if counter < 10 {
            lines.push((counter.to_string()+".  "+&line).into());
            continue
        }
        lines.push((counter.to_string()+". "+&line).into());
    }

    let create_block = |title| {
        Block::default()
            .borders(Borders::ALL)
            .gray()
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
    };

    let paragraph = Paragraph::new(lines)
        .style(Style::default().fg(Color::Gray))
        .block(create_block(app.get_page())).alignment(Alignment::Left)
        .wrap(Wrap { trim: true });
    f.render_widget(paragraph, layout[0]);
}