# Tasks

All steps:

- [x] Run `cargo new tx_engine`
- [x] Add files:
    - .gitattributes
    - .gitignore
    - LICENCE
    - README.md
- [x] Make transaction & read_transactions
- [x] Add ParseTxError & report/change context
- [x] Add Account{balance}
- [x] Add way to deposit, withdraw, dispute, resolve, & chargeback money + basic tests
- [x] Add extra check to validate the optional amount field
      + tests in both invalid cases
- [x] Add bank containing clients + incl tests in same file
- [x] Add client report + output
/Solutions:
- [ ] BAD: Select all disputable txs so amount can be found for indisputable txs, make sure a failed withdrawal is removed from the HashMap.
           Issues seen:
           - Cannot be (easily) parallelized as transactions are based on previous ones
           - If a withdrawal is removed, it will still report an error on the partner's side, even the transaction did exist in the CSV
           --> Benchmark for test bench_handle_transactions:      28,299 ns/iter (+/- 4,332)
- [x] FINE: Group transactions per client...
           --> Benchmark for test bench_handle_transactions:      27,622 ns/iter (+/- 2,848)
            ...and execute each group on separate thread
           --> not done due to design limitation with the need to search through mutable transactions :(
- [x] Add benchmark test
- [x] Improve error reporting: remove most `unwrap`s & `expect`s
- [x] Add logging to successful & failed transactions
/todo
- [ ] Refactor: client txs can be handled on separate threads
- [ ] Extract common logic from tests, incl. parameterized tests
- [ ] Add more tests for checking different results
- [ ]
