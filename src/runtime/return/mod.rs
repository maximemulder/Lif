pub mod control;
pub mod flow;
pub mod jump;

use crate::runtime::error::Error;
use crate::runtime::reference::GcReference;
use crate::runtime::value::GcValue;

pub use control::Control;
pub use flow::Flow;
pub use jump::Jump;

pub type Return<T> = Result<T, Error>;

pub type ReturnReference<'a> = Return<GcReference<'a>>;

pub type ReturnValue<'a> = Return<GcValue<'a>>;

pub type ReturnFlow<'a> = Result<GcReference<'a>, Flow<'a>>;

pub fn flow<'a, T>(result: Result<T, Error>) -> Result<T, Flow<'a>> {
    result.map_err(Flow::Error)
}

pub fn flow_control_is<'a>(r#return: &ReturnFlow<'a>, control: Control) -> bool {
    if let Err(flow) = r#return {
        if let Flow::Jump(jump) = flow {
            if jump.control == control {
                return true;
            }
        }
    }

    false
}

pub fn flow_loop_reference<'a>(r#return: &ReturnFlow<'a>) -> Option<GcReference<'a>> {
    match r#return {
        Ok(reference) => Some(reference.clone()),
        Err(flow) => match flow {
            Flow::Error(_) => None,
            Flow::Jump(jump) => match jump.control {
                Control::Continue => Some(jump.reference),
                Control::Break => Some(jump.reference),
                Control::Return => None,
            },
        },
    }
}
