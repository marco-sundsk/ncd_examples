use crate::*;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum NoteBook {
    TextNoteBook(TextNoteBook),
    RichNoteBook(RichNoteBook),
}

impl NoteBook {
    pub fn kind(&self) -> String {
        match self {
            NoteBook::TextNoteBook(_) => "Text_NoteBook".to_string(),
            NoteBook::RichNoteBook(_) => "Rich_NoteBook".to_string(),
        }
    }

    pub fn add_notes(&mut self, sender_id: &AccountId, content: &String) {
        match self {
            NoteBook::TextNoteBook(notebook) => notebook.add_notes(sender_id, content),
            NoteBook::RichNoteBook(notebook) => notebook.add_notes(sender_id, content),
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct TextNoteBook {
    pub id: u32,
    pub notes: Vec<String>,
}

impl TextNoteBook {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            notes: Vec::new(),
        }
    }

    pub fn add_notes(&mut self, sender_id: &AccountId, content: &String) {
        let record = format!("{}:{}", sender_id, content);
        self.notes.push(record);
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct RichNoteBook {
    pub id: u32,
    pub notes: LookupMap<AccountId, String>,
    pub count: u32,
}

impl RichNoteBook {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            notes: LookupMap::new(StorageKey::RichNotes { notes_id: id }),
            count: 0,
        }
    }

    pub fn add_notes(&mut self, sender_id: &AccountId, content: &String) {
        self.notes.insert(sender_id, content);
        self.count += 1;
    }
}
