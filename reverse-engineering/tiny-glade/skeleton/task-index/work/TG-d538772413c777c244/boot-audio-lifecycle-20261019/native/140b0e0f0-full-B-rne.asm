140b0e0f0: 56 push rsi
140b0e0f1: 57 push rdi
140b0e0f2: 53 push rbx
140b0e0f3: 4881ece0000000 sub rsp, 0xe0
140b0e0fa: 4889d6 mov rsi, rdx
140b0e0fd: 4889cf mov rdi, rcx
140b0e100: e8db542500 call 0x140d635e0 ; std::time::Instant::now
140b0e105: 4889442430 mov qword ptr [rsp + 0x30], rax
140b0e10a: 89542438 mov dword ptr [rsp + 0x38], edx
140b0e10e: 8b471c mov eax, dword ptr [rdi + 0x1c]
140b0e111: 488b4f10 mov rcx, qword ptr [rdi + 0x10]
140b0e115: 8901 mov dword ptr [rcx], eax
140b0e117: 488b0f mov rcx, qword ptr [rdi]
140b0e11a: e8e18ff4ff call 0x140a57100 ; country_core::resources::audio_resources::AudioSamplesLoadedPromise::recv
140b0e11f: 488b0582e2b102 mov rax, qword ptr [rip + 0x2b1e282]
140b0e126: 488b00 mov rax, qword ptr [rax]
140b0e129: 4883f802 cmp rax, 2
140b0e12d: 0f86eb000000 jbe 0x140b0e21e
140b0e133: 488d4c2430 lea rcx, [rsp + 0x30]
140b0e138: e8d3572500 call 0x140d63910 ; std::time::Instant::elapsed
140b0e13d: 4889442450 mov qword ptr [rsp + 0x50], rax
140b0e142: 89542458 mov dword ptr [rsp + 0x58], edx
140b0e146: 488d442450 lea rax, [rsp + 0x50]
140b0e14b: 4889442440 mov qword ptr [rsp + 0x40], rax
140b0e150: 488d05f9c32700 lea rax, [rip + 0x27c3f9]
140b0e157: 4889442448 mov qword ptr [rsp + 0x48], rax
140b0e15c: 488d0d359cfd01 lea rcx, [rip + 0x1fd9c35]
140b0e163: e818fcb6ff call 0x14067dd80 ; toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut | toolbar::mei::widgets::ActionHintView::as_inner_mut | system_decorator::decorator_visual_state::DecoratorPersistentVisualState::entities | country_core::resources::walls::decorator::DecoratorId::new | country_core::resources::walls::decorator::DeferredDecoratorOpEvents::events | country_core::resources::world_raster_defs::impl$45::deref_mut | country_core::resources::world_raster_defs::impl$69::deref | country_core::resources::image_formation::ImageFormationConfigs::initial | country_core::resources::rt::RtBvhBuilderGlueResource::new | country_core::systems::collision::world::SubworldIndex::from_usize_unchecked | country_core::systems::ui::resources::ui_state::UiState::widget | country_core::resources::gui::impl$0::deref | country_core::resources::gui::PendingEguiContextResource::into_egui_context_resource | country_core::render::impl$1::deref | country_core::systems::steam::AppSteamIntegration::client | image::traits::impl$0::as_bytes | rhapsody::raw::buffer::impl$0::as_managed_handle | rhapsody::raw::shader::impl$2::deref | rhapsody::raw::shader::impl$3::deref | rhapsody::rg::executor::PassPgoNameId::from_name | rhapsody
140b0e168: 0f1000 movups xmm0, xmmword ptr [rax]
140b0e16b: 8b4010 mov eax, dword ptr [rax + 0x10]
140b0e16e: 48c784249000000003000000 mov qword ptr [rsp + 0x90], 3
140b0e17a: 488d0d2f9cfd01 lea rcx, [rip + 0x1fd9c2f] ; 0x142ae7db0 'country_core::startup::load_audio'
140b0e181: 48898c2498000000 mov qword ptr [rsp + 0x98], rcx
140b0e189: 48c78424a000000021000000 mov qword ptr [rsp + 0xa0], 0x21
140b0e195: 488d15dc9bfd01 lea rdx, [rip + 0x1fd9bdc]
140b0e19c: 48899424b0000000 mov qword ptr [rsp + 0xb0], rdx
140b0e1a4: 48c78424b800000002000000 mov qword ptr [rsp + 0xb8], 2
140b0e1b0: 488d542440 lea rdx, [rsp + 0x40]
140b0e1b5: 48899424c0000000 mov qword ptr [rsp + 0xc0], rdx
140b0e1bd: 48c78424c800000001000000 mov qword ptr [rsp + 0xc8], 1
140b0e1c9: 48c78424d000000000000000 mov qword ptr [rsp + 0xd0], 0
140b0e1d5: 48c744246000000000 mov qword ptr [rsp + 0x60], 0
140b0e1de: 48894c2468 mov qword ptr [rsp + 0x68], rcx
140b0e1e3: 48c744247021000000 mov qword ptr [rsp + 0x70], 0x21
140b0e1ec: 48c744247800000000 mov qword ptr [rsp + 0x78], 0
140b0e1f5: 0f11842480000000 movups xmmword ptr [rsp + 0x80], xmm0
140b0e1fd: c78424a800000001000000 mov dword ptr [rsp + 0xa8], 1
140b0e208: 898424ac000000 mov dword ptr [rsp + 0xac], eax
140b0e20f: 488d4c242f lea rcx, [rsp + 0x2f]
140b0e214: 488d542460 lea rdx, [rsp + 0x60]
140b0e219: e812c12300 call 0x140d4a330 ; log::__private_api::impl$2::log
140b0e21e: 0fb605e469b202 movzx eax, byte ptr [rip + 0x2b269e4]
140b0e225: b91c000000 mov ecx, 0x1c
140b0e22a: ba04000000 mov edx, 4
140b0e22f: e8dc59b0ff call 0x140613c10 ; 
140b0e234: 4885c0 test rax, rax
140b0e237: 0f841e010000 je 0x140b0e35b
140b0e23d: c70000000000 mov dword ptr [rax], 0
140b0e243: c7401800000000 mov dword ptr [rax + 0x18], 0
140b0e24a: 48c744246001000000 mov qword ptr [rsp + 0x60], 1
140b0e253: 4889442468 mov qword ptr [rsp + 0x68], rax
140b0e258: 48c744247001000000 mov qword ptr [rsp + 0x70], 1
140b0e261: 48c744247801000000 mov qword ptr [rsp + 0x78], 1
140b0e26a: 4c8d05679bfd01 lea r8, [rip + 0x1fd9b67]
140b0e271: 488d542460 lea rdx, [rsp + 0x60]
140b0e276: 4889f1 mov rcx, rsi
140b0e279: e8b2107d00 call 0x1412df330 ; bevy_ecs::system::commands::Commands::insert_resource<country_core::asset_libraries::audio_trigger_library::AudioTriggerLibrary>
140b0e27e: 488b7e08 mov rdi, qword ptr [rsi + 8]
140b0e282: 4885ff test rdi, rdi
140b0e285: 7504 jne 0x140b0e28b
140b0e287: 488b7e10 mov rdi, qword ptr [rsi + 0x10]
140b0e28b: 488b07 mov rax, qword ptr [rdi]
140b0e28e: 488b5f10 mov rbx, qword ptr [rdi + 0x10]
140b0e292: 4829d8 sub rax, rbx
140b0e295: 4883f80b cmp rax, 0xb
140b0e299: 7679 jbe 0x140b0e314
140b0e29b: 488b4708 mov rax, qword ptr [rdi + 8]
140b0e29f: 488d0d4aea7c00 lea rcx, [rip + 0x7cea4a]
140b0e2a6: 48890c18 mov qword ptr [rax + rbx], rcx
140b0e2aa: c744180800000000 mov dword ptr [rax + rbx + 8], 0
140b0e2b2: 4883c30c add rbx, 0xc
140b0e2b6: 48895f10 mov qword ptr [rdi + 0x10], rbx
140b0e2ba: 48c744246002000000 mov qword ptr [rsp + 0x60], 2
140b0e2c3: c784249000000000000000 mov dword ptr [rsp + 0x90], 0
140b0e2ce: 4c8d051b9bfd01 lea r8, [rip + 0x1fd9b1b]
140b0e2d5: 488d542460 lea rdx, [rsp + 0x60]
140b0e2da: 4889f1 mov rcx, rsi
140b0e2dd: e8be2c7d00 call 0x1412e0fa0 ; bevy_ecs::system::commands::Commands::insert_resource<country_core::systems::audio::game_loading_audio::GameLoadingAudioState>
140b0e2e2: 488b07 mov rax, qword ptr [rdi]
140b0e2e5: 488b7710 mov rsi, qword ptr [rdi + 0x10]
140b0e2e9: 4829f0 sub rax, rsi
140b0e2ec: 4883f807 cmp rax, 7
140b0e2f0: 7647 jbe 0x140b0e339
140b0e2f2: 488b4708 mov rax, qword ptr [rdi + 8]
140b0e2f6: 488d0db3ae7c00 lea rcx, [rip + 0x7caeb3]
140b0e2fd: 48890c30 mov qword ptr [rax + rsi], rcx
140b0e301: 4883c608 add rsi, 8
140b0e305: 48897710 mov qword ptr [rdi + 0x10], rsi
140b0e309: 4881c4e0000000 add rsp, 0xe0
140b0e310: 5b pop rbx
140b0e311: 5f pop rdi
140b0e312: 5e pop rsi
140b0e313: c3 ret 
140b0e314: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
140b0e31d: 41b80c000000 mov r8d, 0xc
140b0e323: 41b901000000 mov r9d, 1
140b0e329: 4889f9 mov rcx, rdi
140b0e32c: 4889da mov rdx, rbx
140b0e32f: e82cb7d901 call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140b0e334: e962ffffff jmp 0x140b0e29b
140b0e339: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
140b0e342: 41b808000000 mov r8d, 8
140b0e348: 41b901000000 mov r9d, 1
140b0e34e: 4889f9 mov rcx, rdi
140b0e351: 4889f2 mov rdx, rsi
140b0e354: e807b7d901 call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140b0e359: eb97 jmp 0x140b0e2f2
140b0e35b: 4c8d0566f51502 lea r8, [rip + 0x215f566]
140b0e362: b904000000 mov ecx, 4
140b0e367: ba1c000000 mov edx, 0x1c
140b0e36c: e872acdc01 call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140b0e371: cc int3 
