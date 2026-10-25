struct ViewConstants
{
    column_major float4x4 view_to_clip;
    column_major float4x4 clip_to_view;
    column_major float4x4 view_to_sample;
    column_major float4x4 sample_to_view;
    column_major float4x4 world_to_view;
    column_major float4x4 view_to_world;
    column_major float4x4 world_to_sample;
    column_major float4x4 clip_to_prev_clip;
    column_major float4x4 prev_view_to_prev_clip;
    column_major float4x4 prev_clip_to_prev_view;
    column_major float4x4 prev_world_to_prev_view;
    column_major float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

struct LanternPacked
{
    float3 position;
    uint intensity_color_id;
};

static uint _86;

RWByteAddressBuffer frame_constants[] : register(u2, space0);
ByteAddressBuffer lantern_buf[] : register(t2, space0);
RWByteAddressBuffer _Globals[] : register(u2, space0);
RWByteAddressBuffer _alias_indirection_grid[] : register(u2, space0);
RWByteAddressBuffer _alias_light_indices[] : register(u2, space0);
RWByteAddressBuffer _343[] : register(u2, space0);
cbuffer _344_346
{
    uint _346_m0 : packoffset(c0);
};


static uint3 gl_GlobalInvocationID;
struct SPIRV_Cross_Input
{
    uint3 gl_GlobalInvocationID : SV_DispatchThreadID;
};

uint spvPackHalf2x16(float2 value)
{
    uint2 Packed = f32tof16(value);
    return Packed.x | (Packed.y << 16);
}

float2 spvUnpackHalf2x16(uint value)
{
    return f16tof32(uint2(value & 0xffff, value >> 16));
}

void comp_main()
{
    do
    {
        if (gl_GlobalInvocationID.x >= _Globals[_343[_346_m0].Load(4)].Load(0))
        {
            break;
        }
        float2 _121 = spvUnpackHalf2x16(lantern_buf[_343[_346_m0].Load(0)].Load(gl_GlobalInvocationID.x * 16 + 12) & 65535u);
        float _122 = _121.x;
        if (_122 <= 0.0f)
        {
            break;
        }
        float _130 = 6.0f * asfloat(532487669 + (asint(_122) >> 1));
        float3 _131 = _130.xxx;
        uint3 _138 = uint3(clamp(floor(((asfloat(lantern_buf[_343[_346_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)) - _131) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        uint3 _143 = uint3(clamp(ceil(((asfloat(lantern_buf[_343[_346_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)) + _131) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(32.0f, 16.0f, 32.0f)));
        uint _147 = frame_constants[_343[_346_m0].Load(16)].Load(128) + (frame_constants[_343[_346_m0].Load(16)].Load(128) << 10u);
        uint _149 = _147 ^ (_147 >> 6u);
        uint _151 = _149 + (_149 << 3u);
        uint _153 = _151 ^ (_151 >> 11u);
        uint _159 = (((gl_GlobalInvocationID.x * 1664525u) + (_153 + (_153 << 15u))) + 1013904223u) * 1664525u;
        uint _161 = _159 ^ (_159 >> 11u);
        uint _164 = _161 ^ ((_161 << 7u) & 2636928640u);
        uint _167 = _164 ^ ((_164 << 15u) & 4022730752u);
        uint _169 = _167 ^ (_167 >> 18u);
        uint _170 = _138.z;
        uint _176;
        uint _178;
        bool _180;
        bool _182;
        uint _184;
        uint _172 = _170;
        uint _175 = 0u;
        uint _177 = 0u;
        bool _179 = false;
        bool _181 = false;
        uint _183 = _169;
        for (; _172 < _143.z; _172++, _175 = _176, _177 = _178, _179 = _180, _181 = _182, _183 = _184)
        {
            uint _189 = _138.y;
            _176 = _175;
            _178 = _177;
            _180 = _179;
            _182 = _181;
            _184 = _183;
            uint _194;
            uint _195;
            bool _196;
            bool _197;
            uint _198;
            for (uint _191 = _189; _191 < _143.y; _191++, _176 = _194, _178 = _195, _180 = _196, _182 = _197, _184 = _198)
            {
                uint _203 = _138.x;
                _194 = _176;
                _195 = _178;
                _196 = _180;
                _197 = _182;
                _198 = _184;
                uint _205;
                uint _207;
                bool _208;
                bool _209;
                uint _210;
                for (uint _211 = _203; _211 < _143.x; _194 = _205, _195 = _207, _196 = _208, _197 = _209, _198 = _210, _211++)
                {
                    float3 _218 = float3(uint3(_211, _191, _172));
                    float3 _220 = (_218 - float3(16.0f, 2.0f, 16.0f)) * float3(4.0625f, 8.125f, 4.0625f);
                    float3 _222 = (_218 + float3(-15.0f, -1.0f, -15.0f)) * float3(4.0625f, 8.125f, 4.0625f);
                    float _223 = _130 * _130;
                    float _225 = _220.x;
                    float _241;
                    if (asfloat(lantern_buf[_343[_346_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).x < _225)
                    {
                        float _238 = asfloat(lantern_buf[_343[_346_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).x - _225;
                        _241 = _223 - (_238 * _238);
                    }
                    else
                    {
                        float _230 = _222.x;
                        float _237;
                        if (asfloat(lantern_buf[_343[_346_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).x > _230)
                        {
                            float _234 = asfloat(lantern_buf[_343[_346_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).x - _230;
                            _237 = _223 - (_234 * _234);
                        }
                        else
                        {
                            _237 = _223;
                        }
                        _241 = _237;
                    }
                    float _243 = _220.y;
                    float _259;
                    if (asfloat(lantern_buf[_343[_346_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).y < _243)
                    {
                        float _256 = asfloat(lantern_buf[_343[_346_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).y - _243;
                        _259 = _241 - (_256 * _256);
                    }
                    else
                    {
                        float _248 = _222.y;
                        float _255;
                        if (asfloat(lantern_buf[_343[_346_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).y > _248)
                        {
                            float _252 = asfloat(lantern_buf[_343[_346_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).y - _248;
                            _255 = _241 - (_252 * _252);
                        }
                        else
                        {
                            _255 = _241;
                        }
                        _259 = _255;
                    }
                    float _261 = _220.z;
                    float _277;
                    if (asfloat(lantern_buf[_343[_346_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).z < _261)
                    {
                        float _274 = asfloat(lantern_buf[_343[_346_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).z - _261;
                        _277 = _259 - (_274 * _274);
                    }
                    else
                    {
                        float _266 = _222.z;
                        float _273;
                        if (asfloat(lantern_buf[_343[_346_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).z > _266)
                        {
                            float _270 = asfloat(lantern_buf[_343[_346_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).z - _266;
                            _273 = _259 - (_270 * _270);
                        }
                        else
                        {
                            _273 = _259;
                        }
                        _277 = _273;
                    }
                    if (_277 > 0.0f)
                    {
                        uint _286;
                        _alias_indirection_grid[_343[_346_m0].Load(8)].InterlockedAdd(((32u * ((32u * _191) + _172)) + _211) * 4 + 0, 1048576u, _286);
                        uint _287 = _286 & 1048575u;
                        uint _288 = _286 >> 20u;
                        uint _333;
                        uint _334;
                        bool _335;
                        bool _336;
                        uint _337;
                        if (_288 < 64u)
                        {
                            _alias_light_indices[_343[_346_m0].Load(12)].Store((_287 + _288) * 4 + 0, gl_GlobalInvocationID.x);
                            _333 = _194;
                            _334 = _195;
                            _335 = _196;
                            _336 = _197;
                            _337 = _198;
                        }
                        else
                        {
                            uint _294 = _198 + (_198 << 10u);
                            uint _296 = _294 ^ (_294 >> 6u);
                            uint _298 = _296 + (_296 << 3u);
                            uint _300 = _298 ^ (_298 >> 11u);
                            uint _302 = _300 + (_300 << 15u);
                            uint _304 = _197 ? _195 : 8388607u;
                            uint _306 = _196 ? _194 : 1065353216u;
                            uint _330;
                            if ((asfloat((_198 & _304) | _306) - 1.0f) < (64.0f / float(_288 + 1u)))
                            {
                                uint _318 = _302 + (_302 << 10u);
                                uint _320 = _318 ^ (_318 >> 6u);
                                uint _322 = _320 + (_320 << 3u);
                                uint _324 = _322 ^ (_322 >> 11u);
                                _alias_light_indices[_343[_346_m0].Load(12)].Store((_287 + (_302 % 64u)) * 4 + 0, gl_GlobalInvocationID.x);
                                _330 = _324 + (_324 << 15u);
                            }
                            else
                            {
                                _330 = _302;
                            }
                            _333 = _306;
                            _334 = _304;
                            _335 = _196 ? _196 : true;
                            _336 = _197 ? _197 : true;
                            _337 = _330;
                        }
                        _205 = _333;
                        _207 = _334;
                        _208 = _335;
                        _209 = _336;
                        _210 = _337;
                    }
                    else
                    {
                        _205 = _194;
                        _207 = _195;
                        _208 = _196;
                        _209 = _197;
                        _210 = _198;
                    }
                }
            }
        }
        break;
    } while(false);
}

[numthreads(64, 1, 1)]
void main(SPIRV_Cross_Input stage_input)
{
    gl_GlobalInvocationID = stage_input.gl_GlobalInvocationID;
    comp_main();
}
