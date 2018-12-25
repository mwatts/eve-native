use indexes::WatchDiff;
use ops::Interner;

pub trait Watcher {
    fn get_name(&self) -> String;
    fn set_name(&mut self, &str);
    fn on_diff(&mut self, interner: &mut Interner, diff: WatchDiff);
}

pub mod compiler;
pub mod console;
pub mod editor;
pub mod file;
pub mod remote;
pub mod system;
pub mod textcompiler;
pub mod websocket;
