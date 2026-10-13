#define SHADER_VARIANT_DEPTH_ONLY 1
#line 1 "shaders/terrain_editing/floor.raster.hlsl"


#line 1 "shaders/inc/common.hlsl"





#ifndef TINYGLADE_INC_COMMON_HLSL
#define TINYGLADE_INC_COMMON_HLSL

#line 1 "shaders/inc/samplers.hlsl"







#ifndef TINYGLADE_INC_SAMPLERS_HLSL
#define TINYGLADE_INC_SAMPLERS_HLSL

#line 1 "shaders/inc/debug_utils.hlsl"





#ifndef TINYGLADE_INC_DEBUG_UTILS_HLSL
#define TINYGLADE_INC_DEBUG_UTILS_HLSL

#line 1 "shaders/inc/tracked_buffer.hlsl"






#ifndef TINYGLADE_INC_TRACKED_BUFFER_HLSL
#define TINYGLADE_INC_TRACKED_BUFFER_HLSL




uint tracked_buffer_reserve(RWByteAddressBuffer counter, uint counter_offset)
{
    uint previous;
    counter.InterlockedAdd(counter_offset, 1u, previous);
    return previous;
}


void tracked_buffer_store_float(RWByteAddressBuffer buffer, uint base_byte_offset, uint element_index, float value)
{
    buffer.Store<float>(base_byte_offset + element_index * 4u, value);
}

#endif 
#line 9 "shaders/inc/debug_utils.hlsl"



float3 debug_heatmap(float v)
{
    float t = clamp(v, 0.0f, 1.0f);
    return lerp(float3(0.0f, 0.0f, 1.0f), float3(1.0f, 0.0f, 0.0f), t);
}

#endif 
#line 11 "shaders/inc/samplers.hlsl"


SamplerState sampler_lnc : register(s0, space0);
SamplerState sampler_llr : register(s1, space0);
SamplerState sampler_nnc : register(s2, space0);
SamplerState sampler_llc : register(s3, space0);
SamplerState sampler_llb : register(s4, space0);
SamplerState sampler_s5 : register(s5, space0);
SamplerState sampler_s6 : register(s6, space0);
SamplerState sampler_s7 : register(s7, space0);
SamplerState sampler_s8 : register(s8, space0);
SamplerState sampler_default : register(s9, space0);
SamplerState sampler_mipmap : register(s10, space0);
SamplerState sampler_shadow_nearest : register(s11, space0);
SamplerComparisonState sampler_shadow_linear : register(s12, space0);

#endif 
#line 9 "shaders/inc/common.hlsl"


#endif 
#line 3 "shaders/terrain_editing/floor.raster.hlsl"

#line 1 "shaders/inc/consts.hlsl"






#ifndef TINYGLADE_INC_CONSTS_HLSL
#define TINYGLADE_INC_CONSTS_HLSL


static const float RENDER_SCALE = 1.0f;

#endif 
#line 4 "shaders/terrain_editing/floor.raster.hlsl"

#line 1 "shaders/inc/frame_constants.hlsl"








#ifndef TINYGLADE_INC_FRAME_CONSTANTS_HLSL
#define TINYGLADE_INC_FRAME_CONSTANTS_HLSL

#line 1 "shaders/inc/uv.hlsl"






#ifndef TINYGLADE_INC_UV_HLSL
#define TINYGLADE_INC_UV_HLSL


float2 pixel_to_uv(int2 pixel, float2 dims)
{
    return (float2(pixel) + 0.5f) / dims;
}




float2 uv_to_sample(float2 uv)
{
    return uv - 0.5f;
}


float2 sample_to_clip(float2 sample)
{
    return sample * float2(2.0f, -2.0f);
}



float3 sample_to_view(float4x4 sample_to_view_mat, float2 sample_uv, float depth)
{
    return mul(sample_to_view_mat, float4(sample_uv * float2(2.0f, -2.0f), depth, 1.0f)).xyz;
}


float2 view_to_sample(float4x4 view_to_sample_mat, float3 view_pos)
{
    return mul(view_to_sample_mat, float4(view_pos, 1.0f)).xy;
}


float2 view_to_clip(float4x4 view_to_clip_mat, float3 view_pos)
{
    float4 clip = mul(view_to_clip_mat, float4(view_pos, 1.0f));
    return clip.xy / clip.w;
}


float3 world_to_view(float4x4 world_to_view_mat, float3 world_pos)
{
    return mul(world_to_view_mat, float4(world_pos, 1.0f)).xyz;
}


float2 world_to_sample(float4x4 world_to_sample_mat, float3 world_pos)
{
    return mul(world_to_sample_mat, float4(world_pos, 1.0f)).xy;
}

#endif 
#line 12 "shaders/inc/frame_constants.hlsl"










Texture2D<float4> bindless_textures_dyn[] : register(t0, space2);


struct ViewConstants
{
    float4x4 view_to_clip;
    float4x4 clip_to_view;
    float4x4 view_to_sample;
    float4x4 sample_to_view;
    float4x4 world_to_view;
    float4x4 view_to_world;
    float4x4 world_to_sample;
    float4x4 clip_to_prev_clip;
    float4x4 prev_view_to_prev_clip;
    float4x4 prev_clip_to_prev_view;
    float4x4 prev_world_to_prev_view;
    float4x4 prev_view_to_prev_world;
    float2 sample_offset_pixels;
    float2 sample_offset_clip;
    float2 prev_sample_offset_pixels;
    float2 prev_sample_offset_clip;
};

cbuffer frame_constants : register(b0, space1)
{
    float4 sun_color;
    float4 sky_low_color;
    float4 sky_high_color;
    float4 fog_tint_thickness;
    float4 fake_light_color;
    float4 fake_light_dir;
    float4 fake_light2_color;
    float4 fake_light2_dir;
    uint frame_index;
    float delta_time_seconds;
    float ev_mult;
    uint glade_theme_id;
    uint2 inner_rendering_resolution;
    uint game_time_sec;
    float game_time_frac;
    float3 editing_highlight_color;
    float time_of_day;
    float3 dir_to_sun;
    uint camera_idx;
    float3 dir_to_moon;
    uint render_overrides;
    float global_mip_bias;
    uint highlighted_transient_stair_edge_id;
    uint pad1;
    uint pad2;
    float4 debug_tweakers;
    ViewConstants view_constants[5];
};

#endif 
#line 5 "shaders/terrain_editing/floor.raster.hlsl"

#line 1 "shaders/inc/uv.hlsl"






#ifndef TINYGLADE_INC_UV_HLSL
#define TINYGLADE_INC_UV_HLSL


float2 pixel_to_uv(int2 pixel, float2 dims)
{
    return (float2(pixel) + 0.5f) / dims;
}




float2 uv_to_sample(float2 uv)
{
    return uv - 0.5f;
}


float2 sample_to_clip(float2 sample)
{
    return sample * float2(2.0f, -2.0f);
}



float3 sample_to_view(float4x4 sample_to_view_mat, float2 sample_uv, float depth)
{
    return mul(sample_to_view_mat, float4(sample_uv * float2(2.0f, -2.0f), depth, 1.0f)).xyz;
}


float2 view_to_sample(float4x4 view_to_sample_mat, float3 view_pos)
{
    return mul(view_to_sample_mat, float4(view_pos, 1.0f)).xy;
}


float2 view_to_clip(float4x4 view_to_clip_mat, float3 view_pos)
{
    float4 clip = mul(view_to_clip_mat, float4(view_pos, 1.0f));
    return clip.xy / clip.w;
}


float3 world_to_view(float4x4 world_to_view_mat, float3 world_pos)
{
    return mul(world_to_view_mat, float4(world_pos, 1.0f)).xyz;
}


float2 world_to_sample(float4x4 world_to_sample_mat, float3 world_pos)
{
    return mul(world_to_sample_mat, float4(world_pos, 1.0f)).xy;
}

#endif 
#line 6 "shaders/terrain_editing/floor.raster.hlsl"

Texture2D<float4> terrain_heightmap : register(t16, space0);
#line 1 "shaders/inc/terrain_heightmap.inc.hlsl"







#ifndef TINYGLADE_INC_TERRAIN_HEIGHTMAP_HLSL
#define TINYGLADE_INC_TERRAIN_HEIGHTMAP_HLSL


static const float TERRAIN_WORLD_HALF_EXTENT = 65.0;
static const float TERRAIN_HEIGHT_MIN = -2.5;
static const float TERRAIN_HEIGHT_MAX = 24.0;
static const float TERRAIN_SAMPLE_SCALE = 0.0076923076994717121124267578125; 


float2 terrain_heightmap_uv(float2 world_xz)
{
    return world_xz * TERRAIN_SAMPLE_SCALE + 0.5;
}


float get_terrain_height(Texture2D terrain_heightmap, SamplerState s, float2 world_xz)
{
    float t = terrain_heightmap.SampleLevel(s, terrain_heightmap_uv(world_xz), 0.0).x;
    return lerp(TERRAIN_HEIGHT_MIN, TERRAIN_HEIGHT_MAX, t);
}










#ifndef TINYGLADE_TERRAIN_HEIGHTMAP_NO_GLOBAL
float terrain_height(float2 world_xz)
{
    return get_terrain_height(terrain_heightmap, sampler_default, world_xz);
}
#endif


float3 get_terrain_normal(Texture2D terrain_heightmap, SamplerState s, float2 world_xz, float2 heightmap_dims)
{
    float4 g = terrain_heightmap.GatherRed(s, terrain_heightmap_uv(world_xz), int2(0, 0));
    float2 grad = (float2((g.z + g.y) - (g.w + g.x), (g.x + g.y) - (g.w + g.z)) * 0.5) * 26.5;
    grad /= (130.0 / max(heightmap_dims, 1e-6));
    float3 right = normalize(float3(1.0, grad.x, 0.0));
    float3 fwd = normalize(float3(0.0, grad.y, 1.0));
    return normalize(cross(fwd, right));
}

#endif 
#line 8 "shaders/terrain_editing/floor.raster.hlsl"

#line 1 "shaders/inc/rocky_terrain.inc.hlsl"


#ifndef TINYGLADE_INC_ROCKY_TERRAIN_HLSL
#define TINYGLADE_INC_ROCKY_TERRAIN_HLSL


static const float ROCKY_TERRAIN_SAMPLE_SCALE = 0.0076923076994717121124267578125;



float get_rocky_terrain_height(Texture2D rocky_terrain, SamplerState s, float2 world_xz)
{
    float2 uv = world_xz * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5;
    float t = rocky_terrain.SampleLevel(s, uv, 0.0).x;
    return lerp(-2.5, 24.0, t);
}



bool is_rocky_terrain(Texture2D rocky_terrain, SamplerState s, float2 world_xz, float threshold)
{
    float2 uv = world_xz * ROCKY_TERRAIN_SAMPLE_SCALE + 0.5;
    return rocky_terrain.SampleLevel(s, uv, 0.0).x > threshold;
}

#endif 
#line 9 "shaders/terrain_editing/floor.raster.hlsl"

#line 1 "shaders/inc/texture_utils.hlsl"




#ifndef TINYGLADE_INC_TEXTURE_UTILS_HLSL
#define TINYGLADE_INC_TEXTURE_UTILS_HLSL


float4 sample_lod_bias(Texture2D<float4> tex, SamplerState samp, float2 uv, float mip_bias)
{
    return tex.SampleBias(samp, uv, mip_bias);
}


float4 sample_lod(Texture2D<float4> tex, SamplerState samp, float2 uv, float lod)
{
    return tex.SampleLevel(samp, uv, lod);
}


float4 load_texel(Texture2D<float4> tex, int2 pixel)
{
    return tex.Load(int3(pixel, 0));
}

#endif 
#line 10 "shaders/terrain_editing/floor.raster.hlsl"

#line 1 "shaders/terrain_editing/inc_highlight_active_terrain_chunk.inc.hlsl"


#ifndef TINYGLADE_TERRAIN_EDITING_INC_HIGHLIGHT_ACTIVE_TERRAIN_CHUNK_HLSL
#define TINYGLADE_TERRAIN_EDITING_INC_HIGHLIGHT_ACTIVE_TERRAIN_CHUNK_HLSL




float3 highlight_active_terrain_chunk(float3 surface_color, float3 highlight_color, float2 chunk_uv, float strength)
{
    return lerp(surface_color, highlight_color, strength);
}

#endif 
#line 11 "shaders/terrain_editing/floor.raster.hlsl"

#line 1 "shaders/terrain_editing/path_cobblestone_surface.inc.hlsl"


#ifndef TINYGLADE_TERRAIN_EDITING_PATH_COBBLESTONE_SURFACE_HLSL
#define TINYGLADE_TERRAIN_EDITING_PATH_COBBLESTONE_SURFACE_HLSL



struct CobblestoneSurface
{
    float3 albedo;
    float3 normal;
    float roughness;
    float height;
};




CobblestoneSurface evaluate_cobblestone_surface(
    float2 path_uv,
    Texture2D dirthpath_heightmap, SamplerState s,
    Texture2D brick_colors,
    Texture2D stone_floor_normal, Texture2D stone_floor_seed,
    Texture2D stone_floor_height, Texture2D stone_floor_roughness,
    float4 brick_color)
{
    CobblestoneSurface surf;
    float2 stone_uv = path_uv;
    surf.height = dirthpath_heightmap.SampleLevel(s, path_uv, 0.0f).x;
    surf.albedo = brick_color.xyz;
    surf.roughness = stone_floor_roughness.SampleLevel(s, stone_uv, 0.0f).x;
    float3 normal_ts = stone_floor_normal.SampleLevel(s, stone_uv, 0.0f).xyz * 2.0f - 1.0f;
    surf.normal = normalize(normal_ts);
    return surf;
}

#endif 
#line 12 "shaders/terrain_editing/floor.raster.hlsl"




#define TINYGLADE_INC_COLOR_LIBRARY_HLSL
#line 1 "shaders/inc/color_library.hlsl"


#ifndef TINYGLADE_INC_COLOR_LIBRARY_HLSL
#define TINYGLADE_INC_COLOR_LIBRARY_HLSL



struct ColorLibrary {
    float4 temp_color_1;
    float4 temp_color_2;
    float4 temp_color_3;
    float4 roof_color_1;
    float4 roof_color_2;
    float4 sky_low_day;
    float4 sky_high_day;
    float4 sun_tint_day;
    float4 sky_low_dusk;
    float4 sky_high_dusk;
    float4 sun_tint_dusk;
    float4 indirect_light_tint;
    float4 fence_color_1;
    float4 fence_color_2;
    float4 flower_color_1;
    float4 flower_color_2;
    float4 canopy_color_1;
    float4 canopy_color_2;
    float4 grass_1;
    float4 grass_2;
    float4 path;
    float4 sky_base;
    float4 sky_horizon;
    float4 sky_clouds_1;
    float4 sky_clouds_2;
    float4 terrain_1;
    float4 terrain_2;
    float4 terrain_flattened;
    float4 wall_1;
    float4 wall_2;
    float4 wall_moss;
    float4 wall_bricks;
    float4 ivy_branch;
    float4 ivy_leaf_1;
    float4 ivy_leaf_2;
    float4 terrain_highlight;
    float4 night_tint;
};





cbuffer color_library : register(b14, space0) {
    ColorLibrary color_library;
};

#endif 
#line 17 "shaders/terrain_editing/floor.raster.hlsl"


struct ColorLibrary
{
    float4 temp_color_1;
    float4 temp_color_2;
    float4 temp_color_3;
    float4 roof_color_1;
    float4 roof_color_2;
    float4 sky_low_day;
    float4 sky_high_day;
    float4 sun_tint_day;
    float4 sky_low_dusk;
    float4 sky_high_dusk;
    float4 sun_tint_dusk;
    float4 indirect_light_tint;
    float4 fence_color_1;
    float4 fence_color_2;
    float4 flower_color_1;
    float4 flower_color_2;
    float4 canopy_color_1;
    float4 canopy_color_2;
    float4 grass_1;
    float4 grass_2;
    float4 path;
    float4 sky_base;
    float4 sky_horizon;
    float4 sky_clouds_1;
    float4 sky_clouds_2;
    float4 terrain_1;
    float4 terrain_2;
    float4 terrain_flattened;
    float4 wall_1;
    float4 wall_2;
    float4 wall_moss;
    float4 wall_bricks;
    float4 ivy_branch;
    float4 ivy_leaf_1;
    float4 ivy_leaf_2;
    float4 terrain_highlight;
    float4 night_tint;
};

cbuffer color_library : register(b14, space0)
{
    ColorLibrary colors;
};

cbuffer $Globals : register(b13, space0)
{
    uint current_view_index;
    uint2 buildings_mask_dims;
    uint2 terrain_heightmap_dims;
    uint2 visual_terrain_heightmap_dims;
    uint2 rocky_terrain_dims;
    uint2 paths_visual_dims;
    uint2 water_mask_dims;
    uint2 highlight_active_hill_dims;
    uint2 mud_dims;
    uint2 noise_debug_dims;
    uint2 floor_heightmap_dims;
    uint2 dirthpath_heightmap_dims;
    uint2 dirt_1_dims;
    uint2 dirt_2_dims;
    uint2 grass_dims;
    uint2 dirt_mask_dims;
    uint2 stone_floor_normal_dims;
    uint2 stone_floor_seed_dims;
    uint2 stone_floor_height_dims;
    uint2 stone_floor_roughness_dims;
    uint2 stone_floor_seed_2_dims;
    uint2 stone_floor_normal_2_dims;
    uint2 stone_floor_height_2_dims;
    uint2 stone_floor_roughness_2_dims;
    uint2 stone_floor_seed_3_dims;
    uint2 stone_floor_normal_3_dims;
    uint2 stone_floor_height_3_dims;
    uint2 stone_floor_roughness_3_dims;
    uint2 garden_mask_dims;
    uint force_top_down_render;
};

Texture2D<float4> rocky_terrain : register(t18, space0);
Texture2D<float4> water_mask : register(t20, space0);

Texture2D<float4> buildings_mask : register(t15, space0);
Texture2D<float4> paths_visual : register(t19, space0);
Texture2D<float4> mud : register(t22, space0);
Texture2D<float4> dirthpath_heightmap : register(t25, space0);
Texture2D<float4> dirt_1 : register(t26, space0);
Texture2D<float4> dirt_2 : register(t27, space0);
Texture2D<float4> grass : register(t28, space0);
Texture2D<float4> dirt_mask : register(t29, space0);
Texture2DArray<float4> brick_colors : register(t31, space0);
Texture2D<float4> stone_floor_normal : register(t33, space0);
Texture2D<float4> stone_floor_seed : register(t34, space0);
Texture2D<float4> stone_floor_height : register(t35, space0);
Texture2D<float4> stone_floor_roughness : register(t36, space0);
Texture2D<float4> stone_floor_seed_2 : register(t37, space0);
Texture2D<float4> stone_floor_normal_2 : register(t38, space0);
Texture2D<float4> stone_floor_height_2 : register(t39, space0);
Texture2D<float4> stone_floor_roughness_2 : register(t40, space0);
Texture2D<float4> stone_floor_seed_3 : register(t41, space0);
Texture2D<float4> stone_floor_normal_3 : register(t42, space0);
Texture2D<float4> stone_floor_height_3 : register(t43, space0);
Texture2D<float4> stone_floor_roughness_3 : register(t44, space0);
Texture2D<float4> garden_mask : register(t45, space0);



struct VS_OUTPUT
{
    float4 position : SV_Position;
    float3 normal : TEXCOORD0;
    float3 world_pos : TEXCOORD1;
};

float sample_rock_axis(float2 uv, int channel)
{
    bool oob = uv.x < 0.0f || uv.x > 1.0f || uv.y < 0.0f || uv.y > 1.0f;
    if (oob)
    {
        return 0.0f;
    }
    float4 rocky = rocky_terrain.SampleLevel(sampler_default, uv, 0.0f);
    return channel == 0 ? rocky.x : rocky.z;
}

[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
VS_OUTPUT vs_main(uint vertex_id : SV_VertexID)
{
    VS_OUTPUT output;
    float2 grid = float2(float(vertex_id % 91u), float(vertex_id / 91u));
    float2 uv = (((grid + 0.5f.xx) * (1.0f / 91.0f).xx) * 2.0f) - 1.0f.xx;
    float2 world_xz = uv * 65.72222137451171875f.xx;

    float3 pos = float3(world_xz.x, 0.0f, world_xz.y);
    pos.y = lerp(TERRAIN_HEIGHT_MIN, TERRAIN_HEIGHT_MAX,
                 terrain_heightmap.SampleLevel(sampler_default, pos.xz * TERRAIN_SAMPLE_SCALE.xx + 0.5f.xx, 0.0f).x);

    float2 height_uv = pos.xz * TERRAIN_SAMPLE_SCALE.xx + 0.5f.xx;
    float rock_x = sample_rock_axis(height_uv, 0);
    float rock_z = sample_rock_axis(height_uv, 2);

    float4 gathered = terrain_heightmap.GatherRed(sampler_default, height_uv, int2(0, 0));
    float2 world_per_pixel = 130.0f.xx / float2(terrain_heightmap_dims);
    float2 grad = (float2((gathered.z + gathered.y) - (gathered.w + gathered.x), (gathered.x + gathered.y) - (gathered.w + gathered.z)) * 0.5f) * 26.5f;
    grad /= world_per_pixel;
    float3 normal = cross(normalize(float3(0.0f, grad.y, 1.0f)), normalize(float3(1.0f, grad.x, 0.0f)));
    float slope = length(grad);

    float3 displaced = pos;
    if (slope < 2.0f)
    {
        displaced = pos - normal * lerp(0.0f, 0.5f, clamp(rock_x + rock_z, 0.0f, 1.0f));
    }

    float water_blend = smoothstep(0.2f, 0.6f, water_mask.SampleLevel(sampler_llc, displaced.xz * TERRAIN_SAMPLE_SCALE.xx + 0.5f.xx, 0.0f).y) * smoothstep(2.0f, 3.0f, slope);
    float3 water_pos = displaced;
    water_pos.y = lerp(displaced.y, -0.8f, water_blend);
    float4 clip_pos = float4(displaced.x, water_pos.y, displaced.z, 1.0f);

    if (force_top_down_render != 0u)
    {
        output.position = float4(clip_pos.xz * float2(0.0153846154f, -0.0153846154f),
                                 clamp((water_pos.y - TERRAIN_HEIGHT_MIN) * 0.0377358496f, 0.0f, 1.0f), 1.0f);
    }
    else
    {
        output.position = mul(view_constants[current_view_index].world_to_sample, clip_pos);
    }
    output.normal = normal;
    output.world_pos = water_pos;
    return output;
}




float2 pack_material(float3 color, float normal_y, float material_id)
{
    return float2(dot(color, float3(1.0f, 1.0f, 1.0f)) * 0.333f + normal_y * 0.5f, material_id);
}

[[vk::ext_capability(5302)]]
[[vk::ext_extension("SPV_EXT_descriptor_indexing")]]
void ps_main(VS_OUTPUT input,
             out uint2 out_color : SV_Target0, out float out_depth : SV_DepthLessEqual)
{
    float2 height_uv = input.world_pos.xz * TERRAIN_SAMPLE_SCALE.xx + 0.5f.xx;
    float4 paths = paths_visual.SampleLevel(sampler_lnc, height_uv, 0.0f);
    float3 path_weights = paths.yzw;
    float path_presence = paths.x;

    float2 grass_uv = input.world_pos.xz * 0.2f;
    float3 grass_base = lerp(colors.grass_1.xyz, colors.grass_2.xyz, 0.5f);
    float3 albedo = grass_base;

    
    float lod = stone_floor_height.CalculateLevelOfDetail(sampler_llr, grass_uv).x + global_mip_bias;
    float3 sample_heights = float3(stone_floor_height.SampleLevel(sampler_llr, grass_uv, lod).x,
                                   stone_floor_height_2.SampleLevel(sampler_llr, grass_uv, lod).x,
                                   stone_floor_height_3.SampleLevel(sampler_llr, grass_uv, lod).x);
    float3 surface_normal = normalize(cross(ddy(input.world_pos), ddx(input.world_pos)));

    float2 packed = pack_material(albedo, surface_normal.y, 0.0f);
    out_color = uint2(packed);
    out_depth = 1.0f;
}
