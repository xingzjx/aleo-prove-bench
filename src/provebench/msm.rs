use snarkvm::{algorithms::msm::VariableBase, fields::PrimeField, prelude::AffineCurve};
use benchmarking;
use rand::thread_rng;
use crate::metrics::print_result;

fn create_scalar_bases<G: AffineCurve<ScalarField = F>, F: PrimeField>(size: usize) -> (Vec<G>, Vec<F::BigInteger>) {
    let rng = &mut thread_rng();

    let bases = std::iter::repeat((0..(size / 1000)).into_iter().map(|_| G::rand(rng)).collect::<Vec<_>>())
        .take(1000)
        .flatten()
        .collect::<Vec<_>>();
    let scalars = (0..size).into_iter().map(|_| F::rand(rng).to_bigint()).collect::<Vec<_>>();
    (bases, scalars)
}

pub fn variable_base_bls12_377(size: usize) {
    use snarkvm::curves::bls12_377::{Fr, G1Affine};

    let (bases, scalars) = create_scalar_bases::<G1Affine, Fr>(size);
    let result = benchmarking::measure_function(move |b| b.measure(|| VariableBase::msm(&bases[..size], &scalars[..size]))).unwrap();

    print_result("MSM on BLS12-377", result);
}

fn variable_base_edwards_bls12(size: usize) {
    use snarkvm::curves::edwards_bls12::{EdwardsAffine, Fr};

    let (bases, scalars) = create_scalar_bases::<EdwardsAffine, Fr>(size);
    let result = benchmarking::measure_function(move |b| b.measure(|| VariableBase::msm(&bases[..size], &scalars[..size]))).unwrap();

    print_result("MSM on Edwards-BLS12", result);
}

// fn test3(size: usize) {
//     use snarkvm::curves::bls12_377::{Fr, G1Projective};
//     const SAMPLES: usize = 1 << 10;

//     let mut rng = Default::default();

//     let v = (0..SAMPLES - 1).map(|_| Fr::rand(&mut rng).to_bigint()).collect::<Vec<_>>();
//     let g = (0..SAMPLES).map(|_| G1Projective::rand(&mut rng).to_affine()).collect::<Vec<_>>();

//     // let naive = naive_variable_base_msm(g.as_slice(), v.as_slice());
//     // let fast = VariableBase::msm(g.as_slice(), v.as_slice());

//     let result = benchmarking::measure_function(move |b| b.measure(|| VariableBase::msm(g.as_slice(), v.as_slice()))).unwrap();

//     print_result("MSM on test3", result);
// }

pub fn bench(size: usize) {
    variable_base_bls12_377(size);
    variable_base_edwards_bls12(size);
    // test3(size);
}