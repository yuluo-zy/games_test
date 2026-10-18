140c9ae20: 56 push rsi
140c9ae21: 4883ec70 sub rsp, 0x70
140c9ae25: 440f29442460 movaps xmmword ptr [rsp + 0x60], xmm8
140c9ae2b: 0f297c2450 movaps xmmword ptr [rsp + 0x50], xmm7
140c9ae30: 0f29742440 movaps xmmword ptr [rsp + 0x40], xmm6
140c9ae35: 0f28fa movaps xmm7, xmm2
140c9ae38: 0f28f1 movaps xmm6, xmm1
140c9ae3b: 4889ce mov rsi, rcx
140c9ae3e: e88dfbffff call 0x140c9a9d0 ; utils::geometry::rectangle::Rectangle2d::get_closest_edge_pt_indices
140c9ae43: f20f104e10 movsd xmm1, qword ptr [rsi + 0x10]
140c9ae48: f30f16c1 movshdup xmm0, xmm1
140c9ae4c: f30f59c1 mulss xmm0, xmm1
140c9ae50: 0f57db xorps xmm3, xmm3
140c9ae53: 0f2ec3 ucomiss xmm0, xmm3
140c9ae56: 0f8686010000 jbe 0x140c9afe2
140c9ae5c: f20f1016 movsd xmm2, qword ptr [rsi]
140c9ae60: 0fc6c911 shufps xmm1, xmm1, 0x11
140c9ae64: 31c9 xor ecx, ecx
140c9ae66: 4883f804 cmp rax, 4
140c9ae6a: 480f44c1 cmove rax, rcx
140c9ae6e: 0f590debaee001 mulps xmm1, xmmword ptr [rip + 0x1e0aeeb]
140c9ae75: f30f16c1 movshdup xmm0, xmm1
140c9ae79: f30f11442420 movss dword ptr [rsp + 0x20], xmm0
140c9ae7f: 0f114c2424 movups xmmword ptr [rsp + 0x24], xmm1
140c9ae84: 0f28c1 movaps xmm0, xmm1
140c9ae87: 660f15c1 unpckhpd xmm0, xmm1
140c9ae8b: f30f11442434 movss dword ptr [rsp + 0x34], xmm0
140c9ae91: 0f28c1 movaps xmm0, xmm1
140c9ae94: 0fc6c1ff shufps xmm0, xmm1, 0xff
140c9ae98: f30f11442438 movss dword ptr [rsp + 0x38], xmm0
140c9ae9e: f30f114c243c movss dword ptr [rsp + 0x3c], xmm1
140c9aea4: f20f1044c420 movsd xmm0, qword ptr [rsp + rax*8 + 0x20]
140c9aeaa: 0f14c0 unpcklps xmm0, xmm0
140c9aead: f30f16e2 movshdup xmm4, xmm2
140c9aeb1: 0f572538aac801 xorps xmm4, xmmword ptr [rip + 0x1c8aa38]
140c9aeb8: 0f28ea movaps xmm5, xmm2
140c9aebb: 0f16ec movlhps xmm5, xmm4
140c9aebe: 0fc6d524 shufps xmm2, xmm5, 0x24
140c9aec2: 0f59c2 mulps xmm0, xmm2
140c9aec5: 0f28e8 movaps xmm5, xmm0
140c9aec8: 660f15e8 unpckhpd xmm5, xmm0
140c9aecc: 4883fa04 cmp rdx, 4
140c9aed0: 480f44d1 cmove rdx, rcx
140c9aed4: 0f58e8 addps xmm5, xmm0
140c9aed7: 0f28e1 movaps xmm4, xmm1
140c9aeda: 0fc6e191 shufps xmm4, xmm1, 0x91
140c9aede: 0f29642420 movaps xmmword ptr [rsp + 0x20], xmm4
140c9aee3: 0fc6c93b shufps xmm1, xmm1, 0x3b
140c9aee7: 0f294c2430 movaps xmmword ptr [rsp + 0x30], xmm1
140c9aeec: f20f1044d420 movsd xmm0, qword ptr [rsp + rdx*8 + 0x20]
140c9aef2: 0f14c0 unpcklps xmm0, xmm0
140c9aef5: 0f59c2 mulps xmm0, xmm2
140c9aef8: 440f28c0 movaps xmm8, xmm0
140c9aefc: 66440f15c0 unpckhpd xmm8, xmm0
140c9af01: 440f58c0 addps xmm8, xmm0
140c9af05: f20f104608 movsd xmm0, qword ptr [rsi + 8]
140c9af0a: 0f58e8 addps xmm5, xmm0
140c9af0d: 440f58c0 addps xmm8, xmm0
140c9af11: 0f14f7 unpcklps xmm6, xmm7
140c9af14: 0f5cf5 subps xmm6, xmm5
140c9af17: 440f5cc5 subps xmm8, xmm5
140c9af1b: 410f59f0 mulps xmm6, xmm8
140c9af1f: f30f16ee movshdup xmm5, xmm6
140c9af23: f30f58ee addss xmm5, xmm6
140c9af27: 450f59c0 mulps xmm8, xmm8
140c9af2b: f3410f16f0 movshdup xmm6, xmm8
140c9af30: f3410f58f0 addss xmm6, xmm8
140c9af35: f30f5f35630bde01 maxss xmm6, dword ptr [rip + 0x1de0b63]
140c9af3d: f30f5eee divss xmm5, xmm6
140c9af41: f30f5fdd maxss xmm3, xmm5
140c9af45: f30f102db7a9c801 movss xmm5, dword ptr [rip + 0x1c8a9b7]
140c9af4d: 0f28f5 movaps xmm6, xmm5
140c9af50: f30f5df3 minss xmm6, xmm3
140c9af54: 0f29642420 movaps xmmword ptr [rsp + 0x20], xmm4
140c9af59: 0f294c2430 movaps xmmword ptr [rsp + 0x30], xmm1
140c9af5e: f20f105cc420 movsd xmm3, qword ptr [rsp + rax*8 + 0x20]
140c9af64: 0f14db unpcklps xmm3, xmm3
140c9af67: 0f59da mulps xmm3, xmm2
140c9af6a: 0f28fb movaps xmm7, xmm3
140c9af6d: 660f15fb unpckhpd xmm7, xmm3
140c9af71: 0f58fb addps xmm7, xmm3
140c9af74: 0f28d8 movaps xmm3, xmm0
140c9af77: f30f58df addss xmm3, xmm7
140c9af7b: 0f29642420 movaps xmmword ptr [rsp + 0x20], xmm4
140c9af80: 0f294c2430 movaps xmmword ptr [rsp + 0x30], xmm1
140c9af85: f20f104cd420 movsd xmm1, qword ptr [rsp + rdx*8 + 0x20]
140c9af8b: 0f14c9 unpcklps xmm1, xmm1
140c9af8e: 0f59ca mulps xmm1, xmm2
140c9af91: 0f28d1 movaps xmm2, xmm1
140c9af94: 660f15d1 unpckhpd xmm2, xmm1
140c9af98: 0f58d1 addps xmm2, xmm1
140c9af9b: f30f16c8 movshdup xmm1, xmm0
140c9af9f: f30f58c2 addss xmm0, xmm2
140c9afa3: f30f5cee subss xmm5, xmm6
140c9afa7: f30f59dd mulss xmm3, xmm5
140c9afab: f30f59c6 mulss xmm0, xmm6
140c9afaf: f30f58c3 addss xmm0, xmm3
140c9afb3: 0fc6d711 shufps xmm2, xmm7, 0x11
140c9afb7: 0fc6d7e2 shufps xmm2, xmm7, 0xe2
140c9afbb: 0f58d1 addps xmm2, xmm1
140c9afbe: 0f14ee unpcklps xmm5, xmm6
140c9afc1: 0f59ea mulps xmm5, xmm2
140c9afc4: f30f16cd movshdup xmm1, xmm5
140c9afc8: f30f58cd addss xmm1, xmm5
140c9afcc: 0f28742440 movaps xmm6, xmmword ptr [rsp + 0x40]
140c9afd1: 0f287c2450 movaps xmm7, xmmword ptr [rsp + 0x50]
140c9afd6: 440f28442460 movaps xmm8, xmmword ptr [rsp + 0x60]
140c9afdc: 4883c470 add rsp, 0x70
140c9afe0: 5e pop rsi
140c9afe1: c3 ret 
140c9afe2: 488d0d8336e901 lea rcx, [rip + 0x1e93683] ; @142b2e66c 'assertion failed: self.area() > 0.0crates/utils/src/geometry/rectangle.rs'
140c9afe9: 4c8d05c836e901 lea r8, [rip + 0x1e936c8]
140c9aff0: ba23000000 mov edx, 0x23
140c9aff5: e836e4c301 call 0x1428d9430 ; core::panicking::panic
140c9affa: cc int3 
