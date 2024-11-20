use std::ptr::read;
use json::JsonValue;

pub(crate) struct Special {
    pub(crate) name: String,
    pub(crate) traits: Vec<String>,
    pub(crate) range: String,
    pub(crate) actions: String,
    pub(crate) special_type: String,
    pub(crate) description: String,
}

pub(crate) struct DC {
    pub(crate) value: String,
    pub(crate) benchmark: String,
    pub(crate) note: String
}

impl Special {
    pub(crate) fn to_string(&self) -> String {
        let mut s = String::new();
        if !self.name.is_empty() {
            s += &format!("> **{}**", self.name);
            if !self.traits.is_empty() {
                s += &format!(" (*{}*)", self.traits.join(", "));
            }
            if !self.range.is_empty() {
                s += &format!(" ({})", self.range);
            }

            if !self.actions.is_empty() {
                s += &format!(" ({} actions)", self.actions);
            }
            s+=": ";
        }
        if !self.special_type.is_empty(){
            s += &format!("> *{}*\n", self.special_type);
        }
        if !self.description.is_empty(){
            // Remove \n from strings anmd replace with > {} \n
            let mut desc = self.description.replace("\n", "\n> ");
            desc = desc.trim().to_string();
            s += &format!("> {}\n", desc);
        }
        s
    }
}

impl DC {
    pub(crate) fn new(j : &JsonValue) -> DC {
        DC {
            value : j["value"].to_string(),
            benchmark : j["benchmark"].to_string(),
            note : j["note"].to_string()
        }
    }

    pub(crate) fn to_string(&self) -> String{
        let mut s = String::new();
        if self.value.is_empty() {
            return "".to_string();
        }
        s += &format!(" {}", self.value);
        if !self.benchmark.is_empty(){
            s += &format!(" ({})", self.benchmark);
        }
        if !self.note.is_empty(){
            s += &format!(" {}", self.note);
        }
        s
    }
}

pub(crate) struct Strike {
    pub(crate) name: String,
    pub(crate) traits: Vec<String>,
    pub(crate) attack: String,
    pub(crate) damage: String,
    pub(crate) type_: String
}

impl Strike {
    pub(crate) fn to_string(&self) -> String {
        let mut s = String::new();
        if !self.name.is_empty() {
            s += &format!("> **{}**", self.name);
            if !self.traits.is_empty() {
                s += &format!(" {}", self.traits.join(", "));
            }
            s+= ": ";
        }
        if !self.attack.is_empty(){
            s += &format!(" *({} actions)*\n", self.attack);
        }
        if !self.damage.is_empty(){
            s += &format!("> *{}*\n", self.damage);
        }
        if !self.type_.is_empty(){
            s += &format!("> *{}*\n", self.type_);
        }
        s
    }
}
