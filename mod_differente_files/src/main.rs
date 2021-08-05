mod front_of_house;
mod mod_using_mod_file;

pub use crate::front_of_house::hosting;
use crate::mod_using_mod_file::haha::print_haha;

fn main() {
    hosting::add_to_waitlist();
    print_haha();
}
