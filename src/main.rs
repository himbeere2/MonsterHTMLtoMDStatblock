mod pf2_structs;

use std::env::args_os;
use std::ffi::OsString;
use std::fs::File;
use std::io::Read;
use std::ops::Add;
use json::{Array, JsonValue};
use crate::pf2_structs::DC;

fn main() {
    let mut output_text = String::new();

    let args: Vec<OsString> = args_os().collect();
    // Use second argument as path to file to open
    let path = args.get(1).unwrap_or_else(|| {
        println!("No file path provided");
        std::process::exit(1);
    });
    println!("Opening file: {:?}", path);
    let mut file = File::open(path).unwrap_or_else(|e| {
        println!("Error opening file: {}", e);
        std::process::exit(1);
    });

    // println!("File opened successfully");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap_or_else(|e| {
        println!("Error reading file: {}", e);
        std::process::exit(1);
    });

    // Make buffers into string
    let content = String::from_utf8(buffer).unwrap_or_else(|e| {
        println!("Error converting buffer to string: {}", e);
        std::process::exit(1);
    });

    // println!("File read successfully, content: {}", content);
    // String to json
    let mut j_value: json::JsonValue = json::parse(&content).unwrap_or_else(|e| {
        println!("Error parsing json: {}", e);
        std::process::exit(1);
    });

    // Start
    output_text += "> [!statblock]\n";

    let name = &j_value["name"];
    output_text.push_str(&format!("> {}", name));

    let show_level = &j_value["showlevel"];
    output_text.push_str(&format!("     **{}**\n", show_level));

    let traits = &j_value["traits"].to_string();
    output_text.push_str(&format!("> *{}*\n", traits.to_uppercase()));

    let tags = &j_value["tags"].members().map(|t| t.to_string().to_uppercase()).collect::<Vec<String>>().join(", ");
    output_text.push_str(&format!("> *{}*\n", &tags));

    /*    let level: i8 = (&j_value["level"]).as_i8().unwrap_or_else(|| {
            println!("Error parsing level");
            std::process::exit(1);
        });
        output_text += &format!("**Level {}**\n", level);*/

    let alignment = &j_value["alignment"].to_string();
    output_text.push_str(&format!("> {} | ", &alignment.to_uppercase()));

    let size = &j_value["size"].to_string();
    output_text.push_str(&format!("{} | ", &size.to_uppercase()));
    let type_ = (&j_value["type"]).as_str().unwrap();
    output_text.push_str(&format!("{}\n", type_.to_uppercase()));

    // Most important stats
    let strength = DC::new(&j_value["strength"]);
    output_text.push_str(&format!("> STR {}", strength.to_string().replace("\n", "")));

    let dexterity = DC::new(&j_value["dexterity"]);
    output_text.push_str(&format!(" | DEX {}", dexterity.to_string().replace("\n", "")));

    let constitution = DC::new(&j_value["constitution"]);
    output_text.push_str(&format!(" | CON {}", constitution.to_string().replace("\n", "")));

    let intelligence = DC::new(&j_value["intelligence"]);
    output_text.push_str(&format!(" | INT {}", intelligence.to_string().replace("\n", "")));

    let wisdom = DC::new(&j_value["wisdom"]);
    output_text.push_str(&format!(" | WIS {}", wisdom.to_string().replace("\n", "")));

    let charisma = DC::new(&j_value["charisma"]);
    output_text.push_str(&format!(" | CHA {}\n", charisma.to_string()));

    let perception = DC::new(&j_value["perception"]).to_string();
    if !perception.is_empty() {output_text.push_str(&format!("> Perception {}\n", perception));}

    let speed = &j_value["speed"];
    output_text.push_str(&format!("> Speed: {}\n", speed));

    // All of them skills

    // This is a complex one and needs to be unwrapped futher

    /*    let save_note = &j_value["savenote"];
        output_text += &format!("> {}\n", save_note);*/

    /*    let creature = &j_value["creature"];
        output_text += &format!("> {}\n", creature);*/

/*    let img_url = &j_value["imgUrl"];
    if !img_url.is_empty() && !img_url.eq("null") {
        output_text += &format!("> ![img]({})\n", img_url);
    }*/

    let spell_type = &j_value["spelltype"];
    if !spell_type.is_empty() {
        output_text.push_str(&format!("> Spell Type: {}\n", spell_type));
    }

    if !j_value["focuspoints"].is_empty() {
        let focus_points = &j_value["focuspoints"].as_u8().unwrap();
        if !focus_points == 0 {
            output_text.push_str(&format!("> Focus Points: {}\n", focus_points));
        }
    }

    let cantrip_level = &j_value["cantriplevel"].to_string();
    if !cantrip_level.is_empty() { output_text.push_str(&format!("> Cantrip Level: {}\n", cantrip_level)); }

    let ritual_type = &j_value["ritualtype"];
    if !ritual_type.is_empty() { output_text.push_str(&format!("> Ritual Type: {}\n", ritual_type)); }

    let rituals = &j_value["rituals"];
    if !rituals.is_empty() { output_text.push_str(&format!("> Rituals: {}\n", rituals)); }

    let spells_r = &j_value["spells"];
    if !spells_r.is_empty() {
        let spells: Vec<String> = spells_r.members().map(|s| s.to_string()).filter(|s| !s.is_empty()).collect();
        if !spells.is_empty() {output_text.push_str(&format!("> Spells: {:?}\n", spells));}
    }

    let constant = &j_value["constant"];
    if !constant.is_empty() {output_text.push_str(&format!("> {}\n", constant));}

    let minprof = DC::new(&j_value["minprof"]).to_string();
    if !minprof.is_empty() {output_text.push_str(&format!("> Min Prof {}", minprof.to_string()));}



    // Adds
    let ac: DC = DC::new(&j_value["ac"]);
    output_text.push_str(&format!("> AC {}", ac.to_string()));

    let hp: DC = DC::new(&j_value["hp"]);
    output_text.push_str(&format!(" | HP {}\n", hp.to_string()));

    let fort: DC = DC::new(&j_value["fortitude"]);
    output_text.push_str(&format!("> Saves: Fortitude {}", fort.to_string()));

    let ref_: DC = DC::new(&j_value["reflex"]);
    output_text.push_str(&format!(" | Reflex {}", ref_.to_string()));

    let will: DC = DC::new(&j_value["will"]);
    output_text.push_str(&format!(" | Will {}\n", will.to_string()));

    // Resistances and immunities
    let resistances = DC::new(&j_value["resistance"]).to_string();
    output_text.push_str(&format!("> **Resistances**: {}", resistances));

    let immunities = DC::new(&j_value["immunity"]).to_string();
    output_text.push_str(&format!("> **Immunities**: {}", immunities));

    let weaknesses = DC::new(&j_value["weakness"]).to_string();
    output_text.push_str(&format!("> **Weaknesses**: {}", weaknesses));

    let info = (&j_value["info"]).to_string();
    output_text.push_str(&format!("> Info: {}\n", info));

    extract_abilities(&mut output_text, &mut j_value);


    let strikes_r = &j_value["strikes"];
    let _ = strikes_r.members().map(|s| {
        let name = &s["name"];
        let traits = &s["traits"];
        let attack = &s["attack"];
        let damage = &s["damage"];
        let type_ = &s["type"];
        let strike = pf2_structs::Strike {
            name: name.to_string(),
            traits: traits.members().map(|t| t.to_string()).collect(),
            attack: attack.to_string(),
            damage: damage.to_string(),
            type_: type_.to_string(),
        };
        output_text.push_str(&format!("> {}\n", strike.to_string()));
    });


    let specials = &j_value["specials"];
    let mut specials_vec = Vec::new();
    // Append to collection
    for special in specials.members() {
        let name = &special["name"];
        let traits = &special["traits"];
        let range = &special["range"];
        let actions = &special["actions"];
        let special_type = &special["type"];
        let description = &special["description"];
        let special = pf2_structs::Special {
            name: name.to_string(),
            traits: traits.members().map(|t| t.to_string()).collect(),
            range: range.to_string(),
            actions: actions.to_string(),
            special_type: special_type.to_string(),
            description: description.to_string(),
        };
        specials_vec.push(special);
    }
    output_text.push_str(specials_vec.iter().map(|s| s.to_string()).collect::<String>().as_str());
    let more_spells = &j_value["morespells"];
    if !more_spells.is_empty() {output_text.push_str(&format!("> {}\n", more_spells));}

    /*    let adjust = &j_value["adjust"].as_u8().unwrap();
        output_text += &format!("> {}\n", adjust);*/

    let items = &j_value["items"];
    if !items.is_empty() {
        let items: Vec<String> = items.members().map(|i| i.to_string()).collect();
        output_text.push_str(&format!("> {:?}\n", items));
    }
    let spell_attack_r = &j_value["spellattack"];
    let spell_attack: DC = DC {
        value: spell_attack_r["value"].to_string(),
        benchmark: spell_attack_r["benchmark"].to_string(),
        note: spell_attack_r["note"].to_string(),
    };
    if !spell_attack.to_string().is_empty() {
        output_text.push_str(&format!("> Spell Attack {}", spell_attack.to_string()));
    }
    let spell_dc: DC = DC::new(&j_value["spelldc"]);
    if !spell_dc.to_string().is_empty() {
        output_text.push_str(&format!("> Spell DC {}", spell_dc.to_string()));
    }

    let languages = &j_value["languages"];
    output_text.push_str(&format!("> Languages: *{}*\n", languages.to_string()));

    let description = &j_value["description"].to_string().replace("\n", "\n> ");
    output_text.push_str(&format!("> {}\n", description));

    println!("{}", output_text);
}

fn extract_abilities(output_text: &mut String, j_value: &mut JsonValue) {

    let acrobatics = DC::new(&j_value["acrobatics"]).to_string();
    if !acrobatics.is_empty() {
        output_text.push_str(&format!("> Acrobatics {}", acrobatics));
    }

    let arcana = DC::new(&j_value["arcana"]).to_string();
    if !arcana.is_empty() {
        output_text.push_str( &format!(", Arcana {}", arcana));
    }

    let athletics = DC::new(&j_value["athletics"]).to_string();
    if !athletics.is_empty() {
        output_text.push_str(&format!(", Athletics {}", athletics));
    }

    let crafting = DC::new(&j_value["crafting"]).to_string();
    if !crafting.is_empty() {
        output_text.push_str(&format!(", Crafting {}", crafting));
    }

    let deception = DC::new(&j_value["deception"]).to_string();
    if !deception.is_empty() {
        output_text.push_str(&format!(", Deception {}", deception));
    }

    let diplomacy = DC::new(&j_value["diplomacy"]).to_string();
    if !diplomacy.is_empty() {
        output_text.push_str(&format!(", Diplomacy {}", diplomacy));
    }

    let intimidation = DC::new(&j_value["intimidation"]).to_string();
    if !intimidation.is_empty() {
        output_text.push_str(&format!(", Intimidation {}", intimidation));
    }

    let medicine = DC::new(&j_value["medicine"]).to_string();
    if !medicine.is_empty() {
        output_text.push_str(&format!(", Medicine {}", medicine));
    }

    let nature = DC::new(&j_value["nature"]).to_string();
    if !nature.is_empty() {
        output_text.push_str(&format!(", Nature {}", nature));
    }

    let occultism = DC::new(&j_value["occultism"]).to_string();
    if !occultism.is_empty() {
        output_text.push_str(&format!(", Occultism {}", occultism));
    }

    let performance = DC::new(&j_value["performance"]).to_string();
    if !performance.is_empty() {
        output_text.push_str(&format!(", Performance {}", performance));
    }

    let religion = DC::new(&j_value["religion"]).to_string();
    if !religion.is_empty() {
        output_text.push_str(&format!(", Religion {}", religion));
    }

    let society = DC::new(&j_value["society"]).to_string();
    if !society.is_empty() {
        output_text.push_str(&format!(", Society {}", society));
    }

    let stealth = DC::new(&j_value["stealth"]).to_string();
    if !stealth.is_empty() {
        output_text.push_str(&format!(", Stealth {}", stealth));
    }

    let survival = DC::new(&j_value["survival"]).to_string();
    if !survival.is_empty() {
        output_text.push_str(&format!(", Survival {}", survival));
    }

    let thievery = DC::new(&j_value["thievery"]).to_string();
    if !thievery.is_empty() {
        output_text.push_str(&format!(", Thievery {}", thievery));
    }

    let lore = DC::new(&j_value["lore"]).to_string();
    if !lore.is_empty() {
        output_text.push_str(&format!(", Lore {}", lore));
    }

    let lorealt = DC::new(&j_value["lorealt"]).to_string();
    if !lorealt.is_empty() {
        output_text.push_str(&format!(", Lore II {}", lorealt));
    }
    output_text.push_str("\n");
}