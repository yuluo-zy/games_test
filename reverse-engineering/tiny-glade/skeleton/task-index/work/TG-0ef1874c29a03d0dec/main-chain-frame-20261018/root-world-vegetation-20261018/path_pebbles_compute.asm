140958f70: push rbp
140958f71: push rsi
140958f72: push rdi
140958f73: sub rsp, 0x500
140958f7a: lea rbp, [rsp + 0x80]
140958f82: mov qword ptr [rbp + 0x478], 0xfffffffffffffffe
140958f8d: mov qword ptr [rsp + 0x30], rcx
140958f92: mov qword ptr [rsp + 0x28], rdx
140958f97: mov qword ptr [rsp + 0x20], 0x15
140958fa0: lea rdx, [rip + 0x215ba99]
140958fa7: lea r9, [rip + 0x215ba9d]
140958fae: lea rsi, [rbp + 0x2e0]
140958fb5: mov r8d, 0xb
140958fbb: mov rcx, rsi
140958fbe: call 0x1416032a0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::new<ref$<str$> >
140958fc3: lea rdi, [rbp + 0x150]
140958fca: movabs r8, 0xe853ba5b3b695a54
140958fd4: mov rcx, rdi
140958fd7: mov rdx, rsi
140958fda: call 0x1416058b0 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer<country_core::systems::render_loop::compute_passes::path_pebbles::PebblePlacementSsbo>
140958fdf: lea rcx, [rbp + 0x2e0]
140958fe6: movabs r8, 0xf1c07a2b8012765a
140958ff0: mov rdx, rdi
140958ff3: mov r9d, 1
140958ff9: call 0x141608c10 ; country_core::render::pass_builder::compute_pass_builder::ComputePass::write_storage_buffer_concurrent<country_core::systems::render_loop::indirect_params::DrawIndexedIndirectBuffer>
140958ffe: cmp byte ptr [rbp + 0x416], 8
140959005: je 0x14095902f
140959007: lea rcx, [rbp + 0x3f0]
14095900e: mov edx, 0xe8
140959013: add rdx, qword ptr [rbp + 0x418]
14095901a: movabs r8, 0xe9511a616b75789
140959024: mov r9d, 3
14095902a: call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
14095902f: lea rcx, [rbp - 0x40]
140959033: lea rdx, [rbp + 0x2e0]
14095903a: mov r8d, 0x190
140959040: call 0x142923c00
140959045: cmp byte ptr [rbp + 0xf6], 8
14095904c: je 0x140959076
14095904e: lea rcx, [rbp + 0xd0]
140959055: mov edx, 0xe8
14095905a: add rdx, qword ptr [rbp + 0xf8]
140959061: movabs r8, 0x94de63d0ee6b5af4
14095906b: mov r9d, 0x240
140959071: call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
140959076: lea rcx, [rbp + 0x2e0]
14095907d: lea rdx, [rbp - 0x40]
140959081: mov r8d, 0x190
140959087: call 0x142923c00
14095908c: cmp byte ptr [rbp + 0x416], 8
140959093: je 0x1409590bd
140959095: lea rcx, [rbp + 0x3f0]
14095909c: mov edx, 0xe8
1409590a1: add rdx, qword ptr [rbp + 0x418]
1409590a8: movabs r8, 0x1714e971d34a7947
1409590b2: mov r9d, 0x18
1409590b8: call 0x14096b570 ; country_core::render::pass_builder::uniform_binder::UniformBinder::write_uniform<country_core::render::pass_builder::compute_pass_builder::impl$3::uniform::closure_env$0<i32>,i32>
1409590bd: mov r8d, 0x190
1409590c3: mov rcx, rdi
1409590c6: mov rdx, rsi
1409590c9: call 0x142923c00
1409590ce: mov byte ptr [rbp + 0x477], 0xe
1409590d5: mov dword ptr [rbp + 0x2e0], 2
1409590df: mov qword ptr [rsp + 0x20], rsi
1409590e4: lea rcx, [rbp + 0x477]
1409590eb: movabs r8, 0x4ee098b38612d2b9
1409590f5: mov rdx, rdi
1409590f8: mov r9b, 0x19
1409590fb: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140959100: lea rdi, [rbp - 0x40]
140959104: lea rdx, [rbp + 0x150]
14095910b: mov r8d, 0x190
140959111: mov rcx, rdi
140959114: call 0x142923c00
140959119: mov byte ptr [rbp + 0x150], 9
140959120: mov dword ptr [rbp + 0x2e0], 2
14095912a: mov qword ptr [rsp + 0x20], rsi
14095912f: lea rsi, [rbp + 0x150]
140959136: movabs r8, 0x61b08b5e0470c5f2
140959140: mov rcx, rsi
140959143: mov rdx, rdi
140959146: mov r9b, 0x19
140959149: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
14095914e: lea rdi, [rbp + 0x2e0]
140959155: lea rdx, [rbp - 0x40]
140959159: mov r8d, 0x190
14095915f: mov rcx, rdi
140959162: call 0x142923c00
140959167: mov byte ptr [rbp + 0x477], 0xa
14095916e: mov dword ptr [rbp + 0x150], 2
140959178: mov qword ptr [rsp + 0x20], rsi
14095917d: lea rcx, [rbp + 0x477]
140959184: movabs r8, 0x297d1e4cdae8f657
14095918e: mov rdx, rdi
140959191: mov r9b, 0x19
140959194: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140959199: mov r8d, 0x190
14095919f: mov rcx, rsi
1409591a2: mov rdx, rdi
1409591a5: call 0x142923c00
1409591aa: mov byte ptr [rbp - 0x40], 6
1409591ae: mov dword ptr [rbp + 0x2e0], 2
1409591b8: mov qword ptr [rsp + 0x20], rdi
1409591bd: lea rcx, [rbp - 0x40]
1409591c1: movabs r8, 0xed502fff6dc4021f
1409591cb: mov rdx, rsi
1409591ce: mov r9b, 0x19
1409591d1: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
1409591d6: lea rsi, [rbp + 0x2e0]
1409591dd: lea rdi, [rbp + 0x150]
1409591e4: mov r8d, 0x190
1409591ea: mov rcx, rsi
1409591ed: mov rdx, rdi
1409591f0: call 0x142923c00
1409591f5: mov byte ptr [rbp - 0x40], 0x13
1409591f9: mov dword ptr [rbp + 0x150], 2
140959203: mov qword ptr [rsp + 0x20], rdi
140959208: lea rcx, [rbp - 0x40]
14095920c: movabs r8, 0x6ad63ccd18c8f041
140959216: mov rdx, rsi
140959219: mov r9b, 0x19
14095921c: call 0x140905260 ; country_core::render::pass_builder::compute_pass_builder::impl$4::bind_texture_to_compute_pass<country_core::resources::texture_presets::TexturePreset>
140959221: lea rcx, [rbp + 0x150]
140959228: lea rdx, [rbp + 0x2e0]
14095922f: mov r8d, 0x190
140959235: call 0x142923c00
14095923a: movups xmm0, xmmword ptr [rbp + 0x1f0]
140959241: movaps xmmword ptr [rbp], xmm0
140959245: movups xmm0, xmmword ptr [rbp + 0x1e0]
14095924c: movaps xmmword ptr [rbp - 0x10], xmm0
140959250: movups xmm0, xmmword ptr [rbp + 0x1d0]
140959257: movaps xmmword ptr [rbp - 0x20], xmm0
14095925b: movups xmm0, xmmword ptr [rbp + 0x1c0]
140959262: movaps xmmword ptr [rbp - 0x30], xmm0
140959266: movups xmm0, xmmword ptr [rbp + 0x1b0]
14095926d: movaps xmmword ptr [rbp - 0x40], xmm0
140959271: mov rax, qword ptr [rbp + 0x288]
140959278: mov rcx, qword ptr [rbp + 0x2d0]
14095927f: mov rdx, qword ptr [rbp + 0x160]
140959286: mov qword ptr [rbp + 0x2f0], rdx
14095928d: movups xmm0, xmmword ptr [rbp + 0x150]
140959294: movaps xmmword ptr [rbp + 0x2e0], xmm0
14095929b: movups xmm0, xmmword ptr [rbp + 0x168]
1409592a2: movaps xmmword ptr [rbp + 0x310], xmm0
1409592a9: mov rdx, qword ptr [rbp + 0x178]
1409592b0: mov qword ptr [rbp + 0x320], rdx
1409592b7: movups xmm0, xmmword ptr [rbp + 0x180]
1409592be: movups xmmword ptr [rbp + 0x2f8], xmm0
1409592c5: mov rdx, qword ptr [rbp + 0x190]
1409592cc: mov qword ptr [rbp + 0x308], rdx
1409592d3: movups xmm0, xmmword ptr [rbp + 0x200]
1409592da: movups xmm1, xmmword ptr [rbp + 0x210]
1409592e1: movups xmm2, xmmword ptr [rbp + 0x220]
1409592e8: movups xmm3, xmmword ptr [rbp + 0x230]
1409592ef: movups xmmword ptr [rbp + 0x358], xmm0
1409592f6: movups xmmword ptr [rbp + 0x368], xmm1
1409592fd: movups xmmword ptr [rbp + 0x378], xmm2
140959304: movups xmmword ptr [rbp + 0x388], xmm3
14095930b: movups xmm0, xmmword ptr [rbp + 0x240]
140959312: movups xmmword ptr [rbp + 0x398], xmm0
140959319: movups xmm0, xmmword ptr [rbp + 0x250]
140959320: movups xmmword ptr [rbp + 0x3a8], xmm0
140959327: mov rdx, qword ptr [rbp + 0x1a8]
14095932e: mov qword ptr [rbp + 0x338], rdx
140959335: movups xmm0, xmmword ptr [rbp + 0x198]
14095933c: movups xmmword ptr [rbp + 0x328], xmm0
140959343: mov qword ptr [rbp + 0x3c0], rcx
14095934a: mov qword ptr [rbp + 0x3b8], rax
140959351: movaps xmm0, xmmword ptr [rip + 0x215b6d8]
140959358: movaps xmmword ptr [rbp + 0x340], xmm0
14095935f: lea rcx, [rbp - 0x40]
140959363: lea rdx, [rbp + 0x2e0]
14095936a: call 0x1409dd4d0 ; rhapsody::rg::pass_builder::PassBuilder::render<country_core::render::pass_builder::pass_resources::PassResources,country_core::render::pass_builder::compute_pass_builder::impl$0::dispatch_impl::closure_env$0>
14095936f: cmp byte ptr [rbp + 0x286], 8
140959376: je 0x140959384
140959378: lea rcx, [rbp + 0x270]
14095937f: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
140959384: nop 
140959385: add rsp, 0x500
14095938c: pop rdi
14095938d: pop rsi
14095938e: pop rbp
14095938f: ret 
140959390: mov qword ptr [rsp + 0x10], rdx
140959395: push rbp
140959396: push rsi
140959397: push rdi
140959398: sub rsp, 0x40
14095939c: lea rbp, [rdx + 0x80]
1409593a3: cmp byte ptr [rbp + 0x286], 8
1409593aa: je 0x1409593b8
1409593ac: lea rcx, [rbp + 0x270]
1409593b3: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
1409593b8: nop 
1409593b9: add rsp, 0x40
1409593bd: pop rdi
1409593be: pop rsi
1409593bf: pop rbp
1409593c0: ret 
1409593c1: nop word ptr cs:[rax + rax]
1409593d0: mov qword ptr [rsp + 0x10], rdx
1409593d5: push rbp
1409593d6: push rsi
1409593d7: push rdi
1409593d8: sub rsp, 0x40
1409593dc: lea rbp, [rdx + 0x80]
1409593e3: lea rcx, [rbp + 0x2e0]
1409593ea: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1409593ef: nop 
1409593f0: add rsp, 0x40
1409593f4: pop rdi
1409593f5: pop rsi
1409593f6: pop rbp
1409593f7: ret 
1409593f8: nop dword ptr [rax + rax]
140959400: mov qword ptr [rsp + 0x10], rdx
140959405: push rbp
140959406: push rsi
140959407: push rdi
140959408: sub rsp, 0x40
14095940c: lea rbp, [rdx + 0x80]
140959413: lea rcx, [rbp - 0x40]
140959417: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
14095941c: nop 
14095941d: add rsp, 0x40
140959421: pop rdi
140959422: pop rsi
140959423: pop rbp
140959424: ret 
140959425: nop word ptr cs:[rax + rax]
140959430: mov qword ptr [rsp + 0x10], rdx
140959435: push rbp
140959436: push rsi
140959437: push rdi
140959438: sub rsp, 0x40
14095943c: lea rbp, [rdx + 0x80]
140959443: lea rcx, [rbp + 0x2e0]
14095944a: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
14095944f: nop 
140959450: add rsp, 0x40
140959454: pop rdi
140959455: pop rsi
140959456: pop rbp
140959457: ret 
140959458: nop dword ptr [rax + rax]
140959460: mov qword ptr [rsp + 0x10], rdx
140959465: push rbp
140959466: push rsi
140959467: push rdi
140959468: sub rsp, 0x40
14095946c: lea rbp, [rdx + 0x80]
140959473: lea rcx, [rbp + 0x150]
14095947a: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
14095947f: nop 
140959480: add rsp, 0x40
140959484: pop rdi
140959485: pop rsi
140959486: pop rbp
140959487: ret 
140959488: nop dword ptr [rax + rax]
140959490: mov qword ptr [rsp + 0x10], rdx
140959495: push rbp
140959496: push rsi
140959497: push rdi
140959498: sub rsp, 0x40
14095949c: lea rbp, [rdx + 0x80]
1409594a3: lea rcx, [rbp - 0x40]
1409594a7: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1409594ac: nop 
1409594ad: add rsp, 0x40
1409594b1: pop rdi
1409594b2: pop rsi
1409594b3: pop rbp
1409594b4: ret 
1409594b5: nop word ptr cs:[rax + rax]
1409594c0: mov qword ptr [rsp + 0x10], rdx
1409594c5: push rbp
1409594c6: push rsi
1409594c7: push rdi
1409594c8: sub rsp, 0x40
1409594cc: lea rbp, [rdx + 0x80]
1409594d3: lea rcx, [rbp + 0x2e0]
1409594da: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
1409594df: nop 
1409594e0: add rsp, 0x40
1409594e4: pop rdi
1409594e5: pop rsi
1409594e6: pop rbp
1409594e7: ret 
1409594e8: nop dword ptr [rax + rax]
1409594f0: mov qword ptr [rsp + 0x10], rdx
1409594f5: push rbp
1409594f6: push rsi
1409594f7: push rdi
1409594f8: sub rsp, 0x40
1409594fc: lea rbp, [rdx + 0x80]
140959503: lea rcx, [rbp + 0x150]
14095950a: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
14095950f: nop 
140959510: add rsp, 0x40
140959514: pop rdi
140959515: pop rsi
140959516: pop rbp
140959517: ret 
140959518: nop dword ptr [rax + rax]
140959520: mov qword ptr [rsp + 0x10], rdx
140959525: push rbp
140959526: push rsi
140959527: push rdi
140959528: sub rsp, 0x40
14095952c: lea rbp, [rdx + 0x80]
140959533: lea rcx, [rbp + 0x2e0]
14095953a: call 0x140896fb0 ; core::ptr::drop_in_place<country_core::render::pass_builder::compute_pass_builder::ComputePass>
14095953f: nop 
140959540: add rsp, 0x40
140959544: pop rdi
140959545: pop rsi
140959546: pop rbp
140959547: ret 
