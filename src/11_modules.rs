use app::hello;
use app::utils::echo as show;

mod app {

    use crate::app::utils::echo;

    pub mod utils;

    pub fn hello() {
        echo("Hello");
    }

}

fn main()  {
    hello();
    show("test");
}
