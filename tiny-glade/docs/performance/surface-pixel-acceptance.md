# Frozen water/ice pixel acceptance

These tools run the existing `gpu_surface_collect` integration-test entry. They do not compile, launch Cargo, create another render host or provide a CPU-performance result. The lake fixture preserves the shipped scene, 1280×720 output, world scale 1.0 and 30 fixed 1/60 updates in Autumn and Winter.

The main agent freezes the already-built test EXE, its matching PDB and required DLLs in one folder. Use the existing freeze metadata fields, or save this schema from the verified build evidence:

```json
{
  "schema": "surface-frozen-build-v1",
  "executable": "gpu_surface_collect-<build-id>.exe",
  "executable_sha256": "<actual EXE SHA256>",
  "source_commit": "<40-character source commit>",
  "source_content_sha256": "<source snapshot SHA256 from freeze>",
  "files": {
    "gpu_surface_collect-<build-id>.exe": "<actual EXE SHA256>",
    "gpu_surface_collect-<build-id>.pdb": "<actual PDB SHA256>",
    "fmod.dll": "<actual frozen DLL SHA256>",
    "fmodstudio.dll": "<actual frozen DLL SHA256>"
  }
}
```

The existing `headless-frozen-build-v1` schema is accepted when its EXE actually contains this surface test. This runner validates file bytes against the metadata; it never creates build provenance by guessing a source snapshot from the current checkout. Every inventory entry must be one file beside the metadata. EXE and matching PDB are required. Include every runtime DLL supplied with the frozen executable. An optional `binary_blake3` field supplies an expected fixture declaration; Python still does not independently compute BLAKE3.

After the source/build identity is frozen, the main agent runs:

```powershell
python tools/run_surface_pixels.py `
  --metadata C:\game\.perf\surface-frozen\build.json `
  --out C:\game\.perf\surface-inline-collect-01 `
  --install 'D:\game\ljxsj_92385\Tiny Glade' `
  --root C:\game
```

Use a new output directory for every run. The controller checks EXE/PDB/DLL SHA256 before and after each process. It launches that exact EXE twice, with the exact existing test name, first `inline`, then `collect`; each child covers both water and ice. It clears inherited `TG_*`/`RHAPSODY_*` variables and sets its own resource path, mode, output and source identity. It requests `VK_LAYER_KHRONOS_validation` and strict Vulkan initialization. The parent environment remains unchanged. There is no worker-count, draw-chunk, quality, resolution or simulation reduction.

The controller also independently hashes the installed `assets/starting-builds/full/05-rocky-lake/history.json` before launching and after each child. Its actual SHA256 must equal the recorded original reference `f85cba25b4a193d517402a57df3590323f3d388b4f8181899c4b7fed7272fec3`. Two matching but modified installation scenes cannot satisfy this original-scene gate.

Each child must report one passed test, zero failures and zero ignored tests. A nonzero exit, timeout, missing selected test, `VUID-`, validation error, initialization failure or skip invalidates the run. The first invalid process stops the pair; its logs and available artifacts remain. Immediately after each child exits, `artifacts.sha256.json` seals both manifests and all eight expected raw RGBA/PNG files. Before accepting the pair, the controller checks sealed files again to detect changes after collection.

Outputs:

- `pair.json`: frozen metadata/file identities, both commands, environment overrides, process/log/sidecar hashes, per-season comparison results and final validity.
- `{inline,collect}/process.log` and `artifacts.sha256.json`: original process evidence and independently computed SHA256 for manifests, raw RGBA and PNG files.
- `water-compare.json` and `ice-compare.json`: full input rejection reasons and exact per-frame changed-pixel/max-channel/mean-channel differences. Each frame includes independently computed `raw_file_sha256` for both inputs.

The comparator checks scene identity, nonempty lake raster, full draw-list inputs, all 30 ordered camera/clock/scene records, selected season pass and exact frame-20/frame-30 captures. It rejects missing fields even when both reports omit the same field. It verifies 30 surface calls and the exact draw total derived from each frame's actual list 15, with real worker tasks and no owner fallback in Collect mode. Cross-mode comparisons require matching source and binary declarations; the controller additionally binds both runs to its verified frozen EXE SHA256.

BLAKE3 values in fixture manifests are retained as **fixture declarations**, not independently verified Python hashes. Actual file evidence always uses independently computed SHA256. RGBA bytes are compared directly; an unchanged declared BLAKE3 never hides changed pixels. No image tolerance or clock/entity difference is silently ignored.

For existing evidence, run the strict comparator directly:

```powershell
python tools/compare_surface_pixels.py <inline-water-directory> <collect-water-directory> --output <water-report.json>
python tools/compare_surface_pixels.py <inline-ice-directory> <collect-ice-directory> --output <ice-report.json>
python -m unittest discover -s tools -p test_surface_pixels.py -v
```

The regression suite uses synthetic files and a mocked subprocess boundary; it never invokes a GPU or compiler. Its passing result establishes tool behavior only. Real acceptance still requires the physical GPU run, no unexplained pixel differences, and human inspection of the saved full-resolution PNGs to confirm visible representative water/ice regions. A named pass or nonzero raster alone does not establish visible surface pixels. WSI compatibility, frame-CPU p95 and longer stability runs remain separate gates.
