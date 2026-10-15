0x1408b63c0 push     rbp
0x1408b63c1 push     rsi
0x1408b63c2 push     rdi
0x1408b63c3 sub      rsp, 0x110
0x1408b63ca lea      rbp, [rsp + 0x80]
0x1408b63d2 mov      qword ptr [rbp + 0x88], 0xfffffffffffffffe
0x1408b63dd mov      rsi, rcx
0x1408b63e0 mov      qword ptr [rbp - 0x58], 0
0x1408b63e8 mov      qword ptr [rbp - 0x50], 8
0x1408b63f0 mov      qword ptr [rbp - 0x48], 0
0x1408b63f8 mov      rdi, qword ptr [rdx]
0x1408b63fb test     rdi, rdi
0x1408b63fe je       0x1408b6512
0x1408b6404 add      rdi, 0x10
0x1408b6408 lea      r8, [rip + 0x21e91f2] ; @0x142a9f601 'snapshot:/Track Switching'
0x1408b640f lea      rcx, [rbp + 0x58]
0x1408b6413 mov      r9d, 0x19
0x1408b6419 mov      rdx, rdi
0x1408b641c call     0x140a54440 ; country_core::resources::audio_resources::AudioCoreInner::instantiate_audio::h8d884187476fbacc
0x1408b6421 cmp      dword ptr [rbp + 0x58], 2
0x1408b6425 je       0x1408b6890
0x1408b642b movups   xmm0, xmmword ptr [rbp + 0x58]
0x1408b642f movups   xmm1, xmmword ptr [rbp + 0x68]
0x1408b6433 movups   xmm2, xmmword ptr [rbp + 0x78]
0x1408b6437 movaps   xmmword ptr [rbp + 0x10], xmm2
0x1408b643b movaps   xmmword ptr [rbp], xmm1
0x1408b643f movaps   xmmword ptr [rbp - 0x10], xmm0
0x1408b6443 lea      r8, [rip + 0x21e91ee] ; @0x142a9f638 'snapshot:/PhotoMode'
0x1408b644a lea      rcx, [rbp + 0x58]
0x1408b644e mov      r9d, 0x13
0x1408b6454 mov      rdx, rdi
0x1408b6457 call     0x140a54440 ; country_core::resources::audio_resources::AudioCoreInner::instantiate_audio::h8d884187476fbacc
0x1408b645c cmp      dword ptr [rbp + 0x58], 2
0x1408b6460 je       0x1408b68c5
0x1408b6466 movups   xmm0, xmmword ptr [rbp + 0x58]
0x1408b646a movups   xmm1, xmmword ptr [rbp + 0x68]
0x1408b646e movups   xmm2, xmmword ptr [rbp + 0x78]
0x1408b6472 movaps   xmmword ptr [rbp - 0x20], xmm2
0x1408b6476 movaps   xmmword ptr [rbp - 0x30], xmm1
0x1408b647a movaps   xmmword ptr [rbp - 0x40], xmm0
0x1408b647e lea      r8, [rip + 0x21e91e3] ; @0x142a9f668 'snapshot:/FirstPerson'
0x1408b6485 lea      rcx, [rbp + 0x58]
0x1408b6489 mov      r9d, 0x15
0x1408b648f mov      rdx, rdi
0x1408b6492 call     0x140a54440 ; country_core::resources::audio_resources::AudioCoreInner::instantiate_audio::h8d884187476fbacc
0x1408b6497 cmp      dword ptr [rbp + 0x58], 2
0x1408b649b je       0x1408b68fa
0x1408b64a1 movups   xmm0, xmmword ptr [rbp + 0x58]
0x1408b64a5 movups   xmm1, xmmword ptr [rbp + 0x68]
0x1408b64a9 movups   xmm2, xmmword ptr [rbp + 0x78]
0x1408b64ad movaps   xmmword ptr [rbp + 0x40], xmm2
0x1408b64b1 movaps   xmmword ptr [rbp + 0x30], xmm1
0x1408b64b5 movaps   xmmword ptr [rbp + 0x20], xmm0
0x1408b64b9 lea      r8, [rip + 0x21e91d8] ; @0x142a9f698 'snapshot:/MusicFadeIn'
0x1408b64c0 lea      rcx, [rbp + 0x58]
0x1408b64c4 mov      r9d, 0x15
0x1408b64ca mov      rdx, rdi
0x1408b64cd call     0x140a54440 ; country_core::resources::audio_resources::AudioCoreInner::instantiate_audio::h8d884187476fbacc
0x1408b64d2 cmp      qword ptr [rbp + 0x58], 2
0x1408b64d7 jne      0x1408b6760
0x1408b64dd mov      rax, qword ptr [rbp + 0x60]
0x1408b64e1 mov      qword ptr [rbp + 0x50], rax
0x1408b64e5 lea      rax, [rip + 0x21e91c4]
0x1408b64ec mov      qword ptr [rsp + 0x20], rax
0x1408b64f1 lea      rcx, [rip + 0x21e8fa0] ; @0x142a9f498 'called `Result::unwrap()` on an `Err` value'
0x1408b64f8 lea      r9, [rip + 0x21e8f79]
0x1408b64ff lea      r8, [rbp + 0x50]
0x1408b6503 mov      edx, 0x2b
0x1408b6508 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b650d jmp      0x1408b698a ; 
0x1408b6512 movzx    eax, byte ptr [rip + 0x2d7e6f0]
0x1408b6519 mov      ecx, 0x19
0x1408b651e mov      edx, 1
0x1408b6523 call     0x140613c10 ; __rust_alloc
0x1408b6528 test     rax, rax
0x1408b652b je       0x1408b692c
0x1408b6531 movups   xmm0, xmmword ptr [rip + 0x21e90d2] ; @0x142a9f60a '/Track Switching'
0x1408b6538 movups   xmmword ptr [rax + 9], xmm0
0x1408b653c movups   xmm0, xmmword ptr [rip + 0x21e90be] ; @0x142a9f601 'snapshot:/Track Switching'
0x1408b6543 movups   xmmword ptr [rax], xmm0
0x1408b6546 mov      qword ptr [rbp + 0x58], 0
0x1408b654e mov      qword ptr [rbp + 0x68], 0x19
0x1408b6556 mov      qword ptr [rbp + 0x70], rax
0x1408b655a mov      qword ptr [rbp + 0x78], 0x19
0x1408b6562 mov      byte ptr [rbp + 0x80], 2
0x1408b6569 mov      rax, qword ptr [rbp + 0x58]
0x1408b656d mov      qword ptr [rbp - 0x10], rax
0x1408b6571 mov      rax, qword ptr [rbp + 0x60]
0x1408b6575 mov      qword ptr [rbp - 8], rax
0x1408b6579 mov      rax, qword ptr [rbp + 0x68]
0x1408b657d mov      qword ptr [rbp], rax
0x1408b6581 mov      rax, qword ptr [rbp + 0x70]
0x1408b6585 mov      qword ptr [rbp + 8], rax
0x1408b6589 mov      rax, qword ptr [rbp + 0x78]
0x1408b658d mov      qword ptr [rbp + 0x10], rax
0x1408b6591 movzx    eax, byte ptr [rbp + 0x80]
0x1408b6598 mov      byte ptr [rbp + 0x18], al
0x1408b659b mov      eax, dword ptr [rbp + 0x81]
0x1408b65a1 mov      dword ptr [rbp + 0x19], eax
0x1408b65a4 movzx    eax, word ptr [rbp + 0x85]
0x1408b65ab mov      word ptr [rbp + 0x1d], ax
0x1408b65af movzx    eax, byte ptr [rbp + 0x87]
0x1408b65b6 mov      byte ptr [rbp + 0x1f], al
0x1408b65b9 movzx    eax, byte ptr [rip + 0x2d7e649]
0x1408b65c0 mov      ecx, 0x13
0x1408b65c5 mov      edx, 1
0x1408b65ca call     0x140613c10 ; __rust_alloc
0x1408b65cf test     rax, rax
0x1408b65d2 je       0x1408b6944
0x1408b65d8 movups   xmm0, xmmword ptr [rip + 0x21e9059] ; @0x142a9f638 'snapshot:/PhotoMode'
0x1408b65df movups   xmmword ptr [rax], xmm0
0x1408b65e2 mov      dword ptr [rax + 0xf], 0x65646f4d
0x1408b65e9 mov      qword ptr [rbp + 0x58], 0
0x1408b65f1 mov      qword ptr [rbp + 0x68], 0x13
0x1408b65f9 mov      qword ptr [rbp + 0x70], rax
0x1408b65fd mov      qword ptr [rbp + 0x78], 0x13
0x1408b6605 mov      byte ptr [rbp + 0x80], 2
0x1408b660c mov      rax, qword ptr [rbp + 0x58]
0x1408b6610 mov      qword ptr [rbp - 0x40], rax
0x1408b6614 mov      rax, qword ptr [rbp + 0x60]
0x1408b6618 mov      qword ptr [rbp - 0x38], rax
0x1408b661c mov      rax, qword ptr [rbp + 0x68]
0x1408b6620 mov      qword ptr [rbp - 0x30], rax
0x1408b6624 mov      rax, qword ptr [rbp + 0x70]
0x1408b6628 mov      qword ptr [rbp - 0x28], rax
0x1408b662c mov      rax, qword ptr [rbp + 0x78]
0x1408b6630 mov      qword ptr [rbp - 0x20], rax
0x1408b6634 movzx    eax, byte ptr [rbp + 0x80]
0x1408b663b mov      byte ptr [rbp - 0x18], al
0x1408b663e mov      eax, dword ptr [rbp + 0x81]
0x1408b6644 mov      dword ptr [rbp - 0x17], eax
0x1408b6647 movzx    eax, word ptr [rbp + 0x85]
0x1408b664e mov      word ptr [rbp - 0x13], ax
0x1408b6652 movzx    eax, byte ptr [rbp + 0x87]
0x1408b6659 mov      byte ptr [rbp - 0x11], al
0x1408b665c movzx    eax, byte ptr [rip + 0x2d7e5a6]
0x1408b6663 mov      ecx, 0x15
0x1408b6668 mov      edx, 1
0x1408b666d call     0x140613c10 ; __rust_alloc
0x1408b6672 test     rax, rax
0x1408b6675 je       0x1408b695c
0x1408b667b movups   xmm0, xmmword ptr [rip + 0x21e8fe6] ; @0x142a9f668 'snapshot:/FirstPerson'
0x1408b6682 movups   xmmword ptr [rax], xmm0
0x1408b6685 movabs   rcx, 0x6e6f737265507473
0x1408b668f mov      qword ptr [rax + 0xd], rcx
0x1408b6693 mov      qword ptr [rbp + 0x58], 0
0x1408b669b mov      qword ptr [rbp + 0x68], 0x15
0x1408b66a3 mov      qword ptr [rbp + 0x70], rax
0x1408b66a7 mov      qword ptr [rbp + 0x78], 0x15
0x1408b66af mov      byte ptr [rbp + 0x80], 2
0x1408b66b6 mov      rax, qword ptr [rbp + 0x58]
0x1408b66ba mov      qword ptr [rbp + 0x20], rax
0x1408b66be mov      rax, qword ptr [rbp + 0x60]
0x1408b66c2 mov      qword ptr [rbp + 0x28], rax
0x1408b66c6 mov      rax, qword ptr [rbp + 0x68]
0x1408b66ca mov      qword ptr [rbp + 0x30], rax
0x1408b66ce mov      rax, qword ptr [rbp + 0x70]
0x1408b66d2 mov      qword ptr [rbp + 0x38], rax
0x1408b66d6 mov      rax, qword ptr [rbp + 0x78]
0x1408b66da mov      qword ptr [rbp + 0x40], rax
0x1408b66de movzx    eax, byte ptr [rbp + 0x80]
0x1408b66e5 mov      byte ptr [rbp + 0x48], al
0x1408b66e8 mov      eax, dword ptr [rbp + 0x81]
0x1408b66ee mov      dword ptr [rbp + 0x49], eax
0x1408b66f1 movzx    eax, word ptr [rbp + 0x85]
0x1408b66f8 mov      word ptr [rbp + 0x4d], ax
0x1408b66fc movzx    eax, byte ptr [rbp + 0x87]
0x1408b6703 mov      byte ptr [rbp + 0x4f], al
0x1408b6706 movzx    eax, byte ptr [rip + 0x2d7e4fc]
0x1408b670d mov      ecx, 0x15
0x1408b6712 mov      edx, 1
0x1408b6717 call     0x140613c10 ; __rust_alloc
0x1408b671c test     rax, rax
0x1408b671f je       0x1408b6974
0x1408b6725 movups   xmm0, xmmword ptr [rip + 0x21e8f6c] ; @0x142a9f698 'snapshot:/MusicFadeIn'
0x1408b672c movups   xmmword ptr [rax], xmm0
0x1408b672f movabs   rcx, 0x6e49656461466369
0x1408b6739 mov      qword ptr [rax + 0xd], rcx
0x1408b673d mov      qword ptr [rbp + 0x58], 0
0x1408b6745 mov      qword ptr [rbp + 0x68], 0x15
0x1408b674d mov      qword ptr [rbp + 0x70], rax
0x1408b6751 mov      qword ptr [rbp + 0x78], 0x15
0x1408b6759 mov      byte ptr [rbp + 0x80], 2
0x1408b6760 mov      rax, qword ptr [rbp + 0x58]
0x1408b6764 mov      rcx, qword ptr [rbp + 0x60]
0x1408b6768 mov      rdx, qword ptr [rbp + 0x68]
0x1408b676c mov      r8, qword ptr [rbp + 0x70]
0x1408b6770 mov      r9, qword ptr [rbp + 0x78]
0x1408b6774 mov      qword ptr [rsi + 0x60], r9
0x1408b6778 movzx    r9d, byte ptr [rbp + 0x80]
0x1408b6780 mov      byte ptr [rsi + 0x68], r9b
0x1408b6784 mov      r9d, dword ptr [rbp + 0x81]
0x1408b678b mov      dword ptr [rsi + 0x69], r9d
0x1408b678f movzx    r9d, word ptr [rbp + 0x85]
0x1408b6797 mov      word ptr [rsi + 0x6d], r9w
0x1408b679c movzx    r9d, byte ptr [rbp + 0x87]
0x1408b67a4 mov      byte ptr [rsi + 0x6f], r9b
0x1408b67a8 mov      qword ptr [rsi + 0x50], rdx
0x1408b67ac mov      qword ptr [rsi + 0x58], r8
0x1408b67b0 mov      qword ptr [rsi + 0x40], rax
0x1408b67b4 mov      qword ptr [rsi + 0x48], rcx
0x1408b67b8 mov      qword ptr [rsi + 0xd0], 0
0x1408b67c3 mov      qword ptr [rsi + 0xd8], 8
0x1408b67ce mov      qword ptr [rsi + 0xe0], 0
0x1408b67d9 movups   xmm0, xmmword ptr [rbp - 0x58]
0x1408b67dd movups   xmmword ptr [rsi + 0xe8], xmm0
0x1408b67e4 mov      rax, qword ptr [rbp - 0x48]
0x1408b67e8 mov      qword ptr [rsi + 0xf8], rax
0x1408b67ef mov      qword ptr [rsi], 0
0x1408b67f6 mov      dword ptr [rsi + 0x118], 0
0x1408b6800 movaps   xmm0, xmmword ptr [rbp - 0x10]
0x1408b6804 movaps   xmm1, xmmword ptr [rbp]
0x1408b6808 movaps   xmm2, xmmword ptr [rbp + 0x10]
0x1408b680c movups   xmmword ptr [rsi + 0x10], xmm0
0x1408b6810 movups   xmmword ptr [rsi + 0x20], xmm1
0x1408b6814 movups   xmmword ptr [rsi + 0x30], xmm2
0x1408b6818 mov      qword ptr [rsi + 0x100], 0
0x1408b6823 mov      qword ptr [rsi + 0x108], 8
0x1408b682e mov      qword ptr [rsi + 0x110], 0
0x1408b6839 movaps   xmm0, xmmword ptr [rbp - 0x40]
0x1408b683d movaps   xmm1, xmmword ptr [rbp - 0x30]
0x1408b6841 movaps   xmm2, xmmword ptr [rbp - 0x20]
0x1408b6845 movups   xmmword ptr [rsi + 0x70], xmm0
0x1408b6849 movups   xmmword ptr [rsi + 0x80], xmm1
0x1408b6850 movups   xmmword ptr [rsi + 0x90], xmm2
0x1408b6857 movaps   xmm0, xmmword ptr [rbp + 0x20]
0x1408b685b movaps   xmm1, xmmword ptr [rbp + 0x30]
0x1408b685f movaps   xmm2, xmmword ptr [rbp + 0x40]
0x1408b6863 movups   xmmword ptr [rsi + 0xa0], xmm0
0x1408b686a movups   xmmword ptr [rsi + 0xb0], xmm1
0x1408b6871 movups   xmmword ptr [rsi + 0xc0], xmm2
0x1408b6878 mov      dword ptr [rsi + 0x128], 0
0x1408b6882 mov      rax, rsi
0x1408b6885 add      rsp, 0x110
0x1408b688c pop      rdi
0x1408b688d pop      rsi
0x1408b688e pop      rbp
0x1408b688f ret      
0x1408b6890 mov      rax, qword ptr [rbp + 0x60]
0x1408b6894 mov      qword ptr [rbp + 0x20], rax
0x1408b6898 lea      rax, [rip + 0x21e8d81]
0x1408b689f mov      qword ptr [rsp + 0x20], rax
0x1408b68a4 lea      rcx, [rip + 0x21e8bed] ; @0x142a9f498 'called `Result::unwrap()` on an `Err` value'
0x1408b68ab lea      r9, [rip + 0x21e8bc6]
0x1408b68b2 lea      r8, [rbp + 0x20]
0x1408b68b6 mov      edx, 0x2b
0x1408b68bb call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b68c0 jmp      0x1408b698a ; 
0x1408b68c5 mov      rax, qword ptr [rbp + 0x60]
0x1408b68c9 mov      qword ptr [rbp + 0x20], rax
0x1408b68cd lea      rax, [rip + 0x21e8d7c]
0x1408b68d4 mov      qword ptr [rsp + 0x20], rax
0x1408b68d9 lea      rcx, [rip + 0x21e8bb8] ; @0x142a9f498 'called `Result::unwrap()` on an `Err` value'
0x1408b68e0 lea      r9, [rip + 0x21e8b91]
0x1408b68e7 lea      r8, [rbp + 0x20]
0x1408b68eb mov      edx, 0x2b
0x1408b68f0 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b68f5 jmp      0x1408b698a ; 
0x1408b68fa mov      rax, qword ptr [rbp + 0x60]
0x1408b68fe mov      qword ptr [rbp + 0x50], rax
0x1408b6902 lea      rax, [rip + 0x21e8d77]
0x1408b6909 mov      qword ptr [rsp + 0x20], rax
0x1408b690e lea      rcx, [rip + 0x21e8b83] ; @0x142a9f498 'called `Result::unwrap()` on an `Err` value'
0x1408b6915 lea      r9, [rip + 0x21e8b5c]
0x1408b691c lea      r8, [rbp + 0x50]
0x1408b6920 mov      edx, 0x2b
0x1408b6925 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b692a jmp      0x1408b698a ; 
0x1408b692c lea      r8, [rip + 0x2219e7d]
0x1408b6933 mov      ecx, 1
0x1408b6938 mov      edx, 0x19
0x1408b693d call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x1408b6942 jmp      0x1408b698a ; 
0x1408b6944 lea      r8, [rip + 0x2219e65]
0x1408b694b mov      ecx, 1
0x1408b6950 mov      edx, 0x13
0x1408b6955 call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x1408b695a jmp      0x1408b698a ; 
0x1408b695c lea      r8, [rip + 0x2219e4d]
0x1408b6963 mov      ecx, 1
0x1408b6968 mov      edx, 0x15
0x1408b696d call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x1408b6972 jmp      0x1408b698a ; 
0x1408b6974 lea      r8, [rip + 0x2219e35]
0x1408b697b mov      ecx, 1
0x1408b6980 mov      edx, 0x15
0x1408b6985 call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x1408b698a ud2      
0x1408b698c nop      dword ptr [rax]
0x1408b6990 mov      qword ptr [rsp + 0x10], rdx
0x1408b6995 push     rbp
0x1408b6996 push     rsi
0x1408b6997 push     rdi
0x1408b6998 sub      rsp, 0x30
0x1408b699c lea      rbp, [rdx + 0x80]
0x1408b69a3 lea      rcx, [rbp - 0x58]
0x1408b69a7 call     0x140899740 ; core::ptr::drop_in_place$LT$country_core..utils..audio_utils..AudioLoop$GT$::hd174871ff0ad0960 (.llvm.446375132613585674) / core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$country_core..resources..audio_resources..AudioInstance$GT$$GT$::h6882ea263b0e35cc
0x1408b69ac nop      
0x1408b69ad add      rsp, 0x30
0x1408b69b1 pop      rdi
0x1408b69b2 pop      rsi
0x1408b69b3 pop      rbp
0x1408b69b4 ret      
0x1408b69b5 nop      word ptr cs:[rax + rax]
0x1408b69c0 mov      qword ptr [rsp + 0x10], rdx
0x1408b69c5 push     rbp
0x1408b69c6 push     rsi
0x1408b69c7 push     rdi
0x1408b69c8 sub      rsp, 0x30
0x1408b69cc lea      rbp, [rdx + 0x80]
0x1408b69d3 lea      rcx, [rbp + 0x20]
0x1408b69d7 call     0x140d4cc30 ; anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop::h2a52e76aa8e9c217
0x1408b69dc nop      
0x1408b69dd add      rsp, 0x30
0x1408b69e1 pop      rdi
0x1408b69e2 pop      rsi
0x1408b69e3 pop      rbp
0x1408b69e4 ret      
0x1408b69e5 nop      word ptr cs:[rax + rax]
0x1408b69f0 mov      qword ptr [rsp + 0x10], rdx
0x1408b69f5 push     rbp
0x1408b69f6 push     rsi
0x1408b69f7 push     rdi
0x1408b69f8 sub      rsp, 0x30
0x1408b69fc lea      rbp, [rdx + 0x80]
0x1408b6a03 lea      rcx, [rbp - 0x10]
0x1408b6a07 call     0x14014a680 ; core::ptr::drop_in_place$LT$country_core..resources..critters..sheep..SheepAudio$GT$::hfb03c8fee6406828 / bevy_ecs::component::ComponentDescriptor::drop_ptr::h9feafd2efa3d66f6 (.llvm.8002602149119224474)
0x1408b6a0c nop      
0x1408b6a0d add      rsp, 0x30
0x1408b6a11 pop      rdi
0x1408b6a12 pop      rsi
0x1408b6a13 pop      rbp
0x1408b6a14 ret      
0x1408b6a15 nop      word ptr cs:[rax + rax]
0x1408b6a20 mov      qword ptr [rsp + 0x10], rdx
0x1408b6a25 push     rbp
0x1408b6a26 push     rsi
0x1408b6a27 push     rdi
0x1408b6a28 sub      rsp, 0x30
0x1408b6a2c lea      rbp, [rdx + 0x80]
0x1408b6a33 lea      rcx, [rbp + 0x20]
0x1408b6a37 call     0x140d4cc30 ; anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop::h2a52e76aa8e9c217
0x1408b6a3c nop      
0x1408b6a3d add      rsp, 0x30
0x1408b6a41 pop      rdi
0x1408b6a42 pop      rsi
0x1408b6a43 pop      rbp
0x1408b6a44 ret      
0x1408b6a45 nop      word ptr cs:[rax + rax]
0x1408b6a50 mov      qword ptr [rsp + 0x10], rdx
0x1408b6a55 push     rbp
0x1408b6a56 push     rsi
0x1408b6a57 push     rdi
0x1408b6a58 sub      rsp, 0x30
0x1408b6a5c lea      rbp, [rdx + 0x80]
0x1408b6a63 lea      rcx, [rbp - 0x40]
0x1408b6a67 call     0x14014a680 ; core::ptr::drop_in_place$LT$country_core..resources..critters..sheep..SheepAudio$GT$::hfb03c8fee6406828 / bevy_ecs::component::ComponentDescriptor::drop_ptr::h9feafd2efa3d66f6 (.llvm.8002602149119224474)
0x1408b6a6c nop      
0x1408b6a6d add      rsp, 0x30
0x1408b6a71 pop      rdi
0x1408b6a72 pop      rsi
0x1408b6a73 pop      rbp
0x1408b6a74 ret      
0x1408b6a75 nop      word ptr cs:[rax + rax]
0x1408b6a80 mov      qword ptr [rsp + 0x10], rdx
0x1408b6a85 push     rbp
0x1408b6a86 push     rsi
0x1408b6a87 push     rdi
0x1408b6a88 sub      rsp, 0x30
0x1408b6a8c lea      rbp, [rdx + 0x80]
0x1408b6a93 lea      rcx, [rbp + 0x50]
0x1408b6a97 call     0x140d4cc30 ; anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop::h2a52e76aa8e9c217
0x1408b6a9c nop      
0x1408b6a9d add      rsp, 0x30
0x1408b6aa1 pop      rdi
0x1408b6aa2 pop      rsi
0x1408b6aa3 pop      rbp
0x1408b6aa4 ret      
0x1408b6aa5 nop      word ptr cs:[rax + rax]
0x1408b6ab0 mov      qword ptr [rsp + 0x10], rdx
0x1408b6ab5 push     rbp
0x1408b6ab6 push     rsi
0x1408b6ab7 push     rdi
0x1408b6ab8 sub      rsp, 0x30
0x1408b6abc lea      rbp, [rdx + 0x80]
0x1408b6ac3 lea      rcx, [rbp + 0x20]
0x1408b6ac7 call     0x14014a680 ; core::ptr::drop_in_place$LT$country_core..resources..critters..sheep..SheepAudio$GT$::hfb03c8fee6406828 / bevy_ecs::component::ComponentDescriptor::drop_ptr::h9feafd2efa3d66f6 (.llvm.8002602149119224474)
0x1408b6acc nop      
0x1408b6acd add      rsp, 0x30
0x1408b6ad1 pop      rdi
0x1408b6ad2 pop      rsi
0x1408b6ad3 pop      rbp
0x1408b6ad4 ret      
0x1408b6ad5 nop      word ptr cs:[rax + rax]
0x1408b6ae0 mov      qword ptr [rsp + 0x10], rdx
0x1408b6ae5 push     rbp
0x1408b6ae6 push     rsi
0x1408b6ae7 push     rdi
0x1408b6ae8 sub      rsp, 0x30
0x1408b6aec lea      rbp, [rdx + 0x80]
0x1408b6af3 lea      rcx, [rbp + 0x50]
0x1408b6af7 call     0x140d4cc30 ; anyhow::error::_$LT$impl$u20$core..ops..drop..Drop$u20$for$u20$anyhow..Error$GT$::drop::h2a52e76aa8e9c217
0x1408b6afc nop      
0x1408b6afd add      rsp, 0x30
0x1408b6b01 pop      rdi
0x1408b6b02 pop      rsi
0x1408b6b03 pop      rbp
0x1408b6b04 ret      