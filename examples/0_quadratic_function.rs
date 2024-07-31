use plonky2::{
    field::{goldilocks_field::GoldilocksField, types::Field},
    iop::witness::{PartialWitness, WitnessWrite},
    plonk::{
        circuit_builder::CircuitBuilder, circuit_data::CircuitConfig,
        config::PoseidonGoldilocksConfig,
    },
    iop::target::Target,
};

use anyhow::Result;

type F = GoldilocksField;
type C = PoseidonGoldilocksConfig;
const D: usize = 2;

fn build_circuit() -> (CircuitBuilder<F, D>, Target) {
    let config = CircuitConfig::standard_recursion_config();
    let mut builder = CircuitBuilder::<F, D>::new(config);

    let x_t = builder.add_virtual_target();
    let minus_x_t = builder.neg(x_t);
    let minus_2x_t = builder.mul_const(F::from_canonical_u64(2), minus_x_t);
    let x2_t = builder.exp_u64(x_t, 2);
    let one_t = builder.one();
    let zero_t = builder.zero();
    let poly_t = builder.add_many(&[x2_t, minus_2x_t, one_t]);
    builder.connect(poly_t, zero_t); // x^2 - 2x + 1 = 0

    (builder, x_t)
}

fn create_witness(x_t: Target) -> PartialWitness<F> {
    let mut pw = PartialWitness::<F>::new();
    pw.set_target(x_t, GoldilocksField(1)); // set x = 1
    pw
}

fn prove_and_verify(circuit: plonky2::plonk::circuit_data::CircuitData<F, C, D>, pw: PartialWitness<F>) -> Result<()> {
    let proof = circuit.prove(pw)?;
    circuit.verify(proof)?;
    Ok(())
}

fn main() -> Result<()> {
    let (builder, x_t) = build_circuit();
    let circuit = builder.build::<C>();
    let pw = create_witness(x_t);
    prove_and_verify(circuit, pw)
}
