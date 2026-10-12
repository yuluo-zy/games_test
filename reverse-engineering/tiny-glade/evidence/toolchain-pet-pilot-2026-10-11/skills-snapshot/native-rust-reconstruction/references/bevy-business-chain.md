# Bevy registration and complete business-chain recovery

## Recover explicit and implicit components separately

Inspect the actual create tuple/Bundle component-ID calls, component descriptors, derive callbacks and insertion call sites. Distinguish explicit Bundle fields from required components. A derive callback registering/pushing/popping Self, including a nested self call, is not sufficient evidence for a self-required edge. Validate with the relevant upstream macro/source version.

Confirm payload, zero-sized marker, enum and alignment roles. A copied bundle size may include alignment padding; a tag may identify a mesh enum or instance subset rather than lifecycle state. History variant names, UI labels and event consumer names may refer to different operations; confirm the actual writer/reader generic command type before defining a shared API. Shared-address symbol aliases need descriptor/call-site corroboration. Do not create guessed transforms, payloads or component requirements.

Use real Bevy Component/Bundle/Resource/Event/Query/Commands and flush boundaries appropriate to the pinned version. Do not implement entity tables, archetypes, change tracking or event storage to mimic decompiled dependency internals. Unavailable fork details remain an interface gap.

## Trace the whole business path

Follow authoritative input collection and dirty candidate production through publication, cache invalidation/fill, entity association, creation/update, output consumers and deletion. Recover no-change, missing-owner, duplicate association and marked-for-deletion behavior where relevant. Restore base data before derived recomputation if required; repeated scheduling must not accumulate the previous result.

Assignment/Changed ticks, semantic notification equality and recording flags are separate policies. Equal values may still be assigned and marked; union padding may be copied without affecting equality; a recorded previous value may come from the command rather than current state.

Keep command insertion/removal visible at the correct stages with dependency APIs. Different dirty subscriptions may apply different filters. Preserve original per-consumer event order and duplication; coalescing or sorting in a reconstruction needs its own stated evidence/design boundary.

## Separate backend intention from execution

A CPU record or Create/Update/Remove queue does not establish GPU allocation/upload or active handles. Runtime adapters may retain CPU snapshots and emit intentions, but must not fabricate backend success components, handles or unknown resources. Record which schemas and consumer branches have actually been recovered.

## Verify continuous integration

Exercise the actual App/World across creation, unchanged frames, authoritative edits, derived updates, marking/fade, command flush/removal and subsequent clean frames. Assert relevant relationships, cache state, output requests and cleanup; registration alone is not geometry generation. Connect native verified cores to this path rather than stopping at probe output.

Recovered original schedule edges and a reconstruction's explicit dependency order are different claims. Register only proven original components and mark adapter-only resources/types clearly. Engine version/checksum matching alone does not establish fork features, exact original scheduling or final visual acceptance.
