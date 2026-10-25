140342c10: 55 push rbp
140342c11: 4157 push r15
140342c13: 4156 push r14
140342c15: 4154 push r12
140342c17: 56 push rsi
140342c18: 57 push rdi
140342c19: 53 push rbx
140342c1a: 4881ec40070000 sub rsp, 0x740
140342c21: 488dac2480000000 lea rbp, [rsp + 0x80]
140342c29: 440f29a5b0060000 movaps xmmword ptr [rbp + 0x6b0], xmm12
140342c31: 440f299da0060000 movaps xmmword ptr [rbp + 0x6a0], xmm11
140342c39: 440f299590060000 movaps xmmword ptr [rbp + 0x690], xmm10
140342c41: 440f298d80060000 movaps xmmword ptr [rbp + 0x680], xmm9
140342c49: 440f298570060000 movaps xmmword ptr [rbp + 0x670], xmm8
140342c51: 0f29bd60060000 movaps xmmword ptr [rbp + 0x660], xmm7
140342c58: 0f29b550060000 movaps xmmword ptr [rbp + 0x650], xmm6
140342c5f: 48c78548060000feffffff mov qword ptr [rbp + 0x648], 0xfffffffffffffffe
140342c6a: 4889d6 mov rsi, rdx
140342c6d: 488d7a38 lea rdi, [rdx + 0x38]
140342c71: 48ba529634959e263d94 movabs rdx, 0x943d269e95349652
140342c7b: 49b891d7c2897e6be230 movabs r8, 0x30e26b7e89c2d791
140342c85: 4889f9 mov rcx, rdi
140342c88: e863aad9ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::compon
140342c8d: 4885c0 test rax, rax
140342c90: 7433 je 0x140342cc5
140342c92: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140342c99: 762a jbe 0x140342cc5
140342c9b: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140342ca2: 488b04d0 mov rax, qword ptr [rax + rdx*8]
140342ca6: 4885c0 test rax, rax
140342ca9: 741a je 0x140342cc5
140342cab: 488b8ee0010000 mov rcx, qword ptr [rsi + 0x1e0]
140342cb2: 48f7d0 not rax
140342cb5: 488d0440 lea rax, [rax + rax*2]
140342cb9: 48c1e005 shl rax, 5
140342cbd: 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
140342cc3: 7508 jne 0x140342ccd
140342cc5: 4889f1 mov rcx, rsi
140342cc8: e8831e2400 call 0x140584b50 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::resources::walls::decorator::DecoratorOpEvent>
140342ccd: 48baddee2a55401b8775 movabs rdx, 0x75871b40552aeedd
140342cd7: 49b8727d1fc5d3ff54ea movabs r8, 0xea54ffd3c51f7d72
140342ce1: 4889f9 mov rcx, rdi
140342ce4: e807aad9ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::compon
140342ce9: 4885c0 test rax, rax
140342cec: 7433 je 0x140342d21
140342cee: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140342cf5: 762a jbe 0x140342d21
140342cf7: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140342cfe: 488b04d0 mov rax, qword ptr [rax + rdx*8]
140342d02: 4885c0 test rax, rax
140342d05: 741a je 0x140342d21
140342d07: 488b8ee0010000 mov rcx, qword ptr [rsi + 0x1e0]
140342d0e: 48f7d0 not rax
140342d11: 488d0440 lea rax, [rax + rax*2]
140342d15: 48c1e005 shl rax, 5
140342d19: 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
140342d1f: 7508 jne 0x140342d29
140342d21: 4889f1 mov rcx, rsi
140342d24: e807962300 call 0x14057c330 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::systems::wall::eraser::ui_bulldozer::DeleteDecoratorCmd>
140342d29: 48ba48672009bd63bbd1 movabs rdx, 0xd1bb63bd09206748
140342d33: 49b85c33d69e21c9cb75 movabs r8, 0x75cbc9219ed6335c
140342d3d: 4889f9 mov rcx, rdi
140342d40: e8aba9d9ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::compon
140342d45: 4885c0 test rax, rax
140342d48: 7433 je 0x140342d7d
140342d4a: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140342d51: 762a jbe 0x140342d7d
140342d53: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140342d5a: 488b04d0 mov rax, qword ptr [rax + rdx*8]
140342d5e: 4885c0 test rax, rax
140342d61: 741a je 0x140342d7d
140342d63: 488b8ee0010000 mov rcx, qword ptr [rsi + 0x1e0]
140342d6a: 48f7d0 not rax
140342d6d: 488d0440 lea rax, [rax + rax*2]
140342d71: 48c1e005 shl rax, 5
140342d75: 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
140342d7b: 7508 jne 0x140342d85
140342d7d: 4889f1 mov rcx, rsi
140342d80: e85bcd2300 call 0x14057fae0 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::systems::wall::eraser::ui_bulldozer::ChangeDecoratorTypeCmd>
140342d85: 48baca088a191bf92381 movabs rdx, 0x8123f91b198a08ca
140342d8f: 49b8e44a712f533199e4 movabs r8, 0xe49931532f714ae4
140342d99: 4889f9 mov rcx, rdi
140342d9c: e84fa9d9ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::compon
140342da1: 4885c0 test rax, rax
140342da4: 7433 je 0x140342dd9
140342da6: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140342dad: 762a jbe 0x140342dd9
140342daf: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140342db6: 488b04d0 mov rax, qword ptr [rax + rdx*8]
140342dba: 4885c0 test rax, rax
140342dbd: 741a je 0x140342dd9
140342dbf: 488b8ee0010000 mov rcx, qword ptr [rsi + 0x1e0]
140342dc6: 48f7d0 not rax
140342dc9: 488d0440 lea rax, [rax + rax*2]
140342dcd: 48c1e005 shl rax, 5
140342dd1: 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
140342dd7: 7508 jne 0x140342de1
140342dd9: 4889f1 mov rcx, rsi
140342ddc: e89f232400 call 0x140585180 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::systems::wall::eraser::ui_bulldozer::HandleStairAttachedDecoratorsOnStairPointDeletion>
140342de1: 48ba5f8e7a7ac4786448 movabs rdx, 0x486478c47a7a8e5f
140342deb: 49b83a7ebc2dfad994be movabs r8, 0xbe94d9fa2dbc7e3a
140342df5: 4889f9 mov rcx, rdi
140342df8: e8f3a8d9ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::compon
140342dfd: 4885c0 test rax, rax
140342e00: 7433 je 0x140342e35
140342e02: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140342e09: 762a jbe 0x140342e35
140342e0b: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140342e12: 488b04d0 mov rax, qword ptr [rax + rdx*8]
140342e16: 4885c0 test rax, rax
140342e19: 741a je 0x140342e35
140342e1b: 488b8ee0010000 mov rcx, qword ptr [rsi + 0x1e0]
140342e22: 48f7d0 not rax
140342e25: 488d0440 lea rax, [rax + rax*2]
140342e29: 48c1e005 shl rax, 5
140342e2d: 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
140342e33: 7508 jne 0x140342e3d
140342e35: 4889f1 mov rcx, rsi
140342e38: e873c62300 call 0x14057f4b0 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::resources::events::LightLanternCmd>
140342e3d: 48ba969d7655a61d7f93 movabs rdx, 0x937f1da655769d96
140342e47: 49b85f537c98684683f0 movabs r8, 0xf0834668987c535f
140342e51: 4889f9 mov rcx, rdi
140342e54: e897a8d9ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::compon
140342e59: 4885c0 test rax, rax
140342e5c: 7433 je 0x140342e91
140342e5e: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140342e65: 762a jbe 0x140342e91
140342e67: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140342e6e: 488b04d0 mov rax, qword ptr [rax + rdx*8]
140342e72: 4885c0 test rax, rax
140342e75: 741a je 0x140342e91
140342e77: 488b8ee0010000 mov rcx, qword ptr [rsi + 0x1e0]
140342e7e: 48f7d0 not rax
140342e81: 488d0440 lea rax, [rax + rax*2]
140342e85: 48c1e005 shl rax, 5
140342e89: 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
140342e8f: 7508 jne 0x140342e99
140342e91: 4889f1 mov rcx, rsi
140342e94: e857842200 call 0x14056b2f0 ; bevy_ecs::event::registry::EventRegistry::register_event<country_core::resources::events::LightWindowCmd>
140342e99: 48ba212983f24470ccb8 movabs rdx, 0xb8cc7044f2832921
140342ea3: 49b85afe8dff8ecc7e58 movabs r8, 0x587ecc8eff8dfe5a
140342ead: 4889f9 mov rcx, rdi
140342eb0: e83ba8d9ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::compon
140342eb5: 4885c0 test rax, rax
140342eb8: 7433 je 0x140342eed
140342eba: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140342ec1: 762a jbe 0x140342eed
140342ec3: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140342eca: 488b04d0 mov rax, qword ptr [rax + rdx*8]
140342ece: 4885c0 test rax, rax
140342ed1: 741a je 0x140342eed
140342ed3: 488b8ee0010000 mov rcx, qword ptr [rsi + 0x1e0]
140342eda: 48f7d0 not rax
140342edd: 488d0440 lea rax, [rax + rax*2]
140342ee1: 48c1e005 shl rax, 5
140342ee5: 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
140342eeb: 7508 jne 0x140342ef5
140342eed: 4889f1 mov rcx, rsi
140342ef0: e89b772200 call 0x14056a690 ; bevy_ecs::event::registry::EventRegistry::register_event<system_decorator::audio::op_preview_audio::MutePreviewDecoratorOpSounds>
140342ef5: 48ba674cefcf7a567f71 movabs rdx, 0x717f567acfef4c67
140342eff: 49b8aeb083fa5fa63b19 movabs r8, 0x193ba65ffa83b0ae
140342f09: 4889f9 mov rcx, rdi
140342f0c: e8dfa7d9ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::compon
140342f11: 4885c0 test rax, rax
140342f14: 7433 je 0x140342f49
140342f16: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140342f1d: 762a jbe 0x140342f49
140342f1f: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140342f26: 488b04d0 mov rax, qword ptr [rax + rdx*8]
140342f2a: 4885c0 test rax, rax
140342f2d: 741a je 0x140342f49
140342f2f: 488b8ee0010000 mov rcx, qword ptr [rsi + 0x1e0]
140342f36: 48f7d0 not rax
140342f39: 488d0440 lea rax, [rax + rax*2]
140342f3d: 48c1e005 shl rax, 5
140342f41: 48837c012000 cmp qword ptr [rcx + rax + 0x20], 0
140342f47: 7508 jne 0x140342f51
140342f49: 4889f1 mov rcx, rsi
140342f4c: e81f412400 call 0x140587070 ; bevy_ecs::event::registry::EventRegistry::register_event<system_decorator::ui_systems::ui_place_decorator::EvInitPlaceDecoratorInteractionState>
140342f51: 0f57c0 xorps xmm0, xmm0
140342f54: 0f298520030000 movaps xmmword ptr [rbp + 0x320], xmm0
140342f5b: 48c7853003000008000000 mov qword ptr [rbp + 0x330], 8
140342f66: 48c7853803000000000000 mov qword ptr [rbp + 0x338], 0
140342f71: 488d9520030000 lea rdx, [rbp + 0x320]
140342f78: 4889f1 mov rcx, rsi
140342f7b: e8c012ddff call 0x140114240 ; bevy_app::app::App::insert_resource<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>
140342f80: 4889c6 mov rsi, rax
140342f83: e8b8a7a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140342f88: 4889c7 mov rdi, rax
140342f8b: f60001 test byte ptr [rax], 1
140342f8e: 7411 je 0x140342fa1
140342f90: 4889f9 mov rcx, rdi
140342f93: 4883c108 add rcx, 8
140342f97: 488b4708 mov rax, qword ptr [rdi + 8]
140342f9b: 488b5710 mov rdx, qword ptr [rdi + 0x10]
140342f9f: eb1b jmp 0x140342fbc ; unknown
140342fa1: e85a1ea200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140342fa6: 4889f9 mov rcx, rdi
140342fa9: 4883c108 add rcx, 8
140342fad: 48c70701000000 mov qword ptr [rdi], 1
140342fb4: 48894708 mov qword ptr [rdi + 8], rax
140342fb8: 48895710 mov qword ptr [rdi + 0x10], rdx
140342fbc: 4c8d4001 lea r8, [rax + 1]
140342fc0: 4c8901 mov qword ptr [rcx], r8
140342fc3: 0f103596fe6702 movups xmm6, xmmword ptr [rip + 0x267fe96]
140342fca: 0f29b5c0050000 movaps xmmword ptr [rbp + 0x5c0], xmm6
140342fd1: 0f103d78fe6702 movups xmm7, xmmword ptr [rip + 0x267fe78]
140342fd8: 0f29bdb0050000 movaps xmmword ptr [rbp + 0x5b0], xmm7
140342fdf: 488985d0050000 mov qword ptr [rbp + 0x5d0], rax
140342fe6: 488995d8050000 mov qword ptr [rbp + 0x5d8], rdx
140342fed: e84ea7a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140342ff2: 4889c7 mov rdi, rax
140342ff5: f60001 test byte ptr [rax], 1
140342ff8: 7411 je 0x14034300b
140342ffa: 4889f9 mov rcx, rdi
140342ffd: 4883c108 add rcx, 8
140343001: 488b4708 mov rax, qword ptr [rdi + 8]
140343005: 488b5710 mov rdx, qword ptr [rdi + 0x10]
140343009: eb1b jmp 0x140343026 ; unknown
14034300b: e8f01da200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140343010: 4889f9 mov rcx, rdi
140343013: 4883c108 add rcx, 8
140343017: 48c70701000000 mov qword ptr [rdi], 1
14034301e: 48894708 mov qword ptr [rdi + 8], rax
140343022: 48895710 mov qword ptr [rdi + 0x10], rdx
140343026: 4c8d4001 lea r8, [rax + 1]
14034302a: 4c8901 mov qword ptr [rcx], r8
14034302d: 0f29b500010000 movaps xmmword ptr [rbp + 0x100], xmm6
140343034: 0f29bdf0000000 movaps xmmword ptr [rbp + 0xf0], xmm7
14034303b: 48898510010000 mov qword ptr [rbp + 0x110], rax
140343042: 48899518010000 mov qword ptr [rbp + 0x118], rdx
140343049: e8f2a6a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
14034304e: 4889c7 mov rdi, rax
140343051: f60001 test byte ptr [rax], 1
140343054: 7411 je 0x140343067
140343056: 4889f9 mov rcx, rdi
140343059: 4883c108 add rcx, 8
14034305d: 488b4708 mov rax, qword ptr [rdi + 8]
140343061: 488b5710 mov rdx, qword ptr [rdi + 0x10]
140343065: eb1b jmp 0x140343082 ; unknown
140343067: e8941da200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
14034306c: 4889f9 mov rcx, rdi
14034306f: 4883c108 add rcx, 8
140343073: 48c70701000000 mov qword ptr [rdi], 1
14034307a: 48894708 mov qword ptr [rdi + 8], rax
14034307e: 48895710 mov qword ptr [rdi + 0x10], rdx
140343082: 4c8d4001 lea r8, [rax + 1]
140343086: 4c8901 mov qword ptr [rcx], r8
140343089: 0f2885b0050000 movaps xmm0, xmmword ptr [rbp + 0x5b0]
140343090: 0f288dc0050000 movaps xmm1, xmmword ptr [rbp + 0x5c0]
140343097: 0f2895d0050000 movaps xmm2, xmmword ptr [rbp + 0x5d0]
14034309e: 0f299540030000 movaps xmmword ptr [rbp + 0x340], xmm2
1403430a5: 0f298d30030000 movaps xmmword ptr [rbp + 0x330], xmm1
1403430ac: 0f298520030000 movaps xmmword ptr [rbp + 0x320], xmm0
1403430b3: 48c785b003000000000000 mov qword ptr [rbp + 0x3b0], 0
1403430be: 48c7854004000000000000 mov qword ptr [rbp + 0x440], 0
1403430c9: 0f2885f0000000 movaps xmm0, xmmword ptr [rbp + 0xf0]
1403430d0: 0f288d00010000 movaps xmm1, xmmword ptr [rbp + 0x100]
1403430d7: 0f289510010000 movaps xmm2, xmmword ptr [rbp + 0x110]
1403430de: 0f298550030000 movaps xmmword ptr [rbp + 0x350], xmm0
1403430e5: 0f298d60030000 movaps xmmword ptr [rbp + 0x360], xmm1
1403430ec: 0f299570030000 movaps xmmword ptr [rbp + 0x370], xmm2
1403430f3: 0f29bd80030000 movaps xmmword ptr [rbp + 0x380], xmm7
1403430fa: 0f29b590030000 movaps xmmword ptr [rbp + 0x390], xmm6
140343101: 488985a0030000 mov qword ptr [rbp + 0x3a0], rax
140343108: 488995a8030000 mov qword ptr [rbp + 0x3a8], rdx
14034310f: c685d004000000 mov byte ptr [rbp + 0x4d0], 0
140343116: 488d9520030000 lea rdx, [rbp + 0x320]
14034311d: 4889f1 mov rcx, rsi
140343120: e81baadfff call 0x14013db40 ; bevy_app::app::App::insert_resource<country_core::resources::walls::decorator_storage::DecoratorStorage>
140343125: 4889c6 mov rsi, rax
140343128: 488dbd20030000 lea rdi, [rbp + 0x320]
14034312f: 4889f9 mov rcx, rdi
140343132: e839e2ffff call 0x140341370 ; country_core::resources::walls::decorator_archivist::impl$1::default
140343137: 4889f1 mov rcx, rsi
14034313a: 4889fa mov rdx, rdi
14034313d: e83e78ddff call 0x14011a980 ; bevy_app::app::App::insert_resource<country_core::resources::walls::decorator_archivist::DecoratorArchivist>
140343142: 4889c6 mov rsi, rax
140343145: 31c9 xor ecx, ecx
140343147: e8d469d9ff call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > 
14034314c: 4885c0 test rax, rax
14034314f: 0f846d220000 je 0x1403453c2
140343155: 48c7852003000000000000 mov qword ptr [rbp + 0x320], 0
140343160: 48c7852803000008000000 mov qword ptr [rbp + 0x328], 8
14034316b: 48c7853003000000000000 mov qword ptr [rbp + 0x330], 0
140343176: 0f1000 movups xmm0, xmmword ptr [rax]
140343179: 48ff00 inc qword ptr [rax]
14034317c: 440f1005acce6602 movups xmm8, xmmword ptr [rip + 0x266ceac]
140343184: 440f118548030000 movups xmmword ptr [rbp + 0x348], xmm8
14034318c: 440f100d8cce6602 movups xmm9, xmmword ptr [rip + 0x266ce8c]
140343194: 440f118d38030000 movups xmmword ptr [rbp + 0x338], xmm9
14034319c: 0f118558030000 movups xmmword ptr [rbp + 0x358], xmm0
1403431a3: c6856803000000 mov byte ptr [rbp + 0x368], 0
1403431aa: 488d9520030000 lea rdx, [rbp + 0x320]
1403431b1: 4889f1 mov rcx, rsi
1403431b4: e8b719dfff call 0x140134b70 ; bevy_app::app::App::insert_resource<system_decorator::deferred_decorator_history::DeferredDecoratorEditHistory>
1403431b9: 4889c6 mov rsi, rax
1403431bc: e87fa5a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1403431c1: 4889c7 mov rdi, rax
1403431c4: f60001 test byte ptr [rax], 1
1403431c7: 7411 je 0x1403431da
1403431c9: 4889f9 mov rcx, rdi
1403431cc: 4883c108 add rcx, 8
1403431d0: 488b4708 mov rax, qword ptr [rdi + 8]
1403431d4: 488b5710 mov rdx, qword ptr [rdi + 0x10]
1403431d8: eb1b jmp 0x1403431f5 ; unknown
1403431da: e8211ca200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1403431df: 4889f9 mov rcx, rdi
1403431e2: 4883c108 add rcx, 8
1403431e6: 48c70701000000 mov qword ptr [rdi], 1
1403431ed: 48894708 mov qword ptr [rdi + 8], rax
1403431f1: 48895710 mov qword ptr [rdi + 0x10], rdx
1403431f5: 4c8d4001 lea r8, [rax + 1]
1403431f9: 4c8901 mov qword ptr [rcx], r8
1403431fc: 0f29b500010000 movaps xmmword ptr [rbp + 0x100], xmm6
140343203: 0f29bdf0000000 movaps xmmword ptr [rbp + 0xf0], xmm7
14034320a: 48898510010000 mov qword ptr [rbp + 0x110], rax
140343211: 48899518010000 mov qword ptr [rbp + 0x118], rdx
140343218: e823a5a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
14034321d: 4889c7 mov rdi, rax
140343220: f60001 test byte ptr [rax], 1
140343223: 7411 je 0x140343236
140343225: 4889f9 mov rcx, rdi
140343228: 4883c108 add rcx, 8
14034322c: 488b4708 mov rax, qword ptr [rdi + 8]
140343230: 488b5710 mov rdx, qword ptr [rdi + 0x10]
140343234: eb1b jmp 0x140343251 ; unknown
140343236: e8c51ba200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
14034323b: 4889f9 mov rcx, rdi
14034323e: 4883c108 add rcx, 8
140343242: 48c70701000000 mov qword ptr [rdi], 1
140343249: 48894708 mov qword ptr [rdi + 8], rax
14034324d: 48895710 mov qword ptr [rdi + 0x10], rdx
140343251: 4c8d4001 lea r8, [rax + 1]
140343255: 4c8901 mov qword ptr [rcx], r8
140343258: 0f2885f0000000 movaps xmm0, xmmword ptr [rbp + 0xf0]
14034325f: 0f288d00010000 movaps xmm1, xmmword ptr [rbp + 0x100]
140343266: 0f289510010000 movaps xmm2, xmmword ptr [rbp + 0x110]
14034326d: 0f119558030000 movups xmmword ptr [rbp + 0x358], xmm2
140343274: 0f118d48030000 movups xmmword ptr [rbp + 0x348], xmm1
14034327b: 0f118538030000 movups xmmword ptr [rbp + 0x338], xmm0
140343282: 0f11bd68030000 movups xmmword ptr [rbp + 0x368], xmm7
140343289: 0f11b578030000 movups xmmword ptr [rbp + 0x378], xmm6
140343290: 48898588030000 mov qword ptr [rbp + 0x388], rax
140343297: 48899590030000 mov qword ptr [rbp + 0x390], rdx
14034329e: 48c7852003000000000000 mov qword ptr [rbp + 0x320], 0
1403432a9: 48c7852803000010000000 mov qword ptr [rbp + 0x328], 0x10
1403432b4: 48c7853003000000000000 mov qword ptr [rbp + 0x330], 0
1403432bf: 488d9520030000 lea rdx, [rbp + 0x320]
1403432c6: 4889f1 mov rcx, rsi
1403432c9: e8029ddfff call 0x14013cfd0 ; bevy_app::app::App::insert_resource<system_decorator::decorator_visual_state::DecoratorVisualState>
1403432ce: 4889c6 mov rsi, rax
1403432d1: e86aa4a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1403432d6: 4889c7 mov rdi, rax
1403432d9: f60001 test byte ptr [rax], 1
1403432dc: 7411 je 0x1403432ef
1403432de: 4889f9 mov rcx, rdi
1403432e1: 4883c108 add rcx, 8
1403432e5: 488b4708 mov rax, qword ptr [rdi + 8]
1403432e9: 488b5710 mov rdx, qword ptr [rdi + 0x10]
1403432ed: eb1b jmp 0x14034330a ; unknown
1403432ef: e80c1ba200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1403432f4: 4889f9 mov rcx, rdi
1403432f7: 4883c108 add rcx, 8
1403432fb: 48c70701000000 mov qword ptr [rdi], 1
140343302: 48894708 mov qword ptr [rdi + 8], rax
140343306: 48895710 mov qword ptr [rdi + 0x10], rdx
14034330a: 4c8d4001 lea r8, [rax + 1]
14034330e: 4c8901 mov qword ptr [rcx], r8
140343311: 0f29b530030000 movaps xmmword ptr [rbp + 0x330], xmm6
140343318: 0f29bd20030000 movaps xmmword ptr [rbp + 0x320], xmm7
14034331f: 48898540030000 mov qword ptr [rbp + 0x340], rax
140343326: 48899548030000 mov qword ptr [rbp + 0x348], rdx
14034332d: 488d9520030000 lea rdx, [rbp + 0x320]
140343334: 4889f1 mov rcx, rsi
140343337: e864bbdeff call 0x14012eea0 ; bevy_app::app::App::insert_resource<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms>
14034333c: c7852003000000000000 mov dword ptr [rbp + 0x320], 0
140343346: c7853003000000000000 mov dword ptr [rbp + 0x330], 0
140343350: 48c7854003000000000000 mov qword ptr [rbp + 0x340], 0
14034335b: 488d9520030000 lea rdx, [rbp + 0x320]
140343362: 4889c1 mov rcx, rax
140343365: e8962edeff call 0x140126200 ; bevy_app::app::App::insert_resource<system_decorator::ui_systems::ui_move_decorator::MoveDecoratorDeltaPositions>
14034336a: 4889c6 mov rsi, rax
14034336d: e8feefcbff call 0x140002370 ; bevy_ecs::system::schedule_system::impl$1::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(tuple$<bevy_ecs::change_detection::Res<country_core::resources::walls::public_walls::PublicWalls>,bevy_ec
140343372: 4889f1 mov rcx, rsi
140343375: 89c2 mov edx, eax
140343377: e84485dfff call 0x14013b8c0 ; bevy_app::app::App::insert_resource<country_core::resources::walls::decorator::DecoratorIdGen>
14034337c: c7852003000000000000 mov dword ptr [rbp + 0x320], 0
140343386: 48c7852803000000000000 mov qword ptr [rbp + 0x328], 0
140343391: c7852f03000000000000 mov dword ptr [rbp + 0x32f], 0
14034339b: 488d9520030000 lea rdx, [rbp + 0x320]
1403433a2: 4889c1 mov rcx, rax
1403433a5: e82640dfff call 0x1401373d0 ; bevy_app::app::App::insert_resource<system_decorator::ui_systems::ui_place_decorator::PlaceDecoratorInteractionState>
1403433aa: ba03000000 mov edx, 3
1403433af: 4889c1 mov rcx, rax
1403433b2: e8c93cddff call 0x140117080 ; bevy_app::app::App::insert_resource<enum2$<system_decorator::audio::op_preview_audio::PreviousFrameDecoratorPreviewOp> >
1403433b7: 4889c6 mov rsi, rax
1403433ba: e881a3a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1403433bf: 4889c7 mov rdi, rax
1403433c2: f60001 test byte ptr [rax], 1
1403433c5: 7411 je 0x1403433d8
1403433c7: 4889f9 mov rcx, rdi
1403433ca: 4883c108 add rcx, 8
1403433ce: 488b4708 mov rax, qword ptr [rdi + 8]
1403433d2: 488b5710 mov rdx, qword ptr [rdi + 0x10]
1403433d6: eb1b jmp 0x1403433f3 ; unknown
1403433d8: e8231aa200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1403433dd: 4889f9 mov rcx, rdi
1403433e0: 4883c108 add rcx, 8
1403433e4: 48c70701000000 mov qword ptr [rdi], 1
1403433eb: 48894708 mov qword ptr [rdi + 8], rax
1403433ef: 48895710 mov qword ptr [rdi + 0x10], rdx
1403433f3: 4c8d4001 lea r8, [rax + 1]
1403433f7: 4c8901 mov qword ptr [rcx], r8
1403433fa: 0f29b5c0020000 movaps xmmword ptr [rbp + 0x2c0], xmm6
140343401: 0f29bdb0020000 movaps xmmword ptr [rbp + 0x2b0], xmm7
140343408: 488985d0020000 mov qword ptr [rbp + 0x2d0], rax
14034340f: 488995d8020000 mov qword ptr [rbp + 0x2d8], rdx
140343416: e825a3a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
14034341b: 4889c7 mov rdi, rax
14034341e: f60001 test byte ptr [rax], 1
140343421: 7411 je 0x140343434
140343423: 4889f9 mov rcx, rdi
140343426: 4883c108 add rcx, 8
14034342a: 488b4708 mov rax, qword ptr [rdi + 8]
14034342e: 488b5710 mov rdx, qword ptr [rdi + 0x10]
140343432: eb1b jmp 0x14034344f ; unknown
140343434: e8c719a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140343439: 4889f9 mov rcx, rdi
14034343c: 4883c108 add rcx, 8
140343440: 48c70701000000 mov qword ptr [rdi], 1
140343447: 48894708 mov qword ptr [rdi + 8], rax
14034344b: 48895710 mov qword ptr [rdi + 0x10], rdx
14034344f: 4c8d4001 lea r8, [rax + 1]
140343453: 4c8901 mov qword ptr [rcx], r8
140343456: 0f29b5c0050000 movaps xmmword ptr [rbp + 0x5c0], xmm6
14034345d: 0f29bdb0050000 movaps xmmword ptr [rbp + 0x5b0], xmm7
140343464: 488985d0050000 mov qword ptr [rbp + 0x5d0], rax
14034346b: 488995d8050000 mov qword ptr [rbp + 0x5d8], rdx
140343472: e8c9a2a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140343477: 4889c7 mov rdi, rax
14034347a: f60001 test byte ptr [rax], 1
14034347d: 7411 je 0x140343490
14034347f: 4889f9 mov rcx, rdi
140343482: 4883c108 add rcx, 8
140343486: 488b4708 mov rax, qword ptr [rdi + 8]
14034348a: 488b5710 mov rdx, qword ptr [rdi + 0x10]
14034348e: eb1b jmp 0x1403434ab ; unknown
140343490: e86b19a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140343495: 4889f9 mov rcx, rdi
140343498: 4883c108 add rcx, 8
14034349c: 48c70701000000 mov qword ptr [rdi], 1
1403434a3: 48894708 mov qword ptr [rdi + 8], rax
1403434a7: 48895710 mov qword ptr [rdi + 0x10], rdx
1403434ab: 4c8d4001 lea r8, [rax + 1]
1403434af: 4c8901 mov qword ptr [rcx], r8
1403434b2: 0f29b500010000 movaps xmmword ptr [rbp + 0x100], xmm6
1403434b9: 0f29bdf0000000 movaps xmmword ptr [rbp + 0xf0], xmm7
1403434c0: 48898510010000 mov qword ptr [rbp + 0x110], rax
1403434c7: 48899518010000 mov qword ptr [rbp + 0x118], rdx
1403434ce: e86da2a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1403434d3: 4889c7 mov rdi, rax
1403434d6: f60001 test byte ptr [rax], 1
1403434d9: 7411 je 0x1403434ec
1403434db: 4889f9 mov rcx, rdi
1403434de: 4883c108 add rcx, 8
1403434e2: 488b4708 mov rax, qword ptr [rdi + 8]
1403434e6: 488b5710 mov rdx, qword ptr [rdi + 0x10]
1403434ea: eb1b jmp 0x140343507 ; unknown
1403434ec: e80f19a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1403434f1: 4889f9 mov rcx, rdi
1403434f4: 4883c108 add rcx, 8
1403434f8: 48c70701000000 mov qword ptr [rdi], 1
1403434ff: 48894708 mov qword ptr [rdi + 8], rax
140343503: 48895710 mov qword ptr [rdi + 0x10], rdx
140343507: 4c8d4001 lea r8, [rax + 1]
14034350b: 4c8901 mov qword ptr [rcx], r8
14034350e: 0f2885b0020000 movaps xmm0, xmmword ptr [rbp + 0x2b0]
140343515: 0f288dc0020000 movaps xmm1, xmmword ptr [rbp + 0x2c0]
14034351c: 0f2895d0020000 movaps xmm2, xmmword ptr [rbp + 0x2d0]
140343523: 0f299540030000 movaps xmmword ptr [rbp + 0x340], xmm2
14034352a: 0f298d30030000 movaps xmmword ptr [rbp + 0x330], xmm1
140343531: 0f298520030000 movaps xmmword ptr [rbp + 0x320], xmm0
140343538: 0f2885b0050000 movaps xmm0, xmmword ptr [rbp + 0x5b0]
14034353f: 0f288dc0050000 movaps xmm1, xmmword ptr [rbp + 0x5c0]
140343546: 0f2895d0050000 movaps xmm2, xmmword ptr [rbp + 0x5d0]
14034354d: 0f298550030000 movaps xmmword ptr [rbp + 0x350], xmm0
140343554: 0f298d60030000 movaps xmmword ptr [rbp + 0x360], xmm1
14034355b: 0f299570030000 movaps xmmword ptr [rbp + 0x370], xmm2
140343562: 0f2885f0000000 movaps xmm0, xmmword ptr [rbp + 0xf0]
140343569: 0f288d00010000 movaps xmm1, xmmword ptr [rbp + 0x100]
140343570: 0f289510010000 movaps xmm2, xmmword ptr [rbp + 0x110]
140343577: 0f298580030000 movaps xmmword ptr [rbp + 0x380], xmm0
14034357e: 0f298d90030000 movaps xmmword ptr [rbp + 0x390], xmm1
140343585: 0f2995a0030000 movaps xmmword ptr [rbp + 0x3a0], xmm2
14034358c: 0f29b5c0030000 movaps xmmword ptr [rbp + 0x3c0], xmm6
140343593: 0f29bdb0030000 movaps xmmword ptr [rbp + 0x3b0], xmm7
14034359a: 488985d0030000 mov qword ptr [rbp + 0x3d0], rax
1403435a1: 488995d8030000 mov qword ptr [rbp + 0x3d8], rdx
1403435a8: 488d9520030000 lea rdx, [rbp + 0x320]
1403435af: 4889f1 mov rcx, rsi
1403435b2: e869b1ddff call 0x14011e720 ; bevy_app::app::App::insert_resource<system_decorator::audio::op_confirm_audio::DecoratorAudioEvents>
1403435b7: 4889c6 mov rsi, rax
1403435ba: e881a1a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1403435bf: 4889c7 mov rdi, rax
1403435c2: f60001 test byte ptr [rax], 1
1403435c5: 7411 je 0x1403435d8
1403435c7: 4889f9 mov rcx, rdi
1403435ca: 4883c108 add rcx, 8
1403435ce: 488b4708 mov rax, qword ptr [rdi + 8]
1403435d2: 488b5710 mov rdx, qword ptr [rdi + 0x10]
1403435d6: eb1b jmp 0x1403435f3 ; unknown
1403435d8: e82318a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1403435dd: 4889f9 mov rcx, rdi
1403435e0: 4883c108 add rcx, 8
1403435e4: 48c70701000000 mov qword ptr [rdi], 1
1403435eb: 48894708 mov qword ptr [rdi + 8], rax
1403435ef: 48895710 mov qword ptr [rdi + 0x10], rdx
1403435f3: 4c8d4001 lea r8, [rax + 1]
1403435f7: 4c8901 mov qword ptr [rcx], r8
1403435fa: c6855003000000 mov byte ptr [rbp + 0x350], 0
140343601: 0f29bd20030000 movaps xmmword ptr [rbp + 0x320], xmm7
140343608: 0f29b530030000 movaps xmmword ptr [rbp + 0x330], xmm6
14034360f: 48898540030000 mov qword ptr [rbp + 0x340], rax
140343616: 48899548030000 mov qword ptr [rbp + 0x348], rdx
14034361d: 488d9520030000 lea rdx, [rbp + 0x320]
140343624: 4889f1 mov rcx, rsi
140343627: e8f4f2deff call 0x140132920 ; bevy_app::app::App::insert_resource<country_core::resources::window_lights::GlobalWindowLightsControl>
14034362c: 48c7852803000000000000 mov qword ptr [rbp + 0x328], 0
140343637: 48c7852003000006000000 mov qword ptr [rbp + 0x320], 6
140343642: 488d9520030000 lea rdx, [rbp + 0x320]
140343649: 4889c1 mov rcx, rax
14034364c: e83f60dfff call 0x140139690 ; bevy_app::app::App::insert_resource<system_decorator::ui_systems::ui_decorator_affordance_dispatch::DecoratorAffordanceDispatcher>
140343651: 4889c6 mov rsi, rax
140343654: e8e7a0a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140343659: 4889c7 mov rdi, rax
14034365c: f60001 test byte ptr [rax], 1
14034365f: 7411 je 0x140343672
140343661: 4889f9 mov rcx, rdi
140343664: 4883c108 add rcx, 8
140343668: 488b4708 mov rax, qword ptr [rdi + 8]
14034366c: 488b5710 mov rdx, qword ptr [rdi + 0x10]
140343670: eb1b jmp 0x14034368d ; unknown
140343672: e88917a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140343677: 4889f9 mov rcx, rdi
14034367a: 4883c108 add rcx, 8
14034367e: 48c70701000000 mov qword ptr [rdi], 1
140343685: 48894708 mov qword ptr [rdi + 8], rax
140343689: 48895710 mov qword ptr [rdi + 0x10], rdx
14034368d: 4c8d4001 lea r8, [rax + 1]
140343691: 4c8901 mov qword ptr [rcx], r8
140343694: 0f29b530030000 movaps xmmword ptr [rbp + 0x330], xmm6
14034369b: 0f29bd20030000 movaps xmmword ptr [rbp + 0x320], xmm7
1403436a2: 48898540030000 mov qword ptr [rbp + 0x340], rax
1403436a9: 48899548030000 mov qword ptr [rbp + 0x348], rdx
1403436b0: 488d9520030000 lea rdx, [rbp + 0x320]
1403436b7: 4889f1 mov rcx, rsi
1403436ba: e8b1e0deff call 0x140131770 ; bevy_app::app::App::insert_resource<country_core::resources::walls::decorator_subtype::DerivedDecoratorSubtypes>
1403436bf: 4889c6 mov rsi, rax
1403436c2: e879a0a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1403436c7: 4889c7 mov rdi, rax
1403436ca: f60001 test byte ptr [rax], 1
1403436cd: 7411 je 0x1403436e0
1403436cf: 4889f9 mov rcx, rdi
1403436d2: 4883c108 add rcx, 8
1403436d6: 488b4708 mov rax, qword ptr [rdi + 8]
1403436da: 488b5710 mov rdx, qword ptr [rdi + 0x10]
1403436de: eb1b jmp 0x1403436fb ; unknown
1403436e0: e81b17a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1403436e5: 4889f9 mov rcx, rdi
1403436e8: 4883c108 add rcx, 8
1403436ec: 48c70701000000 mov qword ptr [rdi], 1
1403436f3: 48894708 mov qword ptr [rdi + 8], rax
1403436f7: 48895710 mov qword ptr [rdi + 0x10], rdx
1403436fb: 4c8d4001 lea r8, [rax + 1]
1403436ff: 4c8901 mov qword ptr [rcx], r8
140343702: 0f29b5c0020000 movaps xmmword ptr [rbp + 0x2c0], xmm6
140343709: 0f29bdb0020000 movaps xmmword ptr [rbp + 0x2b0], xmm7
140343710: 488985d0020000 mov qword ptr [rbp + 0x2d0], rax
140343717: 488995d8020000 mov qword ptr [rbp + 0x2d8], rdx
14034371e: e81da0a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140343723: 4889c7 mov rdi, rax
140343726: f60001 test byte ptr [rax], 1
140343729: 7411 je 0x14034373c
14034372b: 4889f9 mov rcx, rdi
14034372e: 4883c108 add rcx, 8
140343732: 488b4708 mov rax, qword ptr [rdi + 8]
140343736: 488b5710 mov rdx, qword ptr [rdi + 0x10]
14034373a: eb1b jmp 0x140343757 ; unknown
14034373c: e8bf16a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140343741: 4889f9 mov rcx, rdi
140343744: 4883c108 add rcx, 8
140343748: 48c70701000000 mov qword ptr [rdi], 1
14034374f: 48894708 mov qword ptr [rdi + 8], rax
140343753: 48895710 mov qword ptr [rdi + 0x10], rdx
140343757: 4c8d4001 lea r8, [rax + 1]
14034375b: 4c8901 mov qword ptr [rcx], r8
14034375e: 0f29b5c0050000 movaps xmmword ptr [rbp + 0x5c0], xmm6
140343765: 0f29bdb0050000 movaps xmmword ptr [rbp + 0x5b0], xmm7
14034376c: 488985d0050000 mov qword ptr [rbp + 0x5d0], rax
140343773: 488995d8050000 mov qword ptr [rbp + 0x5d8], rdx
14034377a: e8c19fa000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
14034377f: 4889c7 mov rdi, rax
140343782: f60001 test byte ptr [rax], 1
140343785: 7411 je 0x140343798
140343787: 4889f9 mov rcx, rdi
14034378a: 4883c108 add rcx, 8
14034378e: 488b4708 mov rax, qword ptr [rdi + 8]
140343792: 488b5710 mov rdx, qword ptr [rdi + 0x10]
140343796: eb1b jmp 0x1403437b3 ; unknown
140343798: e86316a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
14034379d: 4889f9 mov rcx, rdi
1403437a0: 4883c108 add rcx, 8
1403437a4: 48c70701000000 mov qword ptr [rdi], 1
1403437ab: 48894708 mov qword ptr [rdi + 8], rax
1403437af: 48895710 mov qword ptr [rdi + 0x10], rdx
1403437b3: 4c8d4001 lea r8, [rax + 1]
1403437b7: 4c8901 mov qword ptr [rcx], r8
1403437ba: 0f29b500010000 movaps xmmword ptr [rbp + 0x100], xmm6
1403437c1: 0f29bdf0000000 movaps xmmword ptr [rbp + 0xf0], xmm7
1403437c8: 48898510010000 mov qword ptr [rbp + 0x110], rax
1403437cf: 48899518010000 mov qword ptr [rbp + 0x118], rdx
1403437d6: e8659fa000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1403437db: 4889c7 mov rdi, rax
1403437de: f60001 test byte ptr [rax], 1
1403437e1: 7411 je 0x1403437f4
1403437e3: 4889f9 mov rcx, rdi
1403437e6: 4883c108 add rcx, 8
1403437ea: 488b4708 mov rax, qword ptr [rdi + 8]
1403437ee: 488b5710 mov rdx, qword ptr [rdi + 0x10]
1403437f2: eb1b jmp 0x14034380f ; unknown
1403437f4: e80716a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1403437f9: 4889f9 mov rcx, rdi
1403437fc: 4883c108 add rcx, 8
140343800: 48c70701000000 mov qword ptr [rdi], 1
140343807: 48894708 mov qword ptr [rdi + 8], rax
14034380b: 48895710 mov qword ptr [rdi + 0x10], rdx
14034380f: 4c8d4001 lea r8, [rax + 1]
140343813: 4c8901 mov qword ptr [rcx], r8
140343816: 0f2885b0020000 movaps xmm0, xmmword ptr [rbp + 0x2b0]
14034381d: 0f288dc0020000 movaps xmm1, xmmword ptr [rbp + 0x2c0]
140343824: 0f2895d0020000 movaps xmm2, xmmword ptr [rbp + 0x2d0]
14034382b: 0f299540030000 movaps xmmword ptr [rbp + 0x340], xmm2
140343832: 0f298d30030000 movaps xmmword ptr [rbp + 0x330], xmm1
140343839: 0f298520030000 movaps xmmword ptr [rbp + 0x320], xmm0
140343840: 0f2885b0050000 movaps xmm0, xmmword ptr [rbp + 0x5b0]
140343847: 0f288dc0050000 movaps xmm1, xmmword ptr [rbp + 0x5c0]
14034384e: 0f2895d0050000 movaps xmm2, xmmword ptr [rbp + 0x5d0]
140343855: 0f298550030000 movaps xmmword ptr [rbp + 0x350], xmm0
14034385c: 0f298d60030000 movaps xmmword ptr [rbp + 0x360], xmm1
140343863: 0f299570030000 movaps xmmword ptr [rbp + 0x370], xmm2
14034386a: 0f2885f0000000 movaps xmm0, xmmword ptr [rbp + 0xf0]
140343871: 0f288d00010000 movaps xmm1, xmmword ptr [rbp + 0x100]
140343878: 0f289510010000 movaps xmm2, xmmword ptr [rbp + 0x110]
14034387f: 0f298580030000 movaps xmmword ptr [rbp + 0x380], xmm0
140343886: 0f298d90030000 movaps xmmword ptr [rbp + 0x390], xmm1
14034388d: 0f2995a0030000 movaps xmmword ptr [rbp + 0x3a0], xmm2
140343894: 0f29b5c0030000 movaps xmmword ptr [rbp + 0x3c0], xmm6
14034389b: 0f29bdb0030000 movaps xmmword ptr [rbp + 0x3b0], xmm7
1403438a2: 488985d0030000 mov qword ptr [rbp + 0x3d0], rax
1403438a9: 488995d8030000 mov qword ptr [rbp + 0x3d8], rdx
1403438b0: 488d9520030000 lea rdx, [rbp + 0x320]
1403438b7: 4889f1 mov rcx, rsi
1403438ba: e811c7deff call 0x14012ffd0 ; bevy_app::app::App::insert_resource<country_core::resources::walls::decorator_subtype::DecoratorMeshLibrary>
1403438bf: 4889c6 mov rsi, rax
1403438c2: e8799ea000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1403438c7: 4889c7 mov rdi, rax
1403438ca: f60001 test byte ptr [rax], 1
1403438cd: 7411 je 0x1403438e0
1403438cf: 4889f9 mov rcx, rdi
1403438d2: 4883c108 add rcx, 8
1403438d6: 488b4708 mov rax, qword ptr [rdi + 8]
1403438da: 488b5710 mov rdx, qword ptr [rdi + 0x10]
1403438de: eb1b jmp 0x1403438fb ; unknown
1403438e0: e81b15a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1403438e5: 4889f9 mov rcx, rdi
1403438e8: 4883c108 add rcx, 8
1403438ec: 48c70701000000 mov qword ptr [rdi], 1
1403438f3: 48894708 mov qword ptr [rdi + 8], rax
1403438f7: 48895710 mov qword ptr [rdi + 0x10], rdx
1403438fb: 4c8d4001 lea r8, [rax + 1]
1403438ff: 4c8901 mov qword ptr [rcx], r8
140343902: 0f29b530030000 movaps xmmword ptr [rbp + 0x330], xmm6
140343909: 0f29bd20030000 movaps xmmword ptr [rbp + 0x320], xmm7
140343910: 48898540030000 mov qword ptr [rbp + 0x340], rax
140343917: 48899548030000 mov qword ptr [rbp + 0x348], rdx
14034391e: 488d9520030000 lea rdx, [rbp + 0x320]
140343925: 4889f1 mov rcx, rsi
140343928: e80307dfff call 0x140134030 ; bevy_app::app::App::insert_resource<system_decorator::blueprint::DecoratorBlueprints>
14034392d: 4889c6 mov rsi, rax
140343930: e80b9ea000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140343935: 4889c7 mov rdi, rax
140343938: f60001 test byte ptr [rax], 1
14034393b: 7411 je 0x14034394e
14034393d: 4889f9 mov rcx, rdi
140343940: 4883c108 add rcx, 8
140343944: 488b4708 mov rax, qword ptr [rdi + 8]
140343948: 488b5710 mov rdx, qword ptr [rdi + 0x10]
14034394c: eb1b jmp 0x140343969 ; unknown
14034394e: e8ad14a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140343953: 4889f9 mov rcx, rdi
140343956: 4883c108 add rcx, 8
14034395a: 48c70701000000 mov qword ptr [rdi], 1
140343961: 48894708 mov qword ptr [rdi + 8], rax
140343965: 48895710 mov qword ptr [rdi + 0x10], rdx
140343969: 4c8d4001 lea r8, [rax + 1]
14034396d: 4c8901 mov qword ptr [rcx], r8
140343970: 0f29b530030000 movaps xmmword ptr [rbp + 0x330], xmm6
140343977: 0f29bd20030000 movaps xmmword ptr [rbp + 0x320], xmm7
14034397e: 48898540030000 mov qword ptr [rbp + 0x340], rax
140343985: 48899548030000 mov qword ptr [rbp + 0x348], rdx
14034398c: 488d9520030000 lea rdx, [rbp + 0x320]
140343993: 4889f1 mov rcx, rsi
140343996: e8b594deff call 0x14012ce50 ; bevy_app::app::App::insert_resource<system_decorator::blueprint::PrevDecoratorBlueprints>
14034399b: 4889c6 mov rsi, rax
14034399e: 31c9 xor ecx, ecx
1403439a0: e87b61d9ff call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > 
1403439a5: 4885c0 test rax, rax
1403439a8: 0f84141a0000 je 0x1403453c2
1403439ae: 0f1000 movups xmm0, xmmword ptr [rax]
1403439b1: 48ff00 inc qword ptr [rax]
1403439b4: 440f1015f4fe6502 movups xmm10, xmmword ptr [rip + 0x265fef4]
1403439bc: 440f299500010000 movaps xmmword ptr [rbp + 0x100], xmm10
1403439c4: 440f101dd4fe6502 movups xmm11, xmmword ptr [rip + 0x265fed4]
1403439cc: 440f299df0000000 movaps xmmword ptr [rbp + 0xf0], xmm11
1403439d4: 0f298510010000 movaps xmmword ptr [rbp + 0x110], xmm0
1403439db: 31c9 xor ecx, ecx
1403439dd: e83e61d9ff call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > 
1403439e2: 4885c0 test rax, rax
1403439e5: 0f845a1a0000 je 0x140345445
1403439eb: 0f1000 movups xmm0, xmmword ptr [rax]
1403439ee: 48ff00 inc qword ptr [rax]
1403439f1: 0f288df0000000 movaps xmm1, xmmword ptr [rbp + 0xf0]
1403439f8: 0f289500010000 movaps xmm2, xmmword ptr [rbp + 0x100]
1403439ff: 0f289d10010000 movaps xmm3, xmmword ptr [rbp + 0x110]
140343a06: 0f299d40030000 movaps xmmword ptr [rbp + 0x340], xmm3
140343a0d: 0f299530030000 movaps xmmword ptr [rbp + 0x330], xmm2
140343a14: 0f298d20030000 movaps xmmword ptr [rbp + 0x320], xmm1
140343a1b: 440f299d50030000 movaps xmmword ptr [rbp + 0x350], xmm11
140343a23: 440f299560030000 movaps xmmword ptr [rbp + 0x360], xmm10
140343a2b: 0f298570030000 movaps xmmword ptr [rbp + 0x370], xmm0
140343a32: 488d9520030000 lea rdx, [rbp + 0x320]
140343a39: 4889f1 mov rcx, rsi
140343a3c: e8bf83dfff call 0x14013be00 ; bevy_app::app::App::insert_resource<country_core::resources::walls::decorator_collision::DecoratorPhysicsColliders>
140343a41: 4889c6 mov rsi, rax
140343a44: e8f79ca000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140343a49: 4889c7 mov rdi, rax
140343a4c: f60001 test byte ptr [rax], 1
140343a4f: 7411 je 0x140343a62
140343a51: 4889f9 mov rcx, rdi
140343a54: 4883c108 add rcx, 8
140343a58: 488b4708 mov rax, qword ptr [rdi + 8]
140343a5c: 488b5710 mov rdx, qword ptr [rdi + 0x10]
140343a60: eb1b jmp 0x140343a7d ; unknown
140343a62: e89913a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140343a67: 4889f9 mov rcx, rdi
140343a6a: 4883c108 add rcx, 8
140343a6e: 48c70701000000 mov qword ptr [rdi], 1
140343a75: 48894708 mov qword ptr [rdi + 8], rax
140343a79: 48895710 mov qword ptr [rdi + 0x10], rdx
140343a7d: 4c8d4001 lea r8, [rax + 1]
140343a81: 4c8901 mov qword ptr [rcx], r8
140343a84: 0f29b5c0050000 movaps xmmword ptr [rbp + 0x5c0], xmm6
140343a8b: 0f29bdb0050000 movaps xmmword ptr [rbp + 0x5b0], xmm7
140343a92: 488985d0050000 mov qword ptr [rbp + 0x5d0], rax
140343a99: 488995d8050000 mov qword ptr [rbp + 0x5d8], rdx
140343aa0: e89b9ca000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140343aa5: 4889c7 mov rdi, rax
140343aa8: f60001 test byte ptr [rax], 1
140343aab: 7411 je 0x140343abe
140343aad: 4889f9 mov rcx, rdi
140343ab0: 4883c108 add rcx, 8
140343ab4: 488b4708 mov rax, qword ptr [rdi + 8]
140343ab8: 488b5710 mov rdx, qword ptr [rdi + 0x10]
140343abc: eb1b jmp 0x140343ad9 ; unknown
140343abe: e83d13a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140343ac3: 4889f9 mov rcx, rdi
140343ac6: 4883c108 add rcx, 8
140343aca: 48c70701000000 mov qword ptr [rdi], 1
140343ad1: 48894708 mov qword ptr [rdi + 8], rax
140343ad5: 48895710 mov qword ptr [rdi + 0x10], rdx
140343ad9: 4c8d4001 lea r8, [rax + 1]
140343add: 4c8901 mov qword ptr [rcx], r8
140343ae0: 0f29b530030000 movaps xmmword ptr [rbp + 0x330], xmm6
140343ae7: 0f29bd20030000 movaps xmmword ptr [rbp + 0x320], xmm7
140343aee: 48898540030000 mov qword ptr [rbp + 0x340], rax
140343af5: 48899548030000 mov qword ptr [rbp + 0x348], rdx
140343afc: e83f9ca000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140343b01: 4889c7 mov rdi, rax
140343b04: f60001 test byte ptr [rax], 1
140343b07: 7411 je 0x140343b1a
140343b09: 4889f9 mov rcx, rdi
140343b0c: 4883c108 add rcx, 8
140343b10: 488b4708 mov rax, qword ptr [rdi + 8]
140343b14: 488b5710 mov rdx, qword ptr [rdi + 0x10]
140343b18: eb1b jmp 0x140343b35 ; unknown
140343b1a: e8e112a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140343b1f: 4889f9 mov rcx, rdi
140343b22: 4883c108 add rcx, 8
140343b26: 48c70701000000 mov qword ptr [rdi], 1
140343b2d: 48894708 mov qword ptr [rdi + 8], rax
140343b31: 48895710 mov qword ptr [rdi + 0x10], rdx
140343b35: 4c8d4001 lea r8, [rax + 1]
140343b39: 4c8901 mov qword ptr [rcx], r8
140343b3c: 0f2885b0050000 movaps xmm0, xmmword ptr [rbp + 0x5b0]
140343b43: 0f288dc0050000 movaps xmm1, xmmword ptr [rbp + 0x5c0]
140343b4a: 0f2895d0050000 movaps xmm2, xmmword ptr [rbp + 0x5d0]
140343b51: 0f299510010000 movaps xmmword ptr [rbp + 0x110], xmm2
140343b58: 0f298d00010000 movaps xmmword ptr [rbp + 0x100], xmm1
140343b5f: 0f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm0
140343b66: 48c7858001000000000000 mov qword ptr [rbp + 0x180], 0
140343b71: 48c7851002000000000000 mov qword ptr [rbp + 0x210], 0
140343b7c: 0f288520030000 movaps xmm0, xmmword ptr [rbp + 0x320]
140343b83: 0f288d30030000 movaps xmm1, xmmword ptr [rbp + 0x330]
140343b8a: 0f289540030000 movaps xmm2, xmmword ptr [rbp + 0x340]
140343b91: 0f298520010000 movaps xmmword ptr [rbp + 0x120], xmm0
140343b98: 0f298d30010000 movaps xmmword ptr [rbp + 0x130], xmm1
140343b9f: 0f299540010000 movaps xmmword ptr [rbp + 0x140], xmm2
140343ba6: 0f29bd50010000 movaps xmmword ptr [rbp + 0x150], xmm7
140343bad: 0f29b560010000 movaps xmmword ptr [rbp + 0x160], xmm6
140343bb4: 48898570010000 mov qword ptr [rbp + 0x170], rax
140343bbb: 48899578010000 mov qword ptr [rbp + 0x178], rdx
140343bc2: c685a002000000 mov byte ptr [rbp + 0x2a0], 0
140343bc9: e8729ba000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140343bce: 4889c7 mov rdi, rax
140343bd1: f60001 test byte ptr [rax], 1
140343bd4: 7411 je 0x140343be7
140343bd6: 4889f9 mov rcx, rdi
140343bd9: 4883c108 add rcx, 8
140343bdd: 488b4708 mov rax, qword ptr [rdi + 8]
140343be1: 488b5710 mov rdx, qword ptr [rdi + 0x10]
140343be5: eb1b jmp 0x140343c02 ; unknown
140343be7: e81412a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140343bec: 4889f9 mov rcx, rdi
140343bef: 4883c108 add rcx, 8
140343bf3: 48c70701000000 mov qword ptr [rdi], 1
140343bfa: 48894708 mov qword ptr [rdi + 8], rax
140343bfe: 48895710 mov qword ptr [rdi + 0x10], rdx
140343c02: 4c8d4001 lea r8, [rax + 1]
140343c06: 4c8901 mov qword ptr [rcx], r8
140343c09: 0f29b5c0020000 movaps xmmword ptr [rbp + 0x2c0], xmm6
140343c10: 0f29bdb0020000 movaps xmmword ptr [rbp + 0x2b0], xmm7
140343c17: 488985d0020000 mov qword ptr [rbp + 0x2d0], rax
140343c1e: 488995d8020000 mov qword ptr [rbp + 0x2d8], rdx
140343c25: e8169ba000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140343c2a: 4889c7 mov rdi, rax
140343c2d: f60001 test byte ptr [rax], 1
140343c30: 7411 je 0x140343c43
140343c32: 4889f9 mov rcx, rdi
140343c35: 4883c108 add rcx, 8
140343c39: 488b4708 mov rax, qword ptr [rdi + 8]
140343c3d: 488b5710 mov rdx, qword ptr [rdi + 0x10]
140343c41: eb1b jmp 0x140343c5e ; unknown
140343c43: e8b811a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140343c48: 4889f9 mov rcx, rdi
140343c4b: 4883c108 add rcx, 8
140343c4f: 48c70701000000 mov qword ptr [rdi], 1
140343c56: 48894708 mov qword ptr [rdi + 8], rax
140343c5a: 48895710 mov qword ptr [rdi + 0x10], rdx
140343c5e: 4c8d4001 lea r8, [rax + 1]
140343c62: 4c8901 mov qword ptr [rcx], r8
140343c65: 0f29b5c0050000 movaps xmmword ptr [rbp + 0x5c0], xmm6
140343c6c: 0f29bdb0050000 movaps xmmword ptr [rbp + 0x5b0], xmm7
140343c73: 488985d0050000 mov qword ptr [rbp + 0x5d0], rax
140343c7a: 488995d8050000 mov qword ptr [rbp + 0x5d8], rdx
140343c81: e8ba9aa000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140343c86: 4989c6 mov r14, rax
140343c89: f60001 test byte ptr [rax], 1
140343c8c: 7411 je 0x140343c9f
140343c8e: 4c89f0 mov rax, r14
140343c91: 4883c008 add rax, 8
140343c95: 498b7e08 mov rdi, qword ptr [r14 + 8]
140343c99: 498b5e10 mov rbx, qword ptr [r14 + 0x10]
140343c9d: eb21 jmp 0x140343cc0 ; unknown
140343c9f: e85c11a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140343ca4: 4889c7 mov rdi, rax
140343ca7: 4889d3 mov rbx, rdx
140343caa: 4c89f0 mov rax, r14
140343cad: 4883c008 add rax, 8
140343cb1: 49c70601000000 mov qword ptr [r14], 1
140343cb8: 49897e08 mov qword ptr [r14 + 8], rdi
140343cbc: 49895610 mov qword ptr [r14 + 0x10], rdx
140343cc0: 488d4f01 lea rcx, [rdi + 1]
140343cc4: 488908 mov qword ptr [rax], rcx
140343cc7: 0f57c0 xorps xmm0, xmm0
140343cca: 0f118580050000 movups xmmword ptr [rbp + 0x580], xmm0
140343cd1: 488d8d38030000 lea rcx, [rbp + 0x338]
140343cd8: 488d95f0000000 lea rdx, [rbp + 0xf0]
140343cdf: 41b8b8010000 mov r8d, 0x1b8
140343ce5: e816ff5d02 call 0x142923c00 ; unknown
140343cea: 0f2885b0020000 movaps xmm0, xmmword ptr [rbp + 0x2b0]
140343cf1: 0f288dc0020000 movaps xmm1, xmmword ptr [rbp + 0x2c0]
140343cf8: 0f2895d0020000 movaps xmm2, xmmword ptr [rbp + 0x2d0]
140343cff: 0f1185f0040000 movups xmmword ptr [rbp + 0x4f0], xmm0
140343d06: 0f118d00050000 movups xmmword ptr [rbp + 0x500], xmm1
140343d0d: 0f119510050000 movups xmmword ptr [rbp + 0x510], xmm2
140343d14: 0f2885b0050000 movaps xmm0, xmmword ptr [rbp + 0x5b0]
140343d1b: 0f288dc0050000 movaps xmm1, xmmword ptr [rbp + 0x5c0]
140343d22: 0f2895d0050000 movaps xmm2, xmmword ptr [rbp + 0x5d0]
140343d29: 0f118520050000 movups xmmword ptr [rbp + 0x520], xmm0
140343d30: 0f118d30050000 movups xmmword ptr [rbp + 0x530], xmm1
140343d37: 0f119540050000 movups xmmword ptr [rbp + 0x540], xmm2
140343d3e: 0f11bd50050000 movups xmmword ptr [rbp + 0x550], xmm7
140343d45: 0f11b560050000 movups xmmword ptr [rbp + 0x560], xmm6
140343d4c: 4889bd70050000 mov qword ptr [rbp + 0x570], rdi
140343d53: 48899d78050000 mov qword ptr [rbp + 0x578], rbx
140343d5a: 48c7859005000008000000 mov qword ptr [rbp + 0x590], 8
140343d65: 48c7859805000000000000 mov qword ptr [rbp + 0x598], 0
140343d70: 48c7852003000000000000 mov qword ptr [rbp + 0x320], 0
140343d7b: 48c7852803000008000000 mov qword ptr [rbp + 0x328], 8
140343d86: 48c7853003000000000000 mov qword ptr [rbp + 0x330], 0
140343d91: c685a205000000 mov byte ptr [rbp + 0x5a2], 0
140343d98: 66c785a00500000000 mov word ptr [rbp + 0x5a0], 0
140343da1: 488d9520030000 lea rdx, [rbp + 0x320]
140343da8: 4889f1 mov rcx, rsi
140343dab: e8a0e5deff call 0x140132350 ; bevy_app::app::App::insert_resource<system_decorator::backup::DecoratorBackup>
140343db0: 4889c6 mov rsi, rax
140343db3: e88899a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140343db8: 4889c7 mov rdi, rax
140343dbb: f60001 test byte ptr [rax], 1
140343dbe: 7411 je 0x140343dd1
140343dc0: 4889f9 mov rcx, rdi
140343dc3: 4883c108 add rcx, 8
140343dc7: 488b4708 mov rax, qword ptr [rdi + 8]
140343dcb: 488b5710 mov rdx, qword ptr [rdi + 0x10]
140343dcf: eb1b jmp 0x140343dec ; unknown
140343dd1: e82a10a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140343dd6: 4889f9 mov rcx, rdi
140343dd9: 4883c108 add rcx, 8
140343ddd: 48c70701000000 mov qword ptr [rdi], 1
140343de4: 48894708 mov qword ptr [rdi + 8], rax
140343de8: 48895710 mov qword ptr [rdi + 0x10], rdx
140343dec: 4c8d4001 lea r8, [rax + 1]
140343df0: 4c8901 mov qword ptr [rcx], r8
140343df3: 0f29b530030000 movaps xmmword ptr [rbp + 0x330], xmm6
140343dfa: 0f29bd20030000 movaps xmmword ptr [rbp + 0x320], xmm7
140343e01: 48898540030000 mov qword ptr [rbp + 0x340], rax
140343e08: 48899548030000 mov qword ptr [rbp + 0x348], rdx
140343e0f: 488d9520030000 lea rdx, [rbp + 0x320]
140343e16: 4889f1 mov rcx, rsi
140343e19: e8f20bdeff call 0x140124a10 ; bevy_app::app::App::insert_resource<country_core::resources::roofs::dormer_tiles::DormerTiles>
140343e1e: 4889c6 mov rsi, rax
140343e21: 488dbd20030000 lea rdi, [rbp + 0x320]
140343e28: 4889f9 mov rcx, rdi
140343e2b: e8307cdcff call 0x14010ba60 ; system_decorator::decorator_visual::deco_differential_state::impl$2::default<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams>
140343e30: 4889f1 mov rcx, rsi
140343e33: 4889fa mov rdx, rdi
140343e36: e8d5a2dfff call 0x14013e110 ; bevy_app::app::App::insert_resource<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::chimney_assemble_bricks::ChimneyAssemblyParams> >
140343e3b: 4889c6 mov rsi, rax
140343e3e: 4889f9 mov rcx, rdi
140343e41: e8fa7adcff call 0x14010b940 ; system_decorator::decorator_visual::deco_differential_state::impl$2::default<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams>
140343e46: 4889f1 mov rcx, rsi
140343e49: 4889fa mov rdx, rdi
140343e4c: e82f52dfff call 0x140139080 ; bevy_app::app::App::insert_resource<system_decorator::decorator_visual::deco_differential_state::WallAttachedDecoTracker<system_decorator::decorator_visual::door_stairs_assemble_bricks::DoorStairsAssemblyParams> >
140343e51: 4889c6 mov rsi, rax
140343e54: 48c7451000000000 mov qword ptr [rbp + 0x10], 0
140343e5c: 48c7451808000000 mov qword ptr [rbp + 0x18], 8
140343e64: 48c7452000000000 mov qword ptr [rbp + 0x20], 0
140343e6c: e8cf98a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140343e71: 4889c7 mov rdi, rax
140343e74: f60001 test byte ptr [rax], 1
140343e77: 7411 je 0x140343e8a
140343e79: 4889f9 mov rcx, rdi
140343e7c: 4883c108 add rcx, 8
140343e80: 488b4708 mov rax, qword ptr [rdi + 8]
140343e84: 488b5710 mov rdx, qword ptr [rdi + 0x10]
140343e88: eb1b jmp 0x140343ea5 ; unknown
140343e8a: e8710fa200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140343e8f: 4889f9 mov rcx, rdi
140343e92: 4883c108 add rcx, 8
140343e96: 48c70701000000 mov qword ptr [rdi], 1
140343e9d: 48894708 mov qword ptr [rdi + 8], rax
140343ea1: 48895710 mov qword ptr [rdi + 0x10], rdx
140343ea5: 4c8d4001 lea r8, [rax + 1]
140343ea9: 4c8901 mov qword ptr [rcx], r8
140343eac: 0f29b590000000 movaps xmmword ptr [rbp + 0x90], xmm6
140343eb3: 0f29bd80000000 movaps xmmword ptr [rbp + 0x80], xmm7
140343eba: 488985a0000000 mov qword ptr [rbp + 0xa0], rax
140343ec1: 488995a8000000 mov qword ptr [rbp + 0xa8], rdx
140343ec8: 488d8db0050000 lea rcx, [rbp + 0x5b0]
140343ecf: e86c02f0ff call 0x140244140 ; petgraph::graphmap::impl$19::default<country_core::resources::walls::decorator::DecoratorId,tuple$<>,enum2$<petgraph::Undirected>,std::hash::random::RandomState>
140343ed4: 488d8df0000000 lea rcx, [rbp + 0xf0]
140343edb: e86002f0ff call 0x140244140 ; petgraph::graphmap::impl$19::default<country_core::resources::walls::decorator::DecoratorId,tuple$<>,enum2$<petgraph::Undirected>,std::hash::random::RandomState>
140343ee0: e85b98a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140343ee5: 4889c7 mov rdi, rax
140343ee8: f60001 test byte ptr [rax], 1
140343eeb: 7411 je 0x140343efe
140343eed: 4889f9 mov rcx, rdi
140343ef0: 4883c108 add rcx, 8
140343ef4: 488b4708 mov rax, qword ptr [rdi + 8]
140343ef8: 488b5710 mov rdx, qword ptr [rdi + 0x10]
140343efc: eb1b jmp 0x140343f19 ; unknown
140343efe: e8fd0ea200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140343f03: 4889f9 mov rcx, rdi
140343f06: 4883c108 add rcx, 8
140343f0a: 48c70701000000 mov qword ptr [rdi], 1
140343f11: 48894708 mov qword ptr [rdi + 8], rax
140343f15: 48895710 mov qword ptr [rdi + 0x10], rdx
140343f19: 4c8d4001 lea r8, [rax + 1]
140343f1d: 4c8901 mov qword ptr [rcx], r8
140343f20: 0f29b5c0020000 movaps xmmword ptr [rbp + 0x2c0], xmm6
140343f27: 0f29bdb0020000 movaps xmmword ptr [rbp + 0x2b0], xmm7
140343f2e: 488985d0020000 mov qword ptr [rbp + 0x2d0], rax
140343f35: 488995d8020000 mov qword ptr [rbp + 0x2d8], rdx
140343f3c: e8ff97a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140343f41: 4989c6 mov r14, rax
140343f44: f60001 test byte ptr [rax], 1
140343f47: 7411 je 0x140343f5a
140343f49: 4c89f0 mov rax, r14
140343f4c: 4883c008 add rax, 8
140343f50: 498b7e08 mov rdi, qword ptr [r14 + 8]
140343f54: 498b5e10 mov rbx, qword ptr [r14 + 0x10]
140343f58: eb21 jmp 0x140343f7b ; unknown
140343f5a: e8a10ea200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140343f5f: 4889c7 mov rdi, rax
140343f62: 4889d3 mov rbx, rdx
140343f65: 4c89f0 mov rax, r14
140343f68: 4883c008 add rax, 8
140343f6c: 49c70601000000 mov qword ptr [r14], 1
140343f73: 49897e08 mov qword ptr [r14 + 8], rdi
140343f77: 49895610 mov qword ptr [r14 + 0x10], rdx
140343f7b: 488d4f01 lea rcx, [rdi + 1]
140343f7f: 488908 mov qword ptr [rax], rcx
140343f82: 488b4520 mov rax, qword ptr [rbp + 0x20]
140343f86: 48898530030000 mov qword ptr [rbp + 0x330], rax
140343f8d: 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
140343f91: 0f298520030000 movaps xmmword ptr [rbp + 0x320], xmm0
140343f98: 0f288580000000 movaps xmm0, xmmword ptr [rbp + 0x80]
140343f9f: 0f288d90000000 movaps xmm1, xmmword ptr [rbp + 0x90]
140343fa6: 0f2895a0000000 movaps xmm2, xmmword ptr [rbp + 0xa0]
140343fad: 0f118558040000 movups xmmword ptr [rbp + 0x458], xmm0
140343fb4: 0f118d68040000 movups xmmword ptr [rbp + 0x468], xmm1
140343fbb: 0f119578040000 movups xmmword ptr [rbp + 0x478], xmm2
140343fc2: 488d8d38030000 lea rcx, [rbp + 0x338]
140343fc9: 488d95b0050000 lea rdx, [rbp + 0x5b0]
140343fd0: 41b890000000 mov r8d, 0x90
140343fd6: e825fc5d02 call 0x142923c00 ; unknown
140343fdb: 488d8dc8030000 lea rcx, [rbp + 0x3c8]
140343fe2: 488d95f0000000 lea rdx, [rbp + 0xf0]
140343fe9: 41b890000000 mov r8d, 0x90
140343fef: e80cfc5d02 call 0x142923c00 ; unknown
140343ff4: 0f2885b0020000 movaps xmm0, xmmword ptr [rbp + 0x2b0]
140343ffb: 0f288dc0020000 movaps xmm1, xmmword ptr [rbp + 0x2c0]
140344002: 0f2895d0020000 movaps xmm2, xmmword ptr [rbp + 0x2d0]
140344009: 0f118588040000 movups xmmword ptr [rbp + 0x488], xmm0
140344010: 0f118d98040000 movups xmmword ptr [rbp + 0x498], xmm1
140344017: 0f1195a8040000 movups xmmword ptr [rbp + 0x4a8], xmm2
14034401e: 0f11bdb8040000 movups xmmword ptr [rbp + 0x4b8], xmm7
140344025: 0f11b5c8040000 movups xmmword ptr [rbp + 0x4c8], xmm6
14034402c: 4889bdd8040000 mov qword ptr [rbp + 0x4d8], rdi
140344033: 48899de0040000 mov qword ptr [rbp + 0x4e0], rbx
14034403a: 488d9520030000 lea rdx, [rbp + 0x320]
140344041: 4889f1 mov rcx, rsi
140344044: e87785ddff call 0x14011c5c0 ; bevy_app::app::App::insert_resource<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyRequests>
140344049: 48c7852003000000000000 mov qword ptr [rbp + 0x320], 0
140344054: 48c7852803000008000000 mov qword ptr [rbp + 0x328], 8
14034405f: 48c7853003000000000000 mov qword ptr [rbp + 0x330], 0
14034406a: 488d9520030000 lea rdx, [rbp + 0x320]
140344071: 4889c1 mov rcx, rax
140344074: e8272addff call 0x140116aa0 ; bevy_app::app::App::insert_resource<country_core::resources::stairs_walls::StairsArchSupportRequests>
140344079: 48c7852003000000000000 mov qword ptr [rbp + 0x320], 0
140344084: 48c7852803000008000000 mov qword ptr [rbp + 0x328], 8
14034408f: 48c7853003000000000000 mov qword ptr [rbp + 0x330], 0
14034409a: 488d9520030000 lea rdx, [rbp + 0x320]
1403440a1: 4889c1 mov rcx, rax
1403440a4: e86715deff call 0x140125610 ; bevy_app::app::App::insert_resource<country_core::resources::stairs_walls::StairsBracketsRequests>
1403440a9: 4889c6 mov rsi, rax
1403440ac: e88f96a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1403440b1: 4889c7 mov rdi, rax
1403440b4: f60001 test byte ptr [rax], 1
1403440b7: 7411 je 0x1403440ca
1403440b9: 4889f9 mov rcx, rdi
1403440bc: 4883c108 add rcx, 8
1403440c0: 488b4708 mov rax, qword ptr [rdi + 8]
1403440c4: 488b5710 mov rdx, qword ptr [rdi + 0x10]
1403440c8: eb1b jmp 0x1403440e5 ; unknown
1403440ca: e8310da200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1403440cf: 4889f9 mov rcx, rdi
1403440d2: 4883c108 add rcx, 8
1403440d6: 48c70701000000 mov qword ptr [rdi], 1
1403440dd: 48894708 mov qword ptr [rdi + 8], rax
1403440e1: 48895710 mov qword ptr [rdi + 0x10], rdx
1403440e5: 4c8d4001 lea r8, [rax + 1]
1403440e9: 4c8901 mov qword ptr [rcx], r8
1403440ec: 0f29b500010000 movaps xmmword ptr [rbp + 0x100], xmm6
1403440f3: 0f29bdf0000000 movaps xmmword ptr [rbp + 0xf0], xmm7
1403440fa: 48898510010000 mov qword ptr [rbp + 0x110], rax
140344101: 48899518010000 mov qword ptr [rbp + 0x118], rdx
140344108: e83396a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
14034410d: 4889c7 mov rdi, rax
140344110: f60001 test byte ptr [rax], 1
140344113: 7411 je 0x140344126
140344115: 4889f9 mov rcx, rdi
140344118: 4883c108 add rcx, 8
14034411c: 488b4708 mov rax, qword ptr [rdi + 8]
140344120: 488b5710 mov rdx, qword ptr [rdi + 0x10]
140344124: eb1b jmp 0x140344141 ; unknown
140344126: e8d50ca200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
14034412b: 4889f9 mov rcx, rdi
14034412e: 4883c108 add rcx, 8
140344132: 48c70701000000 mov qword ptr [rdi], 1
140344139: 48894708 mov qword ptr [rdi + 8], rax
14034413d: 48895710 mov qword ptr [rdi + 0x10], rdx
140344141: 4c8d4001 lea r8, [rax + 1]
140344145: 4c8901 mov qword ptr [rcx], r8
140344148: 0f2885f0000000 movaps xmm0, xmmword ptr [rbp + 0xf0]
14034414f: 0f288d00010000 movaps xmm1, xmmword ptr [rbp + 0x100]
140344156: 0f289510010000 movaps xmm2, xmmword ptr [rbp + 0x110]
14034415d: 0f299540030000 movaps xmmword ptr [rbp + 0x340], xmm2
140344164: 0f298d30030000 movaps xmmword ptr [rbp + 0x330], xmm1
14034416b: 0f298520030000 movaps xmmword ptr [rbp + 0x320], xmm0
140344172: 0f29bd50030000 movaps xmmword ptr [rbp + 0x350], xmm7
140344179: 0f29b560030000 movaps xmmword ptr [rbp + 0x360], xmm6
140344180: 48898570030000 mov qword ptr [rbp + 0x370], rax
140344187: 48899578030000 mov qword ptr [rbp + 0x378], rdx
14034418e: 488d9520030000 lea rdx, [rbp + 0x320]
140344195: 4889f1 mov rcx, rsi
140344198: e8531adeff call 0x140125bf0 ; bevy_app::app::App::insert_resource<country_core::resources::stairs_walls::StairsPillarProposals>
14034419d: 4889c6 mov rsi, rax
1403441a0: e89b95a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1403441a5: 4889c7 mov rdi, rax
1403441a8: f60001 test byte ptr [rax], 1
1403441ab: 7411 je 0x1403441be
1403441ad: 4889f9 mov rcx, rdi
1403441b0: 4883c108 add rcx, 8
1403441b4: 488b4708 mov rax, qword ptr [rdi + 8]
1403441b8: 488b5710 mov rdx, qword ptr [rdi + 0x10]
1403441bc: eb1b jmp 0x1403441d9 ; unknown
1403441be: e83d0ca200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1403441c3: 4889f9 mov rcx, rdi
1403441c6: 4883c108 add rcx, 8
1403441ca: 48c70701000000 mov qword ptr [rdi], 1
1403441d1: 48894708 mov qword ptr [rdi + 8], rax
1403441d5: 48895710 mov qword ptr [rdi + 0x10], rdx
1403441d9: 4c8d4001 lea r8, [rax + 1]
1403441dd: 4c8901 mov qword ptr [rcx], r8
1403441e0: 0f29b530030000 movaps xmmword ptr [rbp + 0x330], xmm6
1403441e7: 0f29bd20030000 movaps xmmword ptr [rbp + 0x320], xmm7
1403441ee: 48898540030000 mov qword ptr [rbp + 0x340], rax
1403441f5: 48899548030000 mov qword ptr [rbp + 0x348], rdx
1403441fc: 488d9520030000 lea rdx, [rbp + 0x320]
140344203: 4889f1 mov rcx, rsi
140344206: e8d56adfff call 0x14013ace0 ; bevy_app::app::App::insert_resource<country_core::resources::window_lights::GlobalSmokeControl>
14034420b: 4889c7 mov rdi, rax
14034420e: 488d8db0050000 lea rcx, [rbp + 0x5b0]
140344215: e826ffefff call 0x140244140 ; petgraph::graphmap::impl$19::default<country_core::resources::walls::decorator::DecoratorId,tuple$<>,enum2$<petgraph::Undirected>,std::hash::random::RandomState>
14034421a: 488d8df0000000 lea rcx, [rbp + 0xf0]
140344221: e81affefff call 0x140244140 ; petgraph::graphmap::impl$19::default<country_core::resources::walls::decorator::DecoratorId,tuple$<>,enum2$<petgraph::Undirected>,std::hash::random::RandomState>
140344226: 31c9 xor ecx, ecx
140344228: e8f358d9ff call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > 
14034422d: 4885c0 test rax, rax
140344230: 0f841d120000 je 0x140345453
140344236: 488d8d48030000 lea rcx, [rbp + 0x348]
14034423d: 488d9dd8030000 lea rbx, [rbp + 0x3d8]
140344244: 440f1020 movups xmm12, xmmword ptr [rax]
140344248: 48ff00 inc qword ptr [rax]
14034424b: 488db5b0050000 lea rsi, [rbp + 0x5b0]
140344252: 41b890000000 mov r8d, 0x90
140344258: 4889f2 mov rdx, rsi
14034425b: e8a0f95d02 call 0x142923c00 ; unknown
140344260: 488d95f0000000 lea rdx, [rbp + 0xf0]
140344267: 41b890000000 mov r8d, 0x90
14034426d: 4889d9 mov rcx, rbx
140344270: e88bf95d02 call 0x142923c00 ; unknown
140344275: c6859804000000 mov byte ptr [rbp + 0x498], 0
14034427c: 48c7852003000000000000 mov qword ptr [rbp + 0x320], 0
140344287: 48c7853003000003000000 mov qword ptr [rbp + 0x330], 3
140344292: 440f118d68040000 movups xmmword ptr [rbp + 0x468], xmm9
14034429a: 440f118578040000 movups xmmword ptr [rbp + 0x478], xmm8
1403442a2: 440f11a588040000 movups xmmword ptr [rbp + 0x488], xmm12
1403442aa: 488d9520030000 lea rdx, [rbp + 0x320]
1403442b1: 4889f9 mov rcx, rdi
1403442b4: e8976cddff call 0x14011af50 ; bevy_app::app::App::insert_resource<country_core::resources::stairs::stairs_state::StairsState>
1403442b9: 4889c7 mov rdi, rax
1403442bc: e87f94a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1403442c1: 4889c3 mov rbx, rax
1403442c4: f60001 test byte ptr [rax], 1
1403442c7: 7411 je 0x1403442da
1403442c9: 4889d9 mov rcx, rbx
1403442cc: 4883c108 add rcx, 8
1403442d0: 488b4308 mov rax, qword ptr [rbx + 8]
1403442d4: 488b5310 mov rdx, qword ptr [rbx + 0x10]
1403442d8: eb1b jmp 0x1403442f5 ; unknown
1403442da: e8210ba200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1403442df: 4889d9 mov rcx, rbx
1403442e2: 4883c108 add rcx, 8
1403442e6: 48c70301000000 mov qword ptr [rbx], 1
1403442ed: 48894308 mov qword ptr [rbx + 8], rax
1403442f1: 48895310 mov qword ptr [rbx + 0x10], rdx
1403442f5: 4c8d4001 lea r8, [rax + 1]
1403442f9: 4c8901 mov qword ptr [rcx], r8
1403442fc: 0f29b530030000 movaps xmmword ptr [rbp + 0x330], xmm6
140344303: 0f29bd20030000 movaps xmmword ptr [rbp + 0x320], xmm7
14034430a: 48898540030000 mov qword ptr [rbp + 0x340], rax
140344311: 48899548030000 mov qword ptr [rbp + 0x348], rdx
140344318: 488d9520030000 lea rdx, [rbp + 0x320]
14034431f: 4889f9 mov rcx, rdi
140344322: e8a96fdfff call 0x14013b2d0 ; bevy_app::app::App::insert_resource<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator>
140344327: 4889c7 mov rdi, rax
14034432a: e81194a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
14034432f: 4889c3 mov rbx, rax
140344332: f60001 test byte ptr [rax], 1
140344335: 7411 je 0x140344348
140344337: 4889d9 mov rcx, rbx
14034433a: 4883c108 add rcx, 8
14034433e: 488b4308 mov rax, qword ptr [rbx + 8]
140344342: 488b5310 mov rdx, qword ptr [rbx + 0x10]
140344346: eb1b jmp 0x140344363 ; unknown
140344348: e8b30aa200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
14034434d: 4889d9 mov rcx, rbx
140344350: 4883c108 add rcx, 8
140344354: 48c70301000000 mov qword ptr [rbx], 1
14034435b: 48894308 mov qword ptr [rbx + 8], rax
14034435f: 48895310 mov qword ptr [rbx + 0x10], rdx
140344363: 4c8d4001 lea r8, [rax + 1]
140344367: 4c8901 mov qword ptr [rcx], r8
14034436a: 0f29b530030000 movaps xmmword ptr [rbp + 0x330], xmm6
140344371: 0f29bd20030000 movaps xmmword ptr [rbp + 0x320], xmm7
140344378: 48898540030000 mov qword ptr [rbp + 0x340], rax
14034437f: 48899548030000 mov qword ptr [rbp + 0x348], rdx
140344386: c6855003000000 mov byte ptr [rbp + 0x350], 0
14034438d: 488d9520030000 lea rdx, [rbp + 0x320]
140344394: 4889f9 mov rcx, rdi
140344397: e854cbddff call 0x140120ef0 ; bevy_app::app::App::insert_resource<country_core::resources::stairs::visual::StairsVisualState>
14034439c: 4889c7 mov rdi, rax
14034439f: 31c9 xor ecx, ecx
1403443a1: e87a57d9ff call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > 
1403443a6: 4885c0 test rax, rax
1403443a9: 0f8413100000 je 0x1403453c2
1403443af: 0f1000 movups xmm0, xmmword ptr [rax]
1403443b2: 48ff00 inc qword ptr [rax]
1403443b5: 440f2995c0020000 movaps xmmword ptr [rbp + 0x2c0], xmm10
1403443bd: 440f299db0020000 movaps xmmword ptr [rbp + 0x2b0], xmm11
1403443c5: 0f2985d0020000 movaps xmmword ptr [rbp + 0x2d0], xmm0
1403443cc: 31c9 xor ecx, ecx
1403443ce: e84d57d9ff call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > 
1403443d3: 4885c0 test rax, rax
1403443d6: 0f8485100000 je 0x140345461
1403443dc: 0f1000 movups xmm0, xmmword ptr [rax]
1403443df: 48ff00 inc qword ptr [rax]
1403443e2: 440f2995c0050000 movaps xmmword ptr [rbp + 0x5c0], xmm10
1403443ea: 440f299db0050000 movaps xmmword ptr [rbp + 0x5b0], xmm11
1403443f2: 0f2985d0050000 movaps xmmword ptr [rbp + 0x5d0], xmm0
1403443f9: 31c9 xor ecx, ecx
1403443fb: e82057d9ff call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > 
140344400: 4885c0 test rax, rax
140344403: 0f8466100000 je 0x14034546f
140344409: 0f1000 movups xmm0, xmmword ptr [rax]
14034440c: 48ff00 inc qword ptr [rax]
14034440f: 440f299500010000 movaps xmmword ptr [rbp + 0x100], xmm10
140344417: 440f299df0000000 movaps xmmword ptr [rbp + 0xf0], xmm11
14034441f: 0f298510010000 movaps xmmword ptr [rbp + 0x110], xmm0
140344426: 31c9 xor ecx, ecx
140344428: e8f356d9ff call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > 
14034442d: 4885c0 test rax, rax
140344430: 0f8447100000 je 0x14034547d
140344436: 0f1000 movups xmm0, xmmword ptr [rax]
140344439: 48ff00 inc qword ptr [rax]
14034443c: 0f288db0020000 movaps xmm1, xmmword ptr [rbp + 0x2b0]
140344443: 0f2895c0020000 movaps xmm2, xmmword ptr [rbp + 0x2c0]
14034444a: 0f289dd0020000 movaps xmm3, xmmword ptr [rbp + 0x2d0]
140344451: 0f299d40030000 movaps xmmword ptr [rbp + 0x340], xmm3
140344458: 0f299530030000 movaps xmmword ptr [rbp + 0x330], xmm2
14034445f: 0f298d20030000 movaps xmmword ptr [rbp + 0x320], xmm1
140344466: 0f288db0050000 movaps xmm1, xmmword ptr [rbp + 0x5b0]
14034446d: 0f2895c0050000 movaps xmm2, xmmword ptr [rbp + 0x5c0]
140344474: 0f289dd0050000 movaps xmm3, xmmword ptr [rbp + 0x5d0]
14034447b: 0f298d50030000 movaps xmmword ptr [rbp + 0x350], xmm1
140344482: 0f299560030000 movaps xmmword ptr [rbp + 0x360], xmm2
140344489: 0f299d70030000 movaps xmmword ptr [rbp + 0x370], xmm3
140344490: 0f288df0000000 movaps xmm1, xmmword ptr [rbp + 0xf0]
140344497: 0f289500010000 movaps xmm2, xmmword ptr [rbp + 0x100]
14034449e: 0f289d10010000 movaps xmm3, xmmword ptr [rbp + 0x110]
1403444a5: 0f298d80030000 movaps xmmword ptr [rbp + 0x380], xmm1
1403444ac: 0f299590030000 movaps xmmword ptr [rbp + 0x390], xmm2
1403444b3: 0f299da0030000 movaps xmmword ptr [rbp + 0x3a0], xmm3
1403444ba: 440f2995c0030000 movaps xmmword ptr [rbp + 0x3c0], xmm10
1403444c2: 440f299db0030000 movaps xmmword ptr [rbp + 0x3b0], xmm11
1403444ca: 0f2985d0030000 movaps xmmword ptr [rbp + 0x3d0], xmm0
1403444d1: 488d9520030000 lea rdx, [rbp + 0x320]
1403444d8: 4889f9 mov rcx, rdi
1403444db: e8305cdeff call 0x14012a110 ; bevy_app::app::App::insert_resource<country_core::resources::stairs::stairs_collider::StairColliderRegistry>
1403444e0: 4889c7 mov rdi, rax
1403444e3: 31c9 xor ecx, ecx
1403444e5: e83656d9ff call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > 
1403444ea: 4885c0 test rax, rax
1403444ed: 0f84cf0e0000 je 0x1403453c2
1403444f3: 0f1000 movups xmm0, xmmword ptr [rax]
1403444f6: 48ff00 inc qword ptr [rax]
1403444f9: 0f298540030000 movaps xmmword ptr [rbp + 0x340], xmm0
140344500: 440f298d20030000 movaps xmmword ptr [rbp + 0x320], xmm9
140344508: 440f298530030000 movaps xmmword ptr [rbp + 0x330], xmm8
140344510: 488d9520030000 lea rdx, [rbp + 0x320]
140344517: 4889f9 mov rcx, rdi
14034451a: e8e1addfff call 0x14013f300 ; bevy_app::app::App::insert_resource<country_core::resources::stairs::stairs_removed_supports::StairsRemovedSupports>
14034451f: 4889c7 mov rdi, rax
140344522: e81992a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140344527: 4889c3 mov rbx, rax
14034452a: f60001 test byte ptr [rax], 1
14034452d: 7411 je 0x140344540
14034452f: 4889d9 mov rcx, rbx
140344532: 4883c108 add rcx, 8
140344536: 488b4308 mov rax, qword ptr [rbx + 8]
14034453a: 488b5310 mov rdx, qword ptr [rbx + 0x10]
14034453e: eb1b jmp 0x14034455b ; unknown
140344540: e8bb08a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140344545: 4889d9 mov rcx, rbx
140344548: 4883c108 add rcx, 8
14034454c: 48c70301000000 mov qword ptr [rbx], 1
140344553: 48894308 mov qword ptr [rbx + 8], rax
140344557: 48895310 mov qword ptr [rbx + 0x10], rdx
14034455b: 4c8d4001 lea r8, [rax + 1]
14034455f: 4c8901 mov qword ptr [rcx], r8
140344562: 0f297520 movaps xmmword ptr [rbp + 0x20], xmm6
140344566: 0f297d10 movaps xmmword ptr [rbp + 0x10], xmm7
14034456a: 48894530 mov qword ptr [rbp + 0x30], rax
14034456e: 48895538 mov qword ptr [rbp + 0x38], rdx
140344572: e8c991a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140344577: 4889c3 mov rbx, rax
14034457a: f60001 test byte ptr [rax], 1
14034457d: 7411 je 0x140344590
14034457f: 4889d9 mov rcx, rbx
140344582: 4883c108 add rcx, 8
140344586: 488b4308 mov rax, qword ptr [rbx + 8]
14034458a: 488b5310 mov rdx, qword ptr [rbx + 0x10]
14034458e: eb1b jmp 0x1403445ab ; unknown
140344590: e86b08a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140344595: 4889d9 mov rcx, rbx
140344598: 4883c108 add rcx, 8
14034459c: 48c70301000000 mov qword ptr [rbx], 1
1403445a3: 48894308 mov qword ptr [rbx + 8], rax
1403445a7: 48895310 mov qword ptr [rbx + 0x10], rdx
1403445ab: 4c8d4001 lea r8, [rax + 1]
1403445af: 4c8901 mov qword ptr [rcx], r8
1403445b2: 0f29b590000000 movaps xmmword ptr [rbp + 0x90], xmm6
1403445b9: 0f29bd80000000 movaps xmmword ptr [rbp + 0x80], xmm7
1403445c0: 488985a0000000 mov qword ptr [rbp + 0xa0], rax
1403445c7: 488995a8000000 mov qword ptr [rbp + 0xa8], rdx
1403445ce: e86d91a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1403445d3: 4889c3 mov rbx, rax
1403445d6: f60001 test byte ptr [rax], 1
1403445d9: 7411 je 0x1403445ec
1403445db: 4889d9 mov rcx, rbx
1403445de: 4883c108 add rcx, 8
1403445e2: 488b4308 mov rax, qword ptr [rbx + 8]
1403445e6: 488b5310 mov rdx, qword ptr [rbx + 0x10]
1403445ea: eb1b jmp 0x140344607 ; unknown
1403445ec: e80f08a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1403445f1: 4889d9 mov rcx, rbx
1403445f4: 4883c108 add rcx, 8
1403445f8: 48c70301000000 mov qword ptr [rbx], 1
1403445ff: 48894308 mov qword ptr [rbx + 8], rax
140344603: 48895310 mov qword ptr [rbx + 0x10], rdx
140344607: 4c8d4001 lea r8, [rax + 1]
14034460b: 4c8901 mov qword ptr [rcx], r8
14034460e: 0f29b5c0020000 movaps xmmword ptr [rbp + 0x2c0], xmm6
140344615: 0f29bdb0020000 movaps xmmword ptr [rbp + 0x2b0], xmm7
14034461c: 488985d0020000 mov qword ptr [rbp + 0x2d0], rax
140344623: 488995d8020000 mov qword ptr [rbp + 0x2d8], rdx
14034462a: e81191a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
14034462f: 4889c3 mov rbx, rax
140344632: f60001 test byte ptr [rax], 1
140344635: 7411 je 0x140344648
140344637: 4889d9 mov rcx, rbx
14034463a: 4883c108 add rcx, 8
14034463e: 488b4308 mov rax, qword ptr [rbx + 8]
140344642: 488b5310 mov rdx, qword ptr [rbx + 0x10]
140344646: eb1b jmp 0x140344663 ; unknown
140344648: e8b307a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
14034464d: 4889d9 mov rcx, rbx
140344650: 4883c108 add rcx, 8
140344654: 48c70301000000 mov qword ptr [rbx], 1
14034465b: 48894308 mov qword ptr [rbx + 8], rax
14034465f: 48895310 mov qword ptr [rbx + 0x10], rdx
140344663: 4c8d4001 lea r8, [rax + 1]
140344667: 4c8901 mov qword ptr [rcx], r8
14034466a: 0f29b5c0050000 movaps xmmword ptr [rbp + 0x5c0], xmm6
140344671: 0f29bdb0050000 movaps xmmword ptr [rbp + 0x5b0], xmm7
140344678: 488985d0050000 mov qword ptr [rbp + 0x5d0], rax
14034467f: 488995d8050000 mov qword ptr [rbp + 0x5d8], rdx
140344686: 0f57c0 xorps xmm0, xmm0
140344689: 0f1185f8000000 movups xmmword ptr [rbp + 0xf8], xmm0
140344690: 0f118508010000 movups xmmword ptr [rbp + 0x108], xmm0
140344697: 48c7851801000000000000 mov qword ptr [rbp + 0x118], 0
1403446a2: 0f1145d0 movups xmmword ptr [rbp - 0x30], xmm0
1403446a6: 0f1145bc movups xmmword ptr [rbp - 0x44], xmm0
1403446aa: 0f1145a8 movups xmmword ptr [rbp - 0x58], xmm0
1403446ae: 0f1145e4 movups xmmword ptr [rbp - 0x1c], xmm0
1403446b2: e88990a000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1403446b7: 4889c3 mov rbx, rax
1403446ba: f60001 test byte ptr [rax], 1
1403446bd: 7411 je 0x1403446d0
1403446bf: 4889d9 mov rcx, rbx
1403446c2: 4883c108 add rcx, 8
1403446c6: 488b4308 mov rax, qword ptr [rbx + 8]
1403446ca: 488b5310 mov rdx, qword ptr [rbx + 0x10]
1403446ce: eb1b jmp 0x1403446eb ; unknown
1403446d0: e82b07a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1403446d5: 4889d9 mov rcx, rbx
1403446d8: 4883c108 add rcx, 8
1403446dc: 48c70301000000 mov qword ptr [rbx], 1
1403446e3: 48894308 mov qword ptr [rbx + 8], rax
1403446e7: 48895310 mov qword ptr [rbx + 0x10], rdx
1403446eb: 4c8d4001 lea r8, [rax + 1]
1403446ef: 4c8901 mov qword ptr [rcx], r8
1403446f2: 0f284510 movaps xmm0, xmmword ptr [rbp + 0x10]
1403446f6: 0f284d20 movaps xmm1, xmmword ptr [rbp + 0x20]
1403446fa: 0f285530 movaps xmm2, xmmword ptr [rbp + 0x30]
1403446fe: 0f1195e0030000 movups xmmword ptr [rbp + 0x3e0], xmm2
140344705: 0f118dd0030000 movups xmmword ptr [rbp + 0x3d0], xmm1
14034470c: 0f1185c0030000 movups xmmword ptr [rbp + 0x3c0], xmm0
140344713: 0f288580000000 movaps xmm0, xmmword ptr [rbp + 0x80]
14034471a: 0f288d90000000 movaps xmm1, xmmword ptr [rbp + 0x90]
140344721: 0f2895a0000000 movaps xmm2, xmmword ptr [rbp + 0xa0]
140344728: 0f1185f0030000 movups xmmword ptr [rbp + 0x3f0], xmm0
14034472f: 0f118d00040000 movups xmmword ptr [rbp + 0x400], xmm1
140344736: 0f119510040000 movups xmmword ptr [rbp + 0x410], xmm2
14034473d: 0f2885b0020000 movaps xmm0, xmmword ptr [rbp + 0x2b0]
140344744: 0f288dc0020000 movaps xmm1, xmmword ptr [rbp + 0x2c0]
14034474b: 0f2895d0020000 movaps xmm2, xmmword ptr [rbp + 0x2d0]
140344752: 0f118520040000 movups xmmword ptr [rbp + 0x420], xmm0
140344759: 0f118d30040000 movups xmmword ptr [rbp + 0x430], xmm1
140344760: 0f119540040000 movups xmmword ptr [rbp + 0x440], xmm2
140344767: 0f2885b0050000 movaps xmm0, xmmword ptr [rbp + 0x5b0]
14034476e: 0f288dc0050000 movaps xmm1, xmmword ptr [rbp + 0x5c0]
140344775: 0f2895d0050000 movaps xmm2, xmmword ptr [rbp + 0x5d0]
14034477c: 0f119570040000 movups xmmword ptr [rbp + 0x470], xmm2
140344783: 0f118d60040000 movups xmmword ptr [rbp + 0x460], xmm1
14034478a: 0f118550040000 movups xmmword ptr [rbp + 0x450], xmm0
140344791: c7852003000000000000 mov dword ptr [rbp + 0x320], 0
14034479b: 0f1085f0000000 movups xmm0, xmmword ptr [rbp + 0xf0]
1403447a2: 0f108d00010000 movups xmm1, xmmword ptr [rbp + 0x100]
1403447a9: 0f109510010000 movups xmm2, xmmword ptr [rbp + 0x110]
1403447b0: 0f118524030000 movups xmmword ptr [rbp + 0x324], xmm0
1403447b7: 0f118d34030000 movups xmmword ptr [rbp + 0x334], xmm1
1403447be: 0f119544030000 movups xmmword ptr [rbp + 0x344], xmm2
1403447c5: c7855403000000000000 mov dword ptr [rbp + 0x354], 0
1403447cf: 48c7855c03000000000000 mov qword ptr [rbp + 0x35c], 0
1403447da: 0f1045cc movups xmm0, xmmword ptr [rbp - 0x34]
1403447de: 0f118564030000 movups xmmword ptr [rbp + 0x364], xmm0
1403447e5: 8b4ddc mov ecx, dword ptr [rbp - 0x24]
1403447e8: 898d74030000 mov dword ptr [rbp + 0x374], ecx
1403447ee: c7857803000000000000 mov dword ptr [rbp + 0x378], 0
1403447f8: 8b4dc8 mov ecx, dword ptr [rbp - 0x38]
1403447fb: 898d8c030000 mov dword ptr [rbp + 0x38c], ecx
140344801: 0f1045b8 movups xmm0, xmmword ptr [rbp - 0x48]
140344805: 0f11857c030000 movups xmmword ptr [rbp + 0x37c], xmm0
14034480c: c7859003000000000000 mov dword ptr [rbp + 0x390], 0
140344816: 8b4db4 mov ecx, dword ptr [rbp - 0x4c]
140344819: 898da4030000 mov dword ptr [rbp + 0x3a4], ecx
14034481f: 0f1045a4 movups xmm0, xmmword ptr [rbp - 0x5c]
140344823: 0f118594030000 movups xmmword ptr [rbp + 0x394], xmm0
14034482a: c785a803000000000000 mov dword ptr [rbp + 0x3a8], 0
140344834: 8b4df0 mov ecx, dword ptr [rbp - 0x10]
140344837: 898dbc030000 mov dword ptr [rbp + 0x3bc], ecx
14034483d: 0f1045e0 movups xmm0, xmmword ptr [rbp - 0x20]
140344841: 0f1185ac030000 movups xmmword ptr [rbp + 0x3ac], xmm0
140344848: 0f11b590040000 movups xmmword ptr [rbp + 0x490], xmm6
14034484f: 0f11bd80040000 movups xmmword ptr [rbp + 0x480], xmm7
140344856: 488985a0040000 mov qword ptr [rbp + 0x4a0], rax
14034485d: 488995a8040000 mov qword ptr [rbp + 0x4a8], rdx
140344864: 488d9520030000 lea rdx, [rbp + 0x320]
14034486b: 4889f9 mov rcx, rdi
14034486e: e8ddd7ddff call 0x140122050 ; bevy_app::app::App::insert_resource<system_decorator::ui_systems::ui_stairs_viz::StairGizmosCache>
140344873: 4889c7 mov rdi, rax
140344876: 488d8df0000000 lea rcx, [rbp + 0xf0]
14034487d: e84efaefff call 0x1402442d0 ; petgraph::graphmap::impl$19::default<country_core::systems::collision::world::SubworldColliderHandle,country_core::resources::stairs::structural_graph::StructuralGraphConnection,enum2$<petgraph::Undirected>,std::hash::ra
140344882: e8b98ea000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140344887: 4889c3 mov rbx, rax
14034488a: f60001 test byte ptr [rax], 1
14034488d: 7411 je 0x1403448a0
14034488f: 4889d9 mov rcx, rbx
140344892: 4883c108 add rcx, 8
140344896: 488b4308 mov rax, qword ptr [rbx + 8]
14034489a: 488b5310 mov rdx, qword ptr [rbx + 0x10]
14034489e: eb1b jmp 0x1403448bb ; unknown
1403448a0: e85b05a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1403448a5: 4889d9 mov rcx, rbx
1403448a8: 4883c108 add rcx, 8
1403448ac: 48c70301000000 mov qword ptr [rbx], 1
1403448b3: 48894308 mov qword ptr [rbx + 8], rax
1403448b7: 48895310 mov qword ptr [rbx + 0x10], rdx
1403448bb: 4c8d4001 lea r8, [rax + 1]
1403448bf: 4c8901 mov qword ptr [rcx], r8
1403448c2: 0f2975f0 movaps xmmword ptr [rbp - 0x10], xmm6
1403448c6: 0f297de0 movaps xmmword ptr [rbp - 0x20], xmm7
1403448ca: 48894500 mov qword ptr [rbp], rax
1403448ce: 48895508 mov qword ptr [rbp + 8], rdx
1403448d2: e8698ea000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
1403448d7: 4889c3 mov rbx, rax
1403448da: f60001 test byte ptr [rax], 1
1403448dd: 7411 je 0x1403448f0
1403448df: 4889d9 mov rcx, rbx
1403448e2: 4883c108 add rcx, 8
1403448e6: 488b4308 mov rax, qword ptr [rbx + 8]
1403448ea: 488b5310 mov rdx, qword ptr [rbx + 0x10]
1403448ee: eb1b jmp 0x14034490b ; unknown
1403448f0: e80b05a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1403448f5: 4889d9 mov rcx, rbx
1403448f8: 4883c108 add rcx, 8
1403448fc: 48c70301000000 mov qword ptr [rbx], 1
140344903: 48894308 mov qword ptr [rbx + 8], rax
140344907: 48895310 mov qword ptr [rbx + 0x10], rdx
14034490b: 4c8d4001 lea r8, [rax + 1]
14034490f: 4c8901 mov qword ptr [rcx], r8
140344912: 0f297520 movaps xmmword ptr [rbp + 0x20], xmm6
140344916: 0f297d10 movaps xmmword ptr [rbp + 0x10], xmm7
14034491a: 48894530 mov qword ptr [rbp + 0x30], rax
14034491e: 48895538 mov qword ptr [rbp + 0x38], rdx
140344922: e8198ea000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140344927: 4889c3 mov rbx, rax
14034492a: f60001 test byte ptr [rax], 1
14034492d: 7411 je 0x140344940
14034492f: 4889d9 mov rcx, rbx
140344932: 4883c108 add rcx, 8
140344936: 488b4308 mov rax, qword ptr [rbx + 8]
14034493a: 488b5310 mov rdx, qword ptr [rbx + 0x10]
14034493e: eb1b jmp 0x14034495b ; unknown
140344940: e8bb04a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140344945: 4889d9 mov rcx, rbx
140344948: 4883c108 add rcx, 8
14034494c: 48c70301000000 mov qword ptr [rbx], 1
140344953: 48894308 mov qword ptr [rbx + 8], rax
140344957: 48895310 mov qword ptr [rbx + 0x10], rdx
14034495b: 4c8d4001 lea r8, [rax + 1]
14034495f: 4c8901 mov qword ptr [rcx], r8
140344962: 0f29b590000000 movaps xmmword ptr [rbp + 0x90], xmm6
140344969: 0f29bd80000000 movaps xmmword ptr [rbp + 0x80], xmm7
140344970: 488985a0000000 mov qword ptr [rbp + 0xa0], rax
140344977: 488995a8000000 mov qword ptr [rbp + 0xa8], rdx
14034497e: e8bd8da000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140344983: 4889c3 mov rbx, rax
140344986: f60001 test byte ptr [rax], 1
140344989: 7411 je 0x14034499c
14034498b: 4889d9 mov rcx, rbx
14034498e: 4883c108 add rcx, 8
140344992: 488b4308 mov rax, qword ptr [rbx + 8]
140344996: 488b5310 mov rdx, qword ptr [rbx + 0x10]
14034499a: eb1b jmp 0x1403449b7 ; unknown
14034499c: e85f04a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
1403449a1: 4889d9 mov rcx, rbx
1403449a4: 4883c108 add rcx, 8
1403449a8: 48c70301000000 mov qword ptr [rbx], 1
1403449af: 48894308 mov qword ptr [rbx + 8], rax
1403449b3: 48895310 mov qword ptr [rbx + 0x10], rdx
1403449b7: 4c8d4001 lea r8, [rax + 1]
1403449bb: 4c8901 mov qword ptr [rcx], r8
1403449be: 0f29b5c0020000 movaps xmmword ptr [rbp + 0x2c0], xmm6
1403449c5: 0f29bdb0020000 movaps xmmword ptr [rbp + 0x2b0], xmm7
1403449cc: 488985d0020000 mov qword ptr [rbp + 0x2d0], rax
1403449d3: 488995d8020000 mov qword ptr [rbp + 0x2d8], rdx
1403449da: 31c9 xor ecx, ecx
1403449dc: e83f51d9ff call 0x1400d9b20 ; core::ops::function::FnOnce::call_once<std::hash::random::impl$0::new::KEYS::constant$0::closure_env$1,tuple$<enum2$<core::option::Option<ref_mut$<enum2$<core::option::Option<core::cell::Cell<tuple$<u64,u64> > > > > > > 
1403449e1: 4885c0 test rax, rax
1403449e4: 0f84a10a0000 je 0x14034548b
1403449ea: 0f1000 movups xmm0, xmmword ptr [rax]
1403449ed: 48ff00 inc qword ptr [rax]
1403449f0: 0f2985d0050000 movaps xmmword ptr [rbp + 0x5d0], xmm0
1403449f7: 440f298db0050000 movaps xmmword ptr [rbp + 0x5b0], xmm9
1403449ff: 440f2985c0050000 movaps xmmword ptr [rbp + 0x5c0], xmm8
140344a07: e8348da000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140344a0c: 4889c3 mov rbx, rax
140344a0f: f60001 test byte ptr [rax], 1
140344a12: 7411 je 0x140344a25
140344a14: 4889d8 mov rax, rbx
140344a17: 4883c008 add rax, 8
140344a1b: 4c8b7308 mov r14, qword ptr [rbx + 8]
140344a1f: 4c8b7b10 mov r15, qword ptr [rbx + 0x10]
140344a23: eb21 jmp 0x140344a46 ; unknown
140344a25: e8d603a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140344a2a: 4989c6 mov r14, rax
140344a2d: 4989d7 mov r15, rdx
140344a30: 4889d8 mov rax, rbx
140344a33: 4883c008 add rax, 8
140344a37: 48c70301000000 mov qword ptr [rbx], 1
140344a3e: 4c897308 mov qword ptr [rbx + 8], r14
140344a42: 48895310 mov qword ptr [rbx + 0x10], rdx
140344a46: 498d4e01 lea rcx, [r14 + 1]
140344a4a: 488908 mov qword ptr [rax], rcx
140344a4d: 4c8da520030000 lea r12, [rbp + 0x320]
140344a54: 488d9df0000000 lea rbx, [rbp + 0xf0]
140344a5b: 41b890000000 mov r8d, 0x90
140344a61: 4c89e1 mov rcx, r12
140344a64: 4889da mov rdx, rbx
140344a67: e894f15d02 call 0x142923c00 ; unknown
140344a6c: 0f2845e0 movaps xmm0, xmmword ptr [rbp - 0x20]
140344a70: 0f284df0 movaps xmm1, xmmword ptr [rbp - 0x10]
140344a74: 0f285500 movaps xmm2, xmmword ptr [rbp]
140344a78: 0f1185b0030000 movups xmmword ptr [rbp + 0x3b0], xmm0
140344a7f: 0f118dc0030000 movups xmmword ptr [rbp + 0x3c0], xmm1
140344a86: 0f1195d0030000 movups xmmword ptr [rbp + 0x3d0], xmm2
140344a8d: 0f284510 movaps xmm0, xmmword ptr [rbp + 0x10]
140344a91: 0f284d20 movaps xmm1, xmmword ptr [rbp + 0x20]
140344a95: 0f285530 movaps xmm2, xmmword ptr [rbp + 0x30]
140344a99: 0f1185e0030000 movups xmmword ptr [rbp + 0x3e0], xmm0
140344aa0: 0f118df0030000 movups xmmword ptr [rbp + 0x3f0], xmm1
140344aa7: 0f119500040000 movups xmmword ptr [rbp + 0x400], xmm2
140344aae: 0f288580000000 movaps xmm0, xmmword ptr [rbp + 0x80]
140344ab5: 0f288d90000000 movaps xmm1, xmmword ptr [rbp + 0x90]
140344abc: 0f2895a0000000 movaps xmm2, xmmword ptr [rbp + 0xa0]
140344ac3: 0f118510040000 movups xmmword ptr [rbp + 0x410], xmm0
140344aca: 0f118d20040000 movups xmmword ptr [rbp + 0x420], xmm1
140344ad1: 0f119530040000 movups xmmword ptr [rbp + 0x430], xmm2
140344ad8: 0f2885b0020000 movaps xmm0, xmmword ptr [rbp + 0x2b0]
140344adf: 0f288dc0020000 movaps xmm1, xmmword ptr [rbp + 0x2c0]
140344ae6: 0f2895d0020000 movaps xmm2, xmmword ptr [rbp + 0x2d0]
140344aed: 0f119560040000 movups xmmword ptr [rbp + 0x460], xmm2
140344af4: 0f118d50040000 movups xmmword ptr [rbp + 0x450], xmm1
140344afb: 0f118540040000 movups xmmword ptr [rbp + 0x440], xmm0
140344b02: 0f2885b0050000 movaps xmm0, xmmword ptr [rbp + 0x5b0]
140344b09: 0f288dc0050000 movaps xmm1, xmmword ptr [rbp + 0x5c0]
140344b10: 0f2895d0050000 movaps xmm2, xmmword ptr [rbp + 0x5d0]
140344b17: 0f118570040000 movups xmmword ptr [rbp + 0x470], xmm0
140344b1e: 0f118d80040000 movups xmmword ptr [rbp + 0x480], xmm1
140344b25: 0f119590040000 movups xmmword ptr [rbp + 0x490], xmm2
140344b2c: 0f11bda0040000 movups xmmword ptr [rbp + 0x4a0], xmm7
140344b33: 0f11b5b0040000 movups xmmword ptr [rbp + 0x4b0], xmm6
140344b3a: 4c89b5c0040000 mov qword ptr [rbp + 0x4c0], r14
140344b41: 4c89bdc8040000 mov qword ptr [rbp + 0x4c8], r15
140344b48: 4889f9 mov rcx, rdi
140344b4b: 4c89e2 mov rdx, r12
140344b4e: e8bd5bdfff call 0x14013a710 ; bevy_app::app::App::insert_resource<country_core::resources::stairs::structural_graph::StairsStructuralGraph>
140344b53: 4889c7 mov rdi, rax
140344b56: e8e58ba000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140344b5b: 4989c6 mov r14, rax
140344b5e: f60001 test byte ptr [rax], 1
140344b61: 7411 je 0x140344b74
140344b63: 4c89f1 mov rcx, r14
140344b66: 4883c108 add rcx, 8
140344b6a: 498b4608 mov rax, qword ptr [r14 + 8]
140344b6e: 498b5610 mov rdx, qword ptr [r14 + 0x10]
140344b72: eb1b jmp 0x140344b8f ; unknown
140344b74: e88702a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140344b79: 4c89f1 mov rcx, r14
140344b7c: 4883c108 add rcx, 8
140344b80: 49c70601000000 mov qword ptr [r14], 1
140344b87: 49894608 mov qword ptr [r14 + 8], rax
140344b8b: 49895610 mov qword ptr [r14 + 0x10], rdx
140344b8f: 4c8d4001 lea r8, [rax + 1]
140344b93: 4c8901 mov qword ptr [rcx], r8
140344b96: 0f29b500010000 movaps xmmword ptr [rbp + 0x100], xmm6
140344b9d: 0f29bdf0000000 movaps xmmword ptr [rbp + 0xf0], xmm7
140344ba4: 48898510010000 mov qword ptr [rbp + 0x110], rax
140344bab: 48899518010000 mov qword ptr [rbp + 0x118], rdx
140344bb2: e8898ba000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140344bb7: 4989c6 mov r14, rax
140344bba: f60001 test byte ptr [rax], 1
140344bbd: 7411 je 0x140344bd0
140344bbf: 4c89f1 mov rcx, r14
140344bc2: 4883c108 add rcx, 8
140344bc6: 498b4608 mov rax, qword ptr [r14 + 8]
140344bca: 498b5610 mov rdx, qword ptr [r14 + 0x10]
140344bce: eb1b jmp 0x140344beb ; unknown
140344bd0: e82b02a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140344bd5: 4c89f1 mov rcx, r14
140344bd8: 4883c108 add rcx, 8
140344bdc: 49c70601000000 mov qword ptr [r14], 1
140344be3: 49894608 mov qword ptr [r14 + 8], rax
140344be7: 49895610 mov qword ptr [r14 + 0x10], rdx
140344beb: 4c8d4001 lea r8, [rax + 1]
140344bef: 4c8901 mov qword ptr [rcx], r8
140344bf2: 0f2885f0000000 movaps xmm0, xmmword ptr [rbp + 0xf0]
140344bf9: 0f288d00010000 movaps xmm1, xmmword ptr [rbp + 0x100]
140344c00: 0f289510010000 movaps xmm2, xmmword ptr [rbp + 0x110]
140344c07: 0f299540030000 movaps xmmword ptr [rbp + 0x340], xmm2
140344c0e: 0f298d30030000 movaps xmmword ptr [rbp + 0x330], xmm1
140344c15: 0f298520030000 movaps xmmword ptr [rbp + 0x320], xmm0
140344c1c: 0f29bd50030000 movaps xmmword ptr [rbp + 0x350], xmm7
140344c23: 0f29b560030000 movaps xmmword ptr [rbp + 0x360], xmm6
140344c2a: 48898570030000 mov qword ptr [rbp + 0x370], rax
140344c31: 48899578030000 mov qword ptr [rbp + 0x378], rdx
140344c38: 0f57c0 xorps xmm0, xmm0
140344c3b: 0f298580030000 movaps xmmword ptr [rbp + 0x380], xmm0
140344c42: 0f298590030000 movaps xmmword ptr [rbp + 0x390], xmm0
140344c49: 48c785a003000000000000 mov qword ptr [rbp + 0x3a0], 0
140344c54: 488d9520030000 lea rdx, [rbp + 0x320]
140344c5b: 4889f9 mov rcx, rdi
140344c5e: e8ddfbdcff call 0x140114840 ; bevy_app::app::App::insert_resource<system_decorator::ui_systems::ui_focus_stairs::MoveAllGizmoCache>
140344c63: 4889c7 mov rdi, rax
140344c66: e8d58aa000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140344c6b: 4989c6 mov r14, rax
140344c6e: f60001 test byte ptr [rax], 1
140344c71: 7411 je 0x140344c84
140344c73: 4c89f1 mov rcx, r14
140344c76: 4883c108 add rcx, 8
140344c7a: 498b4608 mov rax, qword ptr [r14 + 8]
140344c7e: 498b5610 mov rdx, qword ptr [r14 + 0x10]
140344c82: eb1b jmp 0x140344c9f ; unknown
140344c84: e87701a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140344c89: 4c89f1 mov rcx, r14
140344c8c: 4883c108 add rcx, 8
140344c90: 49c70601000000 mov qword ptr [r14], 1
140344c97: 49894608 mov qword ptr [r14 + 8], rax
140344c9b: 49895610 mov qword ptr [r14 + 0x10], rdx
140344c9f: 4c8d4001 lea r8, [rax + 1]
140344ca3: 4c8901 mov qword ptr [rcx], r8
140344ca6: 0f11b538030000 movups xmmword ptr [rbp + 0x338], xmm6
140344cad: 0f11bd28030000 movups xmmword ptr [rbp + 0x328], xmm7
140344cb4: 48898548030000 mov qword ptr [rbp + 0x348], rax
140344cbb: 48899550030000 mov qword ptr [rbp + 0x350], rdx
140344cc2: c7852003000000000000 mov dword ptr [rbp + 0x320], 0
140344ccc: 0f57c0 xorps xmm0, xmm0
140344ccf: 0f118558030000 movups xmmword ptr [rbp + 0x358], xmm0
140344cd6: 488d9520030000 lea rdx, [rbp + 0x320]
140344cdd: 4889f9 mov rcx, rdi
140344ce0: e8fb1adfff call 0x1401367e0 ; bevy_app::app::App::insert_resource<system_decorator::ui_systems::ui_focus_stairs::HeightAllGizmoCache>
140344ce5: 4889c7 mov rdi, rax
140344ce8: e8538aa000 call 0x140d4d740 ; std::hash::random::impl$0::new::KEYS::constant$0::closure$1::VAL
140344ced: 4989c6 mov r14, rax
140344cf0: f60001 test byte ptr [rax], 1
140344cf3: 7411 je 0x140344d06
140344cf5: 4c89f1 mov rcx, r14
140344cf8: 4883c108 add rcx, 8
140344cfc: 498b4608 mov rax, qword ptr [r14 + 8]
140344d00: 498b5610 mov rdx, qword ptr [r14 + 0x10]
140344d04: eb1b jmp 0x140344d21 ; unknown
140344d06: e8f500a200 call 0x140d64e00 ; std::sys::random::hashmap_random_keys
140344d0b: 4c89f1 mov rcx, r14
140344d0e: 4883c108 add rcx, 8
140344d12: 49c70601000000 mov qword ptr [r14], 1
140344d19: 49894608 mov qword ptr [r14 + 8], rax
140344d1d: 49895610 mov qword ptr [r14 + 0x10], rdx
140344d21: 4c8d4001 lea r8, [rax + 1]
140344d25: 4c8901 mov qword ptr [rcx], r8
140344d28: 0f11b538030000 movups xmmword ptr [rbp + 0x338], xmm6
140344d2f: 0f11bd28030000 movups xmmword ptr [rbp + 0x328], xmm7
140344d36: 48898548030000 mov qword ptr [rbp + 0x348], rax
140344d3d: 48899550030000 mov qword ptr [rbp + 0x350], rdx
140344d44: c7852003000000000000 mov dword ptr [rbp + 0x320], 0
140344d4e: 0f57c0 xorps xmm0, xmm0
140344d51: 0f118558030000 movups xmmword ptr [rbp + 0x358], xmm0
140344d58: 0f118564030000 movups xmmword ptr [rbp + 0x364], xmm0
140344d5f: 488d9520030000 lea rdx, [rbp + 0x320]
140344d66: 4889f9 mov rcx, rdi
140344d69: e84209dfff call 0x1401356b0 ; bevy_app::app::App::insert_resource<system_decorator::ui_systems::ui_focus_stairs::WidthAllGizmoCache>
140344d6e: 4889c7 mov rdi, rax
140344d71: 488d4838 lea rcx, [rax + 0x38]
140344d75: 48ba84a72b9651d21b8e movabs rdx, 0x8e1bd251962ba784
140344d7f: 49b89b52e6c07423786a movabs r8, 0x6a782374c0e6529b
140344d89: e86289d9ff call 0x1400dd6f0 ; bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::component::Components::get_resource_id | bevy_ecs::compon
140344d8e: 4885c0 test rax, rax
140344d91: 0f8437060000 je 0x1403453ce
140344d97: 48399718020000 cmp qword ptr [rdi + 0x218], rdx
140344d9e: 0f862a060000 jbe 0x1403453ce
140344da4: 488b8710020000 mov rax, qword ptr [rdi + 0x210]
140344dab: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
140344daf: 4885c9 test rcx, rcx
140344db2: 0f8416060000 je 0x1403453ce
140344db8: 488b87e0010000 mov rax, qword ptr [rdi + 0x1e0]
140344dbf: 48f7d1 not rcx
140344dc2: 488d0c49 lea rcx, [rcx + rcx*2]
140344dc6: 48c1e105 shl rcx, 5
140344dca: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
140344dd0: 0f84f8050000 je 0x1403453ce
140344dd6: 4801c8 add rax, rcx
140344dd9: 488b4810 mov rcx, qword ptr [rax + 0x10]
140344ddd: 8b97b0050000 mov edx, dword ptr [rdi + 0x5b0]
140344de3: 89505c mov dword ptr [rax + 0x5c], edx
140344de6: e8c5c1e2ff call 0x140170fb0 ; bevy_ecs::schedule::schedule::Schedules::add_systems<tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_ecs::change_detection::ResMut<country_core::reso
140344deb: 4889f9 mov rcx, rdi
140344dee: e85d25feff call 0x140327350 ; bevy_app::sub_app::SubApp::add_systems<tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(bevy_e
140344df3: 488db520030000 lea rsi, [rbp + 0x320]
140344dfa: 4889f1 mov rcx, rsi
140344dfd: e80ed2d6ff call 0x1400b2010 ; bevy_ecs::schedule::config::IntoScheduleConfigs::after<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::ResMut<country_core::resources::stairs::s
140344e02: 4c8d0d97e06702 lea r9, [rip + 0x267e097]
140344e09: 4889d9 mov rcx, rbx
140344e0c: 4889f2 mov rdx, rsi
140344e0f: 41b00a mov r8b, 0xa
140344e12: e8a909f5ff call 0x1402957c0 ; bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<tuple$<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::walls::wall_holes::WallHoles>,bevy_ecs::change_detection::ResMut<country_core::resources
140344e17: 4889f9 mov rcx, rdi
140344e1a: 4889da mov rdx, rbx
140344e1d: e8ce28feff call 0x1403276f0 ; bevy_app::sub_app::SubApp::add_systems<tuple$<>,bevy_app::main_schedule::Update,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,e
140344e22: 488d4d10 lea rcx, [rbp + 0x10]
140344e26: e83505dcff call 0x140105360 ; bevy_ecs::schedule::config::IntoScheduleConfigs::chain<tuple$<void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_
140344e2b: 488d8db0050000 lea rcx, [rbp + 0x5b0]
140344e32: e8d9f9f8ff call 0x1402d4810 ; bevy_ecs::schedule::config::IntoScheduleConfigs::chain<tuple$<void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::ui_clutter::debug_spawn_clutter::DebugSpawnAllClutterCmd>,bevy_ecs::event::reader::EventR
140344e37: 488d8df0000000 lea rcx, [rbp + 0xf0]
140344e3e: e8ede12000 call 0x140553030 ; bevy_ecs::schedule::config::IntoScheduleConfigs::chain<tuple$<void (*)(bevy_ecs::change_detection::Res<bevy_time::time::Time<tuple$<> > >,tuple$<bevy_ecs::system::system_param::Local<enum2$<core::option::Option<system_de
140344e43: 0f108510060000 movups xmm0, xmmword ptr [rbp + 0x610]
140344e4a: 0f298580030000 movaps xmmword ptr [rbp + 0x380], xmm0
140344e51: 0f108500060000 movups xmm0, xmmword ptr [rbp + 0x600]
140344e58: 0f298570030000 movaps xmmword ptr [rbp + 0x370], xmm0
140344e5f: 0f1085f0050000 movups xmm0, xmmword ptr [rbp + 0x5f0]
140344e66: 0f298560030000 movaps xmmword ptr [rbp + 0x360], xmm0
140344e6d: 0f1085b0050000 movups xmm0, xmmword ptr [rbp + 0x5b0]
140344e74: 0f108dc0050000 movups xmm1, xmmword ptr [rbp + 0x5c0]
140344e7b: 0f1095d0050000 movups xmm2, xmmword ptr [rbp + 0x5d0]
140344e82: 0f109de0050000 movups xmm3, xmmword ptr [rbp + 0x5e0]
140344e89: 0f299d50030000 movaps xmmword ptr [rbp + 0x350], xmm3
140344e90: 0f299540030000 movaps xmmword ptr [rbp + 0x340], xmm2
140344e97: 0f298d30030000 movaps xmmword ptr [rbp + 0x330], xmm1
140344e9e: 0f298520030000 movaps xmmword ptr [rbp + 0x320], xmm0
140344ea5: 0f1085f0000000 movups xmm0, xmmword ptr [rbp + 0xf0]
140344eac: 0f108d00010000 movups xmm1, xmmword ptr [rbp + 0x100]
140344eb3: 0f109510010000 movups xmm2, xmmword ptr [rbp + 0x110]
140344eba: 0f109d20010000 movups xmm3, xmmword ptr [rbp + 0x120]
140344ec1: 0f298590030000 movaps xmmword ptr [rbp + 0x390], xmm0
140344ec8: 0f298da0030000 movaps xmmword ptr [rbp + 0x3a0], xmm1
140344ecf: 0f2995b0030000 movaps xmmword ptr [rbp + 0x3b0], xmm2
140344ed6: 0f299dc0030000 movaps xmmword ptr [rbp + 0x3c0], xmm3
140344edd: 0f108530010000 movups xmm0, xmmword ptr [rbp + 0x130]
140344ee4: 0f2985d0030000 movaps xmmword ptr [rbp + 0x3d0], xmm0
140344eeb: 0f108540010000 movups xmm0, xmmword ptr [rbp + 0x140]
140344ef2: 0f2985e0030000 movaps xmmword ptr [rbp + 0x3e0], xmm0
140344ef9: 0f108550010000 movups xmm0, xmmword ptr [rbp + 0x150]
140344f00: 0f2985f0030000 movaps xmmword ptr [rbp + 0x3f0], xmm0
140344f07: 488d8d80000000 lea rcx, [rbp + 0x80]
140344f0e: 488d9520030000 lea rdx, [rbp + 0x320]
140344f15: e87678f2ff call 0x14026c790 ; bevy_ecs::schedule::config::IntoScheduleConfigs::chain<tuple$<void (*)(bevy_ecs::change_detection::Res<enum2$<country_core::systems::mode_manager::AppMode> >,bevy_ecs::change_detection::Res<country_core::systems::ui::res
140344f1a: 488d8db0020000 lea rcx, [rbp + 0x2b0]
140344f21: e8ea24f4ff call 0x140287410 ; bevy_ecs::schedule::config::IntoScheduleConfigs::chain<tuple$<void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::walls::decorator_
140344f26: 488d8db0050000 lea rcx, [rbp + 0x5b0]
140344f2d: e8de16f4ff call 0x140286610 ; bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred<tuple$<void (*)(bevy_ecs::change_detection::Res<country_core::resources::history::History>,bevy_ecs::change_detection::Res<country_core::resources::w
140344f32: 488d8d20030000 lea rcx, [rbp + 0x320]
140344f39: e842de1400 call 0x140492d80 ; bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred<tuple$<void (*)(country_core::resources::walls::decorator_subtype::DerivedDecoratorInfoResources,bevy_ecs::change_detection::Res<country_core::resour
140344f3e: 0f108580030000 movups xmm0, xmmword ptr [rbp + 0x380]
140344f45: 0f298550010000 movaps xmmword ptr [rbp + 0x150], xmm0
140344f4c: 0f108570030000 movups xmm0, xmmword ptr [rbp + 0x370]
140344f53: 0f298540010000 movaps xmmword ptr [rbp + 0x140], xmm0
140344f5a: 0f108560030000 movups xmm0, xmmword ptr [rbp + 0x360]
140344f61: 0f298530010000 movaps xmmword ptr [rbp + 0x130], xmm0
140344f68: 0f108520030000 movups xmm0, xmmword ptr [rbp + 0x320]
140344f6f: 0f108d30030000 movups xmm1, xmmword ptr [rbp + 0x330]
140344f76: 0f109540030000 movups xmm2, xmmword ptr [rbp + 0x340]
140344f7d: 0f109d50030000 movups xmm3, xmmword ptr [rbp + 0x350]
140344f84: 0f299d20010000 movaps xmmword ptr [rbp + 0x120], xmm3
140344f8b: 0f299510010000 movaps xmmword ptr [rbp + 0x110], xmm2
140344f92: 0f298d00010000 movaps xmmword ptr [rbp + 0x100], xmm1
140344f99: 0f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm0
140344fa0: 488d8d20030000 lea rcx, [rbp + 0x320]
140344fa7: 488d95f0000000 lea rdx, [rbp + 0xf0]
140344fae: e82d1d0900 call 0x1403d6ce0 ; bevy_ecs::schedule::config::IntoScheduleConfigs::chain_ignore_deferred<tuple$<void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::ResMut
140344fb3: 0f1085b0050000 movups xmm0, xmmword ptr [rbp + 0x5b0]
140344fba: 0f108dc0050000 movups xmm1, xmmword ptr [rbp + 0x5c0]
140344fc1: 0f1095d0050000 movups xmm2, xmmword ptr [rbp + 0x5d0]
140344fc8: 0f109de0050000 movups xmm3, xmmword ptr [rbp + 0x5e0]
140344fcf: 0f2985f0000000 movaps xmmword ptr [rbp + 0xf0], xmm0
140344fd6: 0f298d00010000 movaps xmmword ptr [rbp + 0x100], xmm1
140344fdd: 0f299510010000 movaps xmmword ptr [rbp + 0x110], xmm2
140344fe4: 0f299d20010000 movaps xmmword ptr [rbp + 0x120], xmm3
140344feb: 0f1085f0050000 movups xmm0, xmmword ptr [rbp + 0x5f0]
140344ff2: 0f298530010000 movaps xmmword ptr [rbp + 0x130], xmm0
140344ff9: 0f108500060000 movups xmm0, xmmword ptr [rbp + 0x600]
140345000: 0f298540010000 movaps xmmword ptr [rbp + 0x140], xmm0
140345007: 0f108510060000 movups xmm0, xmmword ptr [rbp + 0x610]
14034500e: 0f298550010000 movaps xmmword ptr [rbp + 0x150], xmm0
140345015: 0f108520030000 movups xmm0, xmmword ptr [rbp + 0x320]
14034501c: 0f108d30030000 movups xmm1, xmmword ptr [rbp + 0x330]
140345023: 0f109540030000 movups xmm2, xmmword ptr [rbp + 0x340]
14034502a: 0f109d50030000 movups xmm3, xmmword ptr [rbp + 0x350]
140345031: 0f298560010000 movaps xmmword ptr [rbp + 0x160], xmm0
140345038: 0f298d70010000 movaps xmmword ptr [rbp + 0x170], xmm1
14034503f: 0f299580010000 movaps xmmword ptr [rbp + 0x180], xmm2
140345046: 0f299d90010000 movaps xmmword ptr [rbp + 0x190], xmm3
14034504d: 0f108560030000 movups xmm0, xmmword ptr [rbp + 0x360]
140345054: 0f2985a0010000 movaps xmmword ptr [rbp + 0x1a0], xmm0
14034505b: 0f108570030000 movups xmm0, xmmword ptr [rbp + 0x370]
140345062: 0f2985b0010000 movaps xmmword ptr [rbp + 0x1b0], xmm0
140345069: 0f108580030000 movups xmm0, xmmword ptr [rbp + 0x380]
140345070: 0f2985c0010000 movaps xmmword ptr [rbp + 0x1c0], xmm0
140345077: 0f104570 movups xmm0, xmmword ptr [rbp + 0x70]
14034507b: 0f298580030000 movaps xmmword ptr [rbp + 0x380], xmm0
140345082: 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
140345086: 0f298570030000 movaps xmmword ptr [rbp + 0x370], xmm0
14034508d: 0f104550 movups xmm0, xmmword ptr [rbp + 0x50]
140345091: 0f298560030000 movaps xmmword ptr [rbp + 0x360], xmm0
140345098: 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
14034509c: 0f104d20 movups xmm1, xmmword ptr [rbp + 0x20]
1403450a0: 0f105530 movups xmm2, xmmword ptr [rbp + 0x30]
1403450a4: 0f105d40 movups xmm3, xmmword ptr [rbp + 0x40]
1403450a8: 0f299d50030000 movaps xmmword ptr [rbp + 0x350], xmm3
1403450af: 0f299540030000 movaps xmmword ptr [rbp + 0x340], xmm2
1403450b6: 0f298d30030000 movaps xmmword ptr [rbp + 0x330], xmm1
1403450bd: 0f298520030000 movaps xmmword ptr [rbp + 0x320], xmm0
1403450c4: 0f1085e0000000 movups xmm0, xmmword ptr [rbp + 0xe0]
1403450cb: 0f2985f0030000 movaps xmmword ptr [rbp + 0x3f0], xmm0
1403450d2: 0f1085d0000000 movups xmm0, xmmword ptr [rbp + 0xd0]
1403450d9: 0f2985e0030000 movaps xmmword ptr [rbp + 0x3e0], xmm0
1403450e0: 0f1085c0000000 movups xmm0, xmmword ptr [rbp + 0xc0]
1403450e7: 0f2985d0030000 movaps xmmword ptr [rbp + 0x3d0], xmm0
1403450ee: 0f108580000000 movups xmm0, xmmword ptr [rbp + 0x80]
1403450f5: 0f108d90000000 movups xmm1, xmmword ptr [rbp + 0x90]
1403450fc: 0f1095a0000000 movups xmm2, xmmword ptr [rbp + 0xa0]
140345103: 0f109db0000000 movups xmm3, xmmword ptr [rbp + 0xb0]
14034510a: 0f299dc0030000 movaps xmmword ptr [rbp + 0x3c0], xmm3
140345111: 0f2995b0030000 movaps xmmword ptr [rbp + 0x3b0], xmm2
140345118: 0f298da0030000 movaps xmmword ptr [rbp + 0x3a0], xmm1
14034511f: 0f298590030000 movaps xmmword ptr [rbp + 0x390], xmm0
140345126: 0f108510030000 movups xmm0, xmmword ptr [rbp + 0x310]
14034512d: 0f298560040000 movaps xmmword ptr [rbp + 0x460], xmm0
140345134: 0f108500030000 movups xmm0, xmmword ptr [rbp + 0x300]
14034513b: 0f298550040000 movaps xmmword ptr [rbp + 0x450], xmm0
140345142: 0f1085f0020000 movups xmm0, xmmword ptr [rbp + 0x2f0]
140345149: 0f298540040000 movaps xmmword ptr [rbp + 0x440], xmm0
140345150: 0f1085b0020000 movups xmm0, xmmword ptr [rbp + 0x2b0]
140345157: 0f108dc0020000 movups xmm1, xmmword ptr [rbp + 0x2c0]
14034515e: 0f1095d0020000 movups xmm2, xmmword ptr [rbp + 0x2d0]
140345165: 0f109de0020000 movups xmm3, xmmword ptr [rbp + 0x2e0]
14034516c: 0f299d30040000 movaps xmmword ptr [rbp + 0x430], xmm3
140345173: 0f299520040000 movaps xmmword ptr [rbp + 0x420], xmm2
14034517a: 0f298d10040000 movaps xmmword ptr [rbp + 0x410], xmm1
140345181: 0f298500040000 movaps xmmword ptr [rbp + 0x400], xmm0
140345188: 488d8d70040000 lea rcx, [rbp + 0x470]
14034518f: 488d95f0000000 lea rdx, [rbp + 0xf0]
140345196: 41b8e0000000 mov r8d, 0xe0
14034519c: e85fea5d02 call 0x142923c00 ; unknown
1403451a1: 488d8df0000000 lea rcx, [rbp + 0xf0]
1403451a8: 488d9520030000 lea rdx, [rbp + 0x320]
1403451af: e83c01d9ff call 0x1400d52f0 ; bevy_ecs::schedule::config::IntoScheduleConfigs::chain<tuple$<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result
1403451b4: c685b005000019 mov byte ptr [rbp + 0x5b0], 0x19
1403451bb: 488b0dae712e03 mov rcx, qword ptr [rip + 0x32e71ae]
1403451c2: 4c8d05cf5d5e02 lea r8, [rip + 0x25e5dcf]
1403451c9: 488d95b0050000 lea rdx, [rbp + 0x5b0]
1403451d0: e84bfb0100 call 0x140364d20 ; bevy_ecs::intern::Interner::intern<dyn$<bevy_ecs::schedule::set::SystemSet> >
1403451d5: 4989d0 mov r8, rdx
1403451d8: 488d8df0000000 lea rcx, [rbp + 0xf0]
1403451df: 4889c2 mov rdx, rax
1403451e2: e8d916d9ff call 0x1400d68c0 ; bevy_ecs::schedule::config::ScheduleConfigs::in_set_inner<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::bevy_error::BevyErro
1403451e7: 0f108550010000 movups xmm0, xmmword ptr [rbp + 0x150]
1403451ee: 0f298580030000 movaps xmmword ptr [rbp + 0x380], xmm0
1403451f5: 0f108540010000 movups xmm0, xmmword ptr [rbp + 0x140]
1403451fc: 0f298570030000 movaps xmmword ptr [rbp + 0x370], xmm0
140345203: 0f108530010000 movups xmm0, xmmword ptr [rbp + 0x130]
14034520a: 0f298560030000 movaps xmmword ptr [rbp + 0x360], xmm0
140345211: 0f1085f0000000 movups xmm0, xmmword ptr [rbp + 0xf0]
140345218: 0f108d00010000 movups xmm1, xmmword ptr [rbp + 0x100]
14034521f: 0f109510010000 movups xmm2, xmmword ptr [rbp + 0x110]
140345226: 0f109d20010000 movups xmm3, xmmword ptr [rbp + 0x120]
14034522d: 0f299d50030000 movaps xmmword ptr [rbp + 0x350], xmm3
140345234: 0f299540030000 movaps xmmword ptr [rbp + 0x340], xmm2
14034523b: 0f298d30030000 movaps xmmword ptr [rbp + 0x330], xmm1
140345242: 0f298520030000 movaps xmmword ptr [rbp + 0x320], xmm0
140345249: 488d9520030000 lea rdx, [rbp + 0x320]
140345250: 4889f9 mov rcx, rdi
140345253: e89824feff call 0x1403276f0 ; bevy_app::sub_app::SubApp::add_systems<tuple$<>,bevy_app::main_schedule::Update,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,e
140345258: 4c8d0559dc6702 lea r8, [rip + 0x267dc59]
14034525f: 488d8db0050000 lea rcx, [rbp + 0x5b0]
140345266: b228 mov dl, 0x28
140345268: e853ddfdff call 0x140322fc0 ; bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(bevy_ecs::change_detection::Res<country_core::resources::time::GameTime>,bevy_ecs::change_detection::ResMut<country_core::resources::history::replay_cmd_bu
14034526d: 4c8d055cdc6702 lea r8, [rip + 0x267dc5c]
140345274: 488d8df0000000 lea rcx, [rbp + 0xf0]
14034527b: b222 mov dl, 0x22
14034527d: e8de2df1ff call 0x140258060 ; bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<void (*)(bevy_ecs::change_detection::ResMut<country_core::resources::history::History>,bevy_ecs::change_detection::ResMut<system_decorator::deferred_decorator_histo
140345282: 0f108510060000 movups xmm0, xmmword ptr [rbp + 0x610]
140345289: 0f298580030000 movaps xmmword ptr [rbp + 0x380], xmm0
140345290: 0f108500060000 movups xmm0, xmmword ptr [rbp + 0x600]
140345297: 0f298570030000 movaps xmmword ptr [rbp + 0x370], xmm0
14034529e: 0f1085f0050000 movups xmm0, xmmword ptr [rbp + 0x5f0]
1403452a5: 0f298560030000 movaps xmmword ptr [rbp + 0x360], xmm0
1403452ac: 0f1085b0050000 movups xmm0, xmmword ptr [rbp + 0x5b0]
1403452b3: 0f108dc0050000 movups xmm1, xmmword ptr [rbp + 0x5c0]
1403452ba: 0f1095d0050000 movups xmm2, xmmword ptr [rbp + 0x5d0]
1403452c1: 0f109de0050000 movups xmm3, xmmword ptr [rbp + 0x5e0]
1403452c8: 0f299d50030000 movaps xmmword ptr [rbp + 0x350], xmm3
1403452cf: 0f299540030000 movaps xmmword ptr [rbp + 0x340], xmm2
1403452d6: 0f298d30030000 movaps xmmword ptr [rbp + 0x330], xmm1
1403452dd: 0f298520030000 movaps xmmword ptr [rbp + 0x320], xmm0
1403452e4: 0f1085f0000000 movups xmm0, xmmword ptr [rbp + 0xf0]
1403452eb: 0f108d00010000 movups xmm1, xmmword ptr [rbp + 0x100]
1403452f2: 0f109510010000 movups xmm2, xmmword ptr [rbp + 0x110]
1403452f9: 0f109d20010000 movups xmm3, xmmword ptr [rbp + 0x120]
140345300: 0f298590030000 movaps xmmword ptr [rbp + 0x390], xmm0
140345307: 0f298da0030000 movaps xmmword ptr [rbp + 0x3a0], xmm1
14034530e: 0f2995b0030000 movaps xmmword ptr [rbp + 0x3b0], xmm2
140345315: 0f299dc0030000 movaps xmmword ptr [rbp + 0x3c0], xmm3
14034531c: 0f108530010000 movups xmm0, xmmword ptr [rbp + 0x130]
140345323: 0f2985d0030000 movaps xmmword ptr [rbp + 0x3d0], xmm0
14034532a: 0f108540010000 movups xmm0, xmmword ptr [rbp + 0x140]
140345331: 0f2985e0030000 movaps xmmword ptr [rbp + 0x3e0], xmm0
140345338: 0f108550010000 movups xmm0, xmmword ptr [rbp + 0x150]
14034533f: 0f2985f0030000 movaps xmmword ptr [rbp + 0x3f0], xmm0
140345346: 488d9520030000 lea rdx, [rbp + 0x320]
14034534d: 4889f9 mov rcx, rdi
140345350: e8fb25feff call 0x140327950 ; bevy_app::sub_app::SubApp::add_systems<tuple$<bevy_ecs::schedule::config::ScheduleConfigTupleMarker,tuple$<>,tuple$<> >,bevy_app::main_schedule::Update,tuple$<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::box
140345355: 4c8d058cdb6702 lea r8, [rip + 0x267db8c]
14034535c: 4889f1 mov rcx, rsi
14034535f: b224 mov dl, 0x24
140345361: e8ca3dd8ff call 0x1400c9130 ; bevy_ecs::schedule::config::IntoScheduleConfigs::in_set<tuple$<void (*)(bevy_ecs::change_detection::ResMut<system_decorator::decorator_visual_state::DecoratorVisualState>,bevy_ecs::change_detection::Res<country_core::ass
140345366: 4889f9 mov rcx, rdi
140345369: 4889f2 mov rdx, rsi
14034536c: e87f23feff call 0x1403276f0 ; bevy_app::sub_app::SubApp::add_systems<tuple$<>,bevy_app::main_schedule::Update,enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,e
140345371: 4889f9 mov rcx, rdi
140345374: b201 mov dl, 1
140345376: 0f28b550060000 movaps xmm6, xmmword ptr [rbp + 0x650]
14034537d: 0f28bd60060000 movaps xmm7, xmmword ptr [rbp + 0x660]
140345384: 440f288570060000 movaps xmm8, xmmword ptr [rbp + 0x670]
14034538c: 440f288d80060000 movaps xmm9, xmmword ptr [rbp + 0x680]
140345394: 440f289590060000 movaps xmm10, xmmword ptr [rbp + 0x690]
14034539c: 440f289da0060000 movaps xmm11, xmmword ptr [rbp + 0x6a0]
1403453a4: 440f28a5b0060000 movaps xmm12, xmmword ptr [rbp + 0x6b0]
1403453ac: 4881c440070000 add rsp, 0x740
1403453b3: 5b pop rbx
1403453b4: 5f pop rdi
1403453b5: 5e pop rsi
1403453b6: 415c pop r12
1403453b8: 415e pop r14
1403453ba: 415f pop r15
1403453bc: 5d pop rbp
1403453bd: e9be1bfeff jmp 0x140326f80 ; bevy_app::sub_app::SubApp::add_systems<tuple$<bevy_ecs::schedule::config::Infallible,tuple$<bevy_ecs::system::function_system::IsFunctionSystem,void (*)(tuple$<bevy_ecs::change_detection::ResMut<country_core::resources::
1403453c2: 488d0d47d65e02 lea rcx, [rip + 0x25ed647]
1403453c9: e8022d5902 call 0x1428d80d0 ; std::thread::local::panic_access_error
1403453ce: 488d05638e6702 lea rax, [rip + 0x2678e63]
1403453d5: 488985f0000000 mov qword ptr [rbp + 0xf0], rax
1403453dc: 48c785f800000027000000 mov qword ptr [rbp + 0xf8], 0x27
1403453e7: 48899db0050000 mov qword ptr [rbp + 0x5b0], rbx
1403453ee: 488d059b4adcff lea rax, [rip - 0x23b565]
1403453f5: 488985b8050000 mov qword ptr [rbp + 0x5b8], rax
1403453fc: 488d05658f6702 lea rax, [rip + 0x2678f65]
140345403: 48898520030000 mov qword ptr [rbp + 0x320], rax
14034540a: 48c7852803000002000000 mov qword ptr [rbp + 0x328], 2
140345415: 48c7854003000000000000 mov qword ptr [rbp + 0x340], 0
140345420: 4889b530030000 mov qword ptr [rbp + 0x330], rsi
140345427: 48c7853803000001000000 mov qword ptr [rbp + 0x338], 1
140345432: 488d15b78d6702 lea rdx, [rip + 0x2678db7]
140345439: 488d8d20030000 lea rcx, [rbp + 0x320]
140345440: e84b3f5902 call 0x1428d9390 ; core::panicking::panic_fmt
140345445: 488d0dc4d55e02 lea rcx, [rip + 0x25ed5c4]
14034544c: e87f2c5902 call 0x1428d80d0 ; std::thread::local::panic_access_error
140345451: eb44 jmp 0x140345497 ; unknown
140345453: 488d0db6d55e02 lea rcx, [rip + 0x25ed5b6]
14034545a: e8712c5902 call 0x1428d80d0 ; std::thread::local::panic_access_error
14034545f: eb36 jmp 0x140345497 ; unknown
140345461: 488d0da8d55e02 lea rcx, [rip + 0x25ed5a8]
140345468: e8632c5902 call 0x1428d80d0 ; std::thread::local::panic_access_error
14034546d: eb28 jmp 0x140345497 ; unknown
14034546f: 488d0d9ad55e02 lea rcx, [rip + 0x25ed59a]
140345476: e8552c5902 call 0x1428d80d0 ; std::thread::local::panic_access_error
14034547b: eb1a jmp 0x140345497 ; unknown
14034547d: 488d0d8cd55e02 lea rcx, [rip + 0x25ed58c]
140345484: e8472c5902 call 0x1428d80d0 ; std::thread::local::panic_access_error
140345489: eb0c jmp 0x140345497 ; unknown
14034548b: 488d0d7ed55e02 lea rcx, [rip + 0x25ed57e]
140345492: e8392c5902 call 0x1428d80d0 ; std::thread::local::panic_access_error
140345497: 0f0b ud2 
140345499: 0f1f8000000000 nop dword ptr [rax]
1403454a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403454a5: 55 push rbp
1403454a6: 4157 push r15
1403454a8: 4156 push r14
1403454aa: 4154 push r12
1403454ac: 56 push rsi
1403454ad: 57 push rdi
1403454ae: 53 push rbx
1403454af: 4881ec90000000 sub rsp, 0x90
1403454b6: 488daa80000000 lea rbp, [rdx + 0x80]
1403454bd: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
1403454c3: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
1403454c9: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
1403454cf: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
1403454d5: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
1403454db: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
1403454e0: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
1403454e8: 488d8db0050000 lea rcx, [rbp + 0x5b0]
1403454ef: 41b898000000 mov r8d, 0x98
1403454f5: 41b910000000 mov r9d, 0x10
1403454fb: 488d95d0050000 lea rdx, [rbp + 0x5d0]
140345502: e8f9b3d0ff call 0x140050900 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,alloc::alloc::Global> | hashbrown::raw::RawTableInner::drop_inn
140345507: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
14034550f: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140345514: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
14034551a: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140345520: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140345526: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
14034552c: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140345532: 4881c490000000 add rsp, 0x90
140345539: 5b pop rbx
14034553a: 5f pop rdi
14034553b: 5e pop rsi
14034553c: 415c pop r12
14034553e: 415e pop r14
140345540: 415f pop r15
140345542: 5d pop rbp
140345543: c3 ret 
140345544: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140345550: 4889542410 mov qword ptr [rsp + 0x10], rdx
140345555: 55 push rbp
140345556: 4157 push r15
140345558: 4156 push r14
14034555a: 4154 push r12
14034555c: 56 push rsi
14034555d: 57 push rdi
14034555e: 53 push rbx
14034555f: 4881ec90000000 sub rsp, 0x90
140345566: 488daa80000000 lea rbp, [rdx + 0x80]
14034556d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140345573: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140345579: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14034557f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140345585: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
14034558b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140345590: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140345598: 488d8df0000000 lea rcx, [rbp + 0xf0]
14034559f: 41b840000000 mov r8d, 0x40
1403455a5: 41b910000000 mov r9d, 0x10
1403455ab: 488d9510010000 lea rdx, [rbp + 0x110]
1403455b2: e80970d0ff call 0x14004c5c0 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,std::collections::hash::set::HashSet<country_core::resources::walls::decorator::Decor
1403455b7: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
1403455bf: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
1403455c4: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
1403455ca: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
1403455d0: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
1403455d6: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
1403455dc: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
1403455e2: 4881c490000000 add rsp, 0x90
1403455e9: 5b pop rbx
1403455ea: 5f pop rdi
1403455eb: 5e pop rsi
1403455ec: 415c pop r12
1403455ee: 415e pop r14
1403455f0: 415f pop r15
1403455f2: 5d pop rbp
1403455f3: c3 ret 
1403455f4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140345600: 4889542410 mov qword ptr [rsp + 0x10], rdx
140345605: 55 push rbp
140345606: 4157 push r15
140345608: 4156 push r14
14034560a: 4154 push r12
14034560c: 56 push rsi
14034560d: 57 push rdi
14034560e: 53 push rbx
14034560f: 4881ec90000000 sub rsp, 0x90
140345616: 488daa80000000 lea rbp, [rdx + 0x80]
14034561d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140345623: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140345629: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14034562f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140345635: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
14034563b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140345640: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140345648: 488d8df0000000 lea rcx, [rbp + 0xf0]
14034564f: 41b828000000 mov r8d, 0x28
140345655: 41b910000000 mov r9d, 0x10
14034565b: 488d9510010000 lea rdx, [rbp + 0x110]
140345662: e899b3d0ff call 0x140050a00 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::decorator::DecoratorId,enum2$<system_decorator::decorator_visual_state::PersistentVisualEntities> >,alloc::alloc::Global>
140345667: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
14034566f: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140345674: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
14034567a: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140345680: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140345686: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
14034568c: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140345692: 4881c490000000 add rsp, 0x90
140345699: 5b pop rbx
14034569a: 5f pop rdi
14034569b: 5e pop rsi
14034569c: 415c pop r12
14034569e: 415e pop r14
1403456a0: 415f pop r15
1403456a2: 5d pop rbp
1403456a3: c3 ret 
1403456a4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1403456b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403456b5: 55 push rbp
1403456b6: 4157 push r15
1403456b8: 4156 push r14
1403456ba: 4154 push r12
1403456bc: 56 push rsi
1403456bd: 57 push rdi
1403456be: 53 push rbx
1403456bf: 4881ec90000000 sub rsp, 0x90
1403456c6: 488daa80000000 lea rbp, [rdx + 0x80]
1403456cd: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
1403456d3: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
1403456d9: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
1403456df: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
1403456e5: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
1403456eb: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
1403456f0: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
1403456f8: 488d8db0020000 lea rcx, [rbp + 0x2b0]
1403456ff: e8acc02900 call 0x1405e17b0 ; core::ptr::drop_in_place<std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,tuple$<f32,glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat>,std::hash::random::RandomState> > | core:
140345704: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
14034570c: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140345711: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140345717: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
14034571d: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140345723: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140345729: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
14034572f: 4881c490000000 add rsp, 0x90
140345736: 5b pop rbx
140345737: 5f pop rdi
140345738: 5e pop rsi
140345739: 415c pop r12
14034573b: 415e pop r14
14034573d: 415f pop r15
14034573f: 5d pop rbp
140345740: c3 ret 
140345741: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140345750: 4889542410 mov qword ptr [rsp + 0x10], rdx
140345755: 55 push rbp
140345756: 4157 push r15
140345758: 4156 push r14
14034575a: 4154 push r12
14034575c: 56 push rsi
14034575d: 57 push rdi
14034575e: 53 push rbx
14034575f: 4881ec90000000 sub rsp, 0x90
140345766: 488daa80000000 lea rbp, [rdx + 0x80]
14034576d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140345773: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140345779: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14034577f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140345785: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
14034578b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140345790: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140345798: 488d8db0050000 lea rcx, [rbp + 0x5b0]
14034579f: e80cc02900 call 0x1405e17b0 ; core::ptr::drop_in_place<std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,tuple$<f32,glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat>,std::hash::random::RandomState> > | core:
1403457a4: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
1403457ac: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
1403457b1: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
1403457b7: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
1403457bd: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
1403457c3: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
1403457c9: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
1403457cf: 4881c490000000 add rsp, 0x90
1403457d6: 5b pop rbx
1403457d7: 5f pop rdi
1403457d8: 5e pop rsi
1403457d9: 415c pop r12
1403457db: 415e pop r14
1403457dd: 415f pop r15
1403457df: 5d pop rbp
1403457e0: c3 ret 
1403457e1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1403457f0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403457f5: 55 push rbp
1403457f6: 4157 push r15
1403457f8: 4156 push r14
1403457fa: 4154 push r12
1403457fc: 56 push rsi
1403457fd: 57 push rdi
1403457fe: 53 push rbx
1403457ff: 4881ec90000000 sub rsp, 0x90
140345806: 488daa80000000 lea rbp, [rdx + 0x80]
14034580d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140345813: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140345819: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14034581f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140345825: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
14034582b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140345830: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140345838: 488d8df0000000 lea rcx, [rbp + 0xf0]
14034583f: e81c53e0ff call 0x14014ab60 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> | core::ptr::drop_in_place<country_core::resources::walls::cached_decorator_trans
140345844: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
14034584c: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140345851: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140345857: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
14034585d: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140345863: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140345869: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
14034586f: 4881c490000000 add rsp, 0x90
140345876: 5b pop rbx
140345877: 5f pop rdi
140345878: 5e pop rsi
140345879: 415c pop r12
14034587b: 415e pop r14
14034587d: 415f pop r15
14034587f: 5d pop rbp
140345880: c3 ret 
140345881: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140345890: 4889542410 mov qword ptr [rsp + 0x10], rdx
140345895: 55 push rbp
140345896: 4157 push r15
140345898: 4156 push r14
14034589a: 4154 push r12
14034589c: 56 push rsi
14034589d: 57 push rdi
14034589e: 53 push rbx
14034589f: 4881ec90000000 sub rsp, 0x90
1403458a6: 488daa80000000 lea rbp, [rdx + 0x80]
1403458ad: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
1403458b3: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
1403458b9: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
1403458bf: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
1403458c5: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
1403458cb: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
1403458d0: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
1403458d8: 488d8db0020000 lea rcx, [rbp + 0x2b0]
1403458df: 41b848000000 mov r8d, 0x48
1403458e5: 41b910000000 mov r9d, 0x10
1403458eb: 488d95d0020000 lea rdx, [rbp + 0x2d0]
1403458f2: e8a986d0ff call 0x14004dfa0 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision>,alloc::alloc::Globa
1403458f7: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
1403458ff: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140345904: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
14034590a: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140345910: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140345916: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
14034591c: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140345922: 4881c490000000 add rsp, 0x90
140345929: 5b pop rbx
14034592a: 5f pop rdi
14034592b: 5e pop rsi
14034592c: 415c pop r12
14034592e: 415e pop r14
140345930: 415f pop r15
140345932: 5d pop rbp
140345933: c3 ret 
140345934: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140345940: 4889542410 mov qword ptr [rsp + 0x10], rdx
140345945: 55 push rbp
140345946: 4157 push r15
140345948: 4156 push r14
14034594a: 4154 push r12
14034594c: 56 push rsi
14034594d: 57 push rdi
14034594e: 53 push rbx
14034594f: 4881ec90000000 sub rsp, 0x90
140345956: 488daa80000000 lea rbp, [rdx + 0x80]
14034595d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140345963: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140345969: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14034596f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140345975: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
14034597b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140345980: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140345988: 488d8db0050000 lea rcx, [rbp + 0x5b0]
14034598f: 41b848000000 mov r8d, 0x48
140345995: 41b910000000 mov r9d, 0x10
14034599b: 488d95d0050000 lea rdx, [rbp + 0x5d0]
1403459a2: e8f985d0ff call 0x14004dfa0 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,country_core::resources::walls::decorator_subtype::DecoratorCollision>,alloc::alloc::Globa
1403459a7: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
1403459af: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
1403459b4: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
1403459ba: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
1403459c0: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
1403459c6: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
1403459cc: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
1403459d2: 4881c490000000 add rsp, 0x90
1403459d9: 5b pop rbx
1403459da: 5f pop rdi
1403459db: 5e pop rsi
1403459dc: 415c pop r12
1403459de: 415e pop r14
1403459e0: 415f pop r15
1403459e2: 5d pop rbp
1403459e3: c3 ret 
1403459e4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1403459f0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403459f5: 55 push rbp
1403459f6: 4157 push r15
1403459f8: 4156 push r14
1403459fa: 4154 push r12
1403459fc: 56 push rsi
1403459fd: 57 push rdi
1403459fe: 53 push rbx
1403459ff: 4881ec90000000 sub rsp, 0x90
140345a06: 488daa80000000 lea rbp, [rdx + 0x80]
140345a0d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140345a13: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140345a19: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
140345a1f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140345a25: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140345a2b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140345a30: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140345a38: 488d8df0000000 lea rcx, [rbp + 0xf0]
140345a3f: 41b828000000 mov r8d, 0x28
140345a45: 41b910000000 mov r9d, 0x10
140345a4b: 488d9510010000 lea rdx, [rbp + 0x110]
140345a52: e829bbd0ff call 0x140051580 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<enum2$<country_core::resources::walls::decorator_subtype::DecoratorLibKey>,enum2$<country_core::resources::walls::decorator_outline::DecoratorOutline> >,alloc::alloc
140345a57: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
140345a5f: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140345a64: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140345a6a: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140345a70: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140345a76: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140345a7c: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140345a82: 4881c490000000 add rsp, 0x90
140345a89: 5b pop rbx
140345a8a: 5f pop rdi
140345a8b: 5e pop rsi
140345a8c: 415c pop r12
140345a8e: 415e pop r14
140345a90: 415f pop r15
140345a92: 5d pop rbp
140345a93: c3 ret 
140345a94: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140345aa0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140345aa5: 55 push rbp
140345aa6: 4157 push r15
140345aa8: 4156 push r14
140345aaa: 4154 push r12
140345aac: 56 push rsi
140345aad: 57 push rdi
140345aae: 53 push rbx
140345aaf: 4881ec90000000 sub rsp, 0x90
140345ab6: 488daa80000000 lea rbp, [rdx + 0x80]
140345abd: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140345ac3: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140345ac9: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
140345acf: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140345ad5: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140345adb: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140345ae0: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140345ae8: 488d8df0000000 lea rcx, [rbp + 0xf0]
140345aef: e86c48e0ff call 0x14014a360 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::window_lights::GlobalWindowLightsControl> | core::ptr::drop_in_place<country_core::systems::collision::world::ColliderRegistry<country_core::res
140345af4: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
140345afc: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140345b01: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140345b07: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140345b0d: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140345b13: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140345b19: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140345b1f: 4881c490000000 add rsp, 0x90
140345b26: 5b pop rbx
140345b27: 5f pop rdi
140345b28: 5e pop rsi
140345b29: 415c pop r12
140345b2b: 415e pop r14
140345b2d: 415f pop r15
140345b2f: 5d pop rbp
140345b30: c3 ret 
140345b31: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140345b40: 4889542410 mov qword ptr [rsp + 0x10], rdx
140345b45: 55 push rbp
140345b46: 4157 push r15
140345b48: 4156 push r14
140345b4a: 4154 push r12
140345b4c: 56 push rsi
140345b4d: 57 push rdi
140345b4e: 53 push rbx
140345b4f: 4881ec90000000 sub rsp, 0x90
140345b56: 488daa80000000 lea rbp, [rdx + 0x80]
140345b5d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140345b63: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140345b69: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
140345b6f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140345b75: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140345b7b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140345b80: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140345b88: 488d8db0050000 lea rcx, [rbp + 0x5b0]
140345b8f: 41b898000000 mov r8d, 0x98
140345b95: 41b910000000 mov r9d, 0x10
140345b9b: 488d95d0050000 lea rdx, [rbp + 0x5d0]
140345ba2: e859add0ff call 0x140050900 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::WallId,country_core::resources::walls::decorator_storage::Decorators>,alloc::alloc::Global> | hashbrown::raw::RawTableInner::drop_inn
140345ba7: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
140345baf: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140345bb4: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140345bba: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140345bc0: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140345bc6: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140345bcc: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140345bd2: 4881c490000000 add rsp, 0x90
140345bd9: 5b pop rbx
140345bda: 5f pop rdi
140345bdb: 5e pop rsi
140345bdc: 415c pop r12
140345bde: 415e pop r14
140345be0: 415f pop r15
140345be2: 5d pop rbp
140345be3: c3 ret 
140345be4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140345bf0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140345bf5: 55 push rbp
140345bf6: 4157 push r15
140345bf8: 4156 push r14
140345bfa: 4154 push r12
140345bfc: 56 push rsi
140345bfd: 57 push rdi
140345bfe: 53 push rbx
140345bff: 4881ec90000000 sub rsp, 0x90
140345c06: 488daa80000000 lea rbp, [rdx + 0x80]
140345c0d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140345c13: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140345c19: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
140345c1f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140345c25: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140345c2b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140345c30: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140345c38: 488d8d20030000 lea rcx, [rbp + 0x320]
140345c3f: 41b840000000 mov r8d, 0x40
140345c45: 41b910000000 mov r9d, 0x10
140345c4b: 488d9540030000 lea rdx, [rbp + 0x340]
140345c52: e86969d0ff call 0x14004c5c0 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<enum2$<country_core::resources::walls::decorator_storage::DecoratorStorageAddr>,std::collections::hash::set::HashSet<country_core::resources::walls::decorator::Decor
140345c57: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
140345c5f: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140345c64: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140345c6a: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140345c70: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140345c76: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140345c7c: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140345c82: 4881c490000000 add rsp, 0x90
140345c89: 5b pop rbx
140345c8a: 5f pop rdi
140345c8b: 5e pop rsi
140345c8c: 415c pop r12
140345c8e: 415e pop r14
140345c90: 415f pop r15
140345c92: 5d pop rbp
140345c93: c3 ret 
140345c94: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140345ca0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140345ca5: 55 push rbp
140345ca6: 4157 push r15
140345ca8: 4156 push r14
140345caa: 4154 push r12
140345cac: 56 push rsi
140345cad: 57 push rdi
140345cae: 53 push rbx
140345caf: 4881ec90000000 sub rsp, 0x90
140345cb6: 488daa80000000 lea rbp, [rdx + 0x80]
140345cbd: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140345cc3: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140345cc9: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
140345ccf: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140345cd5: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140345cdb: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140345ce0: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140345ce8: 488d8df0000000 lea rcx, [rbp + 0xf0]
140345cef: e8bc2b2c00 call 0x1406088b0 ; core::ptr::drop_in_place<country_core::resources::walls::decorator_storage::DecoratorStorage> | core::ptr::drop_in_place<country_core::resources::walls::decorator_storage::DecoratorStorage> | core::ptr::drop_in_place<cou
140345cf4: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
140345cfc: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140345d01: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140345d07: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140345d0d: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140345d13: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140345d19: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140345d1f: 4881c490000000 add rsp, 0x90
140345d26: 5b pop rbx
140345d27: 5f pop rdi
140345d28: 5e pop rsi
140345d29: 415c pop r12
140345d2b: 415e pop r14
140345d2d: 415f pop r15
140345d2f: 5d pop rbp
140345d30: c3 ret 
140345d31: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140345d40: 4889542410 mov qword ptr [rsp + 0x10], rdx
140345d45: 55 push rbp
140345d46: 4157 push r15
140345d48: 4156 push r14
140345d4a: 4154 push r12
140345d4c: 56 push rsi
140345d4d: 57 push rdi
140345d4e: 53 push rbx
140345d4f: 4881ec90000000 sub rsp, 0x90
140345d56: 488daa80000000 lea rbp, [rdx + 0x80]
140345d5d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140345d63: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140345d69: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
140345d6f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140345d75: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140345d7b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140345d80: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140345d88: 488d8db0020000 lea rcx, [rbp + 0x2b0]
140345d8f: 41b810000000 mov r8d, 0x10
140345d95: 41b910000000 mov r9d, 0x10
140345d9b: 488d95d0020000 lea rdx, [rbp + 0x2d0]
140345da2: e8c9ced0ff call 0x140052c70 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::decorator::DecoratorId,alloc::sync::Arc<system_decorator::blueprint::DecoratorBlueprint,alloc::alloc::Global> >,alloc::alloc::Global>
140345da7: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
140345daf: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140345db4: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140345dba: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140345dc0: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140345dc6: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140345dcc: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140345dd2: 4881c490000000 add rsp, 0x90
140345dd9: 5b pop rbx
140345dda: 5f pop rdi
140345ddb: 5e pop rsi
140345ddc: 415c pop r12
140345dde: 415e pop r14
140345de0: 415f pop r15
140345de2: 5d pop rbp
140345de3: c3 ret 
140345de4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140345df0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140345df5: 55 push rbp
140345df6: 4157 push r15
140345df8: 4156 push r14
140345dfa: 4154 push r12
140345dfc: 56 push rsi
140345dfd: 57 push rdi
140345dfe: 53 push rbx
140345dff: 4881ec90000000 sub rsp, 0x90
140345e06: 488daa80000000 lea rbp, [rdx + 0x80]
140345e0d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140345e13: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140345e19: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
140345e1f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140345e25: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140345e2b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140345e30: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140345e38: 488d8db0050000 lea rcx, [rbp + 0x5b0]
140345e3f: e81c4de0ff call 0x14014ab60 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::walls::cached_decorator_transforms::CachedDecoratorTransforms> | core::ptr::drop_in_place<country_core::resources::walls::cached_decorator_trans
140345e44: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
140345e4c: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140345e51: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140345e57: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140345e5d: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140345e63: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140345e69: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140345e6f: 4881c490000000 add rsp, 0x90
140345e76: 5b pop rbx
140345e77: 5f pop rdi
140345e78: 5e pop rsi
140345e79: 415c pop r12
140345e7b: 415e pop r14
140345e7d: 415f pop r15
140345e7f: 5d pop rbp
140345e80: c3 ret 
140345e81: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140345e90: 4889542410 mov qword ptr [rsp + 0x10], rdx
140345e95: 55 push rbp
140345e96: 4157 push r15
140345e98: 4156 push r14
140345e9a: 4154 push r12
140345e9c: 56 push rsi
140345e9d: 57 push rdi
140345e9e: 53 push rbx
140345e9f: 4881ec90000000 sub rsp, 0x90
140345ea6: 488daa80000000 lea rbp, [rdx + 0x80]
140345ead: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140345eb3: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140345eb9: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
140345ebf: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140345ec5: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140345ecb: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140345ed0: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140345ed8: 488d4d10 lea rcx, [rbp + 0x10]
140345edc: e87f372900 call 0x1405d9660 ; core::ptr::drop_in_place<alloc::vec::Vec<country_core::resources::stairs::stairs_assembly_params::PlayermadeStairsAssemblyParams,alloc::alloc::Global> >
140345ee1: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
140345ee9: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140345eee: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140345ef4: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140345efa: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140345f00: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140345f06: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140345f0c: 4881c490000000 add rsp, 0x90
140345f13: 5b pop rbx
140345f14: 5f pop rdi
140345f15: 5e pop rsi
140345f16: 415c pop r12
140345f18: 415e pop r14
140345f1a: 415f pop r15
140345f1c: 5d pop rbp
140345f1d: c3 ret 
140345f1e: 6690 nop 
140345f20: 4889542410 mov qword ptr [rsp + 0x10], rdx
140345f25: 55 push rbp
140345f26: 4157 push r15
140345f28: 4156 push r14
140345f2a: 4154 push r12
140345f2c: 56 push rsi
140345f2d: 57 push rdi
140345f2e: 53 push rbx
140345f2f: 4881ec90000000 sub rsp, 0x90
140345f36: 488daa80000000 lea rbp, [rdx + 0x80]
140345f3d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140345f43: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140345f49: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
140345f4f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140345f55: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140345f5b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140345f60: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140345f68: 488d8df0000000 lea rcx, [rbp + 0xf0]
140345f6f: e83cce2900 call 0x1405e2db0 ; core::ptr::drop_in_place<petgraph::graphmap::GraphMap<country_core::resources::walls::decorator::DecoratorId,country_core::resources::stairs::stairs_assembly_params::AdjustedStairEdge,enum2$<petgraph::Directed>,std::hash
140345f74: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
140345f7c: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140345f81: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140345f87: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140345f8d: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140345f93: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140345f99: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140345f9f: 4881c490000000 add rsp, 0x90
140345fa6: 5b pop rbx
140345fa7: 5f pop rdi
140345fa8: 5e pop rsi
140345fa9: 415c pop r12
140345fab: 415e pop r14
140345fad: 415f pop r15
140345faf: 5d pop rbp
140345fb0: c3 ret 
140345fb1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140345fc0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140345fc5: 55 push rbp
140345fc6: 4157 push r15
140345fc8: 4156 push r14
140345fca: 4154 push r12
140345fcc: 56 push rsi
140345fcd: 57 push rdi
140345fce: 53 push rbx
140345fcf: 4881ec90000000 sub rsp, 0x90
140345fd6: 488daa80000000 lea rbp, [rdx + 0x80]
140345fdd: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140345fe3: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140345fe9: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
140345fef: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140345ff5: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140345ffb: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140346000: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140346008: 488d8db0020000 lea rcx, [rbp + 0x2b0]
14034600f: e85c3ce0ff call 0x140149c70 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::tree_storage::TreeStorage> | core::ptr::drop_in_place<bevy_utils::synccell::SyncCell<system_wall_constructor::construct_floors::FloorStorage> > 
140346014: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
14034601c: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140346021: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140346027: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
14034602d: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140346033: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140346039: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
14034603f: 4881c490000000 add rsp, 0x90
140346046: 5b pop rbx
140346047: 5f pop rdi
140346048: 5e pop rsi
140346049: 415c pop r12
14034604b: 415e pop r14
14034604d: 415f pop r15
14034604f: 5d pop rbp
140346050: c3 ret 
140346051: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140346060: 4889542410 mov qword ptr [rsp + 0x10], rdx
140346065: 55 push rbp
140346066: 4157 push r15
140346068: 4156 push r14
14034606a: 4154 push r12
14034606c: 56 push rsi
14034606d: 57 push rdi
14034606e: 53 push rbx
14034606f: 4881ec90000000 sub rsp, 0x90
140346076: 488daa80000000 lea rbp, [rdx + 0x80]
14034607d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140346083: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140346089: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14034608f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140346095: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
14034609b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
1403460a0: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
1403460a8: 488d8df0000000 lea rcx, [rbp + 0xf0]
1403460af: 41b820000000 mov r8d, 0x20
1403460b5: 41b910000000 mov r9d, 0x10
1403460bb: 488d9510010000 lea rdx, [rbp + 0x110]
1403460c2: e8f970d0ff call 0x14004d1c0 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::stairs::stairs_assembly_params::StairGraphEdgeIdentity,alloc::vec::Vec<country_core::resources::stairs_walls::StairsPillarProposal,alloc::al
1403460c7: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
1403460cf: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
1403460d4: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
1403460da: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
1403460e0: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
1403460e6: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
1403460ec: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
1403460f2: 4881c490000000 add rsp, 0x90
1403460f9: 5b pop rbx
1403460fa: 5f pop rdi
1403460fb: 5e pop rsi
1403460fc: 415c pop r12
1403460fe: 415e pop r14
140346100: 415f pop r15
140346102: 5d pop rbp
140346103: c3 ret 
140346104: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140346110: 4889542410 mov qword ptr [rsp + 0x10], rdx
140346115: 55 push rbp
140346116: 4157 push r15
140346118: 4156 push r14
14034611a: 4154 push r12
14034611c: 56 push rsi
14034611d: 57 push rdi
14034611e: 53 push rbx
14034611f: 4881ec90000000 sub rsp, 0x90
140346126: 488daa80000000 lea rbp, [rdx + 0x80]
14034612d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140346133: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140346139: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14034613f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140346145: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
14034614b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140346150: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140346158: 488d8db0050000 lea rcx, [rbp + 0x5b0]
14034615f: e8ec6b2900 call 0x1405dcd50 ; core::ptr::drop_in_place<petgraph::graphmap::GraphMap<country_core::resources::walls::decorator::DecoratorId,tuple$<>,enum2$<petgraph::Undirected>,std::hash::random::RandomState> >
140346164: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
14034616c: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140346171: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140346177: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
14034617d: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140346183: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140346189: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
14034618f: 4881c490000000 add rsp, 0x90
140346196: 5b pop rbx
140346197: 5f pop rdi
140346198: 5e pop rsi
140346199: 415c pop r12
14034619b: 415e pop r14
14034619d: 415f pop r15
14034619f: 5d pop rbp
1403461a0: c3 ret 
1403461a1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1403461b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403461b5: 55 push rbp
1403461b6: 4157 push r15
1403461b8: 4156 push r14
1403461ba: 4154 push r12
1403461bc: 56 push rsi
1403461bd: 57 push rdi
1403461be: 53 push rbx
1403461bf: 4881ec90000000 sub rsp, 0x90
1403461c6: 488daa80000000 lea rbp, [rdx + 0x80]
1403461cd: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
1403461d3: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
1403461d9: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
1403461df: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
1403461e5: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
1403461eb: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
1403461f0: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
1403461f8: 488d8df0000000 lea rcx, [rbp + 0xf0]
1403461ff: e84c6b2900 call 0x1405dcd50 ; core::ptr::drop_in_place<petgraph::graphmap::GraphMap<country_core::resources::walls::decorator::DecoratorId,tuple$<>,enum2$<petgraph::Undirected>,std::hash::random::RandomState> >
140346204: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
14034620c: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140346211: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140346217: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
14034621d: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140346223: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140346229: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
14034622f: 4881c490000000 add rsp, 0x90
140346236: 5b pop rbx
140346237: 5f pop rdi
140346238: 5e pop rsi
140346239: 415c pop r12
14034623b: 415e pop r14
14034623d: 415f pop r15
14034623f: 5d pop rbp
140346240: c3 ret 
140346241: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140346250: 4889542410 mov qword ptr [rsp + 0x10], rdx
140346255: 55 push rbp
140346256: 4157 push r15
140346258: 4156 push r14
14034625a: 4154 push r12
14034625c: 56 push rsi
14034625d: 57 push rdi
14034625e: 53 push rbx
14034625f: 4881ec90000000 sub rsp, 0x90
140346266: 488daa80000000 lea rbp, [rdx + 0x80]
14034626d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140346273: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140346279: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14034627f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140346285: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
14034628b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140346290: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140346298: 488d8db0020000 lea rcx, [rbp + 0x2b0]
14034629f: e8cc712900 call 0x1405dd470 ; core::ptr::drop_in_place<country_core::systems::collision::world::ColliderRegistry<country_core::resources::stairs::stairs_state::StairGraphEdge> >
1403462a4: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
1403462ac: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
1403462b1: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
1403462b7: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
1403462bd: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
1403462c3: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
1403462c9: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
1403462cf: 4881c490000000 add rsp, 0x90
1403462d6: 5b pop rbx
1403462d7: 5f pop rdi
1403462d8: 5e pop rsi
1403462d9: 415c pop r12
1403462db: 415e pop r14
1403462dd: 415f pop r15
1403462df: 5d pop rbp
1403462e0: c3 ret 
1403462e1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1403462f0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403462f5: 55 push rbp
1403462f6: 4157 push r15
1403462f8: 4156 push r14
1403462fa: 4154 push r12
1403462fc: 56 push rsi
1403462fd: 57 push rdi
1403462fe: 53 push rbx
1403462ff: 4881ec90000000 sub rsp, 0x90
140346306: 488daa80000000 lea rbp, [rdx + 0x80]
14034630d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140346313: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140346319: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14034631f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140346325: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
14034632b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140346330: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140346338: 488d8db0050000 lea rcx, [rbp + 0x5b0]
14034633f: e82c712900 call 0x1405dd470 ; core::ptr::drop_in_place<country_core::systems::collision::world::ColliderRegistry<country_core::resources::stairs::stairs_state::StairGraphEdge> >
140346344: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
14034634c: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140346351: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140346357: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
14034635d: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140346363: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140346369: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
14034636f: 4881c490000000 add rsp, 0x90
140346376: 5b pop rbx
140346377: 5f pop rdi
140346378: 5e pop rsi
140346379: 415c pop r12
14034637b: 415e pop r14
14034637d: 415f pop r15
14034637f: 5d pop rbp
140346380: c3 ret 
140346381: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140346390: 4889542410 mov qword ptr [rsp + 0x10], rdx
140346395: 55 push rbp
140346396: 4157 push r15
140346398: 4156 push r14
14034639a: 4154 push r12
14034639c: 56 push rsi
14034639d: 57 push rdi
14034639e: 53 push rbx
14034639f: 4881ec90000000 sub rsp, 0x90
1403463a6: 488daa80000000 lea rbp, [rdx + 0x80]
1403463ad: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
1403463b3: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
1403463b9: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
1403463bf: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
1403463c5: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
1403463cb: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
1403463d0: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
1403463d8: 488d8df0000000 lea rcx, [rbp + 0xf0]
1403463df: e88c702900 call 0x1405dd470 ; core::ptr::drop_in_place<country_core::systems::collision::world::ColliderRegistry<country_core::resources::stairs::stairs_state::StairGraphEdge> >
1403463e4: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
1403463ec: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
1403463f1: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
1403463f7: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
1403463fd: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140346403: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140346409: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
14034640f: 4881c490000000 add rsp, 0x90
140346416: 5b pop rbx
140346417: 5f pop rdi
140346418: 5e pop rsi
140346419: 415c pop r12
14034641b: 415e pop r14
14034641d: 415f pop r15
14034641f: 5d pop rbp
140346420: c3 ret 
140346421: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140346430: 4889542410 mov qword ptr [rsp + 0x10], rdx
140346435: 55 push rbp
140346436: 4157 push r15
140346438: 4156 push r14
14034643a: 4154 push r12
14034643c: 56 push rsi
14034643d: 57 push rdi
14034643e: 53 push rbx
14034643f: 4881ec90000000 sub rsp, 0x90
140346446: 488daa80000000 lea rbp, [rdx + 0x80]
14034644d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140346453: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140346459: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14034645f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140346465: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
14034646b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140346470: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140346478: 488d4d10 lea rcx, [rbp + 0x10]
14034647c: e8df3ee0ff call 0x14014a360 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::window_lights::GlobalWindowLightsControl> | core::ptr::drop_in_place<country_core::systems::collision::world::ColliderRegistry<country_core::res
140346481: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
140346489: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
14034648e: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140346494: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
14034649a: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
1403464a0: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
1403464a6: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
1403464ac: 4881c490000000 add rsp, 0x90
1403464b3: 5b pop rbx
1403464b4: 5f pop rdi
1403464b5: 5e pop rsi
1403464b6: 415c pop r12
1403464b8: 415e pop r14
1403464ba: 415f pop r15
1403464bc: 5d pop rbp
1403464bd: c3 ret 
1403464be: 6690 nop 
1403464c0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403464c5: 55 push rbp
1403464c6: 4157 push r15
1403464c8: 4156 push r14
1403464ca: 4154 push r12
1403464cc: 56 push rsi
1403464cd: 57 push rdi
1403464ce: 53 push rbx
1403464cf: 4881ec90000000 sub rsp, 0x90
1403464d6: 488daa80000000 lea rbp, [rdx + 0x80]
1403464dd: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
1403464e3: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
1403464e9: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
1403464ef: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
1403464f5: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
1403464fb: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140346500: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140346508: 488d8d80000000 lea rcx, [rbp + 0x80]
14034650f: e84c3ee0ff call 0x14014a360 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::window_lights::GlobalWindowLightsControl> | core::ptr::drop_in_place<country_core::systems::collision::world::ColliderRegistry<country_core::res
140346514: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
14034651c: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140346521: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140346527: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
14034652d: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140346533: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140346539: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
14034653f: 4881c490000000 add rsp, 0x90
140346546: 5b pop rbx
140346547: 5f pop rdi
140346548: 5e pop rsi
140346549: 415c pop r12
14034654b: 415e pop r14
14034654d: 415f pop r15
14034654f: 5d pop rbp
140346550: c3 ret 
140346551: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140346560: 4889542410 mov qword ptr [rsp + 0x10], rdx
140346565: 55 push rbp
140346566: 4157 push r15
140346568: 4156 push r14
14034656a: 4154 push r12
14034656c: 56 push rsi
14034656d: 57 push rdi
14034656e: 53 push rbx
14034656f: 4881ec90000000 sub rsp, 0x90
140346576: 488daa80000000 lea rbp, [rdx + 0x80]
14034657d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140346583: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140346589: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14034658f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140346595: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
14034659b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
1403465a0: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
1403465a8: 488d8db0020000 lea rcx, [rbp + 0x2b0]
1403465af: e8fcb12900 call 0x1405e17b0 ; core::ptr::drop_in_place<std::collections::hash::map::HashMap<country_core::resources::walls::decorator::DecoratorId,tuple$<f32,glam::f32::vec3::Vec3,glam::f32::sse2::quat::Quat>,std::hash::random::RandomState> > | core:
1403465b4: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
1403465bc: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
1403465c1: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
1403465c7: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
1403465cd: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
1403465d3: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
1403465d9: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
1403465df: 4881c490000000 add rsp, 0x90
1403465e6: 5b pop rbx
1403465e7: 5f pop rdi
1403465e8: 5e pop rsi
1403465e9: 415c pop r12
1403465eb: 415e pop r14
1403465ed: 415f pop r15
1403465ef: 5d pop rbp
1403465f0: c3 ret 
1403465f1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140346600: 4889542410 mov qword ptr [rsp + 0x10], rdx
140346605: 55 push rbp
140346606: 4157 push r15
140346608: 4156 push r14
14034660a: 4154 push r12
14034660c: 56 push rsi
14034660d: 57 push rdi
14034660e: 53 push rbx
14034660f: 4881ec90000000 sub rsp, 0x90
140346616: 488daa80000000 lea rbp, [rdx + 0x80]
14034661d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140346623: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140346629: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14034662f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140346635: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
14034663b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140346640: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140346648: 488d8db0050000 lea rcx, [rbp + 0x5b0]
14034664f: 41b860000000 mov r8d, 0x60
140346655: 41b910000000 mov r9d, 0x10
14034665b: 488d95d0050000 lea rdx, [rbp + 0x5d0]
140346662: e889bfd0ff call 0x1400525f0 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::walls::decorator::DecoratorId,system_decorator::ui_systems::ui_stairs_viz::WidthPointGizmo>,alloc::alloc::Global> | hashbrown::raw::RawTable
140346667: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
14034666f: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140346674: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
14034667a: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140346680: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140346686: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
14034668c: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140346692: 4881c490000000 add rsp, 0x90
140346699: 5b pop rbx
14034669a: 5f pop rdi
14034669b: 5e pop rsi
14034669c: 415c pop r12
14034669e: 415e pop r14
1403466a0: 415f pop r15
1403466a2: 5d pop rbp
1403466a3: c3 ret 
1403466a4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1403466b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403466b5: 55 push rbp
1403466b6: 4157 push r15
1403466b8: 4156 push r14
1403466ba: 4154 push r12
1403466bc: 56 push rsi
1403466bd: 57 push rdi
1403466be: 53 push rbx
1403466bf: 4881ec90000000 sub rsp, 0x90
1403466c6: 488daa80000000 lea rbp, [rdx + 0x80]
1403466cd: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
1403466d3: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
1403466d9: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
1403466df: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
1403466e5: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
1403466eb: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
1403466f0: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
1403466f8: 488d8df0000000 lea rcx, [rbp + 0xf0]
1403466ff: e8ecd02900 call 0x1405e37f0 ; core::ptr::drop_in_place<petgraph::graphmap::GraphMap<country_core::systems::collision::world::SubworldColliderHandle,country_core::resources::stairs::structural_graph::StructuralGraphConnection,enum2$<petgraph::Undirect
140346704: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
14034670c: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140346711: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140346717: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
14034671d: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140346723: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140346729: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
14034672f: 4881c490000000 add rsp, 0x90
140346736: 5b pop rbx
140346737: 5f pop rdi
140346738: 5e pop rsi
140346739: 415c pop r12
14034673b: 415e pop r14
14034673d: 415f pop r15
14034673f: 5d pop rbp
140346740: c3 ret 
140346741: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140346750: 4889542410 mov qword ptr [rsp + 0x10], rdx
140346755: 55 push rbp
140346756: 4157 push r15
140346758: 4156 push r14
14034675a: 4154 push r12
14034675c: 56 push rsi
14034675d: 57 push rdi
14034675e: 53 push rbx
14034675f: 4881ec90000000 sub rsp, 0x90
140346766: 488daa80000000 lea rbp, [rdx + 0x80]
14034676d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140346773: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140346779: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14034677f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140346785: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
14034678b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140346790: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140346798: 488d4de0 lea rcx, [rbp - 0x20]
14034679c: e8bf3be0ff call 0x14014a360 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::window_lights::GlobalWindowLightsControl> | core::ptr::drop_in_place<country_core::systems::collision::world::ColliderRegistry<country_core::res
1403467a1: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
1403467a9: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
1403467ae: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
1403467b4: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
1403467ba: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
1403467c0: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
1403467c6: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
1403467cc: 4881c490000000 add rsp, 0x90
1403467d3: 5b pop rbx
1403467d4: 5f pop rdi
1403467d5: 5e pop rsi
1403467d6: 415c pop r12
1403467d8: 415e pop r14
1403467da: 415f pop r15
1403467dc: 5d pop rbp
1403467dd: c3 ret 
1403467de: 6690 nop 
1403467e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403467e5: 55 push rbp
1403467e6: 4157 push r15
1403467e8: 4156 push r14
1403467ea: 4154 push r12
1403467ec: 56 push rsi
1403467ed: 57 push rdi
1403467ee: 53 push rbx
1403467ef: 4881ec90000000 sub rsp, 0x90
1403467f6: 488daa80000000 lea rbp, [rdx + 0x80]
1403467fd: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140346803: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140346809: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14034680f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140346815: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
14034681b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140346820: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140346828: 488d4d10 lea rcx, [rbp + 0x10]
14034682c: e82f3be0ff call 0x14014a360 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::window_lights::GlobalWindowLightsControl> | core::ptr::drop_in_place<country_core::systems::collision::world::ColliderRegistry<country_core::res
140346831: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
140346839: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
14034683e: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140346844: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
14034684a: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140346850: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140346856: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
14034685c: 4881c490000000 add rsp, 0x90
140346863: 5b pop rbx
140346864: 5f pop rdi
140346865: 5e pop rsi
140346866: 415c pop r12
140346868: 415e pop r14
14034686a: 415f pop r15
14034686c: 5d pop rbp
14034686d: c3 ret 
14034686e: 6690 nop 
140346870: 4889542410 mov qword ptr [rsp + 0x10], rdx
140346875: 55 push rbp
140346876: 4157 push r15
140346878: 4156 push r14
14034687a: 4154 push r12
14034687c: 56 push rsi
14034687d: 57 push rdi
14034687e: 53 push rbx
14034687f: 4881ec90000000 sub rsp, 0x90
140346886: 488daa80000000 lea rbp, [rdx + 0x80]
14034688d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140346893: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140346899: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14034689f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
1403468a5: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
1403468ab: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
1403468b0: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
1403468b8: 488d8d80000000 lea rcx, [rbp + 0x80]
1403468bf: e8ac33e0ff call 0x140149c70 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::tree_storage::TreeStorage> | core::ptr::drop_in_place<bevy_utils::synccell::SyncCell<system_wall_constructor::construct_floors::FloorStorage> > 
1403468c4: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
1403468cc: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
1403468d1: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
1403468d7: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
1403468dd: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
1403468e3: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
1403468e9: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
1403468ef: 4881c490000000 add rsp, 0x90
1403468f6: 5b pop rbx
1403468f7: 5f pop rdi
1403468f8: 5e pop rsi
1403468f9: 415c pop r12
1403468fb: 415e pop r14
1403468fd: 415f pop r15
1403468ff: 5d pop rbp
140346900: c3 ret 
140346901: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140346910: 4889542410 mov qword ptr [rsp + 0x10], rdx
140346915: 55 push rbp
140346916: 4157 push r15
140346918: 4156 push r14
14034691a: 4154 push r12
14034691c: 56 push rsi
14034691d: 57 push rdi
14034691e: 53 push rbx
14034691f: 4881ec90000000 sub rsp, 0x90
140346926: 488daa80000000 lea rbp, [rdx + 0x80]
14034692d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140346933: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140346939: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14034693f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140346945: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
14034694b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140346950: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140346958: 488d8db0020000 lea rcx, [rbp + 0x2b0]
14034695f: e80c33e0ff call 0x140149c70 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::tree_storage::TreeStorage> | core::ptr::drop_in_place<bevy_utils::synccell::SyncCell<system_wall_constructor::construct_floors::FloorStorage> > 
140346964: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
14034696c: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140346971: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140346977: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
14034697d: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140346983: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140346989: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
14034698f: 4881c490000000 add rsp, 0x90
140346996: 5b pop rbx
140346997: 5f pop rdi
140346998: 5e pop rsi
140346999: 415c pop r12
14034699b: 415e pop r14
14034699d: 415f pop r15
14034699f: 5d pop rbp
1403469a0: c3 ret 
1403469a1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1403469b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403469b5: 55 push rbp
1403469b6: 4157 push r15
1403469b8: 4156 push r14
1403469ba: 4154 push r12
1403469bc: 56 push rsi
1403469bd: 57 push rdi
1403469be: 53 push rbx
1403469bf: 4881ec90000000 sub rsp, 0x90
1403469c6: 488daa80000000 lea rbp, [rdx + 0x80]
1403469cd: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
1403469d3: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
1403469d9: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
1403469df: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
1403469e5: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
1403469eb: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
1403469f0: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
1403469f8: 488d8db0050000 lea rcx, [rbp + 0x5b0]
1403469ff: e84c4ce0ff call 0x14014b650 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::stairs::stairs_state::TransientStairEdgeIdAllocator> | core::ptr::drop_in_place<std::collections::hash::set::HashSet<country_core::systems::coll
140346a04: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
140346a0c: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140346a11: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140346a17: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140346a1d: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140346a23: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140346a29: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140346a2f: 4881c490000000 add rsp, 0x90
140346a36: 5b pop rbx
140346a37: 5f pop rdi
140346a38: 5e pop rsi
140346a39: 415c pop r12
140346a3b: 415e pop r14
140346a3d: 415f pop r15
140346a3f: 5d pop rbp
140346a40: c3 ret 
140346a41: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140346a50: 4889542410 mov qword ptr [rsp + 0x10], rdx
140346a55: 55 push rbp
140346a56: 4157 push r15
140346a58: 4156 push r14
140346a5a: 4154 push r12
140346a5c: 56 push rsi
140346a5d: 57 push rdi
140346a5e: 53 push rbx
140346a5f: 4881ec90000000 sub rsp, 0x90
140346a66: 488daa80000000 lea rbp, [rdx + 0x80]
140346a6d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140346a73: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140346a79: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
140346a7f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140346a85: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140346a8b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140346a90: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140346a98: 488d8df0000000 lea rcx, [rbp + 0xf0]
140346a9f: e8cc31e0ff call 0x140149c70 ; bevy_ecs::component::ComponentDescriptor::drop_ptr<country_core::resources::tree_storage::TreeStorage> | core::ptr::drop_in_place<bevy_utils::synccell::SyncCell<system_wall_constructor::construct_floors::FloorStorage> > 
140346aa4: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
140346aac: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140346ab1: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140346ab7: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140346abd: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140346ac3: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140346ac9: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140346acf: 4881c490000000 add rsp, 0x90
140346ad6: 5b pop rbx
140346ad7: 5f pop rdi
140346ad8: 5e pop rsi
140346ad9: 415c pop r12
140346adb: 415e pop r14
140346add: 415f pop r15
140346adf: 5d pop rbp
140346ae0: c3 ret 
140346ae1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140346af0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140346af5: 55 push rbp
140346af6: 4157 push r15
140346af8: 4156 push r14
140346afa: 4154 push r12
140346afc: 56 push rsi
140346afd: 57 push rdi
140346afe: 53 push rbx
140346aff: 4881ec90000000 sub rsp, 0x90
140346b06: 488daa80000000 lea rbp, [rdx + 0x80]
140346b0d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140346b13: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140346b19: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
140346b1f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140346b25: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140346b2b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140346b30: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140346b38: 488d8d80000000 lea rcx, [rbp + 0x80]
140346b3f: 41b878000000 mov r8d, 0x78
140346b45: 41b910000000 mov r9d, 0x10
140346b4b: 488d95a0000000 lea rdx, [rbp + 0xa0]
140346b52: e8e958d0ff call 0x14004c440 ; hashbrown::raw::RawTableInner::drop_inner_table<tuple$<country_core::resources::stairs::stairs_assembly_params::StairGraphEdgeIdentity,country_core::resources::stairs::stairs_assembly_params::SampledStairsPoints>,alloc::
140346b57: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
140346b5f: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140346b64: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140346b6a: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140346b70: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140346b76: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140346b7c: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140346b82: 4881c490000000 add rsp, 0x90
140346b89: 5b pop rbx
140346b8a: 5f pop rdi
140346b8b: 5e pop rsi
140346b8c: 415c pop r12
140346b8e: 415e pop r14
140346b90: 415f pop r15
140346b92: 5d pop rbp
140346b93: c3 ret 
140346b94: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140346ba0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140346ba5: 55 push rbp
140346ba6: 4157 push r15
140346ba8: 4156 push r14
140346baa: 4154 push r12
140346bac: 56 push rsi
140346bad: 57 push rdi
140346bae: 53 push rbx
140346baf: 4881ec90000000 sub rsp, 0x90
140346bb6: 488daa80000000 lea rbp, [rdx + 0x80]
140346bbd: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140346bc3: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140346bc9: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
140346bcf: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140346bd5: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140346bdb: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140346be0: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140346be8: 488d8db0050000 lea rcx, [rbp + 0x5b0]
140346bef: e8bcb82900 call 0x1405e24b0 ; core::ptr::drop_in_place<petgraph::graphmap::GraphMap<country_core::resources::walls::decorator::DecoratorId,enum2$<country_core::resources::stairs::stairs_assembly_params::AlignKind>,enum2$<petgraph::Directed>,std::hash
140346bf4: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
140346bfc: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140346c01: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140346c07: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140346c0d: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140346c13: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140346c19: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140346c1f: 4881c490000000 add rsp, 0x90
140346c26: 5b pop rbx
140346c27: 5f pop rdi
140346c28: 5e pop rsi
140346c29: 415c pop r12
140346c2b: 415e pop r14
140346c2d: 415f pop r15
140346c2f: 5d pop rbp
140346c30: c3 ret 
140346c31: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140346c40: 4889542410 mov qword ptr [rsp + 0x10], rdx
140346c45: 55 push rbp
140346c46: 4157 push r15
140346c48: 4156 push r14
140346c4a: 4154 push r12
140346c4c: 56 push rsi
140346c4d: 57 push rdi
140346c4e: 53 push rbx
140346c4f: 4881ec90000000 sub rsp, 0x90
140346c56: 488daa80000000 lea rbp, [rdx + 0x80]
140346c5d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140346c63: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140346c69: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
140346c6f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140346c75: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140346c7b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140346c80: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140346c88: 488d4d10 lea rcx, [rbp + 0x10]
140346c8c: e8ef2bceff call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::be
140346c91: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
140346c99: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140346c9e: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140346ca4: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140346caa: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140346cb0: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140346cb6: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140346cbc: 4881c490000000 add rsp, 0x90
140346cc3: 5b pop rbx
140346cc4: 5f pop rdi
140346cc5: 5e pop rsi
140346cc6: 415c pop r12
140346cc8: 415e pop r14
140346cca: 415f pop r15
140346ccc: 5d pop rbp
140346ccd: c3 ret 
140346cce: 6690 nop 
140346cd0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140346cd5: 55 push rbp
140346cd6: 4157 push r15
140346cd8: 4156 push r14
140346cda: 4154 push r12
140346cdc: 56 push rsi
140346cdd: 57 push rdi
140346cde: 53 push rbx
140346cdf: 4881ec90000000 sub rsp, 0x90
140346ce6: 488daa80000000 lea rbp, [rdx + 0x80]
140346ced: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140346cf3: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140346cf9: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
140346cff: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140346d05: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140346d0b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140346d10: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140346d18: 488d8db0050000 lea rcx, [rbp + 0x5b0]
140346d1f: e85c2bceff call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::be
140346d24: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
140346d2c: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140346d31: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140346d37: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140346d3d: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140346d43: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140346d49: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140346d4f: 4881c490000000 add rsp, 0x90
140346d56: 5b pop rbx
140346d57: 5f pop rdi
140346d58: 5e pop rsi
140346d59: 415c pop r12
140346d5b: 415e pop r14
140346d5d: 415f pop r15
140346d5f: 5d pop rbp
140346d60: c3 ret 
140346d61: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140346d70: 4889542410 mov qword ptr [rsp + 0x10], rdx
140346d75: 55 push rbp
140346d76: 4157 push r15
140346d78: 4156 push r14
140346d7a: 4154 push r12
140346d7c: 56 push rsi
140346d7d: 57 push rdi
140346d7e: 53 push rbx
140346d7f: 4881ec90000000 sub rsp, 0x90
140346d86: 488daa80000000 lea rbp, [rdx + 0x80]
140346d8d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140346d93: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140346d99: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
140346d9f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140346da5: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140346dab: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140346db0: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140346db8: 488d8d80000000 lea rcx, [rbp + 0x80]
140346dbf: e8bc2aceff call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::be
140346dc4: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
140346dcc: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140346dd1: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140346dd7: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140346ddd: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140346de3: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140346de9: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140346def: 4881c490000000 add rsp, 0x90
140346df6: 5b pop rbx
140346df7: 5f pop rdi
140346df8: 5e pop rsi
140346df9: 415c pop r12
140346dfb: 415e pop r14
140346dfd: 415f pop r15
140346dff: 5d pop rbp
140346e00: c3 ret 
140346e01: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140346e10: 4889542410 mov qword ptr [rsp + 0x10], rdx
140346e15: 55 push rbp
140346e16: 4157 push r15
140346e18: 4156 push r14
140346e1a: 4154 push r12
140346e1c: 56 push rsi
140346e1d: 57 push rdi
140346e1e: 53 push rbx
140346e1f: 4881ec90000000 sub rsp, 0x90
140346e26: 488daa80000000 lea rbp, [rdx + 0x80]
140346e2d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140346e33: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140346e39: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
140346e3f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140346e45: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140346e4b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140346e50: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140346e58: 488d8db0020000 lea rcx, [rbp + 0x2b0]
140346e5f: e81c2aceff call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::be
140346e64: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
140346e6c: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140346e71: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140346e77: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140346e7d: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140346e83: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140346e89: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140346e8f: 4881c490000000 add rsp, 0x90
140346e96: 5b pop rbx
140346e97: 5f pop rdi
140346e98: 5e pop rsi
140346e99: 415c pop r12
140346e9b: 415e pop r14
140346e9d: 415f pop r15
140346e9f: 5d pop rbp
140346ea0: c3 ret 
140346ea1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140346eb0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140346eb5: 55 push rbp
140346eb6: 4157 push r15
140346eb8: 4156 push r14
140346eba: 4154 push r12
140346ebc: 56 push rsi
140346ebd: 57 push rdi
140346ebe: 53 push rbx
140346ebf: 4881ec90000000 sub rsp, 0x90
140346ec6: 488daa80000000 lea rbp, [rdx + 0x80]
140346ecd: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140346ed3: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140346ed9: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
140346edf: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140346ee5: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140346eeb: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140346ef0: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140346ef8: 488d8db0050000 lea rcx, [rbp + 0x5b0]
140346eff: e87c29ceff call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::be
140346f04: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
140346f0c: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140346f11: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140346f17: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140346f1d: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140346f23: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140346f29: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140346f2f: 4881c490000000 add rsp, 0x90
140346f36: 5b pop rbx
140346f37: 5f pop rdi
140346f38: 5e pop rsi
140346f39: 415c pop r12
140346f3b: 415e pop r14
140346f3d: 415f pop r15
140346f3f: 5d pop rbp
140346f40: c3 ret 
140346f41: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140346f50: 4889542410 mov qword ptr [rsp + 0x10], rdx
140346f55: 55 push rbp
140346f56: 4157 push r15
140346f58: 4156 push r14
140346f5a: 4154 push r12
140346f5c: 56 push rsi
140346f5d: 57 push rdi
140346f5e: 53 push rbx
140346f5f: 4881ec90000000 sub rsp, 0x90
140346f66: 488daa80000000 lea rbp, [rdx + 0x80]
140346f6d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140346f73: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140346f79: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
140346f7f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140346f85: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140346f8b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140346f90: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140346f98: 488d8df0000000 lea rcx, [rbp + 0xf0]
140346f9f: e8dc28ceff call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::be
140346fa4: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
140346fac: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140346fb1: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140346fb7: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
140346fbd: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140346fc3: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140346fc9: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
140346fcf: 4881c490000000 add rsp, 0x90
140346fd6: 5b pop rbx
140346fd7: 5f pop rdi
140346fd8: 5e pop rsi
140346fd9: 415c pop r12
140346fdb: 415e pop r14
140346fdd: 415f pop r15
140346fdf: 5d pop rbp
140346fe0: c3 ret 
140346fe1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140346ff0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140346ff5: 55 push rbp
140346ff6: 4157 push r15
140346ff8: 4156 push r14
140346ffa: 4154 push r12
140346ffc: 56 push rsi
140346ffd: 57 push rdi
140346ffe: 53 push rbx
140346fff: 4881ec90000000 sub rsp, 0x90
140347006: 488daa80000000 lea rbp, [rdx + 0x80]
14034700d: 440f29642420 movaps xmmword ptr [rsp + 0x20], xmm12
140347013: 440f295c2430 movaps xmmword ptr [rsp + 0x30], xmm11
140347019: 440f29542440 movaps xmmword ptr [rsp + 0x40], xmm10
14034701f: 440f294c2450 movaps xmmword ptr [rsp + 0x50], xmm9
140347025: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
14034702b: 0f297c2470 movaps xmmword ptr [rsp + 0x70], xmm7
140347030: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
140347038: 488d8db0050000 lea rcx, [rbp + 0x5b0]
14034703f: e83c28ceff call 0x140029880 ; core::ptr::drop_in_place<enum2$<bevy_ecs::schedule::config::ScheduleConfigs<alloc::boxed::Box<dyn$<bevy_ecs::system::system::System<assoc$<In,tuple$<> >,assoc$<Out,enum2$<core::result::Result<tuple$<>,bevy_ecs::error::be
140347044: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
14034704c: 0f287c2470 movaps xmm7, xmmword ptr [rsp + 0x70]
140347051: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140347057: 440f284c2450 movaps xmm9, xmmword ptr [rsp + 0x50]
14034705d: 440f28542440 movaps xmm10, xmmword ptr [rsp + 0x40]
140347063: 440f285c2430 movaps xmm11, xmmword ptr [rsp + 0x30]
140347069: 440f28642420 movaps xmm12, xmmword ptr [rsp + 0x20]
14034706f: 4881c490000000 add rsp, 0x90
140347076: 5b pop rbx
140347077: 5f pop rdi
140347078: 5e pop rsi
140347079: 415c pop r12
14034707b: 415e pop r14
14034707d: 415f pop r15
14034707f: 5d pop rbp
140347080: c3 ret 
