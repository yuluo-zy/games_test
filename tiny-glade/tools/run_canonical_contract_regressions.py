"""Run staged cold-tools tests with unchanged historical synthetic controls."""
from pathlib import Path
import sys
import tempfile
import unittest

HERE = Path(__file__).resolve().parent
sys.path.insert(0, 'C:/game/tools')
sys.path.insert(0, str(HERE))
import test_perf_import_frozen_bundle as legacy_import
legacy_import.TEMP_ROOT = Path(tempfile.gettempdir())/'canonical-cold-tools-synthetic-tests'

if __name__ == '__main__':
    names = ['test_perf_headless_series', 'test_perf_headless_series_controller',
        'test_perf_headless_workload', 'test_perf_headless_import_binding',
        'test_perf_import_frozen_bundle', 'test_canonical_contract', 'test_canonical_import']
    suite = unittest.defaultTestLoader.loadTestsFromNames(names)
    result = unittest.TextTestRunner(verbosity=1).run(suite)
    raise SystemExit(0 if result.wasSuccessful() else 1)
