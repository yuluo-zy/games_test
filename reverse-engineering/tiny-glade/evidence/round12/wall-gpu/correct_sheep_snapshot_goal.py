from pathlib import Path
p=Path('D:/game/reverse-engineering/tiny-glade/evidence/round12/wall-gpu/candidates-sheep/crates/country-core/src/resources/critters/sheep/control.rs');s=p.read_text(encoding='utf-8');assert '(critter_goal.goal_tag != 0).then_some' in s;s=s.replace('(critter_goal.goal_tag != 0).then_some','(critter_goal.goal_tag == 1).then_some');p.write_text(s,encoding='utf-8')
