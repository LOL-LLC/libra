# Restarting Network from Snapshot
Sometimes the network may need to re-genesis, based off of information from a snapshot of a network:

Use cases: 
- For major revisions of codebase, where vm, stdlib, and framework code are not mutually backwards compatible.
- Simulation of onchain state in a local swarm.
- Catastrophic network halt.

##  TRANSFORM STATE SNAPSHOT
// get state snapshot
// get list of accounts
// for each account on chain


// parse account state
let blob: AccountStateBlob;

let state: AccountState = blob::try_from()

// retrieve state we need.

// apply transformations.

// export to migration.json

## GENESIS
// Genesis reads from migration.json
// tools/vm-genesis/ to have a function to restore from migration.