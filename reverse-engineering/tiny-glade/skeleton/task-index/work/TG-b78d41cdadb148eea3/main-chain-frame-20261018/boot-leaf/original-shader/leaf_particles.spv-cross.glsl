#version 460
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_scalar_block_layout : require
#extension GL_EXT_spirv_intrinsics : require
layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

struct ViewConstants
{
    mat4 view_to_clip;
    mat4 clip_to_view;
    mat4 view_to_sample;
    mat4 sample_to_view;
    mat4 world_to_view;
    mat4 view_to_world;
    mat4 world_to_sample;
    mat4 clip_to_prev_clip;
    mat4 prev_view_to_prev_clip;
    mat4 prev_clip_to_prev_view;
    mat4 prev_world_to_prev_view;
    mat4 prev_view_to_prev_world;
    vec2 sample_offset_pixels;
    vec2 sample_offset_clip;
    vec2 prev_sample_offset_pixels;
    vec2 prev_sample_offset_clip;
};

struct LeafParticle
{
    vec3 position;
    float rotation;
    float scale;
    vec3 up_axis;
};

struct ParticleEmitter
{
    vec3 position;
    float pad0;
};

uint _149;
const float _150[4] = float[](2.0, 4.0, 8.0, 12.0);
const float _151[4] = float[](2.0, 2.0, 0.5, 0.20000000298023223876953125);
const uint _152[4] = uint[](17u, 18u, 20u, 21u);

layout(set = 1, binding = 0, std140) uniform type_ConstantBuffer_FrameConstants
{
    vec4 sun_color;
    vec4 sky_low_color;
    vec4 sky_high_color;
    vec4 fog_tint_thickness;
    vec4 fake_light_color;
    vec4 fake_light_dir;
    vec4 fake_light2_color;
    vec4 fake_light2_dir;
    uint frame_index;
    float delta_time_seconds;
    float ev_mult;
    uint glade_theme_id;
    uvec2 inner_rendering_resolution;
    uint game_time_sec;
    float game_time_frac;
    vec3 editing_highlight_color;
    float time_of_day;
    vec3 dir_to_sun;
    uint camera_idx;
    vec3 dir_to_moon;
    uint render_overrides;
    float global_mip_bias;
    uint highlighted_transient_stair_edge_id;
    uint pad1;
    uint pad2;
    vec4 debug_tweakers;
    layout(row_major) ViewConstants view_constants[5];
} frame_constants;

layout(set = 0, binding = 14, std140) uniform type_Globals
{
    uvec2 water_noise_texture_dims;
    uvec2 grass_object_push_dims;
    int particle_emitter_count;
    int total_particle_count;
} _Globals;

layout(set = 0, binding = 17, scalar) buffer type_RWStructuredBuffer_LeafParticle
{
    LeafParticle _m0[];
} _alias_leaf_particle_buffer;

layout(set = 0, binding = 18, std430) buffer type_RWStructuredBuffer_ParticleEmitter
{
    ParticleEmitter _m0[];
} _alias_particle_emitter_buffer;

layout(set = 0, binding = 1) uniform sampler sampler_llr;
layout(set = 0, binding = 9) uniform sampler sampler_default;
layout(set = 0, binding = 15) uniform texture2D water_noise_texture;
layout(set = 0, binding = 16) uniform texture2D grass_object_push;

spirv_instruction(set = "GLSL.std.450", id = 80) float spvNMax(float, float);
spirv_instruction(set = "GLSL.std.450", id = 80) vec2 spvNMax(vec2, vec2);
spirv_instruction(set = "GLSL.std.450", id = 80) vec3 spvNMax(vec3, vec3);
spirv_instruction(set = "GLSL.std.450", id = 80) vec4 spvNMax(vec4, vec4);

void main()
{
    do
    {
        if (gl_GlobalInvocationID.x >= uint(_Globals.total_particle_count))
        {
            break;
        }
        LeafParticle _185 = _alias_leaf_particle_buffer._m0[gl_GlobalInvocationID.x];
        vec3 _186 = _185.position;
        if (_Globals.particle_emitter_count == 0)
        {
            _alias_leaf_particle_buffer._m0[gl_GlobalInvocationID.x] = LeafParticle(vec3(0.0), _185.rotation, 0.0, _185.up_axis);
            break;
        }
        vec3 _1295;
        float _1296;
        float _1297;
        vec3 _1298;
        if (_186.y > 0.0500000007450580596923828125)
        {
            float _781 = float(frame_constants.frame_index);
            vec3 _793 = normalize(_186);
            vec3 _801 = (_793 * 0.300000011920928955078125) + (((vec3(1.0, 0.0, 0.0) * (((sin((_781 * 0.0588235296308994293212890625) + _186.x) * sin((_781 * 0.02564102597534656524658203125) + _186.z)) + 1.0) * 2.0)) * 0.4000000059604644775390625) * ((_793.x + 1.0) * 0.5));
            vec2 _808 = ((_186.xy + _186.zy) + vec2(float(gl_GlobalInvocationID.x))) * 0.20000000298023223876953125;
            vec2 _809 = _808 + vec2(234.0);
            ivec2 _811 = ivec2(floor(_809));
            vec2 _812 = fract(_809);
            uvec2 _813 = uvec2(_811);
            uint _815 = _813.y;
            uint _817 = _815 + (_815 << 10u);
            uint _819 = _817 ^ (_817 >> 6u);
            uint _821 = _819 + (_819 << 3u);
            uint _823 = _821 ^ (_821 >> 11u);
            uint _829 = (((_813.x * 1664525u) + (_823 + (_823 << 15u))) + 1013904223u) * 1664525u;
            uint _831 = _829 ^ (_829 >> 11u);
            uint _834 = _831 ^ ((_831 << 7u) & 2636928640u);
            uint _837 = _834 ^ ((_834 << 15u) & 4022730752u);
            uvec2 _845 = uvec2(_811 + ivec2(1, 0));
            uint _847 = _845.y;
            uint _849 = _847 + (_847 << 10u);
            uint _851 = _849 ^ (_849 >> 6u);
            uint _853 = _851 + (_851 << 3u);
            uint _855 = _853 ^ (_853 >> 11u);
            uint _861 = (((_845.x * 1664525u) + (_855 + (_855 << 15u))) + 1013904223u) * 1664525u;
            uint _863 = _861 ^ (_861 >> 11u);
            uint _866 = _863 ^ ((_863 << 7u) & 2636928640u);
            uint _869 = _866 ^ ((_866 << 15u) & 4022730752u);
            uvec2 _877 = uvec2(_811 + ivec2(0, 1));
            uint _879 = _877.y;
            uint _881 = _879 + (_879 << 10u);
            uint _883 = _881 ^ (_881 >> 6u);
            uint _885 = _883 + (_883 << 3u);
            uint _887 = _885 ^ (_885 >> 11u);
            uint _893 = (((_877.x * 1664525u) + (_887 + (_887 << 15u))) + 1013904223u) * 1664525u;
            uint _895 = _893 ^ (_893 >> 11u);
            uint _898 = _895 ^ ((_895 << 7u) & 2636928640u);
            uint _901 = _898 ^ ((_898 << 15u) & 4022730752u);
            uvec2 _909 = uvec2(_811 + ivec2(1));
            uint _911 = _909.y;
            uint _913 = _911 + (_911 << 10u);
            uint _915 = _913 ^ (_913 >> 6u);
            uint _917 = _915 + (_915 << 3u);
            uint _919 = _917 ^ (_917 >> 11u);
            uint _925 = (((_909.x * 1664525u) + (_919 + (_919 << 15u))) + 1013904223u) * 1664525u;
            uint _927 = _925 ^ (_925 >> 11u);
            uint _930 = _927 ^ ((_927 << 7u) & 2636928640u);
            uint _933 = _930 ^ ((_930 << 15u) & 4022730752u);
            vec2 _943 = (_812 * _812) * (vec2(3.0) - (_812 * 2.0));
            float _944 = _943.x;
            float _945 = 1.0 - _944;
            float _947 = _943.y;
            vec2 _961 = _808 + vec2(452.0);
            ivec2 _963 = ivec2(floor(_961));
            vec2 _964 = fract(_961);
            uvec2 _965 = uvec2(_963);
            uint _967 = _965.y;
            uint _969 = _967 + (_967 << 10u);
            uint _971 = _969 ^ (_969 >> 6u);
            uint _973 = _971 + (_971 << 3u);
            uint _975 = _973 ^ (_973 >> 11u);
            uint _981 = (((_965.x * 1664525u) + (_975 + (_975 << 15u))) + 1013904223u) * 1664525u;
            uint _983 = _981 ^ (_981 >> 11u);
            uint _986 = _983 ^ ((_983 << 7u) & 2636928640u);
            uint _989 = _986 ^ ((_986 << 15u) & 4022730752u);
            uvec2 _997 = uvec2(_963 + ivec2(1, 0));
            uint _999 = _997.y;
            uint _1001 = _999 + (_999 << 10u);
            uint _1003 = _1001 ^ (_1001 >> 6u);
            uint _1005 = _1003 + (_1003 << 3u);
            uint _1007 = _1005 ^ (_1005 >> 11u);
            uint _1013 = (((_997.x * 1664525u) + (_1007 + (_1007 << 15u))) + 1013904223u) * 1664525u;
            uint _1015 = _1013 ^ (_1013 >> 11u);
            uint _1018 = _1015 ^ ((_1015 << 7u) & 2636928640u);
            uint _1021 = _1018 ^ ((_1018 << 15u) & 4022730752u);
            uvec2 _1029 = uvec2(_963 + ivec2(0, 1));
            uint _1031 = _1029.y;
            uint _1033 = _1031 + (_1031 << 10u);
            uint _1035 = _1033 ^ (_1033 >> 6u);
            uint _1037 = _1035 + (_1035 << 3u);
            uint _1039 = _1037 ^ (_1037 >> 11u);
            uint _1045 = (((_1029.x * 1664525u) + (_1039 + (_1039 << 15u))) + 1013904223u) * 1664525u;
            uint _1047 = _1045 ^ (_1045 >> 11u);
            uint _1050 = _1047 ^ ((_1047 << 7u) & 2636928640u);
            uint _1053 = _1050 ^ ((_1050 << 15u) & 4022730752u);
            uvec2 _1061 = uvec2(_963 + ivec2(1));
            uint _1063 = _1061.y;
            uint _1065 = _1063 + (_1063 << 10u);
            uint _1067 = _1065 ^ (_1065 >> 6u);
            uint _1069 = _1067 + (_1067 << 3u);
            uint _1071 = _1069 ^ (_1069 >> 11u);
            uint _1077 = (((_1061.x * 1664525u) + (_1071 + (_1071 << 15u))) + 1013904223u) * 1664525u;
            uint _1079 = _1077 ^ (_1077 >> 11u);
            uint _1082 = _1079 ^ ((_1079 << 7u) & 2636928640u);
            uint _1085 = _1082 ^ ((_1082 << 15u) & 4022730752u);
            vec2 _1095 = (_964 * _964) * (vec2(3.0) - (_964 * 2.0));
            float _1096 = _1095.x;
            float _1097 = 1.0 - _1096;
            float _1099 = _1095.y;
            vec2 _1113 = _808 + vec2(99452.0);
            ivec2 _1115 = ivec2(floor(_1113));
            vec2 _1116 = fract(_1113);
            uvec2 _1117 = uvec2(_1115);
            uint _1119 = _1117.y;
            uint _1121 = _1119 + (_1119 << 10u);
            uint _1123 = _1121 ^ (_1121 >> 6u);
            uint _1125 = _1123 + (_1123 << 3u);
            uint _1127 = _1125 ^ (_1125 >> 11u);
            uint _1133 = (((_1117.x * 1664525u) + (_1127 + (_1127 << 15u))) + 1013904223u) * 1664525u;
            uint _1135 = _1133 ^ (_1133 >> 11u);
            uint _1138 = _1135 ^ ((_1135 << 7u) & 2636928640u);
            uint _1141 = _1138 ^ ((_1138 << 15u) & 4022730752u);
            uvec2 _1149 = uvec2(_1115 + ivec2(1, 0));
            uint _1151 = _1149.y;
            uint _1153 = _1151 + (_1151 << 10u);
            uint _1155 = _1153 ^ (_1153 >> 6u);
            uint _1157 = _1155 + (_1155 << 3u);
            uint _1159 = _1157 ^ (_1157 >> 11u);
            uint _1165 = (((_1149.x * 1664525u) + (_1159 + (_1159 << 15u))) + 1013904223u) * 1664525u;
            uint _1167 = _1165 ^ (_1165 >> 11u);
            uint _1170 = _1167 ^ ((_1167 << 7u) & 2636928640u);
            uint _1173 = _1170 ^ ((_1170 << 15u) & 4022730752u);
            uvec2 _1181 = uvec2(_1115 + ivec2(0, 1));
            uint _1183 = _1181.y;
            uint _1185 = _1183 + (_1183 << 10u);
            uint _1187 = _1185 ^ (_1185 >> 6u);
            uint _1189 = _1187 + (_1187 << 3u);
            uint _1191 = _1189 ^ (_1189 >> 11u);
            uint _1197 = (((_1181.x * 1664525u) + (_1191 + (_1191 << 15u))) + 1013904223u) * 1664525u;
            uint _1199 = _1197 ^ (_1197 >> 11u);
            uint _1202 = _1199 ^ ((_1199 << 7u) & 2636928640u);
            uint _1205 = _1202 ^ ((_1202 << 15u) & 4022730752u);
            uvec2 _1213 = uvec2(_1115 + ivec2(1));
            uint _1215 = _1213.y;
            uint _1217 = _1215 + (_1215 << 10u);
            uint _1219 = _1217 ^ (_1217 >> 6u);
            uint _1221 = _1219 + (_1219 << 3u);
            uint _1223 = _1221 ^ (_1221 >> 11u);
            uint _1229 = (((_1213.x * 1664525u) + (_1223 + (_1223 << 15u))) + 1013904223u) * 1664525u;
            uint _1231 = _1229 ^ (_1229 >> 11u);
            uint _1234 = _1231 ^ ((_1231 << 7u) & 2636928640u);
            uint _1237 = _1234 ^ ((_1234 << 15u) & 4022730752u);
            vec2 _1247 = (_1116 * _1116) * (vec2(3.0) - (_1116 * 2.0));
            float _1248 = _1247.x;
            float _1249 = 1.0 - _1248;
            float _1251 = _1247.y;
            float _1261 = (((1.0 - _1251) * (((uintBitsToFloat(((_1141 ^ (_1141 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _1249) + ((uintBitsToFloat(((_1173 ^ (_1173 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _1248))) + (((uintBitsToFloat(((_1205 ^ (_1205 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _1249) * _1251)) + (((uintBitsToFloat(((_1237 ^ (_1237 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _1248) * _1251);
            vec3 _1266 = -vec3(_801.x + mix(-0.5, 0.5, (((1.0 - _947) * (((uintBitsToFloat(((_837 ^ (_837 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _945) + ((uintBitsToFloat(((_869 ^ (_869 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _944))) + (((uintBitsToFloat(((_901 ^ (_901 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _945) * _947)) + (((uintBitsToFloat(((_933 ^ (_933 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _944) * _947)), mix(0.4000000059604644775390625, 3.0, _1261 * _1261) * 0.699999988079071044921875, _801.z + mix(-0.5, 0.5, (((1.0 - _1099) * (((uintBitsToFloat(((_989 ^ (_989 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _1097) + ((uintBitsToFloat(((_1021 ^ (_1021 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _1096))) + (((uintBitsToFloat(((_1053 ^ (_1053 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _1097) * _1099)) + (((uintBitsToFloat(((_1085 ^ (_1085 >> 18u)) & 8388607u) | 1065353216u) - 1.0) * _1096) * _1099)));
            vec3 _1270 = (_1266 * frame_constants.delta_time_seconds) * 1.5;
            vec3 _1271 = _186 + _1270;
            uint _1272 = gl_GlobalInvocationID.x ^ 345346u;
            uint _1274 = _1272 + (_1272 << 10u);
            uint _1276 = _1274 ^ (_1274 >> 6u);
            uint _1278 = _1276 + (_1276 << 3u);
            uint _1280 = _1278 ^ (_1278 >> 11u);
            _1271.y = spvNMax(_1271.y, 0.0500000007450580596923828125);
            _1295 = _1271;
            _1296 = _185.rotation + ((length(_1270) * 3.0) * mix(0.5, 1.0, uintBitsToFloat(((_1280 + (_1280 << 15u)) & 8388607u) | 1065353216u) - 1.0));
            _1297 = _185.scale;
            _1298 = _185.up_axis;
        }
        else
        {
            float _204 = _186.y - (frame_constants.delta_time_seconds * 0.012500000186264514923095703125);
            vec3 _205 = _186;
            _205.y = _204;
            vec4 _214 = textureLod(sampler2D(grass_object_push, sampler_default), (_205.xz * vec2(0.0076923076994717121124267578125)) + vec2(0.5), 0.0);
            uint _426;
            uint _427;
            bool _428;
            bool _429;
            uint _430;
            uint _431;
            bool _432;
            bool _433;
            vec3 _434;
            do
            {
                if (frame_constants.glade_theme_id == 4u)
                {
                    _426 = _149;
                    _427 = _149;
                    _428 = false;
                    _429 = false;
                    _430 = _149;
                    _431 = _149;
                    _432 = false;
                    _433 = false;
                    _434 = vec3(0.0);
                    break;
                }
                float _236 = float(frame_constants.game_time_sec) + frame_constants.game_time_frac;
                uint _241;
                uint _243;
                bool _245;
                bool _247;
                uint _249;
                uint _251;
                bool _253;
                bool _255;
                _241 = _149;
                _243 = _149;
                _245 = false;
                _247 = false;
                _249 = _149;
                _251 = _149;
                _253 = false;
                _255 = false;
                vec2 _173[4];
                for (uint _238 = 0u; _238 < 4u; )
                {
                    float _261 = float(_152[_238]);
                    int _263 = int(floor(_261));
                    float _264 = fract(_261);
                    uint _265 = uint(_263);
                    uint _267 = _265 + (_265 << 10u);
                    uint _269 = _267 ^ (_267 >> 6u);
                    uint _271 = _269 + (_269 << 3u);
                    uint _273 = _271 ^ (_271 >> 11u);
                    bool _276 = _255 ? _255 : true;
                    uint _277 = _255 ? _251 : 1664525u;
                    bool _278 = _253 ? _253 : true;
                    uint _279 = _253 ? _249 : 1013904223u;
                    uint _283 = (((_265 * _277) + (_273 + (_273 << 15u))) + _279) * _277;
                    uint _285 = _283 ^ (_283 >> 11u);
                    uint _288 = _285 ^ ((_285 << 7u) & 2636928640u);
                    uint _291 = _288 ^ ((_288 << 15u) & 4022730752u);
                    uint _293 = _291 ^ (_291 >> 18u);
                    uint _295 = _293 + (_293 << 10u);
                    uint _297 = _295 ^ (_295 >> 6u);
                    uint _299 = _297 + (_297 << 3u);
                    uint _301 = _299 ^ (_299 >> 11u);
                    bool _304 = _247 ? _247 : true;
                    uint _305 = _247 ? _243 : 8388607u;
                    bool _306 = _245 ? _245 : true;
                    uint _307 = _245 ? _241 : 1065353216u;
                    bool _312 = _304 ? _304 : true;
                    uint _313 = _304 ? _305 : 8388607u;
                    bool _314 = _306 ? _306 : true;
                    uint _315 = _306 ? _307 : 1065353216u;
                    uint _322 = uint(_263 + 1);
                    uint _324 = _322 + (_322 << 10u);
                    uint _326 = _324 ^ (_324 >> 6u);
                    uint _328 = _326 + (_326 << 3u);
                    uint _330 = _328 ^ (_328 >> 11u);
                    uint _252 = _276 ? _277 : 1664525u;
                    uint _250 = _278 ? _279 : 1013904223u;
                    uint _336 = (((_322 * _252) + (_330 + (_330 << 15u))) + _250) * _252;
                    uint _338 = _336 ^ (_336 >> 11u);
                    uint _341 = _338 ^ ((_338 << 7u) & 2636928640u);
                    uint _344 = _341 ^ ((_341 << 15u) & 4022730752u);
                    uint _346 = _344 ^ (_344 >> 18u);
                    uint _348 = _346 + (_346 << 10u);
                    uint _350 = _348 ^ (_348 >> 6u);
                    uint _352 = _350 + (_350 << 3u);
                    uint _354 = _352 ^ (_352 >> 11u);
                    bool _357 = _312 ? _312 : true;
                    uint _358 = _312 ? _313 : 8388607u;
                    bool _359 = _314 ? _314 : true;
                    uint _360 = _314 ? _315 : 1065353216u;
                    uint _244 = _357 ? _358 : 8388607u;
                    uint _242 = _359 ? _360 : 1065353216u;
                    float _373 = (_264 * _264) * (3.0 - (2.0 * _264));
                    _173[_238] = (normalize(((vec2(uintBitsToFloat((_293 & _305) | _307) - 1.0, uintBitsToFloat(((_301 + (_301 << 15u)) & _313) | _315) - 1.0) * (1.0 - _373)) + (vec2(uintBitsToFloat((_346 & _358) | _360) - 1.0, uintBitsToFloat(((_354 + (_354 << 15u)) & _244) | _242) - 1.0) * _373)) - vec2(0.5)) * _236) * 3.0;
                    _238++;
                    _241 = _242;
                    _243 = _244;
                    _245 = _359 ? _359 : true;
                    _247 = _357 ? _357 : true;
                    _249 = _250;
                    _251 = _252;
                    _253 = _278 ? _278 : true;
                    _255 = _276 ? _276 : true;
                    continue;
                }
                vec3 _384;
                _384 = vec3(0.0);
                for (uint _387 = 0u; _387 < 1u; )
                {
                    uint _388 = _387 + 1u;
                    float _392 = float(_388) * 10.16640758514404296875;
                    float _393 = cos(_392);
                    float _394 = sin(_392);
                    mat2 _398 = mat2(vec2(_393, _394), vec2(-_394, _393));
                    vec2 _425 = _384.yz + (_398 * ((((textureLod(sampler2D(water_noise_texture, sampler_llr), (((_205.xz * _150[_387]) + _173[_387]) * _398) * 0.01200000010430812835693359375, 0.0).xyz * vec3(1.0, 2.0, 2.0)) + vec3(-0.5, -1.0, -1.0)) * _151[_387]) * vec2(1.0, _150[_387] * 0.01200000010430812835693359375).xyy).yz);
                    _384 = vec3(_384.x, _425.x, _425.y);
                    _387 = _388;
                    continue;
                }
                _426 = _249;
                _427 = _251;
                _428 = _253;
                _429 = _255;
                _430 = _241;
                _431 = _243;
                _432 = _245;
                _433 = _247;
                _434 = _384;
                break;
            } while(false);
            vec2 _440 = _205.xz + ((((vec3(_214.x, 0.0, _214.y) * float(clamp(_214.z, 0.0, 1.0) > 0.0)) * 0.3499999940395355224609375) + (_434.yxz * vec3(2.0, 0.00999999977648258209228515625, 2.0))).xz * 0.100000001490116119384765625);
            vec3 _441 = vec3(_440.x, _205.y, _440.y);
            vec2 _452 = (_440.xy * 0.100000001490116119384765625) + (vec2(float(frame_constants.game_time_sec) + frame_constants.game_time_frac, 0.0) * 1.2000000476837158203125);
            ivec2 _454 = ivec2(floor(_452));
            vec2 _455 = fract(_452);
            uvec2 _456 = uvec2(_454);
            uint _458 = _456.y;
            uint _460 = _458 + (_458 << 10u);
            uint _462 = _460 ^ (_460 >> 6u);
            uint _464 = _462 + (_462 << 3u);
            uint _466 = _464 ^ (_464 >> 11u);
            bool _469 = _429 ? _429 : true;
            uint _470 = _429 ? _427 : 1664525u;
            bool _471 = _428 ? _428 : true;
            uint _472 = _428 ? _426 : 1013904223u;
            uint _476 = (((_456.x * _470) + (_466 + (_466 << 15u))) + _472) * _470;
            uint _478 = _476 ^ (_476 >> 11u);
            uint _481 = _478 ^ ((_478 << 7u) & 2636928640u);
            uint _484 = _481 ^ ((_481 << 15u) & 4022730752u);
            bool _487 = _433 ? _433 : true;
            uint _488 = _433 ? _431 : 8388607u;
            bool _489 = _432 ? _432 : true;
            uint _490 = _432 ? _430 : 1065353216u;
            uvec2 _496 = uvec2(_454 + ivec2(1, 0));
            uint _498 = _496.y;
            uint _500 = _498 + (_498 << 10u);
            uint _502 = _500 ^ (_500 >> 6u);
            uint _504 = _502 + (_502 << 3u);
            uint _506 = _504 ^ (_504 >> 11u);
            bool _509 = _469 ? _469 : true;
            uint _510 = _469 ? _470 : 1664525u;
            bool _511 = _471 ? _471 : true;
            uint _512 = _471 ? _472 : 1013904223u;
            uint _516 = (((_496.x * _510) + (_506 + (_506 << 15u))) + _512) * _510;
            uint _518 = _516 ^ (_516 >> 11u);
            uint _521 = _518 ^ ((_518 << 7u) & 2636928640u);
            uint _524 = _521 ^ ((_521 << 15u) & 4022730752u);
            bool _527 = _487 ? _487 : true;
            uint _528 = _487 ? _488 : 8388607u;
            bool _529 = _489 ? _489 : true;
            uint _530 = _489 ? _490 : 1065353216u;
            uvec2 _536 = uvec2(_454 + ivec2(0, 1));
            uint _538 = _536.y;
            uint _540 = _538 + (_538 << 10u);
            uint _542 = _540 ^ (_540 >> 6u);
            uint _544 = _542 + (_542 << 3u);
            uint _546 = _544 ^ (_544 >> 11u);
            uint _550 = _509 ? _510 : 1664525u;
            uint _552 = _511 ? _512 : 1013904223u;
            uint _556 = (((_536.x * _550) + (_546 + (_546 << 15u))) + _552) * _550;
            uint _558 = _556 ^ (_556 >> 11u);
            uint _561 = _558 ^ ((_558 << 7u) & 2636928640u);
            uint _564 = _561 ^ ((_561 << 15u) & 4022730752u);
            bool _567 = _527 ? _527 : true;
            uint _568 = _527 ? _528 : 8388607u;
            bool _569 = _529 ? _529 : true;
            uint _570 = _529 ? _530 : 1065353216u;
            uvec2 _576 = uvec2(_454 + ivec2(1));
            uint _578 = _576.y;
            uint _580 = _578 + (_578 << 10u);
            uint _582 = _580 ^ (_580 >> 6u);
            uint _584 = _582 + (_582 << 3u);
            uint _586 = _584 ^ (_584 >> 11u);
            uint _589 = (_509 ? _509 : true) ? _550 : 1664525u;
            uint _594 = (((_576.x * _589) + (_586 + (_586 << 15u))) + ((_511 ? _511 : true) ? _552 : 1013904223u)) * _589;
            uint _596 = _594 ^ (_594 >> 11u);
            uint _599 = _596 ^ ((_596 << 7u) & 2636928640u);
            uint _602 = _599 ^ ((_599 << 15u) & 4022730752u);
            bool _605 = _567 ? _567 : true;
            uint _606 = _567 ? _568 : 8388607u;
            bool _607 = _569 ? _569 : true;
            uint _608 = _569 ? _570 : 1065353216u;
            vec2 _616 = (_455 * _455) * (vec2(3.0) - (_455 * 2.0));
            float _617 = _616.x;
            float _618 = 1.0 - _617;
            float _620 = _616.y;
            bool _635 = _605 ? _605 : true;
            bool _637 = _607 ? _607 : true;
            uint _638 = _607 ? _608 : 1065353216u;
            _441.x = _440.x - (((sin((((((1.0 - _620) * (((uintBitsToFloat(((_484 ^ (_484 >> 18u)) & _488) | _490) - 1.0) * _618) + ((uintBitsToFloat(((_524 ^ (_524 >> 18u)) & _528) | _530) - 1.0) * _617))) + (((uintBitsToFloat(((_564 ^ (_564 >> 18u)) & _568) | _570) - 1.0) * _618) * _620)) + (((uintBitsToFloat(((_602 ^ (_602 >> 18u)) & _606) | _608) - 1.0) * _617) * _620)) * 3.1400001049041748046875) - 1.57000005245208740234375) + 0.300000011920928955078125) * 0.0056000002659857273101806640625) * mix(0.699999988079071044921875, 1.0, uintBitsToFloat(0u | _638) - 1.0));
            vec3 _776;
            float _777;
            vec3 _778;
            if (_204 < (-0.02999999932944774627685546875))
            {
                int _651 = int(gl_GlobalInvocationID.x);
                uint _663 = uint(_651 ^ 10945);
                uint _665 = _663 + (_663 << 10u);
                uint _667 = _665 ^ (_665 >> 6u);
                uint _669 = _667 + (_667 << 3u);
                uint _671 = _669 ^ (_669 >> 11u);
                bool _674 = _635 ? _635 : true;
                uint _675 = _635 ? (_605 ? _606 : 8388607u) : 8388607u;
                bool _676 = _637 ? _637 : true;
                uint _677 = _637 ? _638 : 1065353216u;
                uint _684 = uint(_651 ^ 49234);
                uint _686 = _684 + (_684 << 10u);
                uint _688 = _686 ^ (_686 >> 6u);
                uint _690 = _688 + (_688 << 3u);
                uint _692 = _690 ^ (_690 >> 11u);
                bool _695 = _674 ? _674 : true;
                uint _696 = _674 ? _675 : 8388607u;
                bool _697 = _676 ? _676 : true;
                uint _698 = _676 ? _677 : 1065353216u;
                uint _705 = uint(_651 ^ 23458);
                uint _707 = _705 + (_705 << 10u);
                uint _709 = _707 ^ (_707 >> 6u);
                uint _711 = _709 + (_709 << 3u);
                uint _713 = _711 ^ (_711 >> 11u);
                bool _716 = _695 ? _695 : true;
                uint _717 = _695 ? _696 : 8388607u;
                bool _718 = _697 ? _697 : true;
                uint _719 = _697 ? _698 : 1065353216u;
                uint _728 = gl_GlobalInvocationID.x + (gl_GlobalInvocationID.x << 10u);
                uint _730 = _728 ^ (_728 >> 6u);
                uint _732 = _730 + (_730 << 3u);
                uint _734 = _732 ^ (_732 >> 11u);
                uint _736 = _734 + (_734 << 15u);
                bool _737 = _716 ? _716 : true;
                uint _738 = _716 ? _717 : 8388607u;
                bool _739 = _718 ? _718 : true;
                uint _740 = _718 ? _719 : 1065353216u;
                uint _748 = _737 ? _738 : 8388607u;
                uint _750 = _739 ? _740 : 1065353216u;
                uint _756 = gl_GlobalInvocationID.x ^ 2345346u;
                uint _758 = _756 + (_756 << 10u);
                uint _760 = _758 ^ (_758 >> 6u);
                uint _762 = _760 + (_760 << 3u);
                uint _764 = _762 ^ (_762 >> 11u);
                _776 = _alias_particle_emitter_buffer._m0[(uint(_651) ^ (345345u * (frame_constants.frame_index + 1u))) % uint(_Globals.particle_emitter_count)].position + vec3(mix(-0.5, 0.5, uintBitsToFloat(((_671 + (_671 << 15u)) & _675) | _677) - 1.0), mix(-0.5, 0.5, uintBitsToFloat(((_692 + (_692 << 15u)) & _696) | _698) - 1.0), mix(-0.5, 0.5, uintBitsToFloat(((_713 + (_713 << 15u)) & _717) | _719) - 1.0));
                _777 = mix(0.60000002384185791015625, 1.0, uintBitsToFloat((_736 & _738) | _740) - 1.0) * 0.800000011920928955078125;
                _778 = normalize(vec3(mix(-1.0, 1.0, uintBitsToFloat((_736 & _748) | _750) - 1.0), 2.0, mix(-1.0, 1.0, uintBitsToFloat(((_764 + (_764 << 15u)) & ((_737 ? _737 : true) ? _748 : 8388607u)) | ((_739 ? _739 : true) ? _750 : 1065353216u)) - 1.0)));
            }
            else
            {
                _776 = _441;
                _777 = _185.scale - (frame_constants.delta_time_seconds * 0.0500000007450580596923828125);
                _778 = vec3(0.0, 1.0, 0.0);
            }
            _1295 = _776;
            _1296 = _185.rotation;
            _1297 = _777;
            _1298 = _778;
        }
        _alias_leaf_particle_buffer._m0[gl_GlobalInvocationID.x] = LeafParticle(_1295, _1296, _1297, _1298);
        break;
    } while(false);
}

