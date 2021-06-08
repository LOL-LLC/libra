# Restarting Network from Snapshot
Sometimes the network may need to re-genesis, based off of information from a snapshot of a network:

Use cases: 
- For major revisions of codebase, where vm, stdlib, and framework code are not mutually backwards compatible.
- Simulation of onchain state in a local swarm.
- Catastrophic network halt.

##  TRANSFORM STATE SNAPSHOT
// get state snapshot
// get list of accounts

```
https://github.com/LOL-LLC/libra/blob/85b291dc68245b0fc955eabad5c9a5474ae8ea3a/storage/backup/backup-cli/src/backup_types/state_snapshot/restore.rs#L132-L138

  for chunk in manifest.chunks {
      let blobs = self.read_account_state_chunk(chunk.blobs).await?;
      let proof = self.storage.load_lcs_file(&chunk.proof).await?;

      receiver.add_chunk(blobs, proof)?;
      leaf_idx.set(chunk.last_idx as i64);
  }

```

// for each account on chain

```
https://github.com/LOL-LLC/libra/blob/85b291dc68245b0fc955eabad5c9a5474ae8ea3a/storage/backup/backup-cli/src/backup_types/state_snapshot/restore.rs#L144

    async fn read_account_state_chunk(
        &self,
        file_handle: FileHandle,
    ) -> Result<Vec<(HashValue, AccountStateBlob)>> {
        let mut file = self.storage.open_for_read(&file_handle).await?;

        let mut chunk = vec![];

        while let Some(record_bytes) = file.read_record_bytes().await? {
            chunk.push(lcs::from_bytes(&record_bytes)?);
        }

        Ok(chunk)
    }
```

// parse account state

```
let blob: AccountStateBlob;

let state: AccountState = blob::try_from()
```

// retrieve state we need.

// apply transformations.

// export to migration.json

## GENESIS
// Genesis reads from migration.json
Create a function such as vm-genesis/lib.rs encode_genesis_transaction, which creates genesis transaction from known account state.

The operator assignments and registrations were already in the original state, so we don't need to run those Move transactions, but simply initialize the account state appropriately. This could possibly be achieved with a WriteSet of the the account state.


```
https://github.com/LOL-LLC/libra/blob/9b01b5e7fc2bca87e3b97d4eb68614e53ad32f33/language/tools/vm-genesis/src/lib.rs#L74-L93

pub fn encode_genesis_transaction(
    libra_root_key: Option<&Ed25519PublicKey>, //////// 0L ////////
    treasury_compliance_key: Option<&Ed25519PublicKey>, //////// 0L ////////
    operator_assignments: &[OperatorAssignment],
    operator_registrations: &[OperatorRegistration],
    vm_publishing_option: Option<VMPublishingOption>,
    chain_id: ChainId,
) -> Transaction {
    Transaction::GenesisTransaction(WriteSetPayload::Direct(encode_genesis_change_set(
        libra_root_key,
        treasury_compliance_key,
        operator_assignments,
        operator_registrations,
        stdlib_modules(StdLibOptions::Compiled), // Must use compiled stdlib,
        //////// 0L ////////
        vm_publishing_option
            .unwrap_or_else(|| VMPublishingOption::open()), // :)
        chain_id,
    )))
}
```