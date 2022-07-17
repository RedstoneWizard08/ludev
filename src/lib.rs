pub use faber;
pub use forgic;
pub use libic;
pub use ludic;
pub use quantum_mc;
pub use vanel;

use neon::prelude::{ModuleContext, NeonResult};

#[neon::main]
pub fn main(mut ctx: ModuleContext) -> NeonResult<()> {
    ctx.export_function("getFabricVersions", ludic::converters::faber::get_fabric_versions)?;
    ctx.export_function("getForgeVersions", ludic::converters::forgic::get_forge_versions)?;
    return Ok(());
}
