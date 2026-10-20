# Audio completion correction, 2026-10-19

This supersedes the `crossbeam`, `zero capacity` and `recv/send_timeout(1s)`
claims in the older audio chapters, round2 audit and round12 startup report.
Those claims are incorrect. No deadline was extended and no error was hidden.
T13 remains excluded.

## Original evidence

* Original B `tiny-glade.rne`, SHA256
  `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`.
* PDB `country-core-audit/data/procs.tsv` identifies @1408d9d30 as
  `std::sync::mpmc::sync_channel<AudioSamplesLoaded>` and @141679d70 as
  `std::sync::mpmc::array::Channel::recv<AudioSamplesLoaded>`.
  The retained complete AudioCore::new decompile also calls the std constructor.
* `audio-new-channel.asm`: @140a557d9 sets EDX=1 immediately before the call to
  @1408d9d30. The constructor's @1408d9d63 zero test selects a different branch
  for zero capacity; the actual caller takes the positive-capacity array branch.
  This is a capacity-one synchronous std channel, not a rendezvous channel.
* `audio-promise-recv.asm`: @140a57121 sets the promise discriminant to None
  before reading; @140a57190 only accepts AL=2 (success), otherwise unwrap panic.
  The mutex poison branch @140a5713d also unwraps. The retained decompile names
  `Mutex<std::sync::mpsc::Receiver<AudioSamplesLoaded>>` in the owned value.
* The apparent `1_000_000_000` is the nanosecond niche for deadline=None.
  `array-recv-prefix.asm` @141679f7d compares the incoming word with exactly
  this value; equality jumps over Instant::now and the deadline comparison.
  `array-send-prefix.asm` @14167bc2f does the same. No conversion from a duration
  to a future Instant occurs at the caller.
* Existing `startup/audio-delayed-closure.asm` @140a5242a supplies this None
  deadline to the array sender. Success (AL=2) and disconnect (AL=1) both drop
  the sender and return; only timeout (AL=0, unreachable for this deadline)
  enters panic. Therefore blocking `SyncSender::send` with ignored disconnect
  expresses the original producer policy.

## Source changes and lifecycle

`AudioSamplesLoadedPromise::channel()` now constructs the real std capacity-one
pair. `recv()` takes the receiver once and blocks on the existing std method;
disconnect still panics. A later call has no receiver and returns None. The
Mutex provides ECS Sync ownership, without copying or reimplementing std.

`native_audio::create_audio_resources` keeps the original two metadata/sample
passes and shared Studio Arc. Its one completion is buffered independently of
when the gameplay App reaches PreStartup. The existing host starts the worker
before common assets and removes the promise before receiving. This no longer
creates a timeout race if common assets need many seconds. Cancellation drops
the receiver and the producer's send failure is ignored, matching the original.
Failed Studio creation still yields no loader or promise and logs via the
existing initialization error arm. No GameLoop/run_game change is necessary
for the channel correction. Original exact worker launch scheduling has not
been newly established by this bounded correction and is not claimed verified.

## Validation prepared, not yet run by this agent

The central agent owns Cargo and full-App validation. New meaningful tests:

* `audio_samples_lifecycle`: early producer completes and exits, receiver waits
  over one second before consuming; late producer wakes a receiver after over
  one second; cancelled receiver rejects send; failed loader remains an error
  and consumes the one-shot receiver.
* `native_audio::audio_completion::initialization_failure_has_no_completion_to_wait_for`
  exercises the exact disabled-owner assembly branch with an injected failed
  owner (it does not force a native FMOD initialization failure).
* Ignored `shipped_fmod_completion_and_shared_studio_survive_delayed_receive`
  requires the original shipped FMOD DLLs/banks and `TG_INSTALL_ROOT`; actual
  create, boot banks, all delayed metadata/sample banks, worker join, receive
  after >1 second, shared Studio surviving initial owner drop, three real event
  lookups/start/stop/release, update, final Arc/Studio teardown must all succeed.
  It must be explicitly run with `--ignored`; an ignored test is not a pass.

Suggested central commands, using its existing original DLL environment:

```text
cargo test -p country-core --lib audio_samples_lifecycle --locked --offline
cargo test -p country-core --lib --features native-audio audio_completion --locked --offline
cargo test -p country-core --lib --features native-audio shipped_fmod_completion_and_shared_studio_survive_delayed_receive --locked --offline -- --ignored --nocapture --test-threads=1
```

Then rerun `original_gameplay_chain` through its unchanged full production App.
These bounded lifecycle tests do not establish audible output or the whole
game's audio event/state interaction completeness. No Cargo command was run by
this agent, and no game-equivalence acceptance status is raised here.

## Reusable reverse-engineering lesson

Do not translate a duration-looking constant at an internal generic call into
a public API policy. Establish exact PDB callee, channel constructor argument,
enum/niche layout and the callee branch which reads the value. In this case
three-channel implementation names referred to std mpmc's implementation,
which had been misattributed to crossbeam; the incorrect one-second policy
then caused the real full-App failure. Restore dependency calls directly and
verify the game-owned handoff, cancellation and consumption lifecycle.
