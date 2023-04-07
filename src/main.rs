mod to_do;

use to_do::enum_factory::to_do_factory;
use to_do::enums::TaskStatus;
use to_do::enum_factory::ItemTypes;
// use to_do::traits::create::Create;
use to_do::traits::edit::Edit;
use to_do::traits::delete::Delete;
use to_do::traits::get::Get;

fn main() {
    let to_do_item = to_do_factory("washing", TaskStatus::DONE);

    match to_do_item {
        ItemTypes::Done(item) => {
            item.get(&item.super_struct.title);
            item.delete(&item.super_struct.title);
        },
        ItemTypes::Pending(item) => {
            item.get(&item.super_struct.title);
            item.set_to_done(&item.super_struct.title);
        }
    }

}