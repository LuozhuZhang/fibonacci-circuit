use std::{marker::PhantomData, default};

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
    
    // * 注意：我们这里采用了第二种写法，把columns放到 MyCircuit 的 configure 函数里面定义
    // * 这样做的好处就是可以复用columns，传到不同的Chip里
    fn configure(
        meta: &mut ConstraintSystem<F>,
        advice: [Column<Advice>; 3],
        selector: Selector,
    ) -> FiboConfig {
        // ConstraintSystem主要做电路约束，里面有许多重要的API：https://docs.rs/halo2_proofs/latest/halo2_proofs/plonk/struct.ConstraintSystem.html
        // 比如 create_gate 和 advice_column 等，用meta作为parameter-argument来调用
        let col_a = advice[0];
        let col_b = advice[1];
        let col_c = advice[2];
        let selector = meta.selector();

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

    // 这里定义的是在Fibochip impl context下的method
    fn assign_first_row(&self, mut layouter: impl layouter<F>, a: Option<F>, b: Option<F>) {

    }
}

#[derive(Default)]
struct MyCircuit<F> {
    pub a: Option<F>,
    pub b: Option<F>,
}

impl<F: FieldExt> Circuit<F> for MyCircuit<F> {
    type Config = FiboConfig;
    type FloorPlanner = SimpleFloorPlanner;

    fn without_witnesses(&self) -> Self {
        Self::default()
    }

    fn configure(meta: &mut ConstraintSystem<F>) -> Self::Config {
        let col_a = meta.advice_column();
        let col_b = meta.advice_column();
        let col_c = meta.advice_column();
        let selector = meta.selector();
        FiboChip::configure(meta, [col_a, col_b, col_c], selector)
        // 这里就会返回FiboConfig -> Config -> FiboConfig
    }

    fn synthesize(&self, config: Self::Config, layouter: impl Layouter<F>) -> Result<(), Error> {
        // 实例化？
        // 我们会复用这个chip，来design许多东西
        // construct里面主要是FibConfig，里面定义了我们需要的columns数量
        let chip = FiboChip::construct(config);
        // assign
        chip.assign_first_row(
            // namespace主要作用就是传入一个name
            // 在circuit::Layouter：https://docs.rs/halo2_proofs/0.2.0/halo2_proofs/circuit/trait.Layouter.html
            layouter.namespace(|| "first row"),
            self.a, self.b,
        )
    }
}

fn main() {
    println!("Hello, world!");
}
