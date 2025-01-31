use std::env;
use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};

pub fn play_sound(sound: &str) {
    // Use rodio to play a sound
    let (_stream, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();

    let exe = env::current_exe().unwrap(); // exe path
    let wd = exe.parent().unwrap();
    let sound = wd.join(format!("assets/{}",sound));

    let file = BufReader::new(File::open(sound).unwrap());
    let source = Decoder::new(file).unwrap();
    sink.append(source);

    // Sleep for the duration of the sound to ensure it plays completely
    sink.sleep_until_end();
}