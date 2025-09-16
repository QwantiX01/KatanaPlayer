use std::{
    fs::File,
    sync::{Arc, Mutex},
};

use lazy_static::lazy_static;
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};

pub struct AudioService {
    _stream_handle: OutputStream,
    sink: Arc<Mutex<Sink>>,
}

impl AudioService {
    pub fn new() -> AudioService {
        let stream_handle =
            OutputStreamBuilder::open_default_stream().expect("open default audio stream");
        let sink = Arc::new(Mutex::new(Sink::connect_new(&stream_handle.mixer())));

        AudioService {
            _stream_handle: stream_handle,
            sink: sink,
        }
    }

    pub fn play(&self, path: String) {
        let file = File::open(path).unwrap();
        let source = Decoder::try_from(file).unwrap();
        self.sink.lock().unwrap().append(source);
        self.sink.lock().unwrap().play();
    }

    pub fn set_volume(&self, percentage: u8) {
        println!("{}", percentage);
        self.sink
            .lock()
            .unwrap()
            .set_volume(percentage as f32 / 100f32);
    }

    pub fn pause(&self) {
        self.sink.lock().unwrap().pause();
    }
}

lazy_static! {
    pub static ref AUDIO_SERVICE_INSTANCE: AudioService = AudioService::new();
}
