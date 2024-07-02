#![allow(non_snake_case)]

use dlopen2::wrapper::{Container, WrapperApi};
use nwd::NwgUi;
use nwg::NativeUi;

#[derive(Default, NwgUi)]
pub struct MainWindow {
    #[nwg_control(size: (640, 480), title: "Encryption Software", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [Self::exit_app])]
    window: nwg::Window,

    #[nwg_control(text: "2", size: (100, 20), position: (10, 10), focus: false)]
    num1: nwg::TextInput,

    #[nwg_control(text: "2", size: (100, 20), position: (120, 10), focus: false)]
    num2: nwg::TextInput,

    #[nwg_control(text: "SUM", size: (50, 20), position: (240, 10))]
    #[nwg_events( OnButtonClick: [MainWindow::sum] )]
    button: nwg::Button,

    #[nwg_control(text: "", size: (100, 20), position: (10, 40))]
    result: nwg::TextInput,
}

impl MainWindow {
    fn exit_app(&self) {
        nwg::stop_thread_dispatch();
    }

    fn sum(&self) {
        let cont: Container<Api> = unsafe { Container::load("lib/target/main.dll") }.expect("Could not open library or load symbols");
        
        let x = self.num1.text().parse::<i64>().unwrap();
        let y = self.num2.text().parse::<i64>().unwrap();
        
        let result = cont.sum(x, y);

        self.result.set_text(&result.to_string());
    }
}

#[derive(WrapperApi)]
struct Api {
    sum: fn(x: i64, y: i64) -> i64,
}

fn main() {
    println!("Hello, world!");

    let cont: Container<Api> = unsafe { Container::load("lib/target/main.dll") }.expect("Could not open library or load symbols");
    let result = cont.sum(2, 2);
    println!("2 + 2 = {}", result);

    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    
    let _app = MainWindow::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}
