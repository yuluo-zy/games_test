# First welcome producer restoration — 2026-10-21

Scope is the original first welcome screen, not complete MenuFlow, NewSession,
daily welcome, tutorial progression, or every menu page. No EscMain/ready/reveal
override is introduced. Current profile and renderer owners are retained.

## Original evidence

`probe_native.py` reads only bounded functions and referenced constants from B
`workspace/input/tiny-glade.exe`, asserting SHA-256
`f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.
`native-manifest.json` records function sizes/body hashes and verified strings.
The target is not executed by this probe.

The prior typed parameter proof is
`skeleton/task-index/work/TG-700584f34568812faf/decorator-menu-entry-20261021/MENU_ENTRY_HARD_GAPS_CONTRACT_V1.md`
and `MENU_TYPED_CONSUMER_REFERENCES_V1.json` beside it. The older truncated
welcome pseudocode and inferred WriteSettings event are not implementation
authority. The concrete function is `141ebe130/2158B`, with exactly ten params:
MeiContext, WelcomeScreenState, VermeerState, GameTime, EscMenuState, GameConfig,
GameProfile, SaveGameProfileCmd writer, optional SteamClientResource, and local
SteamdeckWelcomeMsg.

| Native evidence | Recovered behavior |
| --- | --- |
| `141ebe396..3c0` | skip_welcome (+e5) or profile +206 closes without emitting save |
| `141ebe492..4cc` | initial state 0 waits for Esc Closed(tag2), changes to showing(1,0), returns without drawing |
| `141ebe49f..4b7` | showing + active Vermeer resets timer and returns before menu cancellation |
| `141ebe517..621` | showing + non-Closed menu writes profile+206, sends SaveGameProfile, closes |
| `141ebe62e..6f4` | desktop metrics base18, full/demo template, timer += delta, separate SSE .3 smoothstep |
| `141ebe546..582`, `141ebe75a..` | true opaque safe frame `welcome`, SteamDeck local pages0/1; invalid page panics |
| `140fc9230/771B` | first Deck arrow button; 27×1.5 metrics; confirm19 increments page only |
| `140fc9540/691B` | final/desktop lookup Welcome.LetsGo plus literal space/sheep; confirm19; used state &0x30 |
| `141ebe7a6..7ed` | Deck page0 increments timer/fades; page1 opaque1 without incrementing timer |

The .3 divisor, one/three constants and operation order use the original
separate `divss/maxss/minss/mulss/addss/subss/mulss`, preserving NaN and signed
zero behavior rather than substituting a fused or generic easing function.
Button size multiplier at `1429258d0` is **1.5**, not2.

## Production changes

- Canonical `country-core/resources/onboarding.rs`: explicit Pending/Showing/
  Closed WelcomeScreenState; no duplicate profile or menu state owner.
- `country-core/game_profile.rs`: named access to existing runtime +206
  `hint_reset_flag_206`. Its old field name/default and skip_serializing remain
  intact. Persisted tutorial+1f4 stays distinct. The save event is still sent.
- `toolbar/menu/welcome.rs`: all ten params, original desktop/demo templates,
  SteamDeck two-page local state, slot `buttons`, original labels/metrics,
  confirmation and menu-cancel side effects, reveal gate and timing.
- `toolbar/mei/widgets/menu_button.rs`: shared `menu_hotkey` writes both current
  source projections of one native +25d byte. `14067e6f2` writes +25d and
  `14067ea28` reads that same byte for pre_update. Both current fields are u8.
  The native helper's comparison to0x12 only omits its tooltip child; pre_update
  separately compares to0x28 for an absent action. Full tooltip remains U.
- `toolbar/menu/mod.rs` exports welcome. Host registration is owned by central
  agent: initialize WelcomeScreenState once, add welcome::ui to Game/set36.

## Verification and explicit limits

Six CPU tests in `toolbar/src/menu/welcome/tests.rs` use shipped full/demo/Deck
RON templates and font-role table (fixtures under `toolbar/tests/fixtures/welcome`).
They chain the real `mei_pre_update` into welcome and use actual digital19 input
to trigger the real MenuButtonContent consumer, rather than pre-setting used.
They cover state wait/reveal/cancel order; skip/dismiss gates; 27/40.5 metrics;
both Deck pages and final-only save; existing profile RON unchanged by +206;
and finite/NaN fade behavior. Test fixture deactivates Vermeer deliberately to
exercise the post-reveal condition; production never bypasses that condition.

This agent ran scoped rustfmt and `git diff --check`; it did **not** run Cargo,
GPU, original game or reconstructed game. Central agent owns compilation/test
results and final real installed-font/UI/paper-background draw verification.
Node/template tests are not claimed as complete layout/pixel/GPU validation.

Still outside this change: complete MenuFlow transitions, new_session UI,
daily_welcome, complete menu hotkey tooltip, and independent diagnosis of every
missing-overlay pixel. Before this restoration the existing GUI39 settings/back/
version producer already emitted UI even with Esc Closed; therefore a missing
welcome producer alone cannot prove the cause of all previously invisible UI.
