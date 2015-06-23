use std::fs::File;
use std::path::Path;

mod audio;
mod video;
mod cpu;
mod io;
mod memory;

pub fn run(rom: &str) {
	let rom_file = match File::open(&Path::new(rom)) {
		Ok(file) => file,
		Err(err) => panic!("{}", err),
	};
    
	let mut emu = Emu::new(rom_file);
	
	emu.start();
}

struct Emu {
	audio: audio::Audio,
	video: video::Video,
	cpu: cpu::CPU,
	io: io::IO,
	memory: memory::Memory,
}

impl Emu {
	pub fn new(rom: File) -> Emu {
		let audio = audio::Audio::new();
		let video = video::Video::new();
		let cpu = cpu::CPU::new();
		let io = io::IO::new();
		let memory = memory::Memory::new(&rom);
	
		Emu {
			audio: audio,
			video: video,
			cpu: cpu,
			io: io,
			memory: memory,
		}
	}

	pub fn start(&mut self) {
		// figure this out after filling in the cpu, audio,
		// video, and interrupts

		// cpu execute and return num cycles
		// video execute until caught up with number of cycles executed by cpu
		// audio execute until caught up with number of cycles exected by cpu
		// handle interrupts

		// if we have to, draw stuff (figure out what to use)
		// do we do audio here, too?
		// how is IO handled?
	}

	pub fn interrupts(&self) {
		// figure this out after cpu, video, audio
	}
}

