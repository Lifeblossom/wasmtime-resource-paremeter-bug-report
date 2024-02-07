use crate::bindings::exports::component::bug_report::resource1_interface;
use crate::bindings::exports::component::bug_report::resource2_interface;
use crate::bindings::exports::component::bug_report::resource2_interface::{
    OwnResource1, OwnResource2,
};

mod bindings;

struct Component;

pub struct Resource1;

impl resource1_interface::Guest for Component {
    fn create() -> resource1_interface::OwnResource1 {
        OwnResource1::new(Resource1 {})
    }
}

pub struct Resource2;

impl resource2_interface::Guest for Component {
    fn create_resource2(_resource1: OwnResource1) -> OwnResource2 {
        OwnResource2::new(Resource2 {})
    }

    fn do_stuff(_resource1: OwnResource1, _resource2: OwnResource2) -> OwnResource2 {
        OwnResource2::new(Resource2 {})
    }
}
