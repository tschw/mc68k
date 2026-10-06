
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

    const N: usize = 0x200;
    const I_PI: usize = N >> 1;
    const I_PI_DIV_2: usize = I_PI >> 1;
    const I_PI_DIV_4: usize = I_PI >> 2;
    const STEP_ANGLE: f32 = std::f32::consts::PI / I_PI as f32;

    let mut table = [0; N];

    let step = Complex(
            ( ONE_F * f32::cos(STEP_ANGLE) ) as i16,
            ( ONE_F * f32::sin(STEP_ANGLE) ) as i16 );

    let mut sincos = ONE_C;

	//    -           -
	//  /   \       /    The last segment exists
	// 0  1  2  3  4  5  to get a full cosine from
	//        \   /      the table offset at PI/2.
	//          -
	// Each segment corresponds to a quarter of
	// the unit circle and can be generated from
	// an eighth of it by exploiting the symmetry
	// between the two functions:
	//
	//  ___ x=cos(pi/2)=0, y=sin(pi/2)=1
	// ^   ---
	// |      --
	// |        |
	// |         | x=cos(0)=1
	// 0---------> y=sin(0)=0
	//
	// sin(x) = cos(pi/2 - x)
	// cos(x) = sin(pi/2 - x)
	//
	// 0:  0         We need two writes for each
	// 1: 1/2 pi     segment. The last segment is
	// 2:     pi     only generated in assembler,
	// 3: 3/2 pi     so that is 8 writes here and
	// 4:  2  pi     10 writes in the assembler
	// 5  5/2 pi     version.

    for i in 0..I_PI_DIV_4+1 {
        table[ i ] = sincos.1;
        table[ I_PI_DIV_2 - i ] = sincos.0;

        table[ I_PI_DIV_2 + i ] = sincos.0;
        table[ I_PI - i ] = sincos.1;

        table[ I_PI + i ] = - sincos.1;
        table[ I_PI + I_PI_DIV_2 - i ] = - sincos.0;

        table[ I_PI + I_PI_DIV_2 + i ] = - sincos.0;
        if i > 0 {
            table[ I_PI + I_PI - i ] = - sincos.1;
        }
        sincos = sincos.mul(&step);
    }

    println!("Step: {:04x}+i{:04x}", step.0, step.1);

    let mut max_err = 0.0;
    let mut max_err_index = 0;

    for i in 0..I_PI_DIV_2 {
        let a = i as f32 * STEP_ANGLE;
        let d = f32::abs( f32::sin(a) - table[i] as f32 / ONE_F );
        if d > max_err {
            max_err = d;
            max_err_index = i;
        }
    }

    println!("Error: {:} @ {:}", max_err, max_err_index);

    for i in 0..N {
        let a = i as f32 * STEP_ANGLE;
        println!("{:}: {:04x} {:04x}",
                i, table[i], (ONE_F * f32::sin(a)) as i16);
   }
}
