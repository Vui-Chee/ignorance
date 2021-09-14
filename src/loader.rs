use loading::Loading;

use std::thread;
use std::time::Duration;

pub fn display_loader(iterations: i32) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut loading = Loading::new();

        loading.start();

        for _i in 0..iterations {
            loading.text("Generating .gitignore".to_string());
            thread::sleep(Duration::from_millis(50));
        }

        loading.end();
    })
}
