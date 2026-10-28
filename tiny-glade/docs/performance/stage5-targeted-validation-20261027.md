# Bounded retirement and pressure recovery: targeted validation

On 2026-10-27 the strict Windows Vulkan runs passed 3 pressure-reclaim tests and 7 retirement-reservation tests, with zero failures, zero ignored tests and zero Vulkan validation diagnostics. Logs: `.perf/stage5-pressure-reclaim-strict.log` and `.perf/stage5-retirement-reservation-strict.log`.

Two new memory tests exercise real reserved CPU owner payloads and their storage credits. A proactive eviction and its one retry share the same destruction allowance and completion snapshot; later logical allocation attempts make bounded progress and return credits. Injected HostOOM and DeviceLost outcomes do not retry or replenish a failed snapshot. These injected closure outcomes do not simulate a genuinely lost native device or count driver allocation calls.

The new real-device reservation test rejects image/view retirement admission and the second command-buffer ticket under a host budget, verifies exact credit restoration, then creates successfully after the budget is restored. Existing pending-GPU double-payload and pool-reuse tests also passed. No native-create counter is exposed, so this evidence does not claim zero native calls.

These tests do not complete sustained pressure/plateau testing, every physical multi-queue case or the full shutdown/lifecycle acceptance. The legacy public `Device::retire` HostOOM wait boundary remains separately recorded; internal Drop paths use reserved tickets.
