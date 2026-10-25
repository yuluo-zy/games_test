#version 450
#extension GL_EXT_nonuniform_qualifier : require
#extension GL_EXT_scalar_block_layout : require
#extension GL_EXT_samplerless_texture_functions : require
#extension GL_EXT_spirv_intrinsics : require
#extension GL_KHR_shader_subgroup_basic : require
#extension GL_KHR_shader_subgroup_shuffle : require
layout(local_size_x = 8, local_size_y = 8, local_size_z = 1) in;

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

float _107;

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

layout(set = 0, binding = 13, scalar) uniform type_Globals
{
    layout(offset = 0) uint history_valid;
    layout(offset = 4) uvec2 history_tex_dims;
    layout(offset = 16) uvec2 reprojection_tex_dims;
    layout(offset = 24) uvec2 depth_tex_dims;
    layout(offset = 32) uvec2 output_lum_tex_dims;
    layout(offset = 40) uvec2 output_chrom_tex_dims;
} _Globals;

layout(set = 0, binding = 3) uniform sampler sampler_llc;
layout(set = 0, binding = 14) uniform texture2D history_tex;
layout(set = 0, binding = 15) uniform texture2D reprojection_tex;
layout(set = 0, binding = 16) uniform texture2D depth_tex;
layout(set = 0, binding = 17, r16f) uniform writeonly image2D output_lum_tex;
layout(set = 0, binding = 18, rg16f) uniform writeonly image2D output_chrom_tex;

spirv_instruction(set = "GLSL.std.450", id = 79) float spvNMin(float, float);
spirv_instruction(set = "GLSL.std.450", id = 79) vec2 spvNMin(vec2, vec2);
spirv_instruction(set = "GLSL.std.450", id = 79) vec3 spvNMin(vec3, vec3);
spirv_instruction(set = "GLSL.std.450", id = 79) vec4 spvNMin(vec4, vec4);
spirv_instruction(set = "GLSL.std.450", id = 80) float spvNMax(float, float);
spirv_instruction(set = "GLSL.std.450", id = 80) vec2 spvNMax(vec2, vec2);
spirv_instruction(set = "GLSL.std.450", id = 80) vec3 spvNMax(vec3, vec3);
spirv_instruction(set = "GLSL.std.450", id = 80) vec4 spvNMax(vec4, vec4);

void main()
{
    vec2 _114 = vec2(_Globals.depth_tex_dims);
    vec2 _117 = vec2(_Globals.output_lum_tex_dims);
    uvec2 _122 = uvec2((vec2(gl_GlobalInvocationID.xy) + vec2(0.5)) * (_114 / _117));
    vec2 _130 = texelFetch(reprojection_tex, ivec2(_122 + uvec2(4294967295u)), int(0u)).xy;
    vec2 _133 = texelFetch(reprojection_tex, ivec2(_122 + uvec2(1u, 4294967295u)), int(0u)).xy;
    vec2 _138 = texelFetch(reprojection_tex, ivec2(_122 + uvec2(4294967295u, 1u)), int(0u)).xy;
    vec2 _143 = texelFetch(reprojection_tex, ivec2(_122 + uvec2(1u)), int(0u)).xy;
    vec2 _144 = spvNMin(spvNMin(spvNMin(_130, _133), _138), _143);
    vec2 _145 = spvNMax(spvNMax(spvNMax(_130, _133), _138), _143);
    bool _153 = any(greaterThan(_145 - _144, spvNMax(vec2(1.0) / _114, abs(_145 + _144)) * 0.100000001490116119384765625));
    bool _160 = (int(_153) | int(subgroupShuffle(_153, gl_SubgroupInvocationID ^ 2u))) != 0;
    uvec2 _198;
    if ((int(_160) | int(subgroupShuffle(_160, gl_SubgroupInvocationID ^ 16u))) != 0)
    {
        vec4 _170 = texelFetch(depth_tex, ivec2(_122), int(0u));
        float _171 = _170.x;
        uvec2 _173;
        int _176;
        _173 = _122;
        _176 = -1;
        uvec2 _174;
        float _179;
        for (float _178 = _171; _176 <= 1; _173 = _174, _176++, _178 = _179)
        {
            _179 = _178;
            _174 = _173;
            for (int _186 = -1; _186 <= 1; )
            {
                ivec2 _191 = ivec2(_186, _176);
                uvec2 _193 = _122 + uvec2(_191);
                vec4 _194 = texelFetch(depth_tex, ivec2(_193), int(0u));
                float _195 = _194.x;
                bool _196 = _195 > _179;
                _179 = _196 ? _195 : _179;
                _186++;
                _174 = mix(_174, _193, bvec2(_196));
                continue;
            }
        }
        _198 = _173;
    }
    else
    {
        _198 = _122;
    }
    vec2 _204 = (((vec2(ivec2(gl_GlobalInvocationID.xy)) + vec2(0.5)) / _117) + texelFetch(reprojection_tex, ivec2(_198), int(0u)).xy) * _117;
    vec2 _206 = floor(_204 - vec2(0.5));
    vec2 _207 = _206 + vec2(0.5);
    vec2 _208 = _204 - _207;
    vec2 _209 = _208 * 0.5;
    vec2 _213 = _208 * (vec2(-0.5) + (_208 * (vec2(1.0) - _209)));
    vec2 _214 = _208 * _208;
    vec2 _215 = _208 * 1.5;
    vec2 _222 = _208 * (vec2(0.5) + (_208 * (vec2(2.0) - _215)));
    vec2 _224 = _214 * (vec2(-0.5) + _209);
    vec2 _225 = (vec2(1.0) + (_214 * (vec2(-2.5) + _215))) + _222;
    vec2 _230 = (_206 + vec2(-0.5)) / _117;
    vec2 _231 = (_206 + vec2(2.5)) / _117;
    vec2 _232 = (_207 + (_222 / _225)) / _117;
    float _233 = _232.x;
    float _242 = _225.x;
    float _244 = _213.y;
    float _247 = _232.y;
    float _255 = _213.x;
    float _257 = _225.y;
    float _277 = _224.x;
    float _290 = _224.y;
    vec3 _304 = (((((((vec4(textureLod(sampler2D(history_tex, sampler_llc), vec2(_233, _230.y), 0.0).xyz, _107) * _242) * _244) + ((vec4(textureLod(sampler2D(history_tex, sampler_llc), vec2(_230.x, _247), 0.0).xyz, _107) * _255) * _257)) + ((vec4(textureLod(sampler2D(history_tex, sampler_llc), _232, 0.0).xyz, _107) * _242) * _257)) + ((vec4(textureLod(sampler2D(history_tex, sampler_llc), vec2(_231.x, _247), 0.0).xyz, _107) * _277) * _257)) + ((vec4(textureLod(sampler2D(history_tex, sampler_llc), vec2(_233, _231.y), 0.0).xyz, _107) * _242) * _290)) / vec4(((((_242 * _244) + (_255 * _257)) + (_242 * _257)) + (_277 * _257)) + (_242 * _290))).xyz;
    bool _314;
    if (_Globals.history_valid != 0u)
    {
        _314 = !(0u == frame_constants.frame_index);
    }
    else
    {
        _314 = false;
    }
    vec3 _317 = (_304 * float(_314)) * mat3(vec3(0.2125999927520751953125, 0.715200006961822509765625, 0.072200000286102294921875), vec3(-0.114600002765655517578125, -0.385399997234344482421875, 0.5), vec3(0.5, -0.4541999995708465576171875, -0.0458000004291534423828125));
    imageStore(output_lum_tex, ivec2(gl_GlobalInvocationID.xy), vec4(_317.x));
    imageStore(output_chrom_tex, ivec2(gl_GlobalInvocationID.xy), _317.yz.xyyy);
}

