mod to_do;

use to_do::enum_factory::to_do_factory;
use to_do::enums::TaskStatus;

use to_do::enum_factory::ItemTypes;

fn main() {
    let to_do_item = to_do_factory("washing", TaskStatus::PENDING);
    match to_do_item {
        ItemTypes::Done(item) => {
            println!("Title: {}", item.super_struct.title);
            println!("Status: {}", item.super_struct.status);
        },
        ItemTypes::Pending(item) => {
            println!("Title: {}", item.super_struct.title);
            println!("Status: {}", item.super_struct.status);
        }
    }

}