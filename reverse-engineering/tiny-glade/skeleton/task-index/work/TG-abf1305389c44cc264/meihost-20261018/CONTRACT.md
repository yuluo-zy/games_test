# Canonical B component identities

UiMouseInteractable was a 4-byte/alignment-4 native country_core component (register_component@0x141e35eb0); toolbar had copied a separate local type. Production now owns it in core components/ui.rs and toolbar reexports exactly that type. Actual toolbar spawn and core debug-hide query therefore share one Bevy identity. The new ECS integration test proves the producer/consumer match and preserves position and unrelated entities. Payload bytes remain opaque pending their individual native producers.

KeepOnLevelUnload is a size-0/alignment-1/no-drop component, confirmed by boot fresh register_required_components@0x140852670 in TG-44dd51071513dba304/main-chain-boot-20261018/keep-component-query.json. Added canonical ZST after reading that evidence to support the real animated-logo startup consumer; no business logic is attributed to its derive.
