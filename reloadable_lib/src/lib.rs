#[macro_use]
extern crate live_reload;

mod shared_api;

use live_reload::ShouldQuit;
use shared_api::Host;

live_reload! {
    host: Host;
    state: State;
    init: init;
    reload: reload;
    update: update;
    unload: unload;
    deinit: deinit;
}

#[repr(C)]
struct State {
    counter: usize,
}

fn aaaa() -> i32 {
    5
}

fn init(host: &mut Host, state: &mut State) {
    println!("AAAAAAAAAA");
    host.funcs.push(aaaa);
}

fn reload(host: &mut Host, state: &mut State) {}

fn update(host: &mut Host, state: &mut State) -> ShouldQuit {
    ShouldQuit::No
}

fn unload(host: &mut Host, state: &mut State) {}

fn deinit(host: &mut Host, state: &mut State) {}
