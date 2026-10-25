# Original vertex input consumer

B RNE SHA-256 f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03.

Fresh native instructions in original-vertex-consumer.asm: vector branch at 0x1416a3b40 calls ScalarType::nbyte and multiplies by actual component count at 0x1416a3b4a, passes bytes to original map_vertex_attribute_format at 0x1416a3b5b; scalar branch mirrors it at 0x1416a3b7f/3b91; original Type::nbyte at 0x1416a3ba2 is the actual vertex binding stride. original-format.asm shows ScalarType tags 2/3/4 are selected independently of payload and total width accepts 4/8/12/16 bytes. Existing reflection_ty schema and original ScalarType::nbyte return payload bytes (Float(4)), rather than SPIR-V bit width Float(32).

Production helper removed the incorrect Float(32)/Signed(32)/Unsigned(32) selector and hardcoded count*4. It now uses scalar kind tag and actual Type::nbyte, letting the original format-width bounds reject unsupported payloads. This is intentionally not broadened to matrices/arrays unsupported by the original consumer.

Consumer: packaged shader reflection -> ShaderHost pipeline actual vertex format/location/stride -> live AssetVAOLibrary attributes -> RasterHost graph vertex access -> real vkCmdBindVertexBuffers and indexed/instance draws.

Validation: parent integration agent runs unified strict GPU fixture original_raster_gpu with Float(4), never a fabricated Float(32) fixture. Implementation is unverified until that real GPU result is available.
