use rand::Rng;
use serenity::builder;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};

pub fn register(
    command: &mut builder::CreateApplicationCommand,
) -> &mut builder::CreateApplicationCommand {
    command
        .name("random_exercise")
        .description("Command for randmomizing exercise")
        .create_option(|option| {
            option
                .name("type")
                .description("Choose type of exercise")
                .kind(CommandOptionType::String)
                .add_string_choice("Piano", "piano")
                .add_string_choice("Drawing", "drawing")
                .required(true)
        })
}

pub fn run(options: &[CommandDataOption]) -> String {
    let option = options
        .get(0)
        .expect("Expected type of exercise")
        .resolved
        .as_ref()
        .expect("should work");
    if let CommandDataOptionValue::String(string) = option {
        //TODO: use database
        let exercise = match string.as_ref() {
            "piano" => {
                let piano_exercises = vec![
                    "Straight Beat",
                    "Left note, right cord",
                    "Broken chord ballad",
                    "Half beat bounce",
                    "1-2-123 ballad",
                    "Oom-pah",
                    "1-2-123-5-1 ballad",
                    "1-5-1-3-5 ballad",
                    "6/8 split",
                    "Seventh heaven",
                ];
                piano_exercises[rand::thread_rng().gen_range(0..piano_exercises.len())]
            }
            "drawing" => {
                let drawing_exercises = vec![
                    "Plane of elipsis",
                    "Super imposed lines",
                    "Ghosted lines",
                    "Table of elipsis",
                    "Table of elipsis",
                    "Funnel",
                    "Plotted perspective",
                    "Rough perspective",
                    "Rotated boxes",
                ];
                drawing_exercises[rand::thread_rng().gen_range(0..drawing_exercises.len())]
            }
            _ => "Invalid type",
        };
        format!("Exercise: {}", exercise)
    } else {
        "Invalid type".to_string()
    }
}
