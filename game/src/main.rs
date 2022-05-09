use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Age(u32);

#[derive(Component)]
struct Nickname(String);

#[derive(Component)]
struct Bedford;

pub struct StartPlugin;
pub struct MessagePlugin;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(add_person);
        app.add_system(mess_with_bed.after(add_person));
    }    
}

impl Plugin for MessagePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(greet_person.after(add_person));
        app.add_system(say_person_age.after(greet_person));
        app.add_system(call_person_by_nickname);
    }
}

fn add_person(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Claudete".to_string())).insert(Age(32));
    commands.spawn().insert(Person).insert(Name("Hellyson".to_string()));
    commands.spawn().insert(Person).insert(Name("Galinho".to_string()));
    commands.spawn().insert(Person).insert(Name("Hugo Bedford".to_string())).insert(Age(1381)).insert(Bedford);
    commands.spawn().insert(Person);
}

fn greet_person(query: Query<&Name, With<Person>>){
    for name in query.iter(){
        println!("Eae manito {}", name.0);
    }
}

fn say_person_age(query: Query<(&Name, &Age), With<Person>>){
    for (name, age) in query.iter(){
        if name.0 == "Hugo Bedford"{
            println!("Bedford é velho e tem {} anos", age.0);
        }
        else {
            println!("O manito {} tem {} anos", name.0, age.0);
        }
    }
}

fn mess_with_bed(mut commands: Commands, query: Query<Entity, With<Bedford>>){
    for entity in query.iter(){
        commands.entity(entity).insert(Nickname("Velho".to_string()));
        println!("mess...")
    }
}

fn call_person_by_nickname(query: Query<(&Nickname, &Name),With<Person>>){
    for (nickname, name) in query.iter(){
        println!("O apelido de {} é {}", name.0, nickname.0);
    }    
}


fn main() {
    App::new()
        .add_plugins(bevy::DefaultPlugins)
        .add_plugin(StartPlugin)
        .add_plugin(MessagePlugin)
        .run();
}