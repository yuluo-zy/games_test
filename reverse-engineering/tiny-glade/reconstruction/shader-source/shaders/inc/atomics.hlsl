// ==RECONSTRUCTED== shaders/inc/atomics.hlsl | evidence: embedded in 5 bins (grass_blade_data.hlsl family); no named functions recoverable | status: I/U (names/bodies Inference — atomic helper surface for grass / instance counters)
//
// Atomic counter helpers for append/allocator buffers.

#ifndef TINYGLADE_INC_ATOMICS_HLSL
#define TINYGLADE_INC_ATOMICS_HLSL

// Atomically increment a counter and return the previous value (slot to use).
uint atomic_append(RWByteAddressBuffer counter_buffer, uint counter_byte_offset)
{
    uint previous;
    counter_buffer.InterlockedAdd(counter_byte_offset, 1u, previous);
    return previous;
}

// Atomically add `value` to a counter, returning the previous value.
uint atomic_add(RWByteAddressBuffer buffer, uint byte_offset, uint value)
{
    uint previous;
    buffer.InterlockedAdd(byte_offset, value, previous);
    return previous;
}

#endif // TINYGLADE_INC_ATOMICS_HLSL
