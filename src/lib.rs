use tui::{
    Frame,
    widgets::{
        Widget, Block, Borders, BorderType, Tabs,
    },
    layout::{
        Layout, Constraint, Direction, Rect,
    },
    text::{Spans, Span},
    style::{Style, Color, Modifier},
    backend::Backend,
};
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
mod app;
mod api;
mod api_service;

type Term = tui::Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>;


#[tokio::main]
pub async fn run(
    terminal: &mut Term
) -> Result<(), Box<dyn std::error::Error>> 
{
    let query = vec![
        ("formatted", "false"),
        ("lang", "en-US"),
        ("region", "US"),
        ("corsDomain", "finance.yahoo.com"),
        ("symbols", "spy")
    ];
    let body = api::quote::Response::get(&["spy", "tsla", "aapl"], &query).await?;
    let mut app = app::App::new();
    let quotes = body.response.result.unwrap_or_default();
    for quote in &quotes {
        app.tickers.push(&quote);
    }
    loop {
        draw_term1(terminal, &mut app)?;
        if crossterm::event::poll(Duration::from_secs(1))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Right => app.next(),
                    KeyCode::Left => app.previous(),
                    KeyCode::Char(c) => {
                        println!("pressed key: {c}");
                        break;
                    },
                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn draw<B>(rect: &mut Frame<B>)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints(
            [
            Constraint::Length(3),
            Constraint::Length(6),
            Constraint::Min(0),
            ].as_ref()
        )
        .split(rect.size());
    let block = Block::default()
        .title("Tabs")
        .borders(Borders::ALL);
    rect.render_widget(block, chunks[0]);
    let block = Block::default()
        .title("Header")
        .borders(Borders::ALL);
    rect.render_widget(block, chunks[1]);
    let chunkss = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
            Constraint::Percentage(50),
            Constraint::Percentage(50),
            ].as_ref()
        )
        .split(chunks[2]);
    let block = Block::default()
        .title("b111111111111111111111111111111111111")
        .borders(Borders::ALL);
    rect.render_widget(block, chunkss[0]);
    let block = Block::default()
        .title("b222222222222222222222222222222222222")
        .borders(Borders::ALL);
    rect.render_widget(block, chunkss[1]);
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut app::App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([
            Constraint::Length(3),  /* tabs */
            Constraint::Min(0),     /* main content */
        ].as_ref())
        .split(size);

    let titles = app
        .tickers
        .iter()
        .map(|t| {
            Spans::from(vec![
                Span::styled(&t.symbol, Style::default().fg(Color::Gray))
            ])
        })
    .collect();
    let tabs = Tabs::new(titles)
        .block(Block::default().borders(Borders::ALL).title("Tabs"))
        .select(app.index)
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::Black)
            .fg(Color::Green),
        );
    f.render_widget(tabs, chunks[0]);
    let inner = match app.index {
        0 => draw_main_container(f, app, chunks[1]),
        1 => draw_main_container(f, app, chunks[1]),
        2 => draw_main_container(f, app, chunks[1]),
        _ => {},
    };
}


fn draw_term1(
    terminal: &mut Term,
    app: &mut app::App,
) -> Result<(), Box<dyn std::error::Error>> {

    terminal.draw(|f| {
        ui(f, app);
    })?;
    Ok(())
}

fn draw_main_container<B: Backend>(f: &mut Frame<B>, app: &mut app::App, area: Rect) 
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
            Constraint::Min(0),
            ].as_ref()
        )
        .split(area);
    let block = Block::default().style(Style::default().fg(Color::Cyan)).title(&*app.tickers[app.index].long_name).borders(Borders::ALL);
    f.render_widget(block, area);
    draw_main_content(f, app, chunks[0]);
}

fn draw_main_content<B: Backend>(f: &mut Frame<B>, app: &mut app::App, area: Rect) 
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .vertical_margin(1)
        .horizontal_margin(3)
        .constraints(
            [
            Constraint::Length(10),
            Constraint::Min(0),
            ].as_ref()
        )
        .split(area);
    
    draw_top_bar(f, app, chunks[0]);
    let chunkss = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
            Constraint::Min(0),
            ].as_ref()
        )
        .split(chunks[1]);
    let block = Block::default()
        .style(Style::default().fg(Color::Cyan))
        .borders(Borders::BOTTOM | Borders::TOP);
    f.render_widget(block, chunkss[0]);
}

fn draw_top_bar<B: Backend>(f: &mut Frame<B>, app: &mut app::App, area: Rect) 
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
            Constraint::Min(22),
            Constraint::Length(45),
            ].as_ref()
        )
        .split(area);
    let block = Block::default().style(Style::default().fg(Color::Cyan)).title("inner top left").borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    draw_top_bar_legend(f, app, chunks[1]);
}

fn draw_top_bar_legend<B: Backend>(f: &mut Frame<B>, app: &mut app::App, area: Rect) 
where
    B: Backend,
{
    let block = Block::default().style(Style::default().fg(Color::Cyan)).title("inner top right").borders(Borders::ALL);
    f.render_widget(block, area);
}
