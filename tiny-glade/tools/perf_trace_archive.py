"""Lossless, bounded-memory preservation of a completed run's own trace CSV.

Only archive_run_csv removes a file: the fixed schedule.csv directly inside
one run directory. Existing archives are never overwritten. ETL is untouched.
"""
import gzip
import hashlib
import os
from pathlib import Path

CHUNK_BYTES = 1024 * 1024


def open_trace_text(path):
    """Read raw or gzip CSV incrementally with the existing xperf decoding."""
    path = Path(path)
    opener = gzip.open if path.suffix.lower() == '.gz' else open
    return opener(path, 'rt', encoding='utf-8-sig', errors='replace')


def _stream_identity(source):
    digest, size = hashlib.sha256(), 0
    while chunk := source.read(CHUNK_BYTES):
        digest.update(chunk)
        size += len(chunk)
    return digest.hexdigest(), size


def _verify_archive(path):
    # Reading through EOF also verifies the gzip trailer's CRC and byte count.
    with gzip.open(path, 'rb') as source:
        return _stream_identity(source)


def archive_run_csv(run_directory, runs_root, expected_sha256):
    """Verify a new gzip against the recorded raw hash before removing CSV.

    Caller must have completed capture, Microsoft analysis and accounting.
    An invalid numerical result may still be preserved: this operation never
    changes measurement validity. Paths are fixed, not user-selected filenames.
    """
    root = Path(runs_root).resolve(strict=True)
    directory = Path(run_directory).resolve(strict=True)
    if directory.parent != root:
        raise ValueError('trace archive must belong directly to this runs root')
    source = directory/'schedule.csv'
    archive = directory/'schedule.csv.gz'
    if source.is_symlink() or source.resolve(strict=True) != source or not source.is_file():
        raise ValueError('trace CSV must be a regular file owned by this run')
    if not (directory/'schedule.etl').is_file():
        raise ValueError('original ETL must remain available before CSV archival')
    if not isinstance(expected_sha256, str) or len(expected_sha256) != 64:
        raise ValueError('recorded raw CSV SHA256 required')
    before = source.stat()
    created = False
    verified = False
    try:
        # Exclusive creation also rejects dangling symlinks and existing files.
        with archive.open('xb') as output:
            created = True
            raw_digest, raw_size = hashlib.sha256(), 0
            with source.open('rb') as raw, gzip.GzipFile(
                    filename='', mode='wb', fileobj=output, compresslevel=1, mtime=0) as compressed:
                while chunk := raw.read(CHUNK_BYTES):
                    raw_digest.update(chunk)
                    raw_size += len(chunk)
                    compressed.write(chunk)
            output.flush()
            os.fsync(output.fileno())
        raw_sha256 = raw_digest.hexdigest()
        if raw_sha256 != expected_sha256 or raw_size != before.st_size:
            raise ValueError('raw CSV changed since completed accounting')
        if _verify_archive(archive) != (raw_sha256, raw_size):
            raise ValueError('gzip roundtrip SHA256 or byte count mismatch')
        # Detect source edits while verification ran; retain raw on any failure.
        with source.open('rb') as raw:
            if _stream_identity(raw) != (raw_sha256, raw_size):
                raise ValueError('raw CSV changed during archive verification')
        after = source.stat()
        if (before.st_dev, before.st_ino, before.st_size, before.st_mtime_ns) != (
                after.st_dev, after.st_ino, after.st_size, after.st_mtime_ns):
            raise ValueError('raw CSV identity changed during archive verification')
        with archive.open('rb') as compressed:
            compressed_sha256, compressed_bytes = _stream_identity(compressed)
        result = {'schema': 'lossless-trace-csv-archive-v1', 'path': str(archive),
                  'codec': 'gzip', 'raw_path': str(source), 'raw_sha256': raw_sha256,
                  'compressed_sha256': compressed_sha256, 'raw_bytes': raw_size,
                  'compressed_bytes': compressed_bytes, 'roundtrip_verified': True,
                  'raw_removed': True, 'etl_retained': True}
        verified = True
        source.unlink()
        return result
    except BaseException:
        # Never remove a verified archive after unlink might have succeeded,
        # including an interrupt between unlink and return. Raw stays on an
        # earlier failure; a failed removal can safely leave both copies.
        if created and not verified:
            archive.unlink(missing_ok=True)
        raise
