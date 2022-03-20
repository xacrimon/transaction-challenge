# transaction-challenge

This is a toy project to implement a processor that transforms a list of transactions in a supplied CSV into a list of accounts and their current state.

Some tradeoffs were made here that wouldn't be suitable in a production system:
- Partial transaction data is stored in memory in order to refer back to them from other transactions. For large production data sets, this would probably have to be stored in some kind of searchable database, SQLite and LMDB come to mind as good options.
- Account state is single threaded. In a concurrent production system, it may be useful to use fine lock granularity for the accounts so that multiple threads may access accounts at the same time. Alternatively, just stored account state in some kind of database.

We also make some assumptions about the environment and data:
- We lock accounts indefinently on chargeback, all further transactions fail with an error and get discarded.
- Failed transactions are simply ignored and further ones for an account are still recognized.

CSV output is written to stdout and all errors encountered during processing are written to stderr.

The `insta` crate is employed to perform unit testing on the sample data found in `sample.csv`.
