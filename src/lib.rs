use tui::{
    Frame,
    widgets::{
        Widget, Block, Borders
    },
    layout::{
        Layout, Constraint, Direction
    },
    backend::Backend,
};
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;
mod api;
mod api_service;

type Term = tui::Terminal<tui::backend::CrosstermBackend<std::io::Stdout>>;

#[tokio::main]
pub async fn run(
    terminal: &mut Term,
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
    loop {
        draw_term1(terminal)?;
        if crossterm::event::poll(Duration::from_secs(1))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
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

fn draw_term1(
    terminal: &mut Term,
) -> Result<(), Box<dyn std::error::Error>> {

    terminal.draw(|f| {
        draw(f);
    })?;
    Ok(())
}

