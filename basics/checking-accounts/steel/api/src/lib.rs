pub mod consts;
pub mod instruction;

pub mod prelude {
    pub use crate::consts::*;
    pub use crate::instruction::*;
}

use steel::*;

declare_id!("FFJjpuXzZeBM8k1aTzzUrV9tgboUWtAaKH6U2QudoH2K"); 