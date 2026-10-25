struct LanternPacked
{
    float3 position;
    uint intensity_color_id;
};

ByteAddressBuffer lantern_buf[] : register(t2, space0);
RWByteAddressBuffer _Globals[] : register(u2, space0);
RWByteAddressBuffer _alias_indirection_grid[] : register(u2, space0);
RWByteAddressBuffer _213[] : register(u2, space0);
cbuffer _214_216
{
    uint _216_m0 : packoffset(c0);
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
        if (gl_GlobalInvocationID.x >= _Globals[_213[_216_m0].Load(4)].Load(0))
        {
            break;
        }
        float2 _88 = spvUnpackHalf2x16(lantern_buf[_213[_216_m0].Load(0)].Load(gl_GlobalInvocationID.x * 16 + 12) & 65535u);
        float _89 = _88.x;
        if (_89 <= 0.0f)
        {
            break;
        }
        float _97 = 6.0f * asfloat(532487669 + (asint(_89) >> 1));
        float3 _98 = _97.xxx;
        uint3 _105 = uint3(clamp(floor(((asfloat(lantern_buf[_213[_216_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)) - _98) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(31.0f, 15.0f, 31.0f)));
        uint3 _110 = uint3(clamp(ceil(((asfloat(lantern_buf[_213[_216_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)) + _98) * float3(0.24615384638309478759765625f, 0.123076923191547393798828125f, 0.24615384638309478759765625f)) + float3(16.0f, 2.0f, 16.0f)), 0.0f.xxx, float3(32.0f, 16.0f, 32.0f)));
        uint _111 = _105.z;
        for (uint _113 = _111; _113 < _110.z; _113++)
        {
            uint _120 = _105.y;
            for (uint _122 = _120; _122 < _110.y; _122++)
            {
                uint _129 = _105.x;
                for (uint _131 = _129; _131 < _110.x; _131++)
                {
                    float3 _139 = float3(uint3(_131, _122, _113));
                    float3 _141 = (_139 - float3(16.0f, 2.0f, 16.0f)) * float3(4.0625f, 8.125f, 4.0625f);
                    float3 _143 = (_139 + float3(-15.0f, -1.0f, -15.0f)) * float3(4.0625f, 8.125f, 4.0625f);
                    float _144 = _97 * _97;
                    float _146 = _141.x;
                    float _162;
                    if (asfloat(lantern_buf[_213[_216_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).x < _146)
                    {
                        float _151 = asfloat(lantern_buf[_213[_216_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).x - _146;
                        _162 = _144 - (_151 * _151);
                    }
                    else
                    {
                        float _154 = _143.x;
                        float _161;
                        if (asfloat(lantern_buf[_213[_216_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).x > _154)
                        {
                            float _158 = asfloat(lantern_buf[_213[_216_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).x - _154;
                            _161 = _144 - (_158 * _158);
                        }
                        else
                        {
                            _161 = _144;
                        }
                        _162 = _161;
                    }
                    float _164 = _141.y;
                    float _180;
                    if (asfloat(lantern_buf[_213[_216_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).y < _164)
                    {
                        float _169 = asfloat(lantern_buf[_213[_216_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).y - _164;
                        _180 = _162 - (_169 * _169);
                    }
                    else
                    {
                        float _172 = _143.y;
                        float _179;
                        if (asfloat(lantern_buf[_213[_216_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).y > _172)
                        {
                            float _176 = asfloat(lantern_buf[_213[_216_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).y - _172;
                            _179 = _162 - (_176 * _176);
                        }
                        else
                        {
                            _179 = _162;
                        }
                        _180 = _179;
                    }
                    float _182 = _141.z;
                    float _198;
                    if (asfloat(lantern_buf[_213[_216_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).z < _182)
                    {
                        float _187 = asfloat(lantern_buf[_213[_216_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).z - _182;
                        _198 = _180 - (_187 * _187);
                    }
                    else
                    {
                        float _190 = _143.z;
                        float _197;
                        if (asfloat(lantern_buf[_213[_216_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).z > _190)
                        {
                            float _194 = asfloat(lantern_buf[_213[_216_m0].Load(0)].Load3(gl_GlobalInvocationID.x * 16 + 0)).z - _190;
                            _197 = _180 - (_194 * _194);
                        }
                        else
                        {
                            _197 = _180;
                        }
                        _198 = _197;
                    }
                    if (_198 > 0.0f)
                    {
                        uint _207;
                        _alias_indirection_grid[_213[_216_m0].Load(8)].InterlockedAdd(((32u * ((32u * _122) + _113)) + _131) * 4 + 0, 1u, _207);
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
