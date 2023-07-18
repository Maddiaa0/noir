use noirc_driver::CompiledProgram;
use rand::Rng;

pub struct TestOptions {
    pub fuzz_runs: u32,
}

impl TestOptions {
    pub fn new(fuzz_runs: u32) -> Self {
        Self { fuzz_runs }
    }
}

/// Given a function, it returns a strategy which generates valid calldata
/// for that function's input types.
pub fn fuzz_params(program: &CompiledProgram) -> InputMap {
    // pub fn fuzz_calldata(program: CompiledProgram) -> BoxedStrategy<Vec<AbiParameter>> {
    // We need to compose all the strategies generated for each parameter in all
    // possible combinations
    let mut inputsMap = InputMap::new();
    let strats = program.abi.parameters.iter().for_each(|input| {
        let value = fuzz_param(&input.typ);
        inputsMap.insert(input.name.clone(), value);
    });

    inputsMap
}

// TODO: use proptest for this
pub fn fuzz_param(param: &AbiType) -> InputValue {
    match param {
        AbiType::Field => {
            // Why using an i128 here?
            let new_value = FieldElement::from(rand_i128());
            return InputValue::Field(FieldElement::from(rand_i128()));
        }
        _ => panic!("Implement fuzzing for other types"),
    }
}

fn rand_i128() -> i128 {
    let mut rng = rand::thread_rng();
    let upper: u64 = rng.gen();
    let lower: u64 = rng.gen();

    // lmao itll do
    i128::from(upper) << 64 | i128::from(lower)
}
