# transaction-challenge

This is a toy project to implement a processor that transforms a list of transactions in a supplied CSV into a list of accounts and their current state.

Some tradeoffs were made here that wouldn't be suitable in a production system:
- Partial transaction data is stored in memory in order to refer back to them from other transactions. For large production data sets, this would probably have to be stored in some kind of searchable database, SQLite and LMDB come to mind as good options.
- Account state is single threaded. In a concurrent production system, it may be useful to use fine lock granularity for the accounts so that multiple threads may access accounts at the same time. Alternatively, just stored account state in some kind of database.