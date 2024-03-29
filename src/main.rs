#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::{
    State,
    http::Status, 
    response::Redirect,
    form::{Context, Contextual, Form, FromForm},
};

use rocket_dyn_templates::Template;

use serde::{Deserialize, Serialize};

#[derive(Debug, FromForm, Serialize, Deserialize)]
struct Character {
    info: PersonalData,
    action_skills: Actionskills,
    knowledge_skills: Knowledgeskills,
    social_skills: Socialskills,
    inventory: Inventory,
    notes: Notes,
}

fn context_load(id: &String, folder: &str) -> Option<serde_json::Value> {
    use std::fs::File;
    use std::path::PathBuf;

    let mut id = String::from(id);
    id.push_str(".context.json");

    let mut path = PathBuf::from(folder);
    path.push(id);

    debug!("load from: {:?}", path);
    File::open(path)
        .ok()
        .map(|file| serde_json::from_reader::<File, serde_json::Value>(file).ok())
        .flatten()
}

fn context_store(id: &String, context: &Context, folder: &str) -> anyhow::Result<()> {
    use std::fs::File;
    use std::path::PathBuf;

    let mut filename = String::from(id);
    filename.push_str(".context.json");

    let mut filepath = PathBuf::from(folder);
    filepath.push(filename);

    debug!("writing to {:?}", filepath);
    let file = File::create(filepath)?;
    serde_json::to_writer(file, context)?;
    Ok(())
}

impl Character {
    fn store_to_disk(&self, mut id: String, folder: &str) -> anyhow::Result<()> {
        use std::fs;
        use std::path::{Path, PathBuf};
        let path = Path::new(folder);
        if !path.exists() {
            fs::create_dir(path)?;
        };
        id.push_str(".character.json");
        let filename = Path::new(&id);
        let mut filepath = PathBuf::from(path);
        filepath.push(filename);
        let file = std::fs::File::create(filepath)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    fn from_disk(mut id: String, folder: &str) -> Option<Self> {
        use std::fs::File;
        use std::path::PathBuf;

        id.push_str(".character.json");
        let mut path = PathBuf::from(folder);
        path.push(id);
        debug!("load from: {:?}", path);
        File::open(path)
            .ok()
            .map(|file| serde_json::from_reader::<File, Self>(file).ok())
            .flatten()
    }
}

type Actionskills = Skills;
type Knowledgeskills = Skills;
type Socialskills = Skills;

#[derive(Debug, FromForm, Serialize, Deserialize)]
struct PersonalData {
    #[field(validate = len(1..50))]
    name: String,
    #[field(validate = len(1..50))]
    gender: String,
    age: u64,
    health: i64,
    #[field(validate = len(1..50))]
    look: String,
    #[field(validate = len(1..50))]
    religion: String,
    #[field(validate = len(1..50))]
    profession: String,
    #[field(validate = len(1..50))]
    marital_status: String,
}

#[derive(Debug, FromForm, Serialize, Deserialize)]
struct Inventory {
    #[field(validate = len(0..5000))]
    content: String,
}

#[derive(Debug, FromForm, Serialize, Deserialize)]
struct Notes {
    #[field(validate = len(0..5000))]
    content: String,
}

#[derive(Debug, FromForm, Serialize, Deserialize)]
struct Skills {
    skill0: Skill,
    skill1: Skill,
    skill2: Skill,
    skill3: Skill,
    skill4: Skill,
    skill5: Skill,
    skill6: Skill,
    skill7: Skill,
    skill8: Skill,
    skill9: Skill,
}

#[derive(Debug, FromForm, Serialize, Deserialize)]
struct Skill {
    #[field(validate = len(0..30))]
    name: String,
    points: Option<u8>,
}

#[get("/")]
fn index() -> Redirect {
    //Template::render("index", &Context::default())
    let id: String = rand::random::<u64>().to_string();
    //(Status::TemporaryRedirect, Redirect::to(uri!(submit: id)))
    Redirect::to(uri!(new(id)))
}

#[get("/sheet/<id>")]
fn new(id: String, folder: &State<CharacterDirectory>) -> Template {
    if let Some(context) = context_load(&id, folder.as_str()) {
        debug!("{:#?}", &context);
        debug!("LOADING SHEET");
        Template::render("index", context)
    } else {
        debug!("NEW SHEET");
        Template::render("index", &Context::default())
    }
}

#[post("/sheet/<id>", data = "<form>")]
fn submit<'r>(id: String, form: Form<Contextual<'r, Character>>, folder: &State<CharacterDirectory>) -> (Status, Template) {
    //debug!("{:#?}", &form.value);
    debug!("{:#?}", &form.context);

    if let Err(e) = context_store(&id, &form.context, folder.as_str()) {
        error!("ERROR: Failed storing context. {:?}", e);
    }
    let template = match form.value {
        Some(ref character) => {
            //debug!("Character: {:#?}", character);
            if let Err(e) = character.store_to_disk(id, folder.as_str()) {
                error!("ERROR: Failed storing character. {:?}", e);
            }
            Template::render("index", &form.context)
        }
        None => Template::render("index", &form.context),
    };

    (form.context.status(), template)
}

#[derive(Deserialize)]
struct CharacterDirectory{
    character_directory: String,
}

impl CharacterDirectory {
    fn as_str(&self) -> &str {
        self.character_directory.as_str()
    }
}

#[launch]
fn rocket() -> _ {
    use rocket::fairing::AdHoc;
    use rocket::fs;

    rocket::build()
        .attach(AdHoc::config::<CharacterDirectory>())
        .mount("/", routes![index])
        .mount("/", routes![new, submit])
        .attach(Template::fairing())
        .mount("/", fs::FileServer::from(fs::relative!("static")))
}
