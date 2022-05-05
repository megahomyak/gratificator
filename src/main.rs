use rand::prelude::IteratorRandom;

const DEFAULT_GRATIFICATIONS_FILE_PATH: &str = "gratifications.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tts_engine = tts::Tts::default()?;
    let command_line_arguments: Vec<String> = std::env::args().collect();
    let gratifications_file_path = match command_line_arguments.len() {
        1 => DEFAULT_GRATIFICATIONS_FILE_PATH,
        2 => &command_line_arguments[1],
        _ => panic!("You may only provide an optional path to the gratifications file!"),
    };
    let gratifications: Vec<String> =
        match std::fs::read_to_string(gratifications_file_path) {
            Err(error) => match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("Gratifications file was not found in the provided path!");
                },
                _ => panic!("{}", error)
            },
            Ok(gratifications_string) => gratifications_string,
        }
        .split('\n')
        .map(str::to_owned)
        .collect();
    assert!(
        !gratifications.is_empty(),
        "No gratifications were found in the provided file!"
    );
    loop {
        println!("Enter \"exit\" to exit, or any other string to hear a gratification:");
        let mut input_buffer = String::new();
        std::io::stdin().read_line(&mut input_buffer).unwrap();
        input_buffer = input_buffer.trim_end().to_lowercase();
        if input_buffer == "exit" {
            println!("Ok, bye!");
            break;
        }
        println!("Here you go!");
        tts_engine.speak(
            gratifications.iter().choose(&mut rand::thread_rng()).unwrap(),
            true
        ).unwrap();
    }
    Ok(())
}
