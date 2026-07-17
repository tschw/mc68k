
// fixpoint precision
const ONE: i32 = 0x4000;

const ONE_F: f32 = ONE as f32;

// semantics of the MULS instruction of the MC68K
fn mul(x: i16, y: i16) -> i32 { x as i32 * y as i32 }

struct Complex(i16, i16);

const ONE_C: Complex = Complex(ONE as i16, 0);

impl Complex {

    fn mul(&self, other: &Complex) -> Complex {
        Complex(
            ( ( mul(self.0, other.0) - mul(self.1, other.1) ) / ONE ) as i16,
            ( ( mul(self.0, other.1) + mul(self.1, other.0) ) / ONE ) as i16 )
    }
}

fn main() {

    const N: usize = 0x100;
    const I_PI: usize = N >> 1;
    const COS_OFFS: usize = I_PI >> 1;
    const STEP_ANGLE: f32 = std::f32::consts::PI / I_PI as f32;

    let mut table = [0; N];

    let step = Complex(
            ( ONE_F * f32::cos(STEP_ANGLE) ) as i16,
            ( ONE_F * f32::sin(STEP_ANGLE) ) as i16 );

    println!("Step: {:04x}+i{:04x}", step.0, step.1);

    let mut sincos = ONE_C;

    for i in 0..COS_OFFS {
        table[ i ] = sincos.1;
        table[ i + COS_OFFS ] = sincos.0;
        table[ I_PI + i ] = - sincos.1;
        table[ I_PI + i + COS_OFFS ] = - sincos.0;

        sincos = sincos.mul(&step);
    }

    for i in 0..N {
        let a = i as f32 * STEP_ANGLE;
        println!("{:}: {:04x} {:04x}",
                i, table[i], (ONE_F * f32::sin(a)) as i16);
    }
}
