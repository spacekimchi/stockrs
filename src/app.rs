use crate::api::quote::Quote;

pub struct App<'a> {
    pub tickers: Vec<&'a Quote>,
    pub index: usize,
}

impl<'a> App<'a> {
    pub fn new() -> App<'a> {
        App {
            tickers: vec![],
            index: 0,
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.tickers.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.tickers.len() - 1;
        }
    }
}

