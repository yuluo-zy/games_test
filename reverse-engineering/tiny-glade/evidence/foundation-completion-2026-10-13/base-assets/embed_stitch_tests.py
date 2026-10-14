import json
from pathlib import Path

out = Path(__file__).resolve().parent
rows = json.loads((out/'native-base-fixtures.json').read_text(encoding='utf-8'))['stitch']
path = out/'rdp_tests.rs'
text = path.read_text(encoding='utf-8')
marker = '\n#[test]\nfn original_b_stitch_prefix_and_index_rebase_all_cuts()'
text = text.split(marker)[0]
test = marker + ' {\n'
def points(values):
    return 'vec!['+','.join('Vec2::new('+','.join(f'{v:.1f}' for v in p)+')' for p in values)+']'
for row in rows:
    test += '{\n'
    test += f'let old=Rdp {{indices:vec!{row["old_indices"]},points:{points(row["old_points"])}}};\n'
    test += f'let next=Rdp {{indices:vec!{row["new_indices"]},points:{points(row["new_points"])}}};\n'
    test += f'let result=old.stitch_with({row["at"]},&next);\n'
    test += f'assert_eq!(result.indices,{row["result_indices"]});\n'
    test += f'assert_eq!(result.points,{points(row["result_points"])});\n'
    test += '}\n'
test += '}\n'
path.write_text(text+test,encoding='utf-8')
print('embedded original stitch cuts',len(rows))
