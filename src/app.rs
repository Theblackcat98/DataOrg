use serde_json::{Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};
pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
    Loading,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}

pub enum CurrentlyLoading {
    Load,
    Done,
}

pub struct App {
    pub key_input: String, // the currently being edited json key.
    pub value_input: String, // the currently being edited json value.
    pub pairs: HashMap<String, String>, // The representation of our key and value pairs with serde Serialize support
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_editing: Option<CurrentlyEditing>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
    pub currently_loading: Option<CurrentlyLoading>,
    pub print_json: String,
}

impl App {
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
            currently_loading: None,
            print_json: String::new(),
        }
    }
    
    pub fn load_from_json(&mut self, json_file_path: &str) -> Result<(), serde_json::Error> {
        let mut file = std::fs::File::open(json_file_path)
        .map_err(|err| serde_json::Error::io(io::Error::new(io::ErrorKind::Other, format!("Failed to open file: {}", err))))?;     
        let mut json_content = String::new();
        file.read_to_string(&mut json_content)
        .map_err(|err| serde_json::Error::io(io::Error::new(io::ErrorKind::Other, format!("Failed to read file: {}", err))))?;

        let loaded_pairs: HashMap<String, String> = serde_json::from_str(&json_content)?;

        // Update or add pairs with loaded data
        self.pairs.extend(loaded_pairs);

        Ok(())
    }

    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());

        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }
    

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => {
                    self.currently_editing = Some(CurrentlyEditing::Value)
                }
                CurrentlyEditing::Value => {
                    self.currently_editing = Some(CurrentlyEditing::Key)
                }
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }

    pub fn toggle_loading(&mut self) {
        if let Some(load_mode) = &self.currently_loading {
            match load_mode {
                CurrentlyLoading::Load => {
                    self.currently_loading = Some(CurrentlyLoading::Load)
                }
                CurrentlyLoading::Done => {
                    self.currently_loading = Some(CurrentlyLoading::Done)
                }
            };
        } else {
            self.currently_loading = Some(CurrentlyLoading::Done);
        }
    }

    pub fn print_json(&self) -> Result<(), serde_json::Error> {
        let output = serde_json::to_string(&self.pairs)?;
        println!("{}", output);
        Ok(())
    }
} 
