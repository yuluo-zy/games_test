# B OS → game → frame contract

Target is B only, per current user authorization. Original instruction oracle is
`tiny-glade.rne`, SHA256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.
The fresh read-only Ghidra query targets patched B `tiny-glade.exe` at the
documented installed path. `os-callback-direct.json` retains the response and
revision; `os-callback.asm` retains the original 3,283-byte callback range.
`main-loop.asm` retains the full 36,589-byte native main-loop range. Existing
E-worker decompilation is used with the original jump-table targets, not alone.

## Observed OS event producer and game consumer

The winit callback is the game callback inlined into
`pump_events::closure$0` at 0x140557b50. Upstream WindowEvent ordering (winit
0.29) identifies branches; the game enum below is a semantic projection, not a
claim of portable enum ABI or original Rust variant spelling.

| winit event | native OsWindowEvent tag | main-loop action/address |
| --- | --- | --- |
| Resized | 4 | width < 20,000; WindowSize::set at 0x1404dacb8; config windowed resolution and UI size change |
| Moved with changed monitor | 5 | enumerate monitors 0x1404db4c8; update config monitor and request settings persistence |
| changed monitor refresh | 6 | Swapchain::mark_stale 0x1404db257 |
| CloseRequested | 7 | send ShutdownGameCmd at 0x1404db261; **does not directly exit** |
| F11 / Alt+Enter shortcut | 8 | fullscreen toggle at 0x1404dae72; physical-code constants 0xa9/0x39 map F11/Enter; ALT bit 0x100 |
| display-setting reply | 9 | update GameConfig.video display fields and pending VideoSettings snapshot 0x1404db596 |
| Focused | 10 | Swapchain::on_focus at 0x1404db993, send WindowFocusEvent(payload bool), forward input reset via game consumers |
| KeyboardInput | 11 | winit_to_bevy_keycode 0x1404db407; optional MeiContext key dispatch; Events<KeyboardInput> |
| CursorMoved (uncaptured) | 12 | InputCursorMoved absolute at 0x1404dbb73, gated on capture confirmation IDs |
| CursorMoved (captured) | 13 | InputCursorMoved relative at 0x1404db0c1; OS warps to anchor (callback 0x140558xxx) |
| MouseInput | 14 | Bevy mouse button event at 0x1404dba76 |
| cursor request confirmation | 15 | max(confirmed_id, received_id) at 0x1404dae4f |
| MouseWheel, LineDelta only | 16 | Bevy wheel event at 0x1404dafda |
| ICC profile result | 17 | IccProfileChanged at 0x1404db7dc |

The native enum jump table at 0x1429ea450 is retained in
`event-jump-tables.json`. This corrected decompiler ICF mislabels: event typehash
0x2e383ebb9376bfa1 is ShutdownGameCmd, proven by run_game_thread registration at
0x14059d450, not WriteSettingsFileCmd. The latter typehash is
0xa554fbb3bbf1ca0.

## Observed game → OS requests

Native GameEvent uses niche values i64::MIN + tag. Table at 0x1429a7d68:
0 = cursor visibility, 1 = cursor capture with monotonically increasing request
ID, 2 = toggle minimized state, 3 = egui PlatformOutput, 4 = display config,
5 = quit, 6 = custom cursor SSBO pointer. Tag names are semantic projections.
Visibility command has four raw modes: 0 standard, 1 custom, 2 suppress(origin
bits), 3 unsuppress(origin bits); capture has three raw modes: 0 capture at
position, 1 release, 2 set position. Core producer mouse_capture at 0x140b17830
confirms capture writes anchor, emits mode 0 plus position, and hides origin
bit 2; release clears anchor, emits mode 1, and unhides origin bit 2.

## Settings and shutdown

gather_pending_video_setting_changes 0x1404e3380 ORs each event's byte payload
under mutex. Main-loop checks flags & 2 for swapchain vsync; flags & 5 for OS
display configuration; then emits WriteSettingsFileCmd. ApplyVideoSettingsCmd
therefore cannot remain a unit placeholder.

ShutdownGameCmd consumer 0x140b2ba40 consumes all unread commands, and if any
exist queues insertion of zero-sized ShutdownGame. Main-loop checks that
resource (typehash 0xfb3868c4bf7b3b51), sends GameEvent tag 5, and returns.
run 0x1404d8250 then restores pending fullscreen, logs peak allocator memory,
serializes an optional input stream, drops App/SubApp/channels/render resource
cache/render graphs/Steam/egui in the retained order. Vulkan cache shutdown
must wait for actual GPU work before releasing dependencies.

## Platform adaptation and unresolved evidence

B uses a Windows OS pump and a dedicated game thread. macOS requires winit on
the main thread; a main-thread `EventLoop::run` adapter is explicitly a platform
adaptation. It must call the recovered semantic event handlers and frame
driver, and cannot be described as B's recovered thread layout.

MeiContext's current [u64;14] key API is not portable and must not be populated
with guessed Windows ABI bytes. The original custom cursor points directly to
an SSBO-backed host record; substituting a plain pointer or inventing a renderer
draw implementation is not accepted. Original full InputStream serde is not
yet representable by current generic InputStream; the separate RecordingFile
only stores a first frame and must not be presented as full input recording.
Suspended/Resumed cross-platform notifications are not recovered B gameplay
events and may only be handled as explicit WSI platform adaptation. No device
loss retry may be called original behavior without identifying its original
resource replacement and cache invalidation path.


## Producer-layout correction

The callback's `iStack_210 == 0` is the MouseScrollDelta tag, not TouchPhase.
It copies exactly two f32s into a Bevy MouseWheel payload and emits unit byte 0.
PixelDelta is not forwarded by that observed producer. The initial phase
interpretation was rejected before the code path was implemented.
