141553120: 55 push rbp
141553121: 4157 push r15
141553123: 4156 push r14
141553125: 4155 push r13
141553127: 4154 push r12
141553129: 56 push rsi
14155312a: 57 push rdi
14155312b: 53 push rbx
14155312c: 4881ecd8010000 sub rsp, 0x1d8
141553133: 488dac2480000000 lea rbp, [rsp + 0x80]
14155313b: 440f299d40010000 movaps xmmword ptr [rbp + 0x140], xmm11
141553143: 440f299530010000 movaps xmmword ptr [rbp + 0x130], xmm10
14155314b: 440f298d20010000 movaps xmmword ptr [rbp + 0x120], xmm9
141553153: 440f298510010000 movaps xmmword ptr [rbp + 0x110], xmm8
14155315b: 0f29bd00010000 movaps xmmword ptr [rbp + 0x100], xmm7
141553162: 0f29b5f0000000 movaps xmmword ptr [rbp + 0xf0], xmm6
141553169: 48c785e8000000feffffff mov qword ptr [rbp + 0xe8], 0xfffffffffffffffe
141553174: 4c89cb mov rbx, r9
141553177: 4c8985d8000000 mov qword ptr [rbp + 0xd8], r8
14155317e: 4989d6 mov r14, rdx
141553181: 4889cf mov rdi, rcx
141553184: 488d154d967101 lea rdx, [rip + 0x171964d] ; @142c6c7d8 'assets/meshes/billboard_plants_2.jsoncrates/country-core/src/startup/glade_themes/flowery.rs'
14155318b: 488d4d60 lea rcx, [rbp + 0x60]
14155318f: 41b825000000 mov r8d, 0x25
141553195: e816bd5bff call 0x140b0eeb0 ; country_core::utils::load_json::load_json_as_mesh<ref$<str$> >
14155319a: 31c0 xor eax, eax
14155319c: 483b4560 cmp rax, qword ptr [rbp + 0x60]
1415531a0: 0f8071050000 jo 0x141553717
1415531a6: 488b85d0000000 mov rax, qword ptr [rbp + 0xd0]
1415531ad: 48894540 mov qword ptr [rbp + 0x40], rax
1415531b1: 0f1085c0000000 movups xmm0, xmmword ptr [rbp + 0xc0]
1415531b8: 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
1415531bc: 0f1085b0000000 movups xmm0, xmmword ptr [rbp + 0xb0]
1415531c3: 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
1415531c7: 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
1415531ce: 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
1415531d2: 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
1415531d6: 0f104d70 movups xmm1, xmmword ptr [rbp + 0x70]
1415531da: 0f109580000000 movups xmm2, xmmword ptr [rbp + 0x80]
1415531e1: 0f109d90000000 movups xmm3, xmmword ptr [rbp + 0x90]
1415531e8: 0f295d00 movaps xmmword ptr [rbp], xmm3
1415531ec: 0f2955f0 movaps xmmword ptr [rbp - 0x10], xmm2
1415531f0: 0f294de0 movaps xmmword ptr [rbp - 0x20], xmm1
1415531f4: 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
1415531f8: 488b37 mov rsi, qword ptr [rdi]
1415531fb: 4d8b26 mov r12, qword ptr [r14]
1415531fe: 4c89642420 mov qword ptr [rsp + 0x20], r12
141553203: 488d1546967101 lea rdx, [rip + 0x1719646]
14155320a: 488d4dd0 lea rcx, [rbp - 0x30]
14155320e: 41b812000000 mov r8d, 0x12
141553214: 4989f1 mov r9, rsi
141553217: e8847a5cff call 0x140b1aca0 ; country_core::startup::load_mesh_into_library
14155321c: 4989c6 mov r14, rax
14155321f: 488b3b mov rdi, qword ptr [rbx]
141553222: 488d1539967101 lea rdx, [rip + 0x1719639]
141553229: 41b816000000 mov r8d, 0x16
14155322f: 4889f9 mov rcx, rdi
141553232: e869483eff call 0x140937aa0 ; country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
141553237: a801 test al, 1
141553239: 0f840a050000 je 0x141553749
14155323f: 4989d7 mov r15, rdx
141553242: 4c896558 mov qword ptr [rbp + 0x58], r12
141553246: 48897d50 mov qword ptr [rbp + 0x50], rdi
14155324a: 48897548 mov qword ptr [rbp + 0x48], rsi
14155324e: 488d4d60 lea rcx, [rbp + 0x60]
141553252: 488b95d8000000 mov rdx, qword ptr [rbp + 0xd8]
141553259: e842147cff call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
14155325e: 4c8b6568 mov r12, qword ptr [rbp + 0x68]
141553262: 4d85e4 test r12, r12
141553265: 7504 jne 0x14155326b
141553267: 4c8b6570 mov r12, qword ptr [rbp + 0x70]
14155326b: 488bb5d0010000 mov rsi, qword ptr [rbp + 0x1d0]
141553272: 488bbdc0010000 mov rdi, qword ptr [rbp + 0x1c0]
141553279: 488b9d80000000 mov rbx, qword ptr [rbp + 0x80]
141553280: 498b0424 mov rax, qword ptr [r12]
141553284: 4d8b6c2410 mov r13, qword ptr [r12 + 0x10]
141553289: 4c29e8 sub rax, r13
14155328c: 4883f877 cmp rax, 0x77
141553290: 0f8637040000 jbe 0x1415536cd
141553296: 498b442408 mov rax, qword ptr [r12 + 8]
14155329b: 488d0d3e6bd8ff lea rcx, [rip - 0x2794c2]
1415532a2: 4a890c28 mov qword ptr [rax + r13], rcx
1415532a6: 440f280512ae3e01 movaps xmm8, xmmword ptr [rip + 0x13eae12]
1415532ae: 460f11442808 movups xmmword ptr [rax + r13 + 8], xmm8
1415532b4: 4ac744281800000000 mov qword ptr [rax + r13 + 0x18], 0
1415532bd: 42c744282000000000 mov dword ptr [rax + r13 + 0x20], 0
1415532c6: 48b90000803f0000803f movabs rcx, 0x3f8000003f800000
1415532d0: 4a894c2824 mov qword ptr [rax + r13 + 0x24], rcx
1415532d5: 42c744282c0000803f mov dword ptr [rax + r13 + 0x2c], 0x3f800000
1415532de: 4e89742838 mov qword ptr [rax + r13 + 0x38], r14
1415532e3: 4e897c2840 mov qword ptr [rax + r13 + 0x40], r15
1415532e8: 42c644284800 mov byte ptr [rax + r13 + 0x48], 0
1415532ee: 4a895c2858 mov qword ptr [rax + r13 + 0x58], rbx
1415532f3: 488d0d361a0100 lea rcx, [rip + 0x11a36]
1415532fa: 4a894c2868 mov qword ptr [rax + r13 + 0x68], rcx
1415532ff: 4983c578 add r13, 0x78
141553303: 4d896c2410 mov qword ptr [r12 + 0x10], r13
141553308: 8b471c mov eax, dword ptr [rdi + 0x1c]
14155330b: 488b4f10 mov rcx, qword ptr [rdi + 0x10]
14155330f: 8901 mov dword ptr [rcx], eax
141553311: 4c8b37 mov r14, qword ptr [rdi]
141553314: 488b06 mov rax, qword ptr [rsi]
141553317: 0fb64058 movzx eax, byte ptr [rax + 0x58]
14155331b: c1e003 shl eax, 3
14155331e: 488d0da3987101 lea rcx, [rip + 0x17198a3]
141553325: 4c8b0c08 mov r9, qword ptr [rax + rcx]
141553329: 488d0dc0987101 lea rcx, [rip + 0x17198c0]
141553330: 488b0408 mov rax, qword ptr [rax + rcx]
141553334: 4889442420 mov qword ptr [rsp + 0x20], rax
141553339: 488d15dac15901 lea rdx, [rip + 0x159c1da] ; @142aef51a 'assets/gladestatecrates/country-core/src/resources/session.rsloading_background.jpg'
141553340: 4c8d7dd0 lea r15, [rbp - 0x30]
141553344: 41b80c000000 mov r8d, 0xc
14155334a: 4c89f9 mov rcx, r15
14155334d: e8bef480ff call 0x140d62810 ; std::path::Path::_join
141553352: 4c89f9 mov rcx, r15
141553355: e8763b82ff call 0x140d76ed0 ; std::sys::os_str::wtf8::Buf::as_mut_slice | alloc::boxed::convert::impl$19::from::impl$0::description
14155335a: 4989d0 mov r8, rdx
14155335d: 48c744242014000000 mov qword ptr [rsp + 0x20], 0x14
141553366: 4c8d0d23957101 lea r9, [rip + 0x1719523] ; @142c6c890 'plants/lavender.jsonassets/meshes/meadow_lowpoly_flowers.json'
14155336d: 488d4d60 lea rcx, [rbp + 0x60]
141553371: 4889c2 mov rdx, rax
141553374: e897f480ff call 0x140d62810 ; std::path::Path::_join
141553379: 4c8b85c8010000 mov r8, qword ptr [rbp + 0x1c8]
141553380: 488d4db0 lea rcx, [rbp - 0x50]
141553384: 4c8d4d60 lea r9, [rbp + 0x60]
141553388: 4c89f2 mov rdx, r14
14155338b: e840323eff call 0x1409365d0 ; country_core::asset_libraries::placement_data_library::AssetPlacementDataLibrary::load<std::path::PathBuf>
141553390: 488b45b8 mov rax, qword ptr [rbp - 0x48]
141553394: 488985e0000000 mov qword ptr [rbp + 0xe0], rax
14155339b: 488b45c0 mov rax, qword ptr [rbp - 0x40]
14155339f: 4885c0 test rax, rax
1415533a2: 0f8429010000 je 0x1415534d1
1415533a8: 48c1e004 shl rax, 4
1415533ac: 4c8d2c40 lea r13, [rax + rax*2]
1415533b0: 31db xor ebx, ebx
1415533b2: f3440f100db5243d01 movss xmm9, dword ptr [rip + 0x13d24b5]
1415533bb: 450f57d2 xorps xmm10, xmm10
1415533bf: 488d3dea77d8ff lea rdi, [rip - 0x278816]
1415533c6: 48bee8dedd8fc553fa13 movabs rsi, 0x13fa53c58fdddee8
1415533d0: 4c8bb5e0000000 mov r14, qword ptr [rbp + 0xe0]
1415533d7: f3410f107c1e10 movss xmm7, dword ptr [r14 + rbx + 0x10]
1415533de: f3410f59f9 mulss xmm7, xmm9
1415533e3: 0f28c7 movaps xmm0, xmm7
1415533e6: e8e50b3d01 call 0x142923fd0 ; 
1415533eb: 0f28f0 movaps xmm6, xmm0
1415533ee: 0f28c7 movaps xmm0, xmm7
1415533f1: e88a0a3d01 call 0x142923e80 ; 
1415533f6: 0f28f8 movaps xmm7, xmm0
1415533f9: 450f101c1e movups xmm11, xmmword ptr [r14 + rbx]
1415533fe: 488d4d60 lea rcx, [rbp + 0x60]
141553402: 488b95d8000000 mov rdx, qword ptr [rbp + 0xd8]
141553409: e892127cff call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
14155340e: 4c8b7d68 mov r15, qword ptr [rbp + 0x68]
141553412: 4d85ff test r15, r15
141553415: 7504 jne 0x14155341b
141553417: 4c8b7d70 mov r15, qword ptr [rbp + 0x70]
14155341b: 4c8bb580000000 mov r14, qword ptr [rbp + 0x80]
141553422: 498b07 mov rax, qword ptr [r15]
141553425: 4d8b6710 mov r12, qword ptr [r15 + 0x10]
141553429: 4c29e0 sub rax, r12
14155342c: 483d87000000 cmp rax, 0x87
141553432: 7678 jbe 0x1415534ac
141553434: 0f28c6 movaps xmm0, xmm6
141553437: f3410f59c2 mulss xmm0, xmm10
14155343c: 0f28c8 movaps xmm1, xmm0
14155343f: 0f14cf unpcklps xmm1, xmm7
141553442: 0f14c6 unpcklps xmm0, xmm6
141553445: 0f16c1 movlhps xmm0, xmm1
141553448: 498b4708 mov rax, qword ptr [r15 + 8]
14155344c: 4a893c20 mov qword ptr [rax + r12], rdi
141553450: 420f11442008 movups xmmword ptr [rax + r12 + 8], xmm0
141553456: 460f115c2018 movups xmmword ptr [rax + r12 + 0x18], xmm11
14155345c: 450fc6dbff shufps xmm11, xmm11, 0xff
141553461: 460f135c2028 movlps qword ptr [rax + r12 + 0x28], xmm11
141553467: 42c744203800000000 mov dword ptr [rax + r12 + 0x38], 0
141553470: 4a89742040 mov qword ptr [rax + r12 + 0x40], rsi
141553475: 42c64420480e mov byte ptr [rax + r12 + 0x48], 0xe
14155347b: 42c644205800 mov byte ptr [rax + r12 + 0x58], 0
141553481: 4e89742068 mov qword ptr [rax + r12 + 0x68], r14
141553486: 488d0da3180100 lea rcx, [rip + 0x118a3]
14155348d: 4a894c2078 mov qword ptr [rax + r12 + 0x78], rcx
141553492: 4981c488000000 add r12, 0x88
141553499: 4d896710 mov qword ptr [r15 + 0x10], r12
14155349d: 4883c330 add rbx, 0x30
1415534a1: 4939dd cmp r13, rbx
1415534a4: 0f8526ffffff jne 0x1415533d0
1415534aa: eb25 jmp 0x1415534d1
1415534ac: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1415534b5: 41b888000000 mov r8d, 0x88
1415534bb: 41b901000000 mov r9d, 1
1415534c1: 4c89f9 mov rcx, r15
1415534c4: 4c89e2 mov rdx, r12
1415534c7: e894653501 call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1415534cc: e963ffffff jmp 0x141553434
1415534d1: 488b45b0 mov rax, qword ptr [rbp - 0x50]
1415534d5: 4885c0 test rax, rax
1415534d8: 741a je 0x1415534f4
1415534da: 48c1e004 shl rax, 4
1415534de: 488d1440 lea rdx, [rax + rax*2]
1415534e2: 41b804000000 mov r8d, 4
1415534e8: 488b8de0000000 mov rcx, qword ptr [rbp + 0xe0]
1415534ef: e82c070cff call 0x140613c20 ; 
1415534f4: 488b55d0 mov rdx, qword ptr [rbp - 0x30]
1415534f8: 4885d2 test rdx, rdx
1415534fb: 488b7548 mov rsi, qword ptr [rbp + 0x48]
1415534ff: 488b7d50 mov rdi, qword ptr [rbp + 0x50]
141553503: 488b5d58 mov rbx, qword ptr [rbp + 0x58]
141553507: 740f je 0x141553518
141553509: 488b4dd8 mov rcx, qword ptr [rbp - 0x28]
14155350d: 41b801000000 mov r8d, 1
141553513: e808070cff call 0x140613c20 ; 
141553518: 488d1585937101 lea rdx, [rip + 0x1719385] ; @142c6c8a4 'assets/meshes/meadow_lowpoly_flowers.json'
14155351f: 488d4d60 lea rcx, [rbp + 0x60]
141553523: 41b829000000 mov r8d, 0x29
141553529: e882b95bff call 0x140b0eeb0 ; country_core::utils::load_json::load_json_as_mesh<ref$<str$> >
14155352e: 31c0 xor eax, eax
141553530: 483b4560 cmp rax, qword ptr [rbp + 0x60]
141553534: 0f801b020000 jo 0x141553755
14155353a: 488b85d0000000 mov rax, qword ptr [rbp + 0xd0]
141553541: 48894540 mov qword ptr [rbp + 0x40], rax
141553545: 0f1085c0000000 movups xmm0, xmmword ptr [rbp + 0xc0]
14155354c: 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
141553550: 0f1085b0000000 movups xmm0, xmmword ptr [rbp + 0xb0]
141553557: 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
14155355b: 0f1085a0000000 movups xmm0, xmmword ptr [rbp + 0xa0]
141553562: 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
141553566: 0f104560 movups xmm0, xmmword ptr [rbp + 0x60]
14155356a: 0f104d70 movups xmm1, xmmword ptr [rbp + 0x70]
14155356e: 0f109580000000 movups xmm2, xmmword ptr [rbp + 0x80]
141553575: 0f109d90000000 movups xmm3, xmmword ptr [rbp + 0x90]
14155357c: 0f295d00 movaps xmmword ptr [rbp], xmm3
141553580: 0f2955f0 movaps xmmword ptr [rbp - 0x10], xmm2
141553584: 0f294de0 movaps xmmword ptr [rbp - 0x20], xmm1
141553588: 0f2945d0 movaps xmmword ptr [rbp - 0x30], xmm0
14155358c: 48895c2420 mov qword ptr [rsp + 0x20], rbx
141553591: 488d1550937101 lea rdx, [rip + 0x1719350]
141553598: 488d4dd0 lea rcx, [rbp - 0x30]
14155359c: 41b80b000000 mov r8d, 0xb
1415535a2: 4989f1 mov r9, rsi
1415535a5: e8f6765cff call 0x140b1aca0 ; country_core::startup::load_mesh_into_library
1415535aa: 4989c4 mov r12, rax
1415535ad: 488d153f937101 lea rdx, [rip + 0x171933f]
1415535b4: 41b80d000000 mov r8d, 0xd
1415535ba: 4889f9 mov rcx, rdi
1415535bd: e8de443eff call 0x140937aa0 ; country_core::asset_libraries::shader_library::AssetShaderLibrary::get_raster_handle_by_name
1415535c2: a801 test al, 1
1415535c4: 0f84bd010000 je 0x141553787
1415535ca: 4889d3 mov rbx, rdx
1415535cd: 488d0d44937101 lea rcx, [rip + 0x1719344] ; @142c6c918 'meadow_flowerstrees/rocks.jsontrees/clearing_edge_rocks.jsontrees/stumps.jsontrees/clearing_edge'
1415535d4: ba0e000000 mov edx, 0xe
1415535d9: e8a29774ff call 0x140c9cd80 ; utils::name_hash::NameHash::compute
1415535de: 4989c6 mov r14, rax
1415535e1: 488d4d60 lea rcx, [rbp + 0x60]
1415535e5: 488b95d8000000 mov rdx, qword ptr [rbp + 0xd8]
1415535ec: e8af107cff call 0x140d146a0 ; bevy_ecs::system::commands::Commands::spawn_empty
1415535f1: 488b7568 mov rsi, qword ptr [rbp + 0x68]
1415535f5: 4885f6 test rsi, rsi
1415535f8: 7504 jne 0x1415535fe
1415535fa: 488b7570 mov rsi, qword ptr [rbp + 0x70]
1415535fe: 488bbd80000000 mov rdi, qword ptr [rbp + 0x80]
141553605: 488b06 mov rax, qword ptr [rsi]
141553608: 4c8b7e10 mov r15, qword ptr [rsi + 0x10]
14155360c: 4c29f8 sub rax, r15
14155360f: 483d87000000 cmp rax, 0x87
141553615: 0f86d7000000 jbe 0x1415536f2
14155361b: 488b4608 mov rax, qword ptr [rsi + 8]
14155361f: 488d0d4a5cd8ff lea rcx, [rip - 0x27a3b6]
141553626: 4a890c38 mov qword ptr [rax + r15], rcx
14155362a: 460f11443808 movups xmmword ptr [rax + r15 + 8], xmm8
141553630: 4ac744381800000000 mov qword ptr [rax + r15 + 0x18], 0
141553639: 42c744382000000000 mov dword ptr [rax + r15 + 0x20], 0
141553642: 48b90000803f0000803f movabs rcx, 0x3f8000003f800000
14155364c: 4a894c3824 mov qword ptr [rax + r15 + 0x24], rcx
141553651: 42c744382c0000803f mov dword ptr [rax + r15 + 0x2c], 0x3f800000
14155365a: 4e89643838 mov qword ptr [rax + r15 + 0x38], r12
14155365f: 4a895c3840 mov qword ptr [rax + r15 + 0x40], rbx
141553664: 4e89743848 mov qword ptr [rax + r15 + 0x48], r14
141553669: 42c644385800 mov byte ptr [rax + r15 + 0x58], 0
14155366f: 4a897c3868 mov qword ptr [rax + r15 + 0x68], rdi
141553674: 488d0db5160100 lea rcx, [rip + 0x116b5]
14155367b: 4a894c3878 mov qword ptr [rax + r15 + 0x78], rcx
141553680: 4981c788000000 add r15, 0x88
141553687: 4c897e10 mov qword ptr [rsi + 0x10], r15
14155368b: 0f28b5f0000000 movaps xmm6, xmmword ptr [rbp + 0xf0]
141553692: 0f28bd00010000 movaps xmm7, xmmword ptr [rbp + 0x100]
141553699: 440f288510010000 movaps xmm8, xmmword ptr [rbp + 0x110]
1415536a1: 440f288d20010000 movaps xmm9, xmmword ptr [rbp + 0x120]
1415536a9: 440f289530010000 movaps xmm10, xmmword ptr [rbp + 0x130]
1415536b1: 440f289d40010000 movaps xmm11, xmmword ptr [rbp + 0x140]
1415536b9: 4881c4d8010000 add rsp, 0x1d8
1415536c0: 5b pop rbx
1415536c1: 5f pop rdi
1415536c2: 5e pop rsi
1415536c3: 415c pop r12
1415536c5: 415d pop r13
1415536c7: 415e pop r14
1415536c9: 415f pop r15
1415536cb: 5d pop rbp
1415536cc: c3 ret 
1415536cd: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1415536d6: 41b878000000 mov r8d, 0x78
1415536dc: 41b901000000 mov r9d, 1
1415536e2: 4c89e1 mov rcx, r12
1415536e5: 4c89ea mov rdx, r13
1415536e8: e873633501 call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1415536ed: e9a4fbffff jmp 0x141553296
1415536f2: 48c744242001000000 mov qword ptr [rsp + 0x20], 1
1415536fb: 41b888000000 mov r8d, 0x88
141553701: 41b901000000 mov r9d, 1
141553707: 4889f1 mov rcx, rsi
14155370a: 4c89fa mov rdx, r15
14155370d: e84e633501 call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
141553712: e904ffffff jmp 0x14155361b
141553717: 488b4568 mov rax, qword ptr [rbp + 0x68]
14155371b: 488945d0 mov qword ptr [rbp - 0x30], rax
14155371f: 488d0512917101 lea rax, [rip + 0x1719112]
141553726: 4889442420 mov qword ptr [rsp + 0x20], rax
14155372b: 488d0d8e8a7101 lea rcx, [rip + 0x1718a8e] ; @142c6c1c0 'called `Result::unwrap()` on an `Err` value'
141553732: 4c8d0d678a7101 lea r9, [rip + 0x1718a67]
141553739: 4c8d45d0 lea r8, [rbp - 0x30]
14155373d: ba2b000000 mov edx, 0x2b
141553742: e819603801 call 0x1428d9760 ; core::result::unwrap_failed
141553747: eb3c jmp 0x141553785
141553749: 488d0d28917101 lea rcx, [rip + 0x1719128]
141553750: e8bb5b3801 call 0x1428d9310 ; core::option::unwrap_failed
141553755: 488b4568 mov rax, qword ptr [rbp + 0x68]
141553759: 488945d0 mov qword ptr [rbp - 0x30], rax
14155375d: 488d056c917101 lea rax, [rip + 0x171916c]
141553764: 4889442420 mov qword ptr [rsp + 0x20], rax
141553769: 488d0d508a7101 lea rcx, [rip + 0x1718a50] ; @142c6c1c0 'called `Result::unwrap()` on an `Err` value'
141553770: 4c8d0d298a7101 lea r9, [rip + 0x1718a29]
141553777: 4c8d45d0 lea r8, [rbp - 0x30]
14155377b: ba2b000000 mov edx, 0x2b
141553780: e8db5f3801 call 0x1428d9760 ; core::result::unwrap_failed
141553785: 0f0b ud2 
141553787: 488d0d72917101 lea rcx, [rip + 0x1719172]
14155378e: e87d5b3801 call 0x1428d9310 ; core::option::unwrap_failed
141553793: cc int3 
141553794: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1415537a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1415537a5: 55 push rbp
1415537a6: 4157 push r15
1415537a8: 4156 push r14
1415537aa: 4155 push r13
1415537ac: 4154 push r12
1415537ae: 56 push rsi
1415537af: 57 push rdi
1415537b0: 53 push rbx
1415537b1: 4881ec88000000 sub rsp, 0x88
1415537b8: 488daa80000000 lea rbp, [rdx + 0x80]
1415537bf: 440f295c2420 movaps xmmword ptr [rsp + 0x20], xmm11
1415537c5: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
1415537cb: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
1415537d1: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
1415537d7: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
1415537dc: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
1415537e1: 488b55d0 mov rdx, qword ptr [rbp - 0x30]
1415537e5: 4885d2 test rdx, rdx
1415537e8: 740f je 0x1415537f9
1415537ea: 488b4dd8 mov rcx, qword ptr [rbp - 0x28]
1415537ee: 41b801000000 mov r8d, 1
1415537f4: e827040cff call 0x140613c20 ; 
1415537f9: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
1415537fe: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
141553803: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
141553809: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
14155380f: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
141553815: 440f285c2420 movaps xmm11, xmmword ptr [rsp + 0x20]
14155381b: 4881c488000000 add rsp, 0x88
141553822: 5b pop rbx
141553823: 5f pop rdi
141553824: 5e pop rsi
141553825: 415c pop r12
141553827: 415d pop r13
141553829: 415e pop r14
14155382b: 415f pop r15
14155382d: 5d pop rbp
14155382e: c3 ret 
14155382f: 90 nop 
141553830: 4889542410 mov qword ptr [rsp + 0x10], rdx
141553835: 55 push rbp
141553836: 4157 push r15
141553838: 4156 push r14
14155383a: 4155 push r13
14155383c: 4154 push r12
14155383e: 56 push rsi
14155383f: 57 push rdi
141553840: 53 push rbx
141553841: 4881ec88000000 sub rsp, 0x88
141553848: 488daa80000000 lea rbp, [rdx + 0x80]
14155384f: 440f295c2420 movaps xmmword ptr [rsp + 0x20], xmm11
141553855: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
14155385b: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
141553861: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
141553867: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
14155386c: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
141553871: 488b45b0 mov rax, qword ptr [rbp - 0x50]
141553875: 4885c0 test rax, rax
141553878: 741a je 0x141553894
14155387a: 48c1e004 shl rax, 4
14155387e: 488d1440 lea rdx, [rax + rax*2]
141553882: 41b804000000 mov r8d, 4
141553888: 488b8de0000000 mov rcx, qword ptr [rbp + 0xe0]
14155388f: e88c030cff call 0x140613c20 ; 
141553894: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
141553899: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
14155389e: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
1415538a4: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
1415538aa: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
1415538b0: 440f285c2420 movaps xmm11, xmmword ptr [rsp + 0x20]
1415538b6: 4881c488000000 add rsp, 0x88
1415538bd: 5b pop rbx
1415538be: 5f pop rdi
1415538bf: 5e pop rsi
1415538c0: 415c pop r12
1415538c2: 415d pop r13
1415538c4: 415e pop r14
1415538c6: 415f pop r15
1415538c8: 5d pop rbp
1415538c9: c3 ret 
1415538ca: 660f1f440000 nop word ptr [rax + rax]
1415538d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1415538d5: 55 push rbp
1415538d6: 4157 push r15
1415538d8: 4156 push r14
1415538da: 4155 push r13
1415538dc: 4154 push r12
1415538de: 56 push rsi
1415538df: 57 push rdi
1415538e0: 53 push rbx
1415538e1: 4881ec88000000 sub rsp, 0x88
1415538e8: 488daa80000000 lea rbp, [rdx + 0x80]
1415538ef: 440f295c2420 movaps xmmword ptr [rsp + 0x20], xmm11
1415538f5: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
1415538fb: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
141553901: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
141553907: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
14155390c: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
141553911: 488d4dd0 lea rcx, [rbp - 0x30]
141553915: e816937fff call 0x140d4cc30 ; anyhow::error::impl$6::drop
14155391a: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
14155391f: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
141553924: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
14155392a: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
141553930: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
141553936: 440f285c2420 movaps xmm11, xmmword ptr [rsp + 0x20]
14155393c: 4881c488000000 add rsp, 0x88
141553943: 5b pop rbx
141553944: 5f pop rdi
141553945: 5e pop rsi
141553946: 415c pop r12
141553948: 415d pop r13
14155394a: 415e pop r14
14155394c: 415f pop r15
14155394e: 5d pop rbp
14155394f: c3 ret 
141553950: 4889542410 mov qword ptr [rsp + 0x10], rdx
141553955: 55 push rbp
141553956: 4157 push r15
141553958: 4156 push r14
14155395a: 4155 push r13
14155395c: 4154 push r12
14155395e: 56 push rsi
14155395f: 57 push rdi
141553960: 53 push rbx
141553961: 4881ec88000000 sub rsp, 0x88
141553968: 488daa80000000 lea rbp, [rdx + 0x80]
14155396f: 440f295c2420 movaps xmmword ptr [rsp + 0x20], xmm11
141553975: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
14155397b: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
141553981: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
141553987: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
14155398c: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
141553991: 488d4dd0 lea rcx, [rbp - 0x30]
141553995: e896927fff call 0x140d4cc30 ; anyhow::error::impl$6::drop
14155399a: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
14155399f: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
1415539a4: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
1415539aa: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
1415539b0: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
1415539b6: 440f285c2420 movaps xmm11, xmmword ptr [rsp + 0x20]
1415539bc: 4881c488000000 add rsp, 0x88
1415539c3: 5b pop rbx
1415539c4: 5f pop rdi
1415539c5: 5e pop rsi
1415539c6: 415c pop r12
1415539c8: 415d pop r13
1415539ca: 415e pop r14
1415539cc: 415f pop r15
1415539ce: 5d pop rbp
1415539cf: c3 ret 
