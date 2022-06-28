use std::marker::PhantomData;

use halo2_proofs::{
    arithmetic::FieldExt,
    circuit::*,
    plonk::*, poly::Rotation,
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
    // 传入FiboConfig struct，返回FiboChip
    fn construct(config: FiboConfig) -> Self {
        Self { config, _marker: PhantomData }
    }

    // 输入ConstraintSystem，返回FiboConfig
    // ConstraintSystem必须要带一个参数<F>
    // 不是方法的关联函数，常作为返回一个结构体新实例的构造函数

    // configure是实际写circuit的地方，我们在这里定义custom gate等
    fn configure(meta: &mut ConstraintSystem<F>) -> FiboConfig {
        // ConstraintSystem主要做电路约束，里面有许多重要的API：https://docs.rs/halo2_proofs/latest/halo2_proofs/plonk/struct.ConstraintSystem.html
        // 比如 create_gate 和 advice_column 等，用meta作为parameter-argument来调用
        let col_a: Column<Advice> = meta.advice_column();
        let col_b: Column<Advice> = meta.advice_column();
        let col_c: Column<Advice> = meta.advice_column();
        let select: Selector = meta.selector();

        meta.enable_equality(col_a);
        meta.enable_equality(col_b);
        meta.enable_equality(col_c);

        meta.create_gate(name: "add", construct: |meta: &mut VirtualCells<F>| {
            //
            // col_a | col_b | col_c | selector
            //   a      b        c       s
            //

            // 这里的query也可以叫select，根据一个column得到里面的cell
            // 这里query出selector column
            let s: Expression<F> = meta.query_selector(selector);
            let a: Expression<F>  = meta.query_advice(col_a, Rotation::cur());
            let a: Expression<F> = meta.query_advice(col_b, Rotation::cur());
            let a: Expression<F>  = meta.query_advice(col_c, Rotation::cur());

            // return constraint
            // 让这个constraint = 0，所以可以enable selector
            vec![s * (a + b - c)];
        });

        // 写好circuit gate之后，就可以return了
        FiboConfig { 
            advice: [col_a, col_b, col_c],
            selector,
        }

    // fn assign()
    }
}

fn main() {
    println!("Hello, world!");
}
