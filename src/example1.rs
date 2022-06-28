use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::*,
    plonk::*,
};

#[derive(Debug, Clone)]
// * 1.Config
struct FiboConfig {
    // 在这里定义advice column的数量
    pub advice: [Column<Advice>; 3],
    pub selector: Selector,
}

struct FiboChip<F: FieldExt> {
    config: FiboConfig,
    _marker: PhantomData<F>,
}

fn main() {
    println!("Hello, world!");
}
