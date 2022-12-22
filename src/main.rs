use std::fs::read_to_string;
use std::thread;
use std::time::Duration;

const FRAMECOUNT: u16 = 6532;
const SLEEP_MICROSECONDS: u64 = 32995;

#[derive(Debug, Default)]
struct BadApple {
    frame_num: u16,
    frames_passed: u16,
    fps: u16,
}

impl BadApple {
    fn new(frame_num: u16) -> BadApple {
        BadApple { frame_num, frames_passed: 0, fps: 0 }
    }

    fn next_frame(&mut self) {
        self.frame_num += 1;
        self.frames_passed += 1;
        print!("\x1B[2J\x1B[1;1H");
        println!("| Frame: {} | FPS: {} |\n{}", self.frame_num, self.fps, read_to_string(format!("frames/BA{}.txt", self.frame_num)).unwrap());
        //debug version
        //println!("| Frame: {} | FPS: {} |\n{}\n{}", self.frame_num, self.fps, read_to_string(format!("frames/BA{}.txt", self.frame_num)).unwrap(), format!("{:#?}", self));
    }

    fn update_fps(&mut self) {
        self.fps = self.frames_passed * 2;
        self.frames_passed = 0;
    }
}

fn main() {
    let mut bad_apple = BadApple::new(40);

    let mut previous_secs = std::time::SystemTime::now();

    for _ in 0..FRAMECOUNT {
        bad_apple.next_frame();
        thread::sleep(Duration::from_micros(SLEEP_MICROSECONDS));
        
        //TODO: seperate thread to update fps instead of checking each frame on the main thread
        //but it is fine for now
        if previous_secs.elapsed().unwrap().as_millis() >= 500 {
            bad_apple.update_fps();
            previous_secs = std::time::SystemTime::now();
        }
    }
}
