use std::{fs, path::{PathBuf, Path}}; 
extern crate glob;
use glob::{glob, glob_with, MatchOptions, Pattern};
use std::env;

use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Corner, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame, Terminal,
};
use std::fs::File;
use std::io::BufReader;
use rodio::{Sink, Decoder, OutputStream, source::Source};
use std::ffi::OsStr;
use crate::*;

// TODO encapsulation
pub struct StatefulList<T> {
    pub state: ListState,
    items: Vec<T>,
    curr: usize,
}

impl<T> StatefulList<T> {

    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
            curr: 0,
        }
    }

    pub fn next(&mut self) {
        
        // check if empty
        if self.items.is_empty(){return};

        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.curr = i;
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        // check if empty
        if self.items.is_empty(){return};

        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.curr = i;
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    // add item to items vector
    pub fn add(&mut self, item: T){
        self.items.push(item);
    }

    // remove item from items vector
    pub fn remove(&mut self){

        // if list is empty ignore
        if self.items.len() == 0{
            return;
        // top of queue
        } else if self.items.len() == 1 {
            self.items.remove(self.curr);
            self.unselect();
        // if at bottom of queue, remove item and select item above above
        } else if (self.state.selected().unwrap()) >= (self.items.len() - 1){
            self.items.remove(self.curr);
            self.curr -= 1;
            self.state.select(Some(self.curr));
        // else delete item
        } else if !(self.items.is_empty()){
            self.items.remove(self.curr);
        };
    }

    // return all items contained in vector
    pub fn get_items(&self) -> &Vec<T> {
        &self.items
    }

    // return item at index
    pub fn get_item(&self) -> &T {
        &self.items[self.curr]
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

}
