# X1 SPIR-V to MSL translation (generated)

- SPIRV-Cross: `vulkan-sdk-1.4.350.1 1a6169566c73d3da552748fc372fe2bbb856e46e`
- platform: macos; MSL 30000
- Metal compile step: not run
- stages: 638

| profile | ok | translate_fail | over_limit | metal_compile_fail | timeout |
| --- | ---: | ---: | ---: | ---: | ---: |
| plain | 124 | 514 | 0 | 0 | 0 |
| argbuf-tier2-device | 638 | 0 | 0 | 0 | 0 |
| argbuf-tier2-device-rich | 638 | 0 | 0 | 0 | 0 |
| argbuf-tier1 | 124 | 514 | 0 | 0 | 0 |

## plain: failure reasons

| count | status | reason | example |
| ---: | --- | --- | --- |
| 514 | translate_fail | SPIRV-Cross threw an exception: Unsized array of descriptors requires argument buffer tier N | apply_screen_shadows.cs.hlsl |

## argbuf-tier1: failure reasons

| count | status | reason | example |
| ---: | --- | --- | --- |
| 514 | translate_fail | SPIRV-Cross threw an exception: Unsized array of descriptors requires argument buffer tier N | apply_screen_shadows.cs.hlsl |

## Highest slot count per stage

| profile | buffers | textures | samplers |
| --- | ---: | ---: | ---: |
| argbuf-tier2-device | 2 | 0 | 0 |
| argbuf-tier2-device-rich | 2 | 0 | 0 |
| plain | 0 | 0 | 0 |
| argbuf-tier1 | 0 | 0 | 0 |
