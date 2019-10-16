use crate::test::gas_meter::bls12;
use crate::test::gas_meter::bn;

use crate::public_interface::API;
use crate::public_interface::constants::*;
use crate::public_interface::sane_limits::*;

use crate::test::parsers::*;

use crate::public_interface::constants::*;

use rand::{Rng, thread_rng};
use rand::distributions::Distribution;
use rand::distributions::Uniform;


mod gen_params;

#[test]
fn run_pseudo_curves_monte_carlo() {
    assert!(std::option_env!("GAS_METERING").is_some());

    let mut rng = thread_rng();
    const SAMPLES: usize = 30_000;
    // const SAMPLES: usize = 1_000_000;

    let curve_rng = Uniform::new_inclusive(BLS12, BN);

    let bls12_curves = read_dir_and_grab_curves::<JsonBls12PairingCurveParameters>("src/test/test_vectors/bls12/");
    let bn_curves = read_dir_and_grab_curves::<JsonBnPairingCurveParameters>("src/test/test_vectors/bn/");

    let mut bls12_writer = bls12::Bls12ReportWriter::new_for_path(format!("src/test/gas_meter/bls12/monte_carlo_f_exp_{}.csv", SAMPLES));
    let mut bn_writer = bn::BnReportWriter::new_for_path(format!("src/test/gas_meter/bn/monte_carlo_f_exp_{}.csv", SAMPLES));

    let x_bits_rng = Uniform::new_inclusive(1, MAX_BLS12_X_BIT_LENGTH);
    let u_bits_rng = Uniform::new_inclusive(1, MAX_BN_U_BIT_LENGTH);

    let bls12_curves_rng = Uniform::new(0, bls12_curves.len());
    let bn_curves_rng = Uniform::new(0, bn_curves.len());

    use pbr::ProgressBar;

    let mut pb = ProgressBar::new(SAMPLES as u64);
    let pairs = vec![2, 4, 6];

    for _ in 0..SAMPLES {
        let curve_type = curve_rng.sample(&mut rng);
        pb.inc();
        match curve_type {
            BLS12 => {
                let x_bits = x_bits_rng.sample(&mut rng);
                let x_hamming = Uniform::new_inclusive(1, x_bits);
                let x_hamming = x_hamming.sample(&mut rng);
                let curve_num = bls12_curves_rng.sample(&mut rng);
                let (curve, _) = (& bls12_curves[curve_num]).clone();
                for num_pairs in pairs.iter() {
                    let reports = bls12::process_for_curve_and_bit_sizes(curve.clone(), x_bits, x_hamming, *num_pairs);
                    for r in reports.into_iter() {
                        bls12_writer.write_report(r);
                    }
                }                
            },
            BN => {
                let u_bits = u_bits_rng.sample(&mut rng);
                let u_hamming = Uniform::new_inclusive(1, u_bits);
                let u_hamming = u_hamming.sample(&mut rng);
                let curve_num = bn_curves_rng.sample(&mut rng);
                let (curve, _) = (& bn_curves[curve_num]).clone();
                for num_pairs in pairs.iter() {
                    let reports = bn::process_for_curve_and_bit_sizes(curve.clone(), u_bits, u_hamming, *num_pairs);
                    for r in reports.into_iter() {
                        bn_writer.write_report(r);
                    }
                }
            },
            0u8 => {
                unreachable!();
            },
            _ => {
                unimplemented!();
            }
        }
    }

    pb.finish_print("done");
}