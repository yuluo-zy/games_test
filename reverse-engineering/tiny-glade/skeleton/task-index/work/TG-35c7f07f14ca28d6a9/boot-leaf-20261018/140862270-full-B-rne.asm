140862270: 55 push rbp
140862271: 4157 push r15
140862273: 4156 push r14
140862275: 4155 push r13
140862277: 4154 push r12
140862279: 56 push rsi
14086227a: 57 push rdi
14086227b: 53 push rbx
14086227c: 4881ec28030000 sub rsp, 0x328
140862283: 488dac2480000000 lea rbp, [rsp + 0x80]
14086228b: 440f29b590020000 movaps xmmword ptr [rbp + 0x290], xmm14
140862293: 440f29ad80020000 movaps xmmword ptr [rbp + 0x280], xmm13
14086229b: 440f29a570020000 movaps xmmword ptr [rbp + 0x270], xmm12
1408622a3: 440f299d60020000 movaps xmmword ptr [rbp + 0x260], xmm11
1408622ab: 440f299550020000 movaps xmmword ptr [rbp + 0x250], xmm10
1408622b3: 440f298d40020000 movaps xmmword ptr [rbp + 0x240], xmm9
1408622bb: 440f298530020000 movaps xmmword ptr [rbp + 0x230], xmm8
1408622c3: 660f7fbd20020000 movdqa xmmword ptr [rbp + 0x220], xmm7
1408622cb: 660f7fb510020000 movdqa xmmword ptr [rbp + 0x210], xmm6
1408622d3: 48c78508020000feffffff mov qword ptr [rbp + 0x208], 0xfffffffffffffffe
1408622de: 4c89cb mov rbx, r9
1408622e1: 4989d6 mov r14, rdx
1408622e4: 498b00 mov rax, qword ptr [r8]
1408622e7: 498b5008 mov rdx, qword ptr [r8 + 8]
1408622eb: 4c8b00 mov r8, qword ptr [rax]
1408622ee: 4531c9 xor r9d, r9d
1408622f1: 4d89c2 mov r10, r8
1408622f4: 4c2b5218 sub r10, qword ptr [rdx + 0x18]
1408622f8: 4d0f42d1 cmovb r10, r9
1408622fc: 4c2b4238 sub r8, qword ptr [rdx + 0x38]
140862300: 4d0f42c1 cmovb r8, r9
140862304: 4c8b5a30 mov r11, qword ptr [rdx + 0x30]
140862308: 4d29c3 sub r11, r8
14086230b: 4d0f42d9 cmovb r11, r9
14086230f: 410f97c0 seta r8b
140862313: 488b7210 mov rsi, qword ptr [rdx + 0x10]
140862317: 4c29d6 sub rsi, r10
14086231a: 490f42f1 cmovb rsi, r9
14086231e: 488b5240 mov rdx, qword ptr [rdx + 0x40]
140862322: 410f97c1 seta r9b
140862326: 4c01de add rsi, r11
140862329: 4989d2 mov r10, rdx
14086232c: 4929f2 sub r10, rsi
14086232f: 4c8910 mov qword ptr [rax], r10
140862332: 4508c1 or r9b, r8b
140862335: 0f84ad0a0000 je 0x140862de8
14086233b: 488910 mov qword ptr [rax], rdx
14086233e: 488b0563a0dc02 mov rax, qword ptr [rip + 0x2dca063]
140862345: 488b00 mov rax, qword ptr [rax]
140862348: 4883f802 cmp rax, 2
14086234c: 0f86c3000000 jbe 0x140862415
140862352: 488d05e73d2302 lea rax, [rip + 0x2233de7]
140862359: 4889ce mov rsi, rcx
14086235c: 4889c1 mov rcx, rax
14086235f: e81cbae1ff call 0x14067dd80 ; toolbar::mei::widgets::tweaker::TweakerView::as_inner_mut | toolbar::mei::widgets::ActionHintView::as_inner_mut | system_decorator::decorator_visual_state::DecoratorPersistentVisualState::entities | country_core::resources::walls::decorator::DecoratorId::new | country_core::resources::walls::decorator::DeferredDecoratorOpEvents::events | country_core::resources::world_raster_defs::impl$45::deref_mut | country_core::resources::world_raster_defs::impl$69::deref | country_core::resources::image_formation::ImageFormationConfigs::initial | country_core::resources::rt::RtBvhBuilderGlueResource::new | country_core::systems::collision::world::SubworldIndex::from_usize_unchecked | country_core::systems::ui::resources::ui_state::UiState::widget | country_core::resources::gui::impl$0::deref | country_core::resources::gui::PendingEguiContextResource::into_egui_context_resource | country_core::render::impl$1::deref | country_core::systems::steam::AppSteamIntegration::client | image::traits::impl$0::as_bytes | rhapsody::raw::buffer::impl$0::as_managed_handle | rhapsody::raw::shader::impl$2::deref | rhapsody::raw::shader::impl$3::deref | rhapsody::rg::executor::PassPgoNameId::from_name | rhapsody::raw::allocator::GpuAllocator::inner | gpu_profiler::shared::NanoSecond::from_raw_ns | utils::swizzlers::impl$5::as_vec2 | log::__private_api::loc | country_core::resources::walls::wall_holes::FinalizedHoleStorage::inner | utils::geometry::intersect::Triangle::points_mut | asset::prefab::impl$4::as_ref | country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::systems::render_loop::draw_passes::ui_2d_pass::TextPrimSsbo> | country_core::asset_libraries::ssbo_library::impl$0::buffer<country_core::geometry::instanced_wall::BrickDrawListSsbo> | parry3d::shape::trimesh::impl$8::qbvh | parry3d::shape::trimesh::impl$8::qbvh | bevy_ptr::dangling_with_align | event_listener::notify::impl$33::into_notification | flume::signal::impl$1::as_ptr | cpp_demangle::ast::impl$133::downcast_to_pointer_to_member<alloc::string::String> | cpp_demangle::ast::impl$93::downcast_to_type<alloc::string::String> | cpp_demangle::ast::impl$127::downcast_to_array_type<alloc::string::String> | motomoto::Reader::new | country_core::asset_libraries::ssbo_library::impl$0::buffer<country_core::resources::color_id::GateColorIdSsbo> | country_core::asset_libraries::ssbo_library::impl$0::handle<country_core::resources::color_id::HalftimberColorIdSsbo>
140862364: f30f6f00 movdqu xmm0, xmmword ptr [rax]
140862368: 8b4010 mov eax, dword ptr [rax + 0x10]
14086236b: 48c7850801000003000000 mov qword ptr [rbp + 0x108], 3
140862376: 488d0ddb3d2302 lea rcx, [rip + 0x2233ddb] ; @142a96158 'country_core::startup::spawn_preplaced_treestrees.json'
14086237d: 48898d10010000 mov qword ptr [rbp + 0x110], rcx
140862384: 48c785180100002c000000 mov qword ptr [rbp + 0x118], 0x2c
14086238f: 488d15623d2302 lea rdx, [rip + 0x2233d62]
140862396: 48899528010000 mov qword ptr [rbp + 0x128], rdx
14086239d: 48c7853001000001000000 mov qword ptr [rbp + 0x130], 1
1408623a8: 48c7853801000008000000 mov qword ptr [rbp + 0x138], 8
1408623b3: 48c785d800000000000000 mov qword ptr [rbp + 0xd8], 0
1408623be: 660fefc9 pxor xmm1, xmm1
1408623c2: f30f7f8d40010000 movdqu xmmword ptr [rbp + 0x140], xmm1
1408623ca: 48898de0000000 mov qword ptr [rbp + 0xe0], rcx
1408623d1: 48c785e80000002c000000 mov qword ptr [rbp + 0xe8], 0x2c
1408623dc: 48c785f000000000000000 mov qword ptr [rbp + 0xf0], 0
1408623e7: f30f7f85f8000000 movdqu xmmword ptr [rbp + 0xf8], xmm0
1408623ef: c7852001000001000000 mov dword ptr [rbp + 0x120], 1
1408623f9: 898524010000 mov dword ptr [rbp + 0x124], eax
1408623ff: 488d8d58010000 lea rcx, [rbp + 0x158]
140862406: 488d95d8000000 lea rdx, [rbp + 0xd8]
14086240d: e81e7f4e00 call 0x140d4a330 ; log::__private_api::impl$2::log
140862412: 4889f1 mov rcx, rsi
140862415: 488b01 mov rax, qword ptr [rcx]
140862418: 48894518 mov qword ptr [rbp + 0x18], rax
14086241c: 0fb64058 movzx eax, byte ptr [rax + 0x58]
140862420: c1e003 shl eax, 3
140862423: 488d0d66422302 lea rcx, [rip + 0x2234266]
14086242a: 4c8b0c08 mov r9, qword ptr [rax + rcx]
14086242e: 488d0d83422302 lea rcx, [rip + 0x2234283]
140862435: 488b0408 mov rax, qword ptr [rax + rcx]
140862439: 4889442420 mov qword ptr [rsp + 0x20], rax
14086243e: 488d15d5d02802 lea rdx, [rip + 0x228d0d5] ; @142aef51a 'assets/gladestatecrates/country-core/src/resources/session.rsloading_background.jpg'
140862445: 488dbdd8000000 lea rdi, [rbp + 0xd8]
14086244c: 41b80c000000 mov r8d, 0xc
140862452: 4889f9 mov rcx, rdi
140862455: e8b6035000 call 0x140d62810 ; std::path::Path::_join
14086245a: 4889f9 mov rcx, rdi
14086245d: e86e4a5100 call 0x140d76ed0 ; std::sys::os_str::wtf8::Buf::as_mut_slice | alloc::boxed::convert::impl$19::from::impl$0::description
140862462: 4989d0 mov r8, rdx
140862465: 48c74424200a000000 mov qword ptr [rsp + 0x20], 0xa
14086246e: 4c8d0d0f3d2302 lea r9, [rip + 0x2233d0f] ; @142a96184 'trees.json'
140862475: 488d8d58010000 lea rcx, [rbp + 0x158]
14086247c: 4889c2 mov rdx, rax
14086247f: e88c035000 call 0x140d62810 ; std::path::Path::_join
140862484: 488d8d58010000 lea rcx, [rbp + 0x158]
14086248b: e8404a5100 call 0x140d76ed0 ; std::sys::os_str::wtf8::Buf::as_mut_slice | alloc::boxed::convert::impl$19::from::impl$0::description
140862490: 4989d0 mov r8, rdx
140862493: 488d4d50 lea rcx, [rbp + 0x50]
140862497: 4889c2 mov rdx, rax
14086249a: e8a16c4f00 call 0x140d59140 ; std::fs::read_to_string::inner
14086249f: 488b9558010000 mov rdx, qword ptr [rbp + 0x158]
1408624a6: 4885d2 test rdx, rdx
1408624a9: 7412 je 0x1408624bd
1408624ab: 488b8d60010000 mov rcx, qword ptr [rbp + 0x160]
1408624b2: 41b801000000 mov r8d, 1
1408624b8: e86317dbff call 0x140613c20 ; 
1408624bd: 488b4550 mov rax, qword ptr [rbp + 0x50]
1408624c1: 48898500020000 mov qword ptr [rbp + 0x200], rax
1408624c8: 48f7d8 neg rax
1408624cb: 0f808e090000 jo 0x140862e5f
1408624d1: 488b5558 mov rdx, qword ptr [rbp + 0x58]
1408624d5: 4c8b4560 mov r8, qword ptr [rbp + 0x60]
1408624d9: 488d8d58010000 lea rcx, [rbp + 0x158]
1408624e0: 488995e8010000 mov qword ptr [rbp + 0x1e8], rdx
1408624e7: e8c43dba00 call 0x1414062b0 ; serde_json::de::from_str<alloc::vec::Vec<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global> >
1408624ec: 488bb558010000 mov rsi, qword ptr [rbp + 0x158]
1408624f3: 4889f0 mov rax, rsi
1408624f6: 48f7d8 neg rax
1408624f9: 0f809b090000 jo 0x140862e9a
1408624ff: 4c897520 mov qword ptr [rbp + 0x20], r14
140862503: 4c8bb560010000 mov r14, qword ptr [rbp + 0x160]
14086250a: 488bbd68010000 mov rdi, qword ptr [rbp + 0x168]
140862511: 488b9500020000 mov rdx, qword ptr [rbp + 0x200]
140862518: 4885d2 test rdx, rdx
14086251b: 7412 je 0x14086252f
14086251d: 41b801000000 mov r8d, 1
140862523: 488b8de8010000 mov rcx, qword ptr [rbp + 0x1e8]
14086252a: e8f116dbff call 0x140613c20 ; 
14086252f: 4c8bbd10030000 mov r15, qword ptr [rbp + 0x310]
140862536: 488b95d8000000 mov rdx, qword ptr [rbp + 0xd8]
14086253d: 4885d2 test rdx, rdx
140862540: 7412 je 0x140862554
140862542: 488b8de0000000 mov rcx, qword ptr [rbp + 0xe0]
140862549: 41b801000000 mov r8d, 1
14086254f: e8cc16dbff call 0x140613c20 ; 
140862554: 488d3cbf lea rdi, [rdi + rdi*4]
140862558: 48c1e704 shl rdi, 4
14086255c: 4c01f7 add rdi, r14
14086255f: 4c89b5a0000000 mov qword ptr [rbp + 0xa0], r14
140862566: 4889b5b0000000 mov qword ptr [rbp + 0xb0], rsi
14086256d: 4889bdb8000000 mov qword ptr [rbp + 0xb8], rdi
140862574: 488db560010000 lea rsi, [rbp + 0x160]
14086257b: 8b431c mov eax, dword ptr [rbx + 0x1c]
14086257e: 8985ac010000 mov dword ptr [rbp + 0x1ac], eax
140862584: 488b03 mov rax, qword ptr [rbx]
140862587: 488985e8010000 mov qword ptr [rbp + 0x1e8], rax
14086258e: 488b4310 mov rax, qword ptr [rbx + 0x10]
140862592: 48894530 mov qword ptr [rbp + 0x30], rax
140862596: 418b471c mov eax, dword ptr [r15 + 0x1c]
14086259a: 8985a8010000 mov dword ptr [rbp + 0x1a8], eax
1408625a0: 498b07 mov rax, qword ptr [r15]
1408625a3: 498b4f10 mov rcx, qword ptr [r15 + 0x10]
1408625a7: 48894d28 mov qword ptr [rbp + 0x28], rcx
1408625ab: 48898500020000 mov qword ptr [rbp + 0x200], rax
1408625b2: 4883c020 add rax, 0x20
1408625b6: 48894540 mov qword ptr [rbp + 0x40], rax
1408625ba: 4531ed xor r13d, r13d
1408625bd: 660f6f35bb372302 movdqa xmm6, xmmword ptr [rip + 0x22337bb] ; @142a95d80 'clearing/tree/bush'
1408625c5: 660f6e3dc3372302 movd xmm7, dword ptr [rip + 0x22337c3] ; @142a95d90 'sh'
1408625cd: 4c8d3d46cf2802 lea r15, [rip + 0x228cf46] ; @142aef51a 'assets/gladestatecrates/country-core/src/resources/session.rsloading_background.jpg'
1408625d4: 440f2805a4c30c02 movaps xmm8, xmmword ptr [rip + 0x20cc3a4]
1408625dc: f3440f100d2f122202 movss xmm9, dword ptr [rip + 0x222122f]
1408625e5: f2440f1015b2372302 movsd xmm10, qword ptr [rip + 0x22337b2]
1408625ee: f2440f101db9372302 movsd xmm11, qword ptr [rip + 0x22337b9]
1408625f7: 450f57e4 xorps xmm12, xmm12
1408625fb: 4939fe cmp r14, rdi
1408625fe: 0f84ce070000 je 0x140862dd2
140862604: 4983c650 add r14, 0x50
140862608: eb1b jmp 0x140862625
14086260a: 660f1f440000 nop word ptr [rax + rax]
140862610: 488d43b0 lea rax, [rbx - 0x50]
140862614: 4c8d7350 lea r14, [rbx + 0x50]
140862618: 4883c050 add rax, 0x50
14086261c: 4839f8 cmp rax, rdi
14086261f: 0f84b0070000 je 0x140862dd5
140862625: 4c89f3 mov rbx, r14
140862628: 498b56b0 mov rdx, qword ptr [r14 - 0x50]
14086262c: 498b46f8 mov rax, qword ptr [r14 - 8]
140862630: 48898590000000 mov qword ptr [rbp + 0x90], rax
140862637: 410f1046e8 movups xmm0, xmmword ptr [r14 - 0x18]
14086263c: 0f298580000000 movaps xmmword ptr [rbp + 0x80], xmm0
140862643: 410f1046d8 movups xmm0, xmmword ptr [r14 - 0x28]
140862648: 0f294570 movaps xmmword ptr [rbp + 0x70], xmm0
14086264c: 410f1046c8 movups xmm0, xmmword ptr [r14 - 0x38]
140862651: 0f294560 movaps xmmword ptr [rbp + 0x60], xmm0
140862655: 410f1046b8 movups xmm0, xmmword ptr [r14 - 0x48]
14086265a: 0f294550 movaps xmmword ptr [rbp + 0x50], xmm0
14086265e: 4889d0 mov rax, rdx
140862661: 48f7d8 neg rax
140862664: 0f806b070000 jo 0x140862dd5
14086266a: 48899558010000 mov qword ptr [rbp + 0x158], rdx
140862671: 488b8590000000 mov rax, qword ptr [rbp + 0x90]
140862678: 48894640 mov qword ptr [rsi + 0x40], rax
14086267c: 660f6f4550 movdqa xmm0, xmmword ptr [rbp + 0x50]
140862681: 660f6f4d60 movdqa xmm1, xmmword ptr [rbp + 0x60]
140862686: 0f285570 movaps xmm2, xmmword ptr [rbp + 0x70]
14086268a: 0f289d80000000 movaps xmm3, xmmword ptr [rbp + 0x80]
140862691: 0f115e30 movups xmmword ptr [rsi + 0x30], xmm3
140862695: 0f115620 movups xmmword ptr [rsi + 0x20], xmm2
140862699: f30f7f4e10 movdqu xmmword ptr [rsi + 0x10], xmm1
14086269e: f30f7f06 movdqu xmmword ptr [rsi], xmm0
1408626a2: 488b8d60010000 mov rcx, qword ptr [rbp + 0x160]
1408626a9: 4c8bb568010000 mov r14, qword ptr [rbp + 0x168]
1408626b0: 4983fe12 cmp r14, 0x12
1408626b4: 0f8586000000 jne 0x140862740
1408626ba: f30f6f01 movdqu xmm0, xmmword ptr [rcx]
1408626be: 660f74c6 pcmpeqb xmm0, xmm6
1408626c2: 0fb74110 movzx eax, word ptr [rcx + 0x10]
1408626c6: 660f6ec8 movd xmm1, eax
1408626ca: 660f74cf pcmpeqb xmm1, xmm7
1408626ce: 660fdbc8 pand xmm1, xmm0
1408626d2: 660fd7c1 pmovmskb eax, xmm1
1408626d6: 3dffff0000 cmp eax, 0xffff
1408626db: 7563 jne 0x140862740
1408626dd: 4885d2 test rdx, rdx
1408626e0: 740b je 0x1408626ed
1408626e2: 41b801000000 mov r8d, 1
1408626e8: e83315dbff call 0x140613c20 ; 
1408626ed: 488b9570010000 mov rdx, qword ptr [rbp + 0x170]
1408626f4: 4885d2 test rdx, rdx
1408626f7: 7412 je 0x14086270b
1408626f9: 488b8d78010000 mov rcx, qword ptr [rbp + 0x178]
140862700: 41b801000000 mov r8d, 1
140862706: e81515dbff call 0x140613c20 ; 
14086270b: 488b9588010000 mov rdx, qword ptr [rbp + 0x188]
140862712: 4889d0 mov rax, rdx
140862715: 48f7d8 neg rax
140862718: 0f80f2feffff jo 0x140862610
14086271e: 0f83ecfeffff jae 0x140862610
140862724: 488b8d90010000 mov rcx, qword ptr [rbp + 0x190]
14086272b: 41b801000000 mov r8d, 1
140862731: e8ea14dbff call 0x140613c20 ; 
140862736: e9d5feffff jmp 0x140862610
14086273b: 0f1f440000 nop dword ptr [rax + rax]
140862740: 4c89fa mov rdx, r15
140862743: 4d89ef mov r15, r13
140862746: 48898db8010000 mov qword ptr [rbp + 0x1b8], rcx
14086274d: 48899da8000000 mov qword ptr [rbp + 0xa8], rbx
140862754: 488b9d78010000 mov rbx, qword ptr [rbp + 0x178]
14086275b: 4c8ba580010000 mov r12, qword ptr [rbp + 0x180]
140862762: 488b4518 mov rax, qword ptr [rbp + 0x18]
140862766: 0fb64058 movzx eax, byte ptr [rax + 0x58]
14086276a: c1e003 shl eax, 3
14086276d: 488d0d1c3f2302 lea rcx, [rip + 0x2233f1c]
140862774: 4c8b0c08 mov r9, qword ptr [rax + rcx]
140862778: 488d0d393f2302 lea rcx, [rip + 0x2233f39]
14086277f: 488b0408 mov rax, qword ptr [rax + rcx]
140862783: 4889442420 mov qword ptr [rsp + 0x20], rax
140862788: 41b80c000000 mov r8d, 0xc
14086278e: 488d8dd8000000 lea rcx, [rbp + 0xd8]
140862795: 4889cf mov rdi, rcx
140862798: e873005000 call 0x140d62810 ; std::path::Path::_join
14086279d: 4c8d6db0 lea r13, [rbp - 0x50]
1408627a1: 4c89e9 mov rcx, r13
1408627a4: 4889da mov rdx, rbx
1408627a7: 4d89e0 mov r8, r12
1408627aa: 4989f9 mov r9, rdi
1408627ad: e82e41a800 call 0x1412e68e0 ; relative_path::RelativePath::to_path<std::path::PathBuf>
1408627b2: 4c896d10 mov qword ptr [rbp + 0x10], r13
1408627b6: 4c89e9 mov rcx, r13
1408627b9: e812475100 call 0x140d76ed0 ; std::sys::os_str::wtf8::Buf::as_mut_slice | alloc::boxed::convert::impl$19::from::impl$0::description
1408627be: 4989d0 mov r8, rdx
1408627c1: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1408627c8: 4889c2 mov rdx, rax
1408627cb: e870694f00 call 0x140d59140 ; std::fs::read_to_string::inner
1408627d0: 488b85c0010000 mov rax, qword ptr [rbp + 0x1c0]
1408627d7: 488b95c8010000 mov rdx, qword ptr [rbp + 0x1c8]
1408627de: 488985c0000000 mov qword ptr [rbp + 0xc0], rax
1408627e5: 48f7d8 neg rax
1408627e8: 0f801f070000 jo 0x140862f0d
1408627ee: 4d89fd mov r13, r15
1408627f1: 4c8b85d0010000 mov r8, qword ptr [rbp + 0x1d0]
1408627f8: 4889f9 mov rcx, rdi
1408627fb: 488995d0000000 mov qword ptr [rbp + 0xd0], rdx
140862802: e8e934ba00 call 0x141405cf0 ; serde_json::de::from_str<alloc::vec::Vec<country_core::resources::placement_data::PlacementDataSsbo,alloc::alloc::Global> >
140862807: 488b85d8000000 mov rax, qword ptr [rbp + 0xd8]
14086280e: 488985c8000000 mov qword ptr [rbp + 0xc8], rax
140862815: 48f7d8 neg rax
140862818: 488b8db8010000 mov rcx, qword ptr [rbp + 0x1b8]
14086281f: 0f80ad060000 jo 0x140862ed2
140862825: 488b85e0000000 mov rax, qword ptr [rbp + 0xe0]
14086282c: 488985b0010000 mov qword ptr [rbp + 0x1b0], rax
140862833: 488bbde8000000 mov rdi, qword ptr [rbp + 0xe8]
14086283a: 4c89f2 mov rdx, r14
14086283d: e83e38cf00 call 0x141556080 ; asset::prefab::PrefabName::new
140862842: 488985f0010000 mov qword ptr [rbp + 0x1f0], rax
140862849: 488995f8010000 mov qword ptr [rbp + 0x1f8], rdx
140862850: 8b85a0010000 mov eax, dword ptr [rbp + 0x1a0]
140862856: 8985b8010000 mov dword ptr [rbp + 0x1b8], eax
14086285c: 488d3c7f lea rdi, [rdi + rdi*2]
140862860: 48c1e704 shl rdi, 4
140862864: 4c8bbdb0010000 mov r15, qword ptr [rbp + 0x1b0]
14086286b: 4c01ff add rdi, r15
14086286e: 6690 nop 
140862870: 4939ff cmp r15, rdi
140862873: 0f8467040000 je 0x140862ce0
140862879: 4c89f8 mov rax, r15
14086287c: 4983c730 add r15, 0x30
140862880: f30f1000 movss xmm0, dword ptr [rax]
140862884: 410f54c0 andps xmm0, xmm8
140862888: 440f2ec8 ucomiss xmm9, xmm0
14086288c: 76e2 jbe 0x140862870
14086288e: f30f104008 movss xmm0, dword ptr [rax + 8]
140862893: 410f54c0 andps xmm0, xmm8
140862897: 440f2ec8 ucomiss xmm9, xmm0
14086289b: 76d3 jbe 0x140862870
14086289d: 488d8dd8000000 lea rcx, [rbp + 0xd8]
1408628a4: 488b5520 mov rdx, qword ptr [rbp + 0x20]
1408628a8: e8f31d4b00 call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
1408628ad: 488b9de0000000 mov rbx, qword ptr [rbp + 0xe0]
1408628b4: 4885db test rbx, rbx
1408628b7: 7507 jne 0x1408628c0
1408628b9: 488b9de8000000 mov rbx, qword ptr [rbp + 0xe8]
1408628c0: 4c8bb5f8000000 mov r14, qword ptr [rbp + 0xf8]
1408628c7: 488b03 mov rax, qword ptr [rbx]
1408628ca: 4c8b6310 mov r12, qword ptr [rbx + 0x10]
1408628ce: 4c29e0 sub rax, r12
1408628d1: 4883f82f cmp rax, 0x2f
1408628d5: 0f8671030000 jbe 0x140862c4c
1408628db: 498d95a1860100 lea rdx, [r13 + 0x186a1]
1408628e2: 488b4308 mov rax, qword ptr [rbx + 8]
1408628e6: 488d0d034ca700 lea rcx, [rip + 0xa74c03]
1408628ed: 4a890c20 mov qword ptr [rax + r12], rcx
1408628f1: 488d0d3824d000 lea rcx, [rip + 0xd02438]
1408628f8: 4a894c2008 mov qword ptr [rax + r12 + 8], rcx
1408628fd: 4e89742010 mov qword ptr [rax + r12 + 0x10], r14
140862902: 48895538 mov qword ptr [rbp + 0x38], rdx
140862906: 4a89542018 mov qword ptr [rax + r12 + 0x18], rdx
14086290b: 42c644202001 mov byte ptr [rax + r12 + 0x20], 1
140862911: 42c644202800 mov byte ptr [rax + r12 + 0x28], 0
140862917: 4983c430 add r12, 0x30
14086291b: 4c896310 mov qword ptr [rbx + 0x10], r12
14086291f: 488b85f8000000 mov rax, qword ptr [rbp + 0xf8]
140862926: 488985e0010000 mov qword ptr [rbp + 0x1e0], rax
14086292d: 0f1085d8000000 movups xmm0, xmmword ptr [rbp + 0xd8]
140862934: f30f6f8de8000000 movdqu xmm1, xmmword ptr [rbp + 0xe8]
14086293c: 660f7f8dd0010000 movdqa xmmword ptr [rbp + 0x1d0], xmm1
140862944: 0f2985c0010000 movaps xmmword ptr [rbp + 0x1c0], xmm0
14086294b: 488b95f0010000 mov rdx, qword ptr [rbp + 0x1f0]
140862952: 4c8b85f8010000 mov r8, qword ptr [rbp + 0x1f8]
140862959: f048ff02 lock inc qword ptr [rdx]
14086295d: 0f8e17060000 jle 0x140862f7a
140862963: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
14086296a: 4c8d0d7f382302 lea r9, [rip + 0x223387f]
140862971: e87ac1a700 call 0x1412deaf0 ; bevy_ecs::system::commands::EntityCommands::insert<country_core::components::ConvertedPreplacedTree>
140862976: 488b9dc8010000 mov rbx, qword ptr [rbp + 0x1c8]
14086297d: 4885db test rbx, rbx
140862980: 7507 jne 0x140862989
140862982: 488b9dd0010000 mov rbx, qword ptr [rbp + 0x1d0]
140862989: 4c8bb5e0010000 mov r14, qword ptr [rbp + 0x1e0]
140862990: 488b03 mov rax, qword ptr [rbx]
140862993: 4c8b6310 mov r12, qword ptr [rbx + 0x10]
140862997: 4c29e0 sub rax, r12
14086299a: 4883f82f cmp rax, 0x2f
14086299e: 0f86cd020000 jbe 0x140862c71
1408629a4: 488b4308 mov rax, qword ptr [rbx + 8]
1408629a8: 488d0d213ea700 lea rcx, [rip + 0xa73e21]
1408629af: 4a890c20 mov qword ptr [rax + r12], rcx
1408629b3: 488d0d7623d000 lea rcx, [rip + 0xd02376]
1408629ba: 4a894c2008 mov qword ptr [rax + r12 + 8], rcx
1408629bf: 4e89742010 mov qword ptr [rax + r12 + 0x10], r14
1408629c4: f2460f11542018 movsd qword ptr [rax + r12 + 0x18], xmm10
1408629cb: 42c74420200000803f mov dword ptr [rax + r12 + 0x20], 0x3f800000
1408629d4: 42c644202400 mov byte ptr [rax + r12 + 0x24], 0
1408629da: 42c644202800 mov byte ptr [rax + r12 + 0x28], 0
1408629e0: 4983c430 add r12, 0x30
1408629e4: 4c896310 mov qword ptr [rbx + 0x10], r12
1408629e8: 488b9dc8010000 mov rbx, qword ptr [rbp + 0x1c8]
1408629ef: 4885db test rbx, rbx
1408629f2: 7507 jne 0x1408629fb
1408629f4: 488b9dd0010000 mov rbx, qword ptr [rbp + 0x1d0]
1408629fb: f3450f106fd0 movss xmm13, dword ptr [r15 - 0x30]
140862a01: f3450f1077d8 movss xmm14, dword ptr [r15 - 0x28]
140862a07: 4c8bb5e0010000 mov r14, qword ptr [rbp + 0x1e0]
140862a0e: 488b03 mov rax, qword ptr [rbx]
140862a11: 4c8b6310 mov r12, qword ptr [rbx + 0x10]
140862a15: 4c29e0 sub rax, r12
140862a18: 4883f827 cmp rax, 0x27
140862a1c: 0f8674020000 jbe 0x140862c96
140862a22: 488b4308 mov rax, qword ptr [rbx + 8]
140862a26: 488d0d7387a700 lea rcx, [rip + 0xa78773]
140862a2d: 4a890c20 mov qword ptr [rax + r12], rcx
140862a31: 488d0df822d000 lea rcx, [rip + 0xd022f8]
140862a38: 4a894c2008 mov qword ptr [rax + r12 + 8], rcx
140862a3d: 4e89742010 mov qword ptr [rax + r12 + 0x10], r14
140862a42: f3460f116c2018 movss dword ptr [rax + r12 + 0x18], xmm13
140862a49: f3460f1174201c movss dword ptr [rax + r12 + 0x1c], xmm14
140862a50: 42c644202000 mov byte ptr [rax + r12 + 0x20], 0
140862a56: 4983c428 add r12, 0x28
140862a5a: 4c896310 mov qword ptr [rbx + 0x10], r12
140862a5e: f2450f106fd0 movsd xmm13, qword ptr [r15 - 0x30]
140862a64: 450f58eb addps xmm13, xmm11
140862a68: 488b9dc8010000 mov rbx, qword ptr [rbp + 0x1c8]
140862a6f: 4885db test rbx, rbx
140862a72: 7507 jne 0x140862a7b
140862a74: 488b9dd0010000 mov rbx, qword ptr [rbp + 0x1d0]
140862a7b: f3450f1077d8 movss xmm14, dword ptr [r15 - 0x28]
140862a81: f3450f58f4 addss xmm14, xmm12
140862a86: 4c8bb5e0010000 mov r14, qword ptr [rbp + 0x1e0]
140862a8d: 488b03 mov rax, qword ptr [rbx]
140862a90: 4c8b6310 mov r12, qword ptr [rbx + 0x10]
140862a94: 4c29e0 sub rax, r12
140862a97: 4883f827 cmp rax, 0x27
140862a9b: 0f861a020000 jbe 0x140862cbb
140862aa1: 498d47d0 lea rax, [r15 - 0x30]
140862aa5: 488b4b08 mov rcx, qword ptr [rbx + 8]
140862aa9: 488d15006ba700 lea rdx, [rip + 0xa76b00]
140862ab0: 4a891421 mov qword ptr [rcx + r12], rdx
140862ab4: 488d157522d000 lea rdx, [rip + 0xd02275]
140862abb: 4a89542108 mov qword ptr [rcx + r12 + 8], rdx
140862ac0: 4e89742110 mov qword ptr [rcx + r12 + 0x10], r14
140862ac5: 460f136c2118 movlps qword ptr [rcx + r12 + 0x18], xmm13
140862acb: f3460f11742120 movss dword ptr [rcx + r12 + 0x20], xmm14
140862ad2: 42c644212400 mov byte ptr [rcx + r12 + 0x24], 0
140862ad8: 4983c428 add r12, 0x28
140862adc: 4c896310 mov qword ptr [rbx + 0x10], r12
140862ae0: 488b9de0010000 mov rbx, qword ptr [rbp + 0x1e0]
140862ae7: 0f1000 movups xmm0, xmmword ptr [rax]
140862aea: 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
140862aee: 488b4010 mov rax, qword ptr [rax + 0x10]
140862af2: 488945e0 mov qword ptr [rbp - 0x20], rax
140862af6: 410f1047ec movups xmm0, xmmword ptr [r15 - 0x14]
140862afb: 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
140862aff: 418b47fc mov eax, dword ptr [r15 - 4]
140862b03: 894500 mov dword ptr [rbp], eax
140862b06: 4c8b85f0010000 mov r8, qword ptr [rbp + 0x1f0]
140862b0d: 488b85f8010000 mov rax, qword ptr [rbp + 0x1f8]
140862b14: f049ff00 lock inc qword ptr [r8]
140862b18: 0f8e5c040000 jle 0x140862f7a
140862b1e: 8b8da8010000 mov ecx, dword ptr [rbp + 0x1a8]
140862b24: 488b5528 mov rdx, qword ptr [rbp + 0x28]
140862b28: 890a mov dword ptr [rdx], ecx
140862b2a: 488b9500020000 mov rdx, qword ptr [rbp + 0x200]
140862b31: 488b4a40 mov rcx, qword ptr [rdx + 0x40]
140862b35: 48898d20010000 mov qword ptr [rbp + 0x120], rcx
140862b3c: 4c894548 mov qword ptr [rbp + 0x48], r8
140862b40: 4c8985d8000000 mov qword ptr [rbp + 0xd8], r8
140862b47: 488985e0000000 mov qword ptr [rbp + 0xe0], rax
140862b4e: 48899de8000000 mov qword ptr [rbp + 0xe8], rbx
140862b55: 488b45e0 mov rax, qword ptr [rbp - 0x20]
140862b59: 488d8df0000000 lea rcx, [rbp + 0xf0]
140862b60: 48894110 mov qword ptr [rcx + 0x10], rax
140862b64: 0f2845d0 movaps xmm0, xmmword ptr [rbp - 0x30]
140862b68: 0f1101 movups xmmword ptr [rcx], xmm0
140862b6b: 8b85b8010000 mov eax, dword ptr [rbp + 0x1b8]
140862b71: 898508010000 mov dword ptr [rbp + 0x108], eax
140862b77: 8b4500 mov eax, dword ptr [rbp]
140862b7a: 89412c mov dword ptr [rcx + 0x2c], eax
140862b7d: 0f2845f0 movaps xmm0, xmmword ptr [rbp - 0x10]
140862b81: 0f11411c movups xmmword ptr [rcx + 0x1c], xmm0
140862b85: 4c8b6230 mov r12, qword ptr [rdx + 0x30]
140862b89: 4c3b6220 cmp r12, qword ptr [rdx + 0x20]
140862b8d: 7510 jne 0x140862b9f
140862b8f: 488b4d40 mov rcx, qword ptr [rbp + 0x40]
140862b93: 488d15ae753d02 lea rdx, [rip + 0x23d75ae]
140862b9a: e8a109d0ff call 0x140563540 ; alloc::raw_vec::RawVec::grow_one<tiny_glade::screenshot::ScreenshotRequest,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<system_wall_constructor::construct_elevation_supports::find_overlap_segments::OverhangSupportSegment,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<country_core::resources::time_of_day::TodSequenceEntry,alloc::alloc::Global> | alloc::raw_vec::RawVec::grow_one<asset::texture::desc::TextureAssetDesc,alloc::alloc::Global>
140862b9f: 488b9500020000 mov rdx, qword ptr [rbp + 0x200]
140862ba6: 488b4228 mov rax, qword ptr [rdx + 0x28]
140862baa: 4b8d0ca4 lea rcx, [r12 + r12*4]
140862bae: 48c1e104 shl rcx, 4
140862bb2: 0f1085d8000000 movups xmm0, xmmword ptr [rbp + 0xd8]
140862bb9: f30f6f8de8000000 movdqu xmm1, xmmword ptr [rbp + 0xe8]
140862bc1: 0f1095f8000000 movups xmm2, xmmword ptr [rbp + 0xf8]
140862bc8: 0f109d08010000 movups xmm3, xmmword ptr [rbp + 0x108]
140862bcf: 0f10a518010000 movups xmm4, xmmword ptr [rbp + 0x118]
140862bd6: 0f11640840 movups xmmword ptr [rax + rcx + 0x40], xmm4
140862bdb: 0f115c0830 movups xmmword ptr [rax + rcx + 0x30], xmm3
140862be0: 0f11540820 movups xmmword ptr [rax + rcx + 0x20], xmm2
140862be5: f30f7f4c0810 movdqu xmmword ptr [rax + rcx + 0x10], xmm1
140862beb: 0f110408 movups xmmword ptr [rax + rcx], xmm0
140862bef: 49ffc4 inc r12
140862bf2: 4c896230 mov qword ptr [rdx + 0x30], r12
140862bf6: 48ff4240 inc qword ptr [rdx + 0x40]
140862bfa: 8b85ac010000 mov eax, dword ptr [rbp + 0x1ac]
140862c00: 488b4d30 mov rcx, qword ptr [rbp + 0x30]
140862c04: 8901 mov dword ptr [rcx], eax
140862c06: 488b8de8010000 mov rcx, qword ptr [rbp + 0x1e8]
140862c0d: 488b5538 mov rdx, qword ptr [rbp + 0x38]
140862c11: 4989d8 mov r8, rbx
140862c14: e8579d6800 call 0x140eec970 ; hashbrown::map::HashMap::insert<u64,ref$<system_clutter::autoclutter::AutoClutterObject>,std::hash::random::RandomState,alloc::alloc::Global> | hashbrown::map::HashMap::insert<country_core::resources::walls::WallId,bevy_ecs::entity::Entity,std::hash::random::RandomState,alloc::alloc::Global> | hashbrown::map::HashMap::insert<country_core::resources::walls::WallId,bevy_ecs::entity::Entity,std::hash::random::RandomState,alloc::alloc::Global> | hashbrown::map::HashMap::insert<country_core::components::TreeStorageIndex,country_core::resources::tree_storage::TreeStorageEntry,std::hash::random::RandomState,alloc::alloc::Global>
140862c19: 498d4d01 lea rcx, [r13 + 1]
140862c1d: 4c8b85e8010000 mov r8, qword ptr [rbp + 0x1e8]
140862c24: 498b5030 mov rdx, qword ptr [r8 + 0x30]
140862c28: 4981c5a2860100 add r13, 0x186a2
140862c2f: 4939d5 cmp r13, rdx
140862c32: 4c0f46ea cmovbe r13, rdx
140862c36: 4d896830 mov qword ptr [r8 + 0x30], r13
140862c3a: 48c1e820 shr rax, 0x20
140862c3e: 4989cd mov r13, rcx
140862c41: 0f8429fcffff je 0x140862870
140862c47: e9f6010000 jmp 0x140862e42
140862c4c: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
140862c55: 41b830000000 mov r8d, 0x30
140862c5b: 41b901000000 mov r9d, 1
140862c61: 4889d9 mov rcx, rbx
140862c64: 4c89e2 mov rdx, r12
140862c67: e8f46d0402 call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140862c6c: e96afcffff jmp 0x1408628db
140862c71: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
140862c7a: 41b830000000 mov r8d, 0x30
140862c80: 41b901000000 mov r9d, 1
140862c86: 4889d9 mov rcx, rbx
140862c89: 4c89e2 mov rdx, r12
140862c8c: e8cf6d0402 call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140862c91: e90efdffff jmp 0x1408629a4
140862c96: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
140862c9f: 41b828000000 mov r8d, 0x28
140862ca5: 41b901000000 mov r9d, 1
140862cab: 4889d9 mov rcx, rbx
140862cae: 4c89e2 mov rdx, r12
140862cb1: e8aa6d0402 call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140862cb6: e967fdffff jmp 0x140862a22
140862cbb: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
140862cc4: 41b828000000 mov r8d, 0x28
140862cca: 41b901000000 mov r9d, 1
140862cd0: 4889d9 mov rcx, rbx
140862cd3: 4c89e2 mov rdx, r12
140862cd6: e8856d0402 call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
140862cdb: e9c1fdffff jmp 0x140862aa1
140862ce0: 488b85f0010000 mov rax, qword ptr [rbp + 0x1f0]
140862ce7: f048ff08 lock dec qword ptr [rax]
140862ceb: 750c jne 0x140862cf9
140862ced: 488d8df0010000 lea rcx, [rbp + 0x1f0]
140862cf4: e837bedeff call 0x14064eb30 ; alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global>
140862cf9: 488b85c8000000 mov rax, qword ptr [rbp + 0xc8]
140862d00: 4885c0 test rax, rax
140862d03: 4c8d3d10c82802 lea r15, [rip + 0x228c810] ; @142aef51a 'assets/gladestatecrates/country-core/src/resources/session.rsloading_background.jpg'
140862d0a: 741a je 0x140862d26
140862d0c: 48c1e004 shl rax, 4
140862d10: 488d1440 lea rdx, [rax + rax*2]
140862d14: 41b804000000 mov r8d, 4
140862d1a: 488b8db0010000 mov rcx, qword ptr [rbp + 0x1b0]
140862d21: e8fa0edbff call 0x140613c20 ; 
140862d26: 488b95c0000000 mov rdx, qword ptr [rbp + 0xc0]
140862d2d: 4885d2 test rdx, rdx
140862d30: 7412 je 0x140862d44
140862d32: 41b801000000 mov r8d, 1
140862d38: 488b8dd0000000 mov rcx, qword ptr [rbp + 0xd0]
140862d3f: e8dc0edbff call 0x140613c20 ; 
140862d44: 488b55b0 mov rdx, qword ptr [rbp - 0x50]
140862d48: 4885d2 test rdx, rdx
140862d4b: 740f je 0x140862d5c
140862d4d: 488b4db8 mov rcx, qword ptr [rbp - 0x48]
140862d51: 41b801000000 mov r8d, 1
140862d57: e8c40edbff call 0x140613c20 ; 
140862d5c: 488b9558010000 mov rdx, qword ptr [rbp + 0x158]
140862d63: 4885d2 test rdx, rdx
140862d66: 7412 je 0x140862d7a
140862d68: 488b8d60010000 mov rcx, qword ptr [rbp + 0x160]
140862d6f: 41b801000000 mov r8d, 1
140862d75: e8a60edbff call 0x140613c20 ; 
140862d7a: 488b9570010000 mov rdx, qword ptr [rbp + 0x170]
140862d81: 4885d2 test rdx, rdx
140862d84: 7412 je 0x140862d98
140862d86: 488b8d78010000 mov rcx, qword ptr [rbp + 0x178]
140862d8d: 41b801000000 mov r8d, 1
140862d93: e8880edbff call 0x140613c20 ; 
140862d98: 488b9588010000 mov rdx, qword ptr [rbp + 0x188]
140862d9f: 4889d0 mov rax, rdx
140862da2: 48f7d8 neg rax
140862da5: 7014 jo 0x140862dbb
140862da7: 7312 jae 0x140862dbb
140862da9: 488b8d90010000 mov rcx, qword ptr [rbp + 0x190]
140862db0: 41b801000000 mov r8d, 1
140862db6: e8650edbff call 0x140613c20 ; 
140862dbb: 4c8bb5a8000000 mov r14, qword ptr [rbp + 0xa8]
140862dc2: 488bbdb8000000 mov rdi, qword ptr [rbp + 0xb8]
140862dc9: 4939fe cmp r14, rdi
140862dcc: 0f8532f8ffff jne 0x140862604
140862dd2: 4c89f3 mov rbx, r14
140862dd5: 48899da8000000 mov qword ptr [rbp + 0xa8], rbx
140862ddc: 488d8da0000000 lea rcx, [rbp + 0xa0]
140862de3: e848640100 call 0x140879230 ; core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global> >
140862de8: 0f28b510020000 movaps xmm6, xmmword ptr [rbp + 0x210]
140862def: 0f28bd20020000 movaps xmm7, xmmword ptr [rbp + 0x220]
140862df6: 440f288530020000 movaps xmm8, xmmword ptr [rbp + 0x230]
140862dfe: 440f288d40020000 movaps xmm9, xmmword ptr [rbp + 0x240]
140862e06: 440f289550020000 movaps xmm10, xmmword ptr [rbp + 0x250]
140862e0e: 440f289d60020000 movaps xmm11, xmmword ptr [rbp + 0x260]
140862e16: 440f28a570020000 movaps xmm12, xmmword ptr [rbp + 0x270]
140862e1e: 440f28ad80020000 movaps xmm13, xmmword ptr [rbp + 0x280]
140862e26: 440f28b590020000 movaps xmm14, xmmword ptr [rbp + 0x290]
140862e2e: 4881c428030000 add rsp, 0x328
140862e35: 5b pop rbx
140862e36: 5f pop rdi
140862e37: 5e pop rsi
140862e38: 415c pop r12
140862e3a: 415d pop r13
140862e3c: 415e pop r14
140862e3e: 415f pop r15
140862e40: 5d pop rbp
140862e41: c3 ret 
140862e42: 488d0dbf332302 lea rcx, [rip + 0x22333bf]
140862e49: 4c8d05d8332302 lea r8, [rip + 0x22333d8]
140862e50: ba20000000 mov edx, 0x20
140862e55: e8d6650702 call 0x1428d9430 ; core::panicking::panic
140862e5a: e91b010000 jmp 0x140862f7a
140862e5f: 488b4558 mov rax, qword ptr [rbp + 0x58]
140862e63: 48898558010000 mov qword ptr [rbp + 0x158], rax
140862e6a: 488d051f332302 lea rax, [rip + 0x223331f]
140862e71: 4889442420 mov qword ptr [rsp + 0x20], rax
140862e76: 488d0d07332302 lea rcx, [rip + 0x2233307] ; @142a96184 'trees.json'
140862e7d: 4c8d0d1c302302 lea r9, [rip + 0x223301c]
140862e84: 4c8d8558010000 lea r8, [rbp + 0x158]
140862e8b: ba0a000000 mov edx, 0xa
140862e90: e8cb680702 call 0x1428d9760 ; core::result::unwrap_failed
140862e95: e9e0000000 jmp 0x140862f7a
140862e9a: 488b8560010000 mov rax, qword ptr [rbp + 0x160]
140862ea1: 48894550 mov qword ptr [rbp + 0x50], rax
140862ea5: 488d0514332302 lea rax, [rip + 0x2233314]
140862eac: 4889442420 mov qword ptr [rsp + 0x20], rax
140862eb1: 488d0df0322302 lea rcx, [rip + 0x22332f0] ; @142a961a8 'deserialize trees'
140862eb8: 4c8d0d01302302 lea r9, [rip + 0x2233001]
140862ebf: 4c8d4550 lea r8, [rbp + 0x50]
140862ec3: ba11000000 mov edx, 0x11
140862ec8: e893680702 call 0x1428d9760 ; core::result::unwrap_failed
140862ecd: e9a8000000 jmp 0x140862f7a
140862ed2: 488b85e0000000 mov rax, qword ptr [rbp + 0xe0]
140862ed9: 488985c0010000 mov qword ptr [rbp + 0x1c0], rax
140862ee0: 488d05f1322302 lea rax, [rip + 0x22332f1]
140862ee7: 4889442420 mov qword ptr [rsp + 0x20], rax
140862eec: 488d0d0d302302 lea rcx, [rip + 0x223300d] ; @142a95f00 'called `Result::unwrap()` on an `Err` valuecountry_core::systems::brush_preview::brush_preview::'
140862ef3: 4c8d0dc62f2302 lea r9, [rip + 0x2232fc6]
140862efa: 4c8d85c0010000 lea r8, [rbp + 0x1c0]
140862f01: ba2b000000 mov edx, 0x2b
140862f06: e855680702 call 0x1428d9760 ; core::result::unwrap_failed
140862f0b: eb6d jmp 0x140862f7a
140862f0d: 48895508 mov qword ptr [rbp + 8], rdx
140862f11: 488d4510 lea rax, [rbp + 0x10]
140862f15: 488985f0010000 mov qword ptr [rbp + 0x1f0], rax
140862f1c: 488d05cdb97cff lea rax, [rip - 0x834633]
140862f23: 488985f8010000 mov qword ptr [rbp + 0x1f8], rax
140862f2a: 488d051f332302 lea rax, [rip + 0x223331f]
140862f31: 488985d8000000 mov qword ptr [rbp + 0xd8], rax
140862f38: 48c785e000000001000000 mov qword ptr [rbp + 0xe0], 1
140862f43: 48c785f800000000000000 mov qword ptr [rbp + 0xf8], 0
140862f4e: 488d85f0010000 lea rax, [rbp + 0x1f0]
140862f55: 488985e8000000 mov qword ptr [rbp + 0xe8], rax
140862f5c: 48c785f000000001000000 mov qword ptr [rbp + 0xf0], 1
140862f67: 488d15f2322302 lea rdx, [rip + 0x22332f2]
140862f6e: 488d8dd8000000 lea rcx, [rbp + 0xd8]
140862f75: e816640702 call 0x1428d9390 ; core::panicking::panic_fmt
140862f7a: 0f0b ud2 
140862f7c: 0f1f4000 nop dword ptr [rax]
140862f80: 4889542410 mov qword ptr [rsp + 0x10], rdx
140862f85: 55 push rbp
140862f86: 4157 push r15
140862f88: 4156 push r14
140862f8a: 4155 push r13
140862f8c: 4154 push r12
140862f8e: 56 push rsi
140862f8f: 57 push rdi
140862f90: 53 push rbx
140862f91: 4881ecb8000000 sub rsp, 0xb8
140862f98: 488daa80000000 lea rbp, [rdx + 0x80]
140862f9f: 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
140862fa5: 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
140862fab: 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
140862fb1: 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
140862fb7: 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
140862fbd: 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
140862fc3: 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
140862fcc: 660f7fbc2490000000 movdqa xmmword ptr [rsp + 0x90], xmm7
140862fd5: 660f7fb424a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm6
140862fde: 488b95d8000000 mov rdx, qword ptr [rbp + 0xd8]
140862fe5: 4885d2 test rdx, rdx
140862fe8: 7412 je 0x140862ffc
140862fea: 488b8de0000000 mov rcx, qword ptr [rbp + 0xe0]
140862ff1: 41b801000000 mov r8d, 1
140862ff7: e8240cdbff call 0x140613c20 ; 
140862ffc: 0f28b424a0000000 movaps xmm6, xmmword ptr [rsp + 0xa0]
140863004: 0f28bc2490000000 movaps xmm7, xmmword ptr [rsp + 0x90]
14086300c: 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
140863015: 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
14086301b: 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
140863021: 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
140863027: 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
14086302d: 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
140863033: 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
140863039: 4881c4b8000000 add rsp, 0xb8
140863040: 5b pop rbx
140863041: 5f pop rdi
140863042: 5e pop rsi
140863043: 415c pop r12
140863045: 415d pop r13
140863047: 415e pop r14
140863049: 415f pop r15
14086304b: 5d pop rbp
14086304c: c3 ret 
14086304d: 0f1f00 nop dword ptr [rax]
140863050: 4889542410 mov qword ptr [rsp + 0x10], rdx
140863055: 55 push rbp
140863056: 4157 push r15
140863058: 4156 push r14
14086305a: 4155 push r13
14086305c: 4154 push r12
14086305e: 56 push rsi
14086305f: 57 push rdi
140863060: 53 push rbx
140863061: 4881ecb8000000 sub rsp, 0xb8
140863068: 488daa80000000 lea rbp, [rdx + 0x80]
14086306f: 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
140863075: 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
14086307b: 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
140863081: 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
140863087: 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
14086308d: 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
140863093: 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
14086309c: 660f7fbc2490000000 movdqa xmmword ptr [rsp + 0x90], xmm7
1408630a5: 660f7fb424a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm6
1408630ae: 488b9558010000 mov rdx, qword ptr [rbp + 0x158]
1408630b5: 4885d2 test rdx, rdx
1408630b8: 7412 je 0x1408630cc
1408630ba: 488b8d60010000 mov rcx, qword ptr [rbp + 0x160]
1408630c1: 41b801000000 mov r8d, 1
1408630c7: e8540bdbff call 0x140613c20 ; 
1408630cc: 660f6fb424a0000000 movdqa xmm6, xmmword ptr [rsp + 0xa0]
1408630d5: 660f6fbc2490000000 movdqa xmm7, xmmword ptr [rsp + 0x90]
1408630de: 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
1408630e7: 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
1408630ed: 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
1408630f3: 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
1408630f9: 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
1408630ff: 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
140863105: 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
14086310b: 4881c4b8000000 add rsp, 0xb8
140863112: 5b pop rbx
140863113: 5f pop rdi
140863114: 5e pop rsi
140863115: 415c pop r12
140863117: 415d pop r13
140863119: 415e pop r14
14086311b: 415f pop r15
14086311d: 5d pop rbp
14086311e: c3 ret 
14086311f: 90 nop 
140863120: 4889542410 mov qword ptr [rsp + 0x10], rdx
140863125: 55 push rbp
140863126: 4157 push r15
140863128: 4156 push r14
14086312a: 4155 push r13
14086312c: 4154 push r12
14086312e: 56 push rsi
14086312f: 57 push rdi
140863130: 53 push rbx
140863131: 4881ecb8000000 sub rsp, 0xb8
140863138: 488daa80000000 lea rbp, [rdx + 0x80]
14086313f: 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
140863145: 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
14086314b: 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
140863151: 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
140863157: 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
14086315d: 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
140863163: 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
14086316c: 660f7fbc2490000000 movdqa xmmword ptr [rsp + 0x90], xmm7
140863175: 660f7fb424a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm6
14086317e: 488d8d58010000 lea rcx, [rbp + 0x158]
140863185: e87648d8ff call 0x1405e7a00 ; core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::option::Option<std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::option::Option<std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error>
14086318a: 660f6fb424a0000000 movdqa xmm6, xmmword ptr [rsp + 0xa0]
140863193: 660f6fbc2490000000 movdqa xmm7, xmmword ptr [rsp + 0x90]
14086319c: 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
1408631a5: 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
1408631ab: 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
1408631b1: 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
1408631b7: 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
1408631bd: 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
1408631c3: 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
1408631c9: 4881c4b8000000 add rsp, 0xb8
1408631d0: 5b pop rbx
1408631d1: 5f pop rdi
1408631d2: 5e pop rsi
1408631d3: 415c pop r12
1408631d5: 415d pop r13
1408631d7: 415e pop r14
1408631d9: 415f pop r15
1408631db: 5d pop rbp
1408631dc: c3 ret 
1408631dd: 0f1f00 nop dword ptr [rax]
1408631e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1408631e5: 55 push rbp
1408631e6: 4157 push r15
1408631e8: 4156 push r14
1408631ea: 4155 push r13
1408631ec: 4154 push r12
1408631ee: 56 push rsi
1408631ef: 57 push rdi
1408631f0: 53 push rbx
1408631f1: 4881ecb8000000 sub rsp, 0xb8
1408631f8: 488daa80000000 lea rbp, [rdx + 0x80]
1408631ff: 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
140863205: 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
14086320b: 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
140863211: 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
140863217: 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
14086321d: 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
140863223: 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
14086322c: 660f7fbc2490000000 movdqa xmmword ptr [rsp + 0x90], xmm7
140863235: 660f7fb424a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm6
14086323e: 4883bd0002000000 cmp qword ptr [rbp + 0x200], 0
140863246: 7419 je 0x140863261
140863248: 41b801000000 mov r8d, 1
14086324e: 488b8de8010000 mov rcx, qword ptr [rbp + 0x1e8]
140863255: 488b9500020000 mov rdx, qword ptr [rbp + 0x200]
14086325c: e8bf09dbff call 0x140613c20 ; 
140863261: 660f6fb424a0000000 movdqa xmm6, xmmword ptr [rsp + 0xa0]
14086326a: 660f6fbc2490000000 movdqa xmm7, xmmword ptr [rsp + 0x90]
140863273: 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
14086327c: 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
140863282: 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
140863288: 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
14086328e: 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
140863294: 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
14086329a: 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
1408632a0: 4881c4b8000000 add rsp, 0xb8
1408632a7: 5b pop rbx
1408632a8: 5f pop rdi
1408632a9: 5e pop rsi
1408632aa: 415c pop r12
1408632ac: 415d pop r13
1408632ae: 415e pop r14
1408632b0: 415f pop r15
1408632b2: 5d pop rbp
1408632b3: c3 ret 
1408632b4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1408632c0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1408632c5: 55 push rbp
1408632c6: 4157 push r15
1408632c8: 4156 push r14
1408632ca: 4155 push r13
1408632cc: 4154 push r12
1408632ce: 56 push rsi
1408632cf: 57 push rdi
1408632d0: 53 push rbx
1408632d1: 4881ecb8000000 sub rsp, 0xb8
1408632d8: 488daa80000000 lea rbp, [rdx + 0x80]
1408632df: 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
1408632e5: 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
1408632eb: 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
1408632f1: 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
1408632f7: 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
1408632fd: 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
140863303: 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
14086330c: 660f7fbc2490000000 movdqa xmmword ptr [rsp + 0x90], xmm7
140863315: 660f7fb424a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm6
14086331e: 488b55b0 mov rdx, qword ptr [rbp - 0x50]
140863322: 4885d2 test rdx, rdx
140863325: 740f je 0x140863336
140863327: 488b4db8 mov rcx, qword ptr [rbp - 0x48]
14086332b: 41b801000000 mov r8d, 1
140863331: e8ea08dbff call 0x140613c20 ; 
140863336: 660f6fb424a0000000 movdqa xmm6, xmmword ptr [rsp + 0xa0]
14086333f: 660f6fbc2490000000 movdqa xmm7, xmmword ptr [rsp + 0x90]
140863348: 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
140863351: 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
140863357: 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
14086335d: 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
140863363: 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
140863369: 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
14086336f: 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
140863375: 4881c4b8000000 add rsp, 0xb8
14086337c: 5b pop rbx
14086337d: 5f pop rdi
14086337e: 5e pop rsi
14086337f: 415c pop r12
140863381: 415d pop r13
140863383: 415e pop r14
140863385: 415f pop r15
140863387: 5d pop rbp
140863388: c3 ret 
140863389: 0f1f8000000000 nop dword ptr [rax]
140863390: 4889542410 mov qword ptr [rsp + 0x10], rdx
140863395: 55 push rbp
140863396: 4157 push r15
140863398: 4156 push r14
14086339a: 4155 push r13
14086339c: 4154 push r12
14086339e: 56 push rsi
14086339f: 57 push rdi
1408633a0: 53 push rbx
1408633a1: 4881ecb8000000 sub rsp, 0xb8
1408633a8: 488daa80000000 lea rbp, [rdx + 0x80]
1408633af: 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
1408633b5: 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
1408633bb: 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
1408633c1: 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
1408633c7: 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
1408633cd: 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
1408633d3: 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
1408633dc: 660f7fbc2490000000 movdqa xmmword ptr [rsp + 0x90], xmm7
1408633e5: 660f7fb424a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm6
1408633ee: 488d4d08 lea rcx, [rbp + 8]
1408633f2: e80946d8ff call 0x1405e7a00 ; core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::option::Option<std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::option::Option<std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<enum2$<core::result::Result<tuple$<>,std::io::error::Error> > > | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error> | core::ptr::drop_in_place<std::io::error::Error>
1408633f7: 660f6fb424a0000000 movdqa xmm6, xmmword ptr [rsp + 0xa0]
140863400: 660f6fbc2490000000 movdqa xmm7, xmmword ptr [rsp + 0x90]
140863409: 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
140863412: 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
140863418: 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
14086341e: 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
140863424: 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
14086342a: 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
140863430: 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
140863436: 4881c4b8000000 add rsp, 0xb8
14086343d: 5b pop rbx
14086343e: 5f pop rdi
14086343f: 5e pop rsi
140863440: 415c pop r12
140863442: 415d pop r13
140863444: 415e pop r14
140863446: 415f pop r15
140863448: 5d pop rbp
140863449: c3 ret 
14086344a: 660f1f440000 nop word ptr [rax + rax]
140863450: 4889542410 mov qword ptr [rsp + 0x10], rdx
140863455: 55 push rbp
140863456: 4157 push r15
140863458: 4156 push r14
14086345a: 4155 push r13
14086345c: 4154 push r12
14086345e: 56 push rsi
14086345f: 57 push rdi
140863460: 53 push rbx
140863461: 4881ecb8000000 sub rsp, 0xb8
140863468: 488daa80000000 lea rbp, [rdx + 0x80]
14086346f: 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
140863475: 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
14086347b: 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
140863481: 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
140863487: 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
14086348d: 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
140863493: 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
14086349c: 660f7fbc2490000000 movdqa xmmword ptr [rsp + 0x90], xmm7
1408634a5: 660f7fb424a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm6
1408634ae: 488b95c0000000 mov rdx, qword ptr [rbp + 0xc0]
1408634b5: 4885d2 test rdx, rdx
1408634b8: 7412 je 0x1408634cc
1408634ba: 41b801000000 mov r8d, 1
1408634c0: 488b8dd0000000 mov rcx, qword ptr [rbp + 0xd0]
1408634c7: e85407dbff call 0x140613c20 ; 
1408634cc: 660f6fb424a0000000 movdqa xmm6, xmmword ptr [rsp + 0xa0]
1408634d5: 660f6fbc2490000000 movdqa xmm7, xmmword ptr [rsp + 0x90]
1408634de: 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
1408634e7: 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
1408634ed: 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
1408634f3: 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
1408634f9: 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
1408634ff: 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
140863505: 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
14086350b: 4881c4b8000000 add rsp, 0xb8
140863512: 5b pop rbx
140863513: 5f pop rdi
140863514: 5e pop rsi
140863515: 415c pop r12
140863517: 415d pop r13
140863519: 415e pop r14
14086351b: 415f pop r15
14086351d: 5d pop rbp
14086351e: c3 ret 
14086351f: 90 nop 
140863520: 4889542410 mov qword ptr [rsp + 0x10], rdx
140863525: 55 push rbp
140863526: 4157 push r15
140863528: 4156 push r14
14086352a: 4155 push r13
14086352c: 4154 push r12
14086352e: 56 push rsi
14086352f: 57 push rdi
140863530: 53 push rbx
140863531: 4881ecb8000000 sub rsp, 0xb8
140863538: 488daa80000000 lea rbp, [rdx + 0x80]
14086353f: 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
140863545: 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
14086354b: 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
140863551: 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
140863557: 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
14086355d: 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
140863563: 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
14086356c: 660f7fbc2490000000 movdqa xmmword ptr [rsp + 0x90], xmm7
140863575: 660f7fb424a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm6
14086357e: 488b85c8000000 mov rax, qword ptr [rbp + 0xc8]
140863585: 4885c0 test rax, rax
140863588: 741a je 0x1408635a4
14086358a: 48c1e004 shl rax, 4
14086358e: 488d1440 lea rdx, [rax + rax*2]
140863592: 41b804000000 mov r8d, 4
140863598: 488b8db0010000 mov rcx, qword ptr [rbp + 0x1b0]
14086359f: e87c06dbff call 0x140613c20 ; 
1408635a4: 660f6fb424a0000000 movdqa xmm6, xmmword ptr [rsp + 0xa0]
1408635ad: 660f6fbc2490000000 movdqa xmm7, xmmword ptr [rsp + 0x90]
1408635b6: 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
1408635bf: 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
1408635c5: 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
1408635cb: 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
1408635d1: 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
1408635d7: 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
1408635dd: 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
1408635e3: 4881c4b8000000 add rsp, 0xb8
1408635ea: 5b pop rbx
1408635eb: 5f pop rdi
1408635ec: 5e pop rsi
1408635ed: 415c pop r12
1408635ef: 415d pop r13
1408635f1: 415e pop r14
1408635f3: 415f pop r15
1408635f5: 5d pop rbp
1408635f6: c3 ret 
1408635f7: 660f1f840000000000 nop word ptr [rax + rax]
140863600: 4889542410 mov qword ptr [rsp + 0x10], rdx
140863605: 55 push rbp
140863606: 4157 push r15
140863608: 4156 push r14
14086360a: 4155 push r13
14086360c: 4154 push r12
14086360e: 56 push rsi
14086360f: 57 push rdi
140863610: 53 push rbx
140863611: 4881ecb8000000 sub rsp, 0xb8
140863618: 488daa80000000 lea rbp, [rdx + 0x80]
14086361f: 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
140863625: 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
14086362b: 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
140863631: 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
140863637: 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
14086363d: 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
140863643: 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
14086364c: 660f7fbc2490000000 movdqa xmmword ptr [rsp + 0x90], xmm7
140863655: 660f7fb424a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm6
14086365e: 488b85f0010000 mov rax, qword ptr [rbp + 0x1f0]
140863665: f048ff08 lock dec qword ptr [rax]
140863669: 750c jne 0x140863677
14086366b: 488d8df0010000 lea rcx, [rbp + 0x1f0]
140863672: e8b9b4deff call 0x14064eb30 ; alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global>
140863677: 660f6fb424a0000000 movdqa xmm6, xmmword ptr [rsp + 0xa0]
140863680: 660f6fbc2490000000 movdqa xmm7, xmmword ptr [rsp + 0x90]
140863689: 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
140863692: 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
140863698: 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
14086369e: 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
1408636a4: 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
1408636aa: 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
1408636b0: 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
1408636b6: 4881c4b8000000 add rsp, 0xb8
1408636bd: 5b pop rbx
1408636be: 5f pop rdi
1408636bf: 5e pop rsi
1408636c0: 415c pop r12
1408636c2: 415d pop r13
1408636c4: 415e pop r14
1408636c6: 415f pop r15
1408636c8: 5d pop rbp
1408636c9: c3 ret 
1408636ca: 660f1f440000 nop word ptr [rax + rax]
1408636d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1408636d5: 55 push rbp
1408636d6: 4157 push r15
1408636d8: 4156 push r14
1408636da: 4155 push r13
1408636dc: 4154 push r12
1408636de: 56 push rsi
1408636df: 57 push rdi
1408636e0: 53 push rbx
1408636e1: 4881ecb8000000 sub rsp, 0xb8
1408636e8: 488daa80000000 lea rbp, [rdx + 0x80]
1408636ef: 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
1408636f5: 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
1408636fb: 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
140863701: 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
140863707: 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
14086370d: 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
140863713: 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
14086371c: 660f7fbc2490000000 movdqa xmmword ptr [rsp + 0x90], xmm7
140863725: 660f7fb424a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm6
14086372e: 488b4548 mov rax, qword ptr [rbp + 0x48]
140863732: f048ff08 lock dec qword ptr [rax]
140863736: 750c jne 0x140863744
140863738: 488d8dd8000000 lea rcx, [rbp + 0xd8]
14086373f: e8ecb3deff call 0x14064eb30 ; alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<slice2$<u8>,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global> | alloc::sync::Arc::drop_slow<str$,alloc::alloc::Global>
140863744: 660f6fb424a0000000 movdqa xmm6, xmmword ptr [rsp + 0xa0]
14086374d: 660f6fbc2490000000 movdqa xmm7, xmmword ptr [rsp + 0x90]
140863756: 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
14086375f: 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
140863765: 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
14086376b: 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
140863771: 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
140863777: 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
14086377d: 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
140863783: 4881c4b8000000 add rsp, 0xb8
14086378a: 5b pop rbx
14086378b: 5f pop rdi
14086378c: 5e pop rsi
14086378d: 415c pop r12
14086378f: 415d pop r13
140863791: 415e pop r14
140863793: 415f pop r15
140863795: 5d pop rbp
140863796: c3 ret 
140863797: 660f1f840000000000 nop word ptr [rax + rax]
1408637a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1408637a5: 55 push rbp
1408637a6: 4157 push r15
1408637a8: 4156 push r14
1408637aa: 4155 push r13
1408637ac: 4154 push r12
1408637ae: 56 push rsi
1408637af: 57 push rdi
1408637b0: 53 push rbx
1408637b1: 4881ecb8000000 sub rsp, 0xb8
1408637b8: 488daa80000000 lea rbp, [rdx + 0x80]
1408637bf: 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
1408637c5: 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
1408637cb: 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
1408637d1: 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
1408637d7: 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
1408637dd: 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
1408637e3: 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
1408637ec: 660f7fbc2490000000 movdqa xmmword ptr [rsp + 0x90], xmm7
1408637f5: 660f7fb424a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm6
1408637fe: 488d4d50 lea rcx, [rbp + 0x50]
140863802: e89950d8ff call 0x1405e88a0 ; core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error>
140863807: 660f6fb424a0000000 movdqa xmm6, xmmword ptr [rsp + 0xa0]
140863810: 660f6fbc2490000000 movdqa xmm7, xmmword ptr [rsp + 0x90]
140863819: 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
140863822: 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
140863828: 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
14086382e: 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
140863834: 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
14086383a: 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
140863840: 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
140863846: 4881c4b8000000 add rsp, 0xb8
14086384d: 5b pop rbx
14086384e: 5f pop rdi
14086384f: 5e pop rsi
140863850: 415c pop r12
140863852: 415d pop r13
140863854: 415e pop r14
140863856: 415f pop r15
140863858: 5d pop rbp
140863859: c3 ret 
14086385a: 660f1f440000 nop word ptr [rax + rax]
140863860: 4889542410 mov qword ptr [rsp + 0x10], rdx
140863865: 55 push rbp
140863866: 4157 push r15
140863868: 4156 push r14
14086386a: 4155 push r13
14086386c: 4154 push r12
14086386e: 56 push rsi
14086386f: 57 push rdi
140863870: 53 push rbx
140863871: 4881ecb8000000 sub rsp, 0xb8
140863878: 488daa80000000 lea rbp, [rdx + 0x80]
14086387f: 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
140863885: 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
14086388b: 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
140863891: 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
140863897: 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
14086389d: 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
1408638a3: 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
1408638ac: 660f7fbc2490000000 movdqa xmmword ptr [rsp + 0x90], xmm7
1408638b5: 660f7fb424a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm6
1408638be: 488d8d58010000 lea rcx, [rbp + 0x158]
1408638c5: e8a6fcdfff call 0x140663570 ; core::ptr::drop_in_place<sentry_types::protocol::attachment::Attachment> | core::ptr::drop_in_place<country_core::startup::clearing::PlacementConfigEntry>
1408638ca: 488d8da0000000 lea rcx, [rbp + 0xa0]
1408638d1: e85a590100 call 0x140879230 ; core::ptr::drop_in_place<alloc::vec::into_iter::IntoIter<country_core::startup::clearing::PlacementConfigEntry,alloc::alloc::Global> >
1408638d6: 0f28b424a0000000 movaps xmm6, xmmword ptr [rsp + 0xa0]
1408638de: 0f28bc2490000000 movaps xmm7, xmmword ptr [rsp + 0x90]
1408638e6: 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
1408638ef: 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
1408638f5: 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
1408638fb: 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
140863901: 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
140863907: 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
14086390d: 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
140863913: 4881c4b8000000 add rsp, 0xb8
14086391a: 5b pop rbx
14086391b: 5f pop rdi
14086391c: 5e pop rsi
14086391d: 415c pop r12
14086391f: 415d pop r13
140863921: 415e pop r14
140863923: 415f pop r15
140863925: 5d pop rbp
140863926: c3 ret 
140863927: 660f1f840000000000 nop word ptr [rax + rax]
140863930: 4889542410 mov qword ptr [rsp + 0x10], rdx
140863935: 55 push rbp
140863936: 4157 push r15
140863938: 4156 push r14
14086393a: 4155 push r13
14086393c: 4154 push r12
14086393e: 56 push rsi
14086393f: 57 push rdi
140863940: 53 push rbx
140863941: 4881ecb8000000 sub rsp, 0xb8
140863948: 488daa80000000 lea rbp, [rdx + 0x80]
14086394f: 440f29742420 movaps xmmword ptr [rsp + 0x20], xmm14
140863955: 440f296c2430 movaps xmmword ptr [rsp + 0x30], xmm13
14086395b: 440f29642440 movaps xmmword ptr [rsp + 0x40], xmm12
140863961: 440f295c2450 movaps xmmword ptr [rsp + 0x50], xmm11
140863967: 440f29542460 movaps xmmword ptr [rsp + 0x60], xmm10
14086396d: 440f294c2470 movaps xmmword ptr [rsp + 0x70], xmm9
140863973: 440f29842480000000 movaps xmmword ptr [rsp + 0x80], xmm8
14086397c: 660f7fbc2490000000 movdqa xmmword ptr [rsp + 0x90], xmm7
140863985: 660f7fb424a0000000 movdqa xmmword ptr [rsp + 0xa0], xmm6
14086398e: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
140863995: e8064fd8ff call 0x1405e88a0 ; core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error> | core::ptr::drop_in_place<serde_json::error::Error>
14086399a: 660f6fb424a0000000 movdqa xmm6, xmmword ptr [rsp + 0xa0]
1408639a3: 660f6fbc2490000000 movdqa xmm7, xmmword ptr [rsp + 0x90]
1408639ac: 440f28842480000000 movaps xmm8, xmmword ptr [rsp + 0x80]
1408639b5: 440f284c2470 movaps xmm9, xmmword ptr [rsp + 0x70]
1408639bb: 440f28542460 movaps xmm10, xmmword ptr [rsp + 0x60]
1408639c1: 440f285c2450 movaps xmm11, xmmword ptr [rsp + 0x50]
1408639c7: 440f28642440 movaps xmm12, xmmword ptr [rsp + 0x40]
1408639cd: 440f286c2430 movaps xmm13, xmmword ptr [rsp + 0x30]
1408639d3: 440f28742420 movaps xmm14, xmmword ptr [rsp + 0x20]
1408639d9: 4881c4b8000000 add rsp, 0xb8
1408639e0: 5b pop rbx
1408639e1: 5f pop rdi
1408639e2: 5e pop rsi
1408639e3: 415c pop r12
1408639e5: 415d pop r13
1408639e7: 415e pop r14
1408639e9: 415f pop r15
1408639eb: 5d pop rbp
1408639ec: c3 ret 
