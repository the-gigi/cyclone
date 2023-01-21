use std::thread::sleep;
use std::time::Duration;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

fn main() {
    println!("Cyclone 🌀 is running...");
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    loop {
        let data = ctx.get_contents().unwrap_or("".to_string());
        if data.starts_with("git@") {
            let modified_data = "git clone ".to_owned() + &data;
            println!("modified clipboard: {modified_data}");
            ctx.set_contents(modified_data).unwrap();
        }
        sleep(Duration::from_secs(1));
    }
}
