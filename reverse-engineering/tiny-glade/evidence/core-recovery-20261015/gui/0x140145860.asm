0x140145860 push     r14
0x140145862 push     rsi
0x140145863 push     rdi
0x140145864 push     rbx
0x140145865 sub      rsp, 0x108
0x14014586c mov      rsi, rdx
0x14014586f mov      rdi, rcx
0x140145872 movzx    eax, byte ptr [rip + 0x34ef390]
0x140145879 mov      ecx, 0x2b
0x14014587e mov      edx, 1
0x140145883 call     0x140613c10 ; __rust_alloc
0x140145888 test     rax, rax
0x14014588b je       0x140145a09
0x140145891 movups   xmm0, xmmword ptr [rip + 0x280b8fd] ; @0x142951195 'ild mode camera.Reset'
0x140145898 movups   xmmword ptr [rax + 0x1b], xmm0
0x14014589c movups   xmm0, xmmword ptr [rip + 0x280b8e7] ; @0x14295118a 'rks with build mode camera.Reset'
0x1401458a3 movups   xmmword ptr [rax + 0x10], xmm0
0x1401458a7 movups   xmm0, xmmword ptr [rip + 0x280b8cc] ; @0x14295117a 'Note: it only works with build mode camera.Reset'
0x1401458ae movups   xmmword ptr [rax], xmm0
0x1401458b1 mov      byte ptr [rsp + 0x90], 0
0x1401458b9 mov      dword ptr [rsp + 0x95], 0
0x1401458c4 mov      dword ptr [rsp + 0x98], 0
0x1401458cf mov      qword ptr [rsp + 0x30], 3
0x1401458d8 mov      qword ptr [rsp + 0x48], 6
0x1401458e1 mov      qword ptr [rsp + 0x60], 0x2b
0x1401458ea mov      qword ptr [rsp + 0x68], rax
0x1401458ef mov      qword ptr [rsp + 0x70], 0x2b
0x1401458f8 mov      dword ptr [rsp + 0x78], 0
0x140145900 mov      dword ptr [rsp + 0x80], 0
0x14014590b mov      qword ptr [rsp + 0x88], 0
0x140145917 mov      word ptr [rsp + 0xa0], 0x200
0x140145921 mov      byte ptr [rsp + 0xa2], 2
0x140145929 mov      byte ptr [rsp + 0xa5], 2
0x140145931 lea      rbx, [rsp + 0xa8]
0x140145939 lea      rdx, [rsp + 0x30]
0x14014593e mov      rcx, rbx
0x140145941 mov      r8, rsi
0x140145944 call     0x140c28d40 ; _$LT$egui..widgets..label..Label$u20$as$u20$egui..widgets..Widget$GT$::ui::hbba1bb39743e9d2d
0x140145949 mov      rax, qword ptr [rsp + 0xb8]
0x140145951 lock dec qword ptr [rax]
0x140145955 jne      0x140145964
0x140145957 lea      rcx, [rsp + 0xb8]
0x14014595f call     0x140c41170 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::he9d75ed3b56676ff
0x140145964 mov      r14, qword ptr [rdi]
0x140145967 mov      rcx, qword ptr [r14]
0x14014596a call     0x1409114d0 ; country_core::systems::camera_rig::MainCamera::target_vertical_fov_degrees::heb6c9d2412ef6093
0x14014596f movss    dword ptr [rsp + 0xa8], xmm0
0x140145978 movzx    eax, byte ptr [rip + 0x34ef28a]
0x14014597f mov      ecx, 0x10
0x140145984 mov      edx, 8
0x140145989 call     0x140613c10 ; __rust_alloc
0x14014598e test     rax, rax
0x140145991 je       0x140145a1f
0x140145997 mov      qword ptr [rax], rbx
0x14014599a mov      qword ptr [rax + 8], r14
0x14014599e lea      rcx, [rip + 0x280b5f3]
0x1401459a5 mov      qword ptr [rsp + 0x20], rcx
0x1401459aa lea      rcx, [rsp + 0x30]
0x1401459af mov      rdx, rsi
0x1401459b2 xor      r8d, r8d
0x1401459b5 mov      r9, rax
0x1401459b8 call     0x1400c4230 ; egui::ui::Ui::horizontal_with_main_wrap_dyn::h07a52f58023f48eb
0x1401459bd mov      rax, qword ptr [rsp + 0x40]
0x1401459c2 lock dec qword ptr [rax]
0x1401459c6 jne      0x1401459d2
0x1401459c8 lea      rcx, [rsp + 0x40]
0x1401459cd call     0x140c41170 ; alloc::sync::Arc$LT$T$C$A$GT$::drop_slow::he9d75ed3b56676ff
0x1401459d2 mov      rax, qword ptr [rdi + 8]
0x1401459d6 mov      ecx, dword ptr [rax + 0x1c]
0x1401459d9 mov      rdx, qword ptr [rax + 0x10]
0x1401459dd mov      dword ptr [rdx], ecx
0x1401459df mov      rdx, qword ptr [rax]
0x1401459e2 mov      rax, qword ptr [rdi + 0x10]
0x1401459e6 mov      ecx, dword ptr [rax + 0x1c]
0x1401459e9 mov      r8, qword ptr [rax + 0x10]
0x1401459ed mov      dword ptr [r8], ecx
0x1401459f0 mov      r8, qword ptr [rax]
0x1401459f3 mov      rcx, rsi
0x1401459f6 call     0x140148cb0 ; tiny_glade::gui::gui_debug::do_camera_snapshot_ui::ha0f95f4e23b8c6ad
0x1401459fb nop      
0x1401459fc add      rsp, 0x108
0x140145a03 pop      rbx
0x140145a04 pop      rdi
0x140145a05 pop      rsi
0x140145a06 pop      r14
0x140145a08 ret      
0x140145a09 lea      r8, [rip + 0x280b718]
0x140145a10 mov      ecx, 1
0x140145a15 mov      edx, 0x2b
0x140145a1a call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x140145a1f mov      ecx, 8
0x140145a24 mov      edx, 0x10
0x140145a29 call     0x1428d9000 ; alloc::alloc::handle_alloc_error::h3e7daf9bcd04547a
0x140145a2e int3     