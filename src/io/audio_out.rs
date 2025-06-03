use std::{
    fs::{DirEntry, read_dir},
    path::Path,
    thread::{self, JoinHandle},
};

use cpal::{
    StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use midi_player::{Player, PlayerController, Settings};

/* use alike
fn main() {
    /* A */
    let settings = Settings::builder().build();
    let (player, mut controller) =
        Player::new("examples/Nice-Steinway-Lite-v3.0.sf2", settings).unwrap();

    thread::spawn(|| {
        start_audio_loop(player);
    });

    /* B */
        let soundfont = "...";
        spawn_audio_loop(soundfont); // spawn
    /* END */

    thread::sleep(Duration::from_secs(2));

    controller
        .set_file(Some("examples/Sibelius_The_Spruce.mid"))
        .unwrap();

    controller.play();

    loop {}
}
*/

pub fn start_audio_loop(mut player: Player) {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("No output device available");
    let channels = 2 as usize;
    let config = StreamConfig {
        channels: channels as u16,
        sample_rate: cpal::SampleRate(player.settings().sample_rate),
        buffer_size: cpal::BufferSize::Fixed(player.settings().audio_buffer_size),
    };

    let err_fn = |err| eprintln!("An error occurred on the output audio stream: {}", err);

    let mut left = vec![0f32; player.settings().audio_buffer_size as usize];
    let mut right = vec![0f32; player.settings().audio_buffer_size as usize];

    let stream = device
        .build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                let sample_count = data.len() / channels;

                player.render(&mut left, &mut right);

                if !left.is_empty() {
                    for i in 0..sample_count {
                        data[channels * i] = left[i];
                        data[channels * i + 1] = right[i];
                    }
                }
            },
            err_fn,
            None,
        )
        .unwrap();

    stream.play().expect("cannot run audio stream");

    std::thread::park();
}

pub fn create_player(soundfont: &str) -> (Player, PlayerController) {
    let settings = Settings::builder().build();
    let (player, controller) = Player::new(soundfont, settings).unwrap();
    return (player, controller);
}

fn list_soundfonts() -> Result<Vec<DirEntry>, String> {
    let path = Path::new("src/sf2");
    let entries = read_dir(path)
        .unwrap()
        .filter_map(|e| e.ok())
        .collect::<Vec<DirEntry>>();

    for entry in &entries {
        let path = entry.path();

        if let Some(name) = path.file_name() {
            println!("{}", name.to_string_lossy());
        }
    }

    if entries.is_empty() {
        return Err("No files found".to_string());
    } else {
        return Ok(entries);
    }
}

fn select_soundfont() -> String {
    let entries = list_soundfonts();
		
		return "".to_string()
}

pub fn spawn_audio_loop() -> JoinHandle<()> {
    let soundfont = select_soundfont();
    let (player, _controller) = create_player(&soundfont);

    return thread::spawn(|| {
        start_audio_loop(player);
    });
}
