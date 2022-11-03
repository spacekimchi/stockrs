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
    /*
    let query = vec![
        ("formatted", "false"),
        ("lang", "en-US"),
        ("region", "US"),
        ("corsDomain", "finance.yahoo.com"),
        ("symbols", "spy")
    ];
    */
    // let body = api::quote::Response::get(&["spy"], &query).await?;
    // println!("body: {}", body.response.result.unwrap().first().unwrap().long_name);
    let mut app = app::App::new();
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

    //std::thread::sleep(std::time::Duration::from_millis(5000));

    Ok(())
}

fn draw<B>(rect: &mut Frame<B>)
where
    B: Backend,
{
    /*
     * full screen block with title "Block" in the top left
     let size = f.size();
     let block = Block::default()
     .title("Block")
     .borders(Borders::ALL);
     f.render_widget(block, size);
     */
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

fn ui<B: Backend>(f: &mut Frame<B>, app: &app::App) {
    let size = f.size();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(5)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let block = Block::default().style(Style::default().fg(Color::Green));
    f.render_widget(block, size);
    let titles = app
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Yellow)),
                Span::styled(rest, Style::default().fg(Color::Green)),
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
            .bg(Color::Black),
        );
    f.render_widget(tabs, chunks[0]);
    let inner = match app.index {
        0 => Block::default().style(Style::default().fg(Color::Red)).title("Inner 0").borders(Borders::ALL),
        1 => Block::default().style(Style::default().fg(Color::LightBlue)).title("Inner 1").borders(Borders::ALL),
        2 => Block::default().style(Style::default().fg(Color::Magenta)).title("Inner 2").borders(Borders::ALL),
        3 => Block::default().style(Style::default().fg(Color::Cyan)).title("Inner 3").borders(Borders::ALL),
        _ => unreachable!(),
    };
    f.render_widget(inner, chunks[1]);
}


fn draw_term1(
    terminal: &mut Term,
    app: &mut app::App,
) -> Result<(), Box<dyn std::error::Error>> {

    terminal.draw(|f| {
        //draw(f);
        ui(f, app);
    })?;
    Ok(())
}

fn draw_tab_2<B: Backend>(f: &mut Frame<B>, app: &mut app::App, area: Rect) 
where
    B: Backend,
{
    let chunks = Block::default();
}
