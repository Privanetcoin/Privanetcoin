use halo2_proofs::{circuit::{Layouter, Value}, plonk::{Circuit, ConstraintSystem, Error}, pasta::pallas};

#[derive(Clone)]
pub struct PrivateTxCircuit {
    sender: Value<pallas::Base>,
    receiver: Value<pallas::Base>,
    amount: Value<pallas::Base>,
}

impl Circuit<pallas::Base> for PrivateTxCircuit {
    type Config = ();
    type FloorPlanner = halo2_proofs::circuit::floor_planner::V1;

    fn configure(_: &mut ConstraintSystem<pallas::Base>) -> Self::Config {}

    fn synthesize(&self, _: Self::Config, mut layouter: impl Layouter<pallas::Base>) -> Result<(), Error> {
        layouter.assign_region(|| "private tx", |mut region| {
            self.sender.copy_advice(|| "sender", &mut region, 0, 0)?;
            self.receiver.copy_advice(|| "receiver", &mut region, 0, 1)?;
            self.amount.copy_advice(|| "amount", &mut region, 0, 2)?;
            Ok(())
        })
    }
}
