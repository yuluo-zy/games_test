import json, os

OUT = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/country-core-render/wp4/_analysis"

def make_job(funcs, name):
    job = [{
        "tool": "get_functions",
        "args": {
            "program": "tiny-glade.exe",
            "functions": ",".join(funcs),
            "fields": "signature,decompiled_code,callees",
            "include_call_context": False
        },
        "timeout_seconds": 300
    }]
    with open(os.path.join(OUT, f"job_{name}.json"), "w") as f:
        json.dump(job, f, indent=1)
    print(name, len(funcs), "functions")

batchA = [
    "0x140937e10",  # ComputeShaderProgram::new
    "0x140938240",  # RasterShaderProgram::new_draw
    "0x1409383b0",  # compile_shader_variants
    "0x140938b00",  # RasterShaderProgramVariant::get_or_generate
    "0x140917860",  # load_compiled_texture_image
    "0x140968ad0",  # FullscreenConfig::resolution
    "0x140968bb0",  # UpscalingPreset::tgsr_resolution_scale
    "0x140976880",  # should_reset_temporal
    "0x140a97650",  # SunSkyColors::lerp
    "0x140b18550",  # DirectionalLight::look_at
    "0x140b18830",  # set_direction_towards_light
    "0x140b18af0",  # set_pixel_offset
    "0x140b18f60",  # ViewportConstantBuilder::build
    "0x140987cb0",  # AvailableGraphicsDevices::from_preference_order
]
make_job(batchA, "A")

batchB = [
    "0x140a07390", "0x140a07440", "0x140a07490", "0x140a074d0",
    "0x140a07540", "0x140a07680", "0x140a07870", "0x140a07a70",
    "0x140a07b30", "0x140a07ca0", "0x140a07e00", "0x140a07f50",
    "0x140a08090", "0x140a081e0", "0x140a08280", "0x140a08440",
    "0x140949170", "0x140a34170", "0x140a35240", "0x140a35b80",
]
make_job(batchB, "B")

batchC = [
    "0x140937560",  # add_compute
    "0x1409378e0",  # get_compute_by_name
    "0x140937cb0",  # debug_raster_get_name
    "0x14091eb30",  # BufferState::new
    "0x14091eb60",  # AssetSsboLibrary::new
    "0x14091ebb0",  # export_from_rg
    "0x1402b31a0",  # get_rg_buffer
    "0x1412098a0",  # get_mut
    "0x1412bc470",  # add_new
    "0x1412badf0",  # add_new_with_data
    "0x14092d450",  # mesh get_by_name
    "0x14092d570",  # mesh try_get_handle_by_name
    "0x14092d750",  # mesh remove
    "0x14092d8d0",  # mesh reload_mesh_src
    "0x14093b530",  # update_color_library
    "0x14093e510",  # ColorLibrarySsbo::deserialize_from
    "0x14093bb10",  # AssetTextureLibrary::new
    "0x14093c200",  # AssetTextureLibrary::add
    "0x14093d310",  # handle_to_bindless_index
    "0x14093d040",  # bindless_textures_dset
]
make_job(batchC, "C")

batchD = [
    "0x14093bd80",  # create_preset_textures
    "0x14093bee0",  # clear_preset_texture
    "0x14093c720",  # create_name_mapping
    "0x14093c970",  # get_handle_and_texture_by_name
    "0x14093ca90",  # texture get_by_name
    "0x14093cd30",  # import_texture_presets_to_rg
    "0x14093ce30",  # export_preset_textures_from_rg
    "0x14093cf30",  # get_rg_texture
    "0x14092d2e0",  # mesh get
    "0x14092d300",  # mesh get_mut
    "0x14092d4f0",  # mesh get_handle_by_name
    "0x140002450",  # mesh transient_meshes
    "0x14092c920",  # mesh new (have from gates, re-decompile for record)
]
make_job(batchD, "D")
