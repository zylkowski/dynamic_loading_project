extern crate live_reload;

mod shared_api;
use live_reload::ShouldQuit;
use shared_api::Host;
use std::io::Write;
use std::thread;
use std::time::Duration;

type App<'a> = live_reload::Reloadable<shared_api::Host<'a>>;

fn main() {
    let mut a: Vec<fn() -> i32> = Vec::new();
    {
        let host = Host { funcs: &mut a };
        let mut app = App::new("target/debug/reloadable.dll", host).expect("Should load!");
        thread::sleep(Duration::from_secs(2));

        println!("{}", app.host().funcs[0]());
    }
    println!("{}", a[0]());
}
