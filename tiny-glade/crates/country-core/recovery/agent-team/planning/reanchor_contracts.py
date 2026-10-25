#!/usr/bin/env python3
# Re-anchor contracts.json evidence refs that point to never-existing
# evidence/core-recovery-20261015/<dom>/REPORT.md onto real evidence files.
import hashlib, json, shutil
from pathlib import Path

CRATE = Path('/Users/yuluo/Desktop/game/games_test/tiny-glade/crates/country-core')
REV = Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade')
CONTRACTS = CRATE / 'recovery/contracts.json'
INVENTORY = CRATE / 'recovery/inventory.json'
README = CRATE / 'recovery/README.md'

# backup
shutil.copy2(CONTRACTS, CRATE / 'recovery/contracts.json.bak-20261015')

contracts = json.loads(CONTRACTS.read_text())
replaced = 0
for key, v in contracts.items():
    if not isinstance(v, dict):
        continue
    ev = v.get('evidence', [])
    if not isinstance(ev, list):
        continue
    new_ev = []
    changed = False
    for e in ev:
        if isinstance(e, str) and 'evidence/core-recovery-20261015/' in e:
            chapter = f'evidence/country-core-deep/chapters/{key}.md'
            dig = 'evidence/country-core-deep/round3/MODULE-DEEP-DIG.md'
            # chapter may be under a different file name for non-file keys
            new_ev.append(chapter)
            if dig not in new_ev:
                new_ev.append(dig)
            changed = True
        else:
            new_ev.append(e)
    if changed:
        contracts[key]['evidence'] = new_ev
        replaced += 1

CONTRACTS.write_text(json.dumps(contracts, ensure_ascii=False, indent=2) + '\n')

# update inventory contracts_sha256
sha = hashlib.sha256(CONTRACTS.read_bytes()).hexdigest()
inv = json.loads(INVENTORY.read_text())
if isinstance(inv, dict) and 'contracts_sha256' in inv:
    old = inv['contracts_sha256']
    inv['contracts_sha256'] = sha
    INVENTORY.write_text(json.dumps(inv, ensure_ascii=False, indent=1) + '\n')
    print(f'inventory contracts_sha256: {old[:16]}.. -> {sha[:16]}..')
else:
    print('inventory has no contracts_sha256 top-level; computed sha=', sha)

print('contracts entries re-anchored:', replaced)
