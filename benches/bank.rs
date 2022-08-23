#[macro_use]
extern crate bencher;

use bencher::Bencher;
use std::path::PathBuf;
use tx_engine::{bank::Bank, transactions};

fn bench_handle_transactions(bench: &mut Bencher) {
    bench.iter(|| {
        let mut csv_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        csv_file.push("input_data");
        csv_file.push("multiple_client_transactions.csv");

        let mut bank = Bank::new();
        let actual_transactions =
            transactions::parse_transactions(&csv_file).expect("Parsing transactions failed");
        bank.handle_transactions(actual_transactions);
    })
}

benchmark_group!(benches, bench_handle_transactions);
benchmark_main!(benches);
