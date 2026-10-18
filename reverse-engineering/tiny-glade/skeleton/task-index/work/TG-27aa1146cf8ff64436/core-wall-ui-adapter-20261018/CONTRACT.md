# Existing UI volume and public-wall state adapters — B

Retain manual WidgetVolumeShape tag/layer/size API and add logical Vec3 center. Sphere tag0 uses size.x radius and center XYZ through sphere(center,radius); original byte1 is padding, so logical compatibility layer0 is not claimed as a native business layer. Original canvas size starts+4; do not claim the Rust logical representation is a native 32-byte union. Existing literals only get default center.

PublicWallState gains derived being_bulldozed field, accessor and publication-only setter, and writes+297 in its existing byte projection. Direct CPU collection initializes no-animation compatibility false; real publisher MUST overwrite every rebuilt row from actual AnimateWallBulldozeBuffer membership before endpoint UI is enabled. Never query the later animation queue at UI time or claim the setter alone closes the chain.

Publisher is claimed by main-chain-boot-20261018: publisher-owner.patch is an unapplied narrow handoff, not root permission to overwrite. Its actual resource is already initialized by both current core registration and actual host. The hook only updates last_delta.changed rows; unchanged published rows retain captured membership. No new independent animation state owner.

Native proof: ui_continue@140aae66c..6d8 tag0/radius1/Vec3 writes; publication14th arg queue@140a8c46c..488; stride98 key+70 search@140a8cb47..bb80, b88 membership, output+297 setne@140a8db81 with no prior flags-clobbering operations. Exact bytes and ASM in referenced frozen evidence. No original runtime or Rust tests claimed here. Endpoint source and actual publication lifecycle regression must be checked before accepting runtime behavior.
