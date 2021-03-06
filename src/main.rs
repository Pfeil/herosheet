#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]extern crate rocket;

use rocket::http::{Status, ContentType};
use rocket::form::{Form, Contextual, FromForm, FromFormField, Context};
use rocket::data::TempFile;

use rocket_contrib::serve::{StaticFiles, crate_relative};
use rocket_contrib::templates::Template;

#[derive(Debug, FromForm)]
struct Password<'v> {
    #[field(validate = len(6..))]
    #[field(validate = eq(self.second))]
    first: &'v str,
    #[field(validate = eq(self.first))]
    second: &'v str,
}

#[derive(Debug, FromFormField)]
enum Rights {
    Public,
    Reserved,
    Exclusive,
}

#[derive(Debug, FromFormField)]
enum Category {
    Biology,
    Chemistry,
    Physics,
    #[field(value = "CS")]
    ComputerScience,
}

#[derive(Debug, FromForm)]
struct Submission<'v> {
    #[field(validate = len(1..))]
    title: &'v str,
    date: time::Date,
    #[field(validate = len(1..=250))]
    r#abstract: &'v str,
    #[field(validate = ext(ContentType::PDF))]
    file: TempFile<'v>,
    #[field(validate = len(1..))]
    category: Vec<Category>,
    rights: Rights,
    ready: bool,
}

#[derive(Debug, FromForm)]
struct Account<'v> {
    #[field(validate = len(1..))]
    name: &'v str,
    password: Password<'v>,
    #[field(validate = contains('@').or_else(msg!("invalid email address")))]
    email: &'v str,
}

#[derive(Debug, FromForm)]
struct Submit<'v> {
    account: Account<'v>,
    submission: Submission<'v>,
}

#[derive(Debug, FromForm)]
struct Character {
    info: PersonalData,
    action_skills: Actionskills,
    knowledge_skills: Knowledgeskills,
    social_skills: Socialskills,
    inventory: Inventory,
    notes: Notes,
}

type Actionskills = Skills;
type Knowledgeskills = Skills;
type Socialskills = Skills;

#[derive(Debug, FromForm)]
struct PersonalData {
    id: Option<String>,
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

#[derive(Debug, FromForm)]
struct Inventory {
    #[field(validate = len(0..5000))]
    content: String,
}

#[derive(Debug, FromForm)]
struct Notes {
    #[field(validate = len(0..5000))]
    content: String,
}

#[derive(Debug, FromForm)]
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

#[derive(Debug, FromForm)]
struct Skill {
    #[field(validate = len(0..30))]
    name: String,
    points: Option<u8>,
}

#[get("/")]
fn index<'r>() -> Template {
    Template::render("index", &Context::default())
}

#[post("/", data = "<form>")]
fn submit<'r>(form: Form<Contextual<'r, Character>>) -> (Status, Template) {
    //println!("{:#?}", &form.value);
    println!("{:#?}", &form.context);
    let template = match form.value {
        Some(ref submission) => {
            println!("submission: {:#?}", submission);
            Template::render("index", &form.context)
        }
        None => Template::render("index", &form.context),
    };

    (form.context.status(), template)
}

#[launch]
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, submit])
        .attach(Template::fairing())
        .mount("/", StaticFiles::from(crate_relative!("/static")))
}
