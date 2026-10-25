"""Copy self-contained original fixtures and reviewed package tests."""
from pathlib import Path
import shutil
OUT=Path(__file__).parent;PKG=OUT.parents[3]/'tiny-glade/crates/utils'
for src,dst in [('consumer-native-fixtures.json','tests/fixtures/deep-consumers.json'),
                ('spline3d-original-fixtures.json','tests/fixtures/deep-spline3d.json'),
                ('deep-consumers-tests.rs','tests/deep_consumers.rs')]:
    shutil.copyfile(OUT/src,PKG/dst)
print('consumer fixture/test sources copied')
