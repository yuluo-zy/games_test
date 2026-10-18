#version 450
#extension GL_KHR_shader_subgroup_basic : require
#extension GL_KHR_shader_subgroup_shuffle : require
layout(local_size_x = 256, local_size_y = 1, local_size_z = 1) in;

struct SpdGlobalAtomicBuffer
{
    uint counter[6];
};

layout(set = 0, binding = 3, std430) coherent buffer type_RWStructuredBuffer_SpdGlobalAtomicBuffer
{
    SpdGlobalAtomicBuffer _m0[];
} spdGlobalAtomic;

layout(push_constant, std430) uniform type_ConstantBuffer_SpdConstants
{
    uint mips;
    uint numWorkGroups;
    uvec2 workGroupOffset;
} spdConstants;

layout(set = 0, binding = 0, r11f_g11f_b10f) uniform readonly image2D imgSrc;
layout(set = 0, binding = 1, r11f_g11f_b10f) uniform writeonly image2DArray imgDst[12];
layout(set = 0, binding = 2, r11f_g11f_b10f) uniform coherent image2DArray imgDst6;

shared uint spdCounter;
shared float spdIntermediateR[16][16];
shared float spdIntermediateG[16][16];
shared float spdIntermediateB[16][16];
shared float spdIntermediateA[16][16];

void main()
{
    uvec2 _96 = gl_WorkGroupID.xy + spdConstants.workGroupOffset;
    do
    {
        uint _123;
        int _124;
        uint _125;
        int _126;
        int _131;
        int _132;
        int _159;
        uint _99 = gl_LocalInvocationIndex % 64u;
        uint _115 = ((_99 & 1u) | (((_99 >> 2u) & 7u) & 4294967294u)) + (8u * ((gl_LocalInvocationIndex >> 6u) % 2u));
        uint _118 = ((((_99 >> 1u) & 3u) & 3u) | (((_99 >> 3u) & 7u) & 4294967292u)) + (8u * (gl_LocalInvocationIndex >> 7u));
        do
        {
            ivec2 _122 = ivec2(_96 * uvec2(64u));
            _123 = _115 * 2u;
            _124 = int(_123);
            _125 = _118 * 2u;
            _126 = int(_125);
            ivec2 _130 = ivec2(_96 * uvec2(32u));
            _131 = int(_115);
            _132 = int(_118);
            uvec2 _135 = uvec2(_122 + ivec2(_124, _126));
            vec4 _142 = imageLoad(imgSrc, ivec2(uvec2(ivec2(_135))));
            vec4 _146 = imageLoad(imgSrc, ivec2(uvec2(ivec2(_135 + uvec2(0u, 1u)))));
            vec4 _150 = imageLoad(imgSrc, ivec2(uvec2(ivec2(_135 + uvec2(1u, 0u)))));
            vec4 _154 = imageLoad(imgSrc, ivec2(uvec2(ivec2(_135 + uvec2(1u)))));
            vec4 _158 = (((_142 + _146) + _150) + _154) * 0.25;
            _159 = int(gl_WorkGroupID.z);
            imageStore(imgDst[0u], ivec3(uvec3(ivec3(_130 + ivec2(_131, _132), _159))), _158);
            int _167 = int(_123 + 32u);
            int _171 = int(_115 + 16u);
            uvec2 _174 = uvec2(_122 + ivec2(_167, _126));
            vec4 _181 = imageLoad(imgSrc, ivec2(uvec2(ivec2(_174))));
            vec4 _185 = imageLoad(imgSrc, ivec2(uvec2(ivec2(_174 + uvec2(0u, 1u)))));
            vec4 _189 = imageLoad(imgSrc, ivec2(uvec2(ivec2(_174 + uvec2(1u, 0u)))));
            vec4 _193 = imageLoad(imgSrc, ivec2(uvec2(ivec2(_174 + uvec2(1u)))));
            vec4 _197 = (((_181 + _185) + _189) + _193) * 0.25;
            imageStore(imgDst[0u], ivec3(uvec3(ivec3(_130 + ivec2(_171, _132), _159))), _197);
            int _204 = int(_125 + 32u);
            int _208 = int(_118 + 16u);
            uvec2 _211 = uvec2(_122 + ivec2(_124, _204));
            vec4 _218 = imageLoad(imgSrc, ivec2(uvec2(ivec2(_211))));
            vec4 _222 = imageLoad(imgSrc, ivec2(uvec2(ivec2(_211 + uvec2(0u, 1u)))));
            vec4 _226 = imageLoad(imgSrc, ivec2(uvec2(ivec2(_211 + uvec2(1u, 0u)))));
            vec4 _230 = imageLoad(imgSrc, ivec2(uvec2(ivec2(_211 + uvec2(1u)))));
            vec4 _234 = (((_218 + _222) + _226) + _230) * 0.25;
            imageStore(imgDst[0u], ivec3(uvec3(ivec3(_130 + ivec2(_131, _208), _159))), _234);
            uvec2 _244 = uvec2(_122 + ivec2(_167, _204));
            vec4 _251 = imageLoad(imgSrc, ivec2(uvec2(ivec2(_244))));
            vec4 _255 = imageLoad(imgSrc, ivec2(uvec2(ivec2(_244 + uvec2(0u, 1u)))));
            vec4 _259 = imageLoad(imgSrc, ivec2(uvec2(ivec2(_244 + uvec2(1u, 0u)))));
            vec4 _263 = imageLoad(imgSrc, ivec2(uvec2(ivec2(_244 + uvec2(1u)))));
            vec4 _267 = (((_251 + _255) + _259) + _263) * 0.25;
            imageStore(imgDst[0u], ivec3(uvec3(ivec3(_130 + ivec2(_171, _208), _159))), _267);
            if (spdConstants.mips <= 1u)
            {
                break;
            }
            uint _277 = gl_SubgroupInvocationID & 4294967292u;
            uint _278 = _277 | 1u;
            vec4 _279 = subgroupShuffle(_158, _278);
            uint _280 = _277 | 2u;
            vec4 _281 = subgroupShuffle(_158, _280);
            uint _282 = _277 | 3u;
            vec4 _283 = subgroupShuffle(_158, _282);
            vec4 _287 = (((_158 + _279) + _281) + _283) * 0.25;
            vec4 _288 = subgroupShuffle(_197, _278);
            vec4 _289 = subgroupShuffle(_197, _280);
            vec4 _290 = subgroupShuffle(_197, _282);
            vec4 _294 = (((_197 + _288) + _289) + _290) * 0.25;
            vec4 _295 = subgroupShuffle(_234, _278);
            vec4 _296 = subgroupShuffle(_234, _280);
            vec4 _297 = subgroupShuffle(_234, _282);
            vec4 _301 = (((_234 + _295) + _296) + _297) * 0.25;
            vec4 _302 = subgroupShuffle(_267, _278);
            vec4 _303 = subgroupShuffle(_267, _280);
            vec4 _304 = subgroupShuffle(_267, _282);
            vec4 _308 = (((_267 + _302) + _303) + _304) * 0.25;
            if ((gl_LocalInvocationIndex % 4u) == 0u)
            {
                ivec2 _314 = ivec2(_96 * uvec2(16u));
                uint _315 = _115 / 2u;
                int _316 = int(_315);
                uint _317 = _118 / 2u;
                int _318 = int(_317);
                imageStore(imgDst[1u], ivec3(uvec3(ivec3(_314 + ivec2(_316, _318), _159))), _287);
                spdIntermediateR[_315][_317] = _287.x;
                spdIntermediateG[_315][_317] = _287.y;
                spdIntermediateB[_315][_317] = _287.z;
                spdIntermediateA[_315][_317] = _287.w;
                uint _335 = _315 + 8u;
                int _336 = int(_335);
                imageStore(imgDst[1u], ivec3(uvec3(ivec3(_314 + ivec2(_336, _318), _159))), _294);
                spdIntermediateR[_335][_317] = _294.x;
                spdIntermediateG[_335][_317] = _294.y;
                spdIntermediateB[_335][_317] = _294.z;
                spdIntermediateA[_335][_317] = _294.w;
                uint _352 = _317 + 8u;
                int _353 = int(_352);
                imageStore(imgDst[1u], ivec3(uvec3(ivec3(_314 + ivec2(_316, _353), _159))), _301);
                spdIntermediateR[_315][_352] = _301.x;
                spdIntermediateG[_315][_352] = _301.y;
                spdIntermediateB[_315][_352] = _301.z;
                spdIntermediateA[_315][_352] = _301.w;
                imageStore(imgDst[1u], ivec3(uvec3(ivec3(_314 + ivec2(_336, _353), _159))), _308);
                spdIntermediateR[_335][_352] = _308.x;
                spdIntermediateG[_335][_352] = _308.y;
                spdIntermediateB[_335][_352] = _308.z;
                spdIntermediateA[_335][_352] = _308.w;
            }
            break;
        } while(false);
        do
        {
            if (spdConstants.mips <= 2u)
            {
                break;
            }
            barrier();
            float _390 = spdIntermediateR[_115][_118];
            float _392 = spdIntermediateG[_115][_118];
            float _394 = spdIntermediateB[_115][_118];
            vec4 _397 = vec4(_390, _392, _394, spdIntermediateA[_115][_118]);
            uint _399 = gl_SubgroupInvocationID & 4294967292u;
            uint _400 = _399 | 1u;
            vec4 _401 = subgroupShuffle(_397, _400);
            uint _402 = _399 | 2u;
            vec4 _403 = subgroupShuffle(_397, _402);
            uint _404 = _399 | 3u;
            vec4 _405 = subgroupShuffle(_397, _404);
            vec4 _409 = (((_397 + _401) + _403) + _405) * 0.25;
            bool _411 = (gl_LocalInvocationIndex % 4u) == 0u;
            if (_411)
            {
                uint _418 = _118 / 2u;
                imageStore(imgDst[2u], ivec3(uvec3(ivec3(ivec2(_96 * uvec2(8u)) + ivec2(int(_115 / 2u), int(_418)), _159))), _409);
                uint _429 = _115 + (_418 % 2u);
                spdIntermediateR[_429][_118] = _409.x;
                spdIntermediateG[_429][_118] = _409.y;
                spdIntermediateB[_429][_118] = _409.z;
                spdIntermediateA[_429][_118] = _409.w;
            }
            if (spdConstants.mips <= 3u)
            {
                break;
            }
            barrier();
            if (gl_LocalInvocationIndex < 64u)
            {
                uint _445 = _123 + (_118 % 2u);
                float _447 = spdIntermediateR[_445][_125];
                float _449 = spdIntermediateG[_445][_125];
                float _451 = spdIntermediateB[_445][_125];
                vec4 _454 = vec4(_447, _449, _451, spdIntermediateA[_445][_125]);
                vec4 _455 = subgroupShuffle(_454, _400);
                vec4 _456 = subgroupShuffle(_454, _402);
                vec4 _457 = subgroupShuffle(_454, _404);
                vec4 _461 = (((_454 + _455) + _456) + _457) * 0.25;
                if (_411)
                {
                    uint _468 = _118 / 2u;
                    imageStore(imgDst[3u], ivec3(uvec3(ivec3(ivec2(_96 * uvec2(4u)) + ivec2(int(_115 / 2u), int(_468)), _159))), _461);
                    uint _478 = _123 + _468;
                    spdIntermediateR[_478][_125] = _461.x;
                    spdIntermediateG[_478][_125] = _461.y;
                    spdIntermediateB[_478][_125] = _461.z;
                    spdIntermediateA[_478][_125] = _461.w;
                }
            }
            if (spdConstants.mips <= 4u)
            {
                break;
            }
            barrier();
            if (gl_LocalInvocationIndex < 16u)
            {
                uint _494 = (_115 * 4u) + _118;
                uint _495 = _118 * 4u;
                float _497 = spdIntermediateR[_494][_495];
                float _499 = spdIntermediateG[_494][_495];
                float _501 = spdIntermediateB[_494][_495];
                vec4 _504 = vec4(_497, _499, _501, spdIntermediateA[_494][_495]);
                vec4 _505 = subgroupShuffle(_504, _400);
                vec4 _506 = subgroupShuffle(_504, _402);
                vec4 _507 = subgroupShuffle(_504, _404);
                vec4 _511 = (((_504 + _505) + _506) + _507) * 0.25;
                if (_411)
                {
                    uint _516 = _115 / 2u;
                    imageStore(imgDst[4u], ivec3(uvec3(ivec3(ivec2(_96 * uvec2(2u)) + ivec2(int(_516), int(_118 / 2u)), _159))), _511);
                    uint _528 = _516 + _118;
                    spdIntermediateR[_528][0u] = _511.x;
                    spdIntermediateG[_528][0u] = _511.y;
                    spdIntermediateB[_528][0u] = _511.z;
                    spdIntermediateA[_528][0u] = _511.w;
                }
            }
            if (spdConstants.mips <= 5u)
            {
                break;
            }
            barrier();
            if (gl_LocalInvocationIndex < 4u)
            {
                vec4 _551 = vec4(spdIntermediateR[gl_LocalInvocationIndex][0u], spdIntermediateG[gl_LocalInvocationIndex][0u], spdIntermediateB[gl_LocalInvocationIndex][0u], spdIntermediateA[gl_LocalInvocationIndex][0u]);
                vec4 _552 = subgroupShuffle(_551, _400);
                vec4 _553 = subgroupShuffle(_551, _402);
                vec4 _554 = subgroupShuffle(_551, _404);
                if (_411)
                {
                    imageStore(imgDst6, ivec3(uvec3(ivec3(ivec2(_96), _159))), (((_551 + _552) + _553) + _554) * 0.25);
                }
            }
            break;
        } while(false);
        if (spdConstants.mips <= 6u)
        {
            break;
        }
        if (gl_LocalInvocationIndex == 0u)
        {
            uint _574 = atomicAdd(spdGlobalAtomic._m0[0u].counter[gl_WorkGroupID.z], 1u);
            spdCounter = (_574);
        }
        barrier();
        if (spdCounter != (spdConstants.numWorkGroups - 1u))
        {
            break;
        }
        uint _584;
        uint _586;
        spdGlobalAtomic._m0[0u].counter[gl_WorkGroupID.z] = 0u;
        do
        {
            _584 = _115 * 4u;
            int _585 = int(_584);
            _586 = _118 * 4u;
            int _587 = int(_586);
            uvec2 _589 = uvec2(ivec2(_585, _587));
            vec4 _599 = imageLoad(imgDst6, ivec3(uvec3(ivec3(ivec2(_589), _159))));
            vec4 _606 = imageLoad(imgDst6, ivec3(uvec3(ivec3(ivec2(_589 + uvec2(0u, 1u)), _159))));
            vec4 _613 = imageLoad(imgDst6, ivec3(uvec3(ivec3(ivec2(_589 + uvec2(1u, 0u)), _159))));
            vec4 _620 = imageLoad(imgDst6, ivec3(uvec3(ivec3(ivec2(_589 + uvec2(1u)), _159))));
            vec4 _624 = (((_599 + _606) + _613) + _620) * 0.25;
            imageStore(imgDst[6u], ivec3(uvec3(ivec3(_124, _126, _159))), _624);
            int _630 = int(_584 + 2u);
            int _633 = int(_123 + 1u);
            uvec2 _634 = uvec2(ivec2(_630, _587));
            vec4 _644 = imageLoad(imgDst6, ivec3(uvec3(ivec3(ivec2(_634), _159))));
            vec4 _651 = imageLoad(imgDst6, ivec3(uvec3(ivec3(ivec2(_634 + uvec2(0u, 1u)), _159))));
            vec4 _658 = imageLoad(imgDst6, ivec3(uvec3(ivec3(ivec2(_634 + uvec2(1u, 0u)), _159))));
            vec4 _665 = imageLoad(imgDst6, ivec3(uvec3(ivec3(ivec2(_634 + uvec2(1u)), _159))));
            vec4 _669 = (((_644 + _651) + _658) + _665) * 0.25;
            imageStore(imgDst[6u], ivec3(uvec3(ivec3(_633, _126, _159))), _669);
            int _674 = int(_586 + 2u);
            int _677 = int(_125 + 1u);
            uvec2 _678 = uvec2(ivec2(_585, _674));
            vec4 _688 = imageLoad(imgDst6, ivec3(uvec3(ivec3(ivec2(_678), _159))));
            vec4 _695 = imageLoad(imgDst6, ivec3(uvec3(ivec3(ivec2(_678 + uvec2(0u, 1u)), _159))));
            vec4 _702 = imageLoad(imgDst6, ivec3(uvec3(ivec3(ivec2(_678 + uvec2(1u, 0u)), _159))));
            vec4 _709 = imageLoad(imgDst6, ivec3(uvec3(ivec3(ivec2(_678 + uvec2(1u)), _159))));
            vec4 _713 = (((_688 + _695) + _702) + _709) * 0.25;
            imageStore(imgDst[6u], ivec3(uvec3(ivec3(_124, _677, _159))), _713);
            uvec2 _718 = uvec2(ivec2(_630, _674));
            vec4 _728 = imageLoad(imgDst6, ivec3(uvec3(ivec3(ivec2(_718), _159))));
            vec4 _735 = imageLoad(imgDst6, ivec3(uvec3(ivec3(ivec2(_718 + uvec2(0u, 1u)), _159))));
            vec4 _742 = imageLoad(imgDst6, ivec3(uvec3(ivec3(ivec2(_718 + uvec2(1u, 0u)), _159))));
            vec4 _749 = imageLoad(imgDst6, ivec3(uvec3(ivec3(ivec2(_718 + uvec2(1u)), _159))));
            vec4 _753 = (((_728 + _735) + _742) + _749) * 0.25;
            imageStore(imgDst[6u], ivec3(uvec3(ivec3(_633, _677, _159))), _753);
            if (spdConstants.mips <= 7u)
            {
                break;
            }
            vec4 _763 = (((_624 + _669) + _713) + _753) * 0.25;
            imageStore(imgDst[7u], ivec3(uvec3(ivec3(_131, _132, _159))), _763);
            spdIntermediateR[_115][_118] = _763.x;
            spdIntermediateG[_115][_118] = _763.y;
            spdIntermediateB[_115][_118] = _763.z;
            spdIntermediateA[_115][_118] = _763.w;
            break;
        } while(false);
        do
        {
            if (spdConstants.mips <= 8u)
            {
                break;
            }
            barrier();
            float _782 = spdIntermediateR[_115][_118];
            float _784 = spdIntermediateG[_115][_118];
            float _786 = spdIntermediateB[_115][_118];
            vec4 _789 = vec4(_782, _784, _786, spdIntermediateA[_115][_118]);
            uint _791 = gl_SubgroupInvocationID & 4294967292u;
            uint _792 = _791 | 1u;
            vec4 _793 = subgroupShuffle(_789, _792);
            uint _794 = _791 | 2u;
            vec4 _795 = subgroupShuffle(_789, _794);
            uint _796 = _791 | 3u;
            vec4 _797 = subgroupShuffle(_789, _796);
            vec4 _801 = (((_789 + _793) + _795) + _797) * 0.25;
            bool _803 = (gl_LocalInvocationIndex % 4u) == 0u;
            if (_803)
            {
                uint _808 = _118 / 2u;
                imageStore(imgDst[8u], ivec3(uvec3(ivec3(int(_115 / 2u), int(_808), _159))), _801);
                uint _815 = _115 + (_808 % 2u);
                spdIntermediateR[_815][_118] = _801.x;
                spdIntermediateG[_815][_118] = _801.y;
                spdIntermediateB[_815][_118] = _801.z;
                spdIntermediateA[_815][_118] = _801.w;
            }
            if (spdConstants.mips <= 9u)
            {
                break;
            }
            barrier();
            if (gl_LocalInvocationIndex < 64u)
            {
                uint _831 = _123 + (_118 % 2u);
                float _833 = spdIntermediateR[_831][_125];
                float _835 = spdIntermediateG[_831][_125];
                float _837 = spdIntermediateB[_831][_125];
                vec4 _840 = vec4(_833, _835, _837, spdIntermediateA[_831][_125]);
                vec4 _841 = subgroupShuffle(_840, _792);
                vec4 _842 = subgroupShuffle(_840, _794);
                vec4 _843 = subgroupShuffle(_840, _796);
                vec4 _847 = (((_840 + _841) + _842) + _843) * 0.25;
                if (_803)
                {
                    uint _852 = _118 / 2u;
                    imageStore(imgDst[9u], ivec3(uvec3(ivec3(int(_115 / 2u), int(_852), _159))), _847);
                    uint _858 = _123 + _852;
                    spdIntermediateR[_858][_125] = _847.x;
                    spdIntermediateG[_858][_125] = _847.y;
                    spdIntermediateB[_858][_125] = _847.z;
                    spdIntermediateA[_858][_125] = _847.w;
                }
            }
            if (spdConstants.mips <= 10u)
            {
                break;
            }
            barrier();
            if (gl_LocalInvocationIndex < 16u)
            {
                uint _873 = _584 + _118;
                float _875 = spdIntermediateR[_873][_586];
                float _877 = spdIntermediateG[_873][_586];
                float _879 = spdIntermediateB[_873][_586];
                vec4 _882 = vec4(_875, _877, _879, spdIntermediateA[_873][_586]);
                vec4 _883 = subgroupShuffle(_882, _792);
                vec4 _884 = subgroupShuffle(_882, _794);
                vec4 _885 = subgroupShuffle(_882, _796);
                vec4 _889 = (((_882 + _883) + _884) + _885) * 0.25;
                if (_803)
                {
                    uint _892 = _115 / 2u;
                    imageStore(imgDst[10u], ivec3(uvec3(ivec3(int(_892), int(_118 / 2u), _159))), _889);
                    uint _900 = _892 + _118;
                    spdIntermediateR[_900][0u] = _889.x;
                    spdIntermediateG[_900][0u] = _889.y;
                    spdIntermediateB[_900][0u] = _889.z;
                    spdIntermediateA[_900][0u] = _889.w;
                }
            }
            if (spdConstants.mips <= 11u)
            {
                break;
            }
            barrier();
            if (gl_LocalInvocationIndex < 4u)
            {
                vec4 _923 = vec4(spdIntermediateR[gl_LocalInvocationIndex][0u], spdIntermediateG[gl_LocalInvocationIndex][0u], spdIntermediateB[gl_LocalInvocationIndex][0u], spdIntermediateA[gl_LocalInvocationIndex][0u]);
                vec4 _924 = subgroupShuffle(_923, _792);
                vec4 _925 = subgroupShuffle(_923, _794);
                vec4 _926 = subgroupShuffle(_923, _796);
                if (_803)
                {
                    imageStore(imgDst[11u], ivec3(uvec3(ivec3(0, 0, _159))), (((_923 + _924) + _925) + _926) * 0.25);
                }
            }
            break;
        } while(false);
        break;
    } while(false);
}

