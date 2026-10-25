from pathlib import Path
import shutil

out = Path(__file__).resolve().parent
history = out / 'history/first-checks'
history.mkdir(parents=True, exist_ok=True)
for name in ['verification.json', 'test.log', 'release-test.log', 'clippy.log', 'gpu-api-check.log', 'fmt.log']:
    shutil.copyfile(out / name, history / name)
path = out.parents[3] / 'tiny-glade/crates/tg-text/src/fragment.rs'
source = path.read_text(encoding='utf-8')
start = source.index('#[cfg(test)]\nmod internal_contract_tests {')
end = source.index('\nimpl TextFragment {', start)
tests = source[start:end]
path.write_text(source[:start] + source[end:].lstrip('\n') + '\n' + tests, encoding='utf-8')
