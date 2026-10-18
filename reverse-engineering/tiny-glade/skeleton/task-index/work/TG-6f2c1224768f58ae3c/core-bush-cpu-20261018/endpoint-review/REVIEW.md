# Independent review: existing-wall endpoint continuation v3

Status: original frozen20 accepted only after the confirmed absent-terrain archive finding is corrected by a separately frozen delta. No other blocking finding is confirmed in this bounded review. This report does not claim the whole wall module or renderer chain recovered.

## Identity and reproducible checks

Target is B PE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03 with matching PDB SHA 2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911. Upstream endpoint-v3 manifest SHA ea7ae040df8b2177b4c67c4f8fe0c03e09c31cc214d517d12a256f141dd71225: all 20 hashes and sizes were independently checked. The three assembly transcripts (UI 7834B, reverse554B, draw13557B) contain 4542 instructions/21945 bytes, all individually matched to the original PE. Applying the candidate to isolated frozen continuation29 source inputs produces both candidate sources byte-for-byte. See verification.json and verify.py. No Cargo, native execution, production edits, or original probe retries occurred.

## Original business and candidate mapping

Entry gates preserve WallFreeform(6/0) or Fences18, state3 resets0, idle suppression returns without reading camera events, and idle camera movement consumes its reader then skips this invocation. Active state1/2 bypass idle gates. Public row validation precedes the published bulldoze flag and live map existence check. Shape+260 and kind+296 belong to the published row; selection later clones the live authoritative source. The candidate deliberately preserves that distinction, including raw kind bytes other than0/1 and snapshot cloning before color bounds failure. Published flag+297 needs the real publication owner, not a newer live animation lookup.

Endpoints are published first(true)/last(false), MAXSS-style Y clamp to positive zero (NaN and signed zero included), sphere XYZ/radius1/tag0, stable formatted names, real WidgetParams history permission. Native sphere byte1 is padding. Phase3 selects and may report outside, but emits no sampling event. Phase4 in state1/2 archives once, reports outside or samples inside, then enters2; outside does not cancel. An unstarted phase4 returns0. Other phases send FinishWall(false); active1/2 become3. Missing/bulldozed live walls retain the original finish behavior.

The selected endpoint constructs InProgressCurve from the actual live curve and preserves stored parameters/length when unreversed. First-end input reverses via the existing shared reverse implementation. Continued output tests direction exactly1, reverses/rebuilds parameters only on that branch, and projects the previous source's last endpoint independently of selected begin-end. Ordered 0<=u<=1 gates publication/audio; failure retains active input. Existing point bound and source/color/height fields remain in their established owners.

## Confirmed finding

The original state1-to-drag transition always archives selector1, including absent terrain storage. Original 0x1413e7230 tests storage+90; zero jumps to empty-collections construction and archive_storage_decorators 0x1413e6150, then copies the cloned empty collection into the first-snapshot slot. The original20 candidate used if-let-Some and skipped this before snapshot. Evidence and exact branch/write addresses are frozen under archive-empty-evidence (manifest87e5719b9c73d0ec0f9e431ebd740ed37587385b1ebe6d011c2d6f51ccd61934). Existing Decorators::default() and archive_storage_decorators support the minimum repair without another storage owner. The original frozen20 must be preserved; its correction is reviewed separately.

## Integration and validation limits

At review time production add_wall_drawing_cpu_systems still does not install ui_continue_wall, register WallContinueState, or provide the new shared sphere/publication-flag API. Root owns these necessary shared adapters. Register suppression before endpoint UI; endpoint UI must run before same-frame history_start_recording, and draw after start. With History.pending=None and no pre-registered spatial widget, actual phase3 must create the permitted endpoint widget and start recording that frame, followed by phase4 input, false finish, deferred record, and actual history finish. The upstream tests explicitly prestart History; they do not prove this automatic lifecycle by themselves. The shared publisher must populate the flag using the original publication-time AnimateWallBulldozeBuffer membership; synthetic false-publication tests do not prove that chain.

No claim is made for the original signifier renderer closure, real renderer hit-volume lifecycle, snap>1 joins/merge/premade-enclosure paths, or native execution of this endpoint entry. The earlier entry fetch-unmapped failure and later usage-limit auto-review refusal remain unaccepted validation, not successful probes. These boundaries do not justify manufacturing successful default inputs.
