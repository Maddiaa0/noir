use std::collections::HashMap;

use self::{artifact::BrilligArtifact, brillig_gen::BrilligGen};

pub(crate) mod artifact;
pub(crate) mod binary;
pub(crate) mod brillig_gen;
pub(crate) mod memory;

use crate::ssa_refactor::{
    ir::function::{Function, FunctionId, RuntimeType},
    ssa_gen::Ssa,
};
/// Context structure for the brillig pass.
/// It stores brillig-related data required for brillig generation.
#[derive(Default)]
pub struct Brillig {
    /// Maps SSA functions to their brillig opcode
    ssa_function_to_brillig: HashMap<FunctionId, BrilligArtifact>,
}

impl Brillig {
    /// Compiles a function into brillig and store the compilation artifacts
    pub(crate) fn compile(&mut self, func: &Function) {
        let obj = BrilligGen::compile(func);
        self.ssa_function_to_brillig.insert(func.id(), obj);
    }
}

impl std::ops::Index<FunctionId> for Brillig {
    type Output = BrilligArtifact;
    fn index(&self, id: FunctionId) -> &Self::Output {
        &self.ssa_function_to_brillig[&id]
    }
}

impl Ssa {
    /// Generate compilation artifacts for brillig functions
    pub(crate) fn to_brillig(&self) -> Brillig {
        let mut brillig = Brillig::default();
        for f in self.functions.values().filter(|func| func.runtime() == RuntimeType::Brillig) {
            let id = f.id();
            if id != self.main_id {
                brillig.compile(f);
            }
        }
        brillig
    }
}
