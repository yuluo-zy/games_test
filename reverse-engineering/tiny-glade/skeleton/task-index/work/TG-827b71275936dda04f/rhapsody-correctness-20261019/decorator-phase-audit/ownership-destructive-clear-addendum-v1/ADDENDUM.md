# Foreign ownership and destructive-clear supplement v1

This is a new supplement, not a modification of the accepted phase/cleanup contract. Base manifest remains SHA256 `99432aa3ae1a883f8f1c88f6152e676784fff765659a9e71d655ab0ca3ee122d`. No production/Cargo/GPU/target execution and no candidate source is supplied.

## Ownership gate

Read-only `task_queue.py show TG-889aa76ca030a2e3cd` confirms the entire A/B/C `crates/country-core/src/resources/walls/decorator_storage.rs` family is **claimed by foreign owner terrain-curve-20261018**. Root has explicitly confirmed that owner is not this team. No lock may be modified, released, superseded, or acquired by this audit. Correcting the production Storage/bucket helper requires collaboration with that owner or a legitimate source-family handoff coordinated by root. The old rhapsody TG827 work permits audit artifacts only; it does not grant any Storage source rights. A future candidate may not be saved under the rhapsody claim as a workaround. `foreign-storage-claim-readonly.json` preserves the exact observed claim.

## Complete original destructive clear

Original `Decorators::clear<DecoratorToStorageMap>` is `0x141257900/690B`, PDB module1488/record16476. Its complete body is retained in the base `native-pack/decorators-destructive-clear.{json,asm.txt}` and its original source lines150..157. It is not the inline tracking cleanup at source100/101.

Observed order, with direct instruction proof:

1. `1257923..7964`: clear bucket modified_ids at+0x60. Existing removed_ids at+0 are preserved initially.
2. `125798b..79d2`: obtain authority map length at+0x48, reserve removed-ID space if needed, and fold **all map keys** into the bucket removed-ID set. The complete154B helper `140b76260` iterates entries of stride0x50, loads key from entry+0, and calls the typed HashMap<DecoratorId,()>::insert `1402c9200`. It has no state-byte filter. This is union with previously removed IDs, not replacement by the current keys.
3. `12579d7..7a09`: save the old map table and count and replace the authority map header with the empty/draining representation. Later callback failure cannot restore the old authority entries.
4. `1257a20..7a94`: traverse saved authority entries, bounded by saved count. Entry stride0x50 consists of key and72B info. At `1257a78`, byte at entry+0x4f (info+0x47) is compared with2. **If equal, `1257a7e` jumps to7a96: it exits the entire removal-notification loop. It does not continue to the next entry.** If unequal, load this entry's key and invoke the actual removal tracker at `1257a8c -> 140a1b700`.
5. `1257a96..7ae7`: clear the whole saved authority table and reinstall its empty header with preserved table allocation/capacity. This happens both after natural exhaustion and after the raw==2 branch, so authority emptiness is not conditional on whether every removal callback ran.
6. The unwind fragment `1257b49..7b99` also clears/reinstalls the saved authority table. There is no all-or-nothing rollback around removed-ID union or already-performed d2s deletion.

The generic operation does not own Storage. It does not itself read Storage immutable or clear the removed destination registry. Those belong to other callers/true tracking cleanup.

### The actual d2s effect is deletion, with failure on missing key

`140a1b700/89B` is identified by original symbolizer as `<DecoratorToStorageMap as DecoRemovalTracker>::remove`, source405..407. This is **impl$10::remove**, not the different149B impl$8 function currently described in source comments as a mere existence guard.

It computes the existing key hash via `1405b0ed0`, invokes the typed **RawTable::remove_entry<(DecoratorId,DecoratorStorageAddr)>** at `140b78140`, then tests returned address discriminant3 (`a1b735`). Missing entry takes panic `a1b744..758`. Present entry returns after the map deletion. The dependency is reused through the existing map removal API; no custom hashbrown implementation is proposed. The literal location argument passed by clear at `142c21330` is the removal failure source location, not an event writer or an additional resource.

Consequently an adapter that only clears bucket.decorators and never removes the corresponding d2s entries does not implement this destructive operation. Metadata cleanup must not call this removal tracker at all.

## Correct the meaning of raw byte2 without inventing a business enum

The current source comments claim `DecoratorInfo+0x47=2` means live/placed and cite insert `14155afe0`. Complete original insert plus its exact map callee contradict that provenance:

- `14155afe0/393B` saves incoming RCX toRDI (`155aff8`), while RDX is the Decorators owner (`155aff5`), R8D is ID and R9 the incoming72B info pointer. RDI is the hidden **return-value buffer**, not the authority entry.
- After `HashMap::insert` writes the old-value result into stack+a0, insert tests result+a0+0x47 at `155b04e`. A previous value takes the72B copy into RDI (`155b07c..b0b4`), then updates d2s through the insert handler. A new key updates/inserts its d2s mapping and writes byte2 to **return buffer** RDI+0x47 (`155b076`). It never sets the incoming or stored info to2 at that instruction.
- The complete typed `HashMap<DecoratorId,DecoratorInfo>::insert` at `1411fbfa0/572B` independently shows the same ABI: RCX is saved as return pointerRSI (`11fbfb7`), RDX is map and R9 incoming value. Existing-key path copies old72B intoRSI (`11fc0a8..c0cc`) and replaces stored info. New-key path increments map length, stores incoming key/info (`11fc14c..c18c`) and writes2 at **RSI+0x47** (`11fc191`) before returningRSI. This is the `Option<DecoratorInfo>::None` niche, not an authoritative live-state write.

The destructive drain's comparison against2 must therefore be retained as an observed native/niche boundary, **not promoted to a valid “stop at a placed decorator” rule**. This audit does not prove the source name of the last info byte or declare all valid business values from the placeholder PDB. In particular it does not grant permission to reinterpret every existing source.state==2 branch, change the public info schema, or add fabricated raw2 gameplay tests. The relevant source owner must validate that schema separately before translating a raw guard into semantic Rust. For original valid values, ordinary `HashMap::drain` plus the proper removal tracker is the appropriate dependency operation; do not reproduce Option ABI bytes in game state.

## Connection to the current restore callers

Current `backup::restore_bucket` and `restore_archived_decorators` call the misnamed bucket tracking helper because it currently clears authority entries. Replacing the helper with true metadata cleanup alone would leave old authority keys absent in a selected backup. Reconnecting them instead to destructive clear also requires the original d2s-removal contract; it is not enough to rename the existing two-line body.

The validate investigator owns validation's separate survivor/reason/state/event contract. Validation calls **Storage tracking cleanup**, not this destructive clear. Its MutePreview EventWriter, UI/backup side effects, and the trackers are independent resources. The supplemental clear proof is provided to that investigator only to avoid borrowing the wrong helper or interpreting OptionNone2 as gameplay state.

Suggested future acceptance is semantic and owner-scoped: a restore removes stale authority IDs and their d2s entries, preserves preexisting removed-ID union, restores selected entries through proper insertion/reindexing, and preserves the missing-d2s panic/partial-effect boundary when that input is deliberately examined. True tracking cleanup retains authority and d2s while clearing dirty notifications. None of these tests has been run by this audit.
