from pathlib import Path
b=Path('D:/game/reverse-engineering/tiny-glade/evidence/round12/wall-gpu');p=b/'candidates-sheep/crates/country-core/src/resources/critters/sheep/control.rs';s=p.read_text(encoding='utf-8').replace('(critter_goal.goal_tag == 1).then_some','(critter_goal.goal_tag != 0).then_some');p.write_text(s,encoding='utf-8');(b/'sheep-capture-goal-followup.patch').write_text('''--- a/tiny-glade/crates/country-core/src/resources/critters/sheep/control.rs
+++ b/tiny-glade/crates/country-core/src/resources/critters/sheep/control.rs
@@ -34,1 +34,1 @@
-                goal: (critter_goal.goal_tag != 0).then_some(critter_goal.position),
+                goal: (critter_goal.goal_tag == 1).then_some(critter_goal.position),
''',encoding='utf-8')
import hashlib,json
m=json.loads((b/'candidates-sheep/manifest.json').read_text(encoding='utf-8'));e=next(e for e in m['files'] if e['path']=='crates/country-core/src/resources/critters/sheep/control.rs');print(hashlib.sha256(p.read_bytes()).hexdigest()==e['candidate_sha256'])
