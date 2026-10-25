# Shared conversion static confirmation

The original history replay restore_decorators consumer at14155c88d..14155c9ac uses the same type-indexed conversion as snapshot restore. Exact table142c6e844 maps types38..3a→c8b8(style6),3b→c8da(style4),3c→c8c9(style7),3d→c91a(style5),3e/outside→cd8a(style3),3f→c8f0(copied style/u30/u3c/u40/u44/u4b). State0 atc968, ID generator reserve atc4e1..c4fd. Thus frozen69's correction of existing shared to_info is compatible with this second actual caller; it does not impose snapshot-only behavior on replay. No code/frozen69 inputs changed.

Two new bounded static packs1280B+7B verified byte-for-byte to original B. No original/Cargo/test execution. This is a local projection proof, not recovery of full history/collision/GPU consumer.
