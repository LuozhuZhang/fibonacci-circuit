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
    // marker在这里并没有实际意义，只不过假装用到了parameter F，防止compiler报错
    _marker: PhantomData<F>,
}

impl<F: FieldExt> FiboChip<F> {
    // 这是一个function（关联函数），返回实例自身
    fn construct(config: FiboConfig) -> Self {

    }

    // 输入ConstraintSystem，返回FiboConfig
    // ConstraintSystem必须要带一个参数<F>
    fn configure(meta: &mut ConstraintSystem<F>) -> FiboConfig {

    }

    fn assign()
}

fn main() {
    println!("Hello, world!");
}
