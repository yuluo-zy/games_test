// X5: Metal capability probe for crates/rhapsody/METAL_DESIGN.md (decisions D5-D8, D13, D18-D25).
//
// Build and run on a Mac (Apple Silicon is the target; also run it on any other Mac you have):
//   swiftc -O -o x5_metal_probe x5_metal_probe.swift
//   MTL_DEBUG_LAYER=1 ./x5_metal_probe --json x5_result.json        # validation on
//   ./x5_metal_probe --json x5_result_novalidation.json             # and once with it off
//   ./x5_metal_probe --only icb,fence                               # run a subset
//
// STATUS: written without access to a Mac or a Swift toolchain; it has never been compiled or run.
// Expect compile errors against your SDK on first build (names changed between macOS 13/14/15);
// fix them in place. Every test is independent and reports one of:
//   pass | fail | unsupported | info | error   -- "error" means the probe itself threw.
// A test is only evidence for the question it names; read its `detail` before drawing conclusions.
//
// Run order matters: with MTL_DEBUG_LAYER=1 an unsupported texture usage or format asserts via an
// NSException, which aborts the process (Swift cannot catch it). So run `--only format-usage,format-risks,
// storage-write-rg11b10,depth32-linear-filter` WITHOUT validation first; a crash there is itself a result
// (note which test it was in). Run the synchronisation and ICB tests WITH validation.
import Foundation
import Metal

// MARK: - plumbing

enum Status: String { case pass, fail, unsupported, info, error }
struct Outcome { let name: String; let status: Status; let detail: String }

var outcomes: [Outcome] = []
let args = CommandLine.arguments
let only: Set<String> = {
    if let i = args.firstIndex(of: "--only"), i + 1 < args.count { return Set(args[i + 1].split(separator: ",").map(String.init)) }
    return []
}()
let jsonPath: String? = { if let i = args.firstIndex(of: "--json"), i + 1 < args.count { return args[i + 1] }; return nil }()

func test(_ name: String, _ body: () throws -> (Status, String)) {
    if !only.isEmpty && !only.contains(name) { return }
    do {
        let (status, detail) = try body()
        outcomes.append(Outcome(name: name, status: status, detail: detail))
    } catch {
        outcomes.append(Outcome(name: name, status: .error, detail: "\(error)"))
    }
    let o = outcomes.last!
    print("[\(o.status.rawValue)] \(o.name): \(o.detail)")
}

struct ProbeError: Error, CustomStringConvertible { let description: String; init(_ s: String) { description = s } }

guard let device = MTLCreateSystemDefaultDevice(), let queue = device.makeCommandQueue() else {
    print("no Metal device"); exit(2)
}

func library(_ source: String, fastMath: Bool = true) throws -> MTLLibrary {
    let options = MTLCompileOptions()
    options.fastMathEnabled = fastMath   // deprecated on macOS 15 in favour of mathMode; still honoured
    return try device.makeLibrary(source: source, options: options)
}

func computePipeline(_ lib: MTLLibrary, _ name: String) throws -> MTLComputePipelineState {
    guard let fn = lib.makeFunction(name: name) else { throw ProbeError("function \(name) missing") }
    return try device.makeComputePipelineState(function: fn)
}

func run(_ body: (MTLCommandBuffer) -> Void) throws -> MTLCommandBuffer {
    guard let cb = queue.makeCommandBuffer() else { throw ProbeError("no command buffer") }
    body(cb)
    cb.commit()
    cb.waitUntilCompleted()
    if cb.status == .error { throw ProbeError("command buffer error: \(String(describing: cb.error))") }
    return cb
}

func sharedBuffer(_ count: Int, _ type: Any.Type = UInt32.self) -> MTLBuffer {
    device.makeBuffer(length: count * MemoryLayout<UInt32>.stride, options: .storageModeShared)!
}

// MARK: - device report (D14)

test("device") {
    var lines: [String] = []
    lines.append("name=\(device.name) unified=\(device.hasUnifiedMemory) lowPower=\(device.isLowPower)")
    lines.append("os=\(ProcessInfo.processInfo.operatingSystemVersionString)")
    lines.append("recommendedMaxWorkingSet=\(device.recommendedMaxWorkingSetSize) allocated=\(device.currentAllocatedSize)")
    lines.append("maxBufferLength=\(device.maxBufferLength) maxThreadsPerThreadgroup=\(device.maxThreadsPerThreadgroup) maxThreadgroupMemory=\(device.maxThreadgroupMemoryLength)")
    let families: [(String, MTLGPUFamily)] = [
        ("apple1", .apple1), ("apple2", .apple2), ("apple3", .apple3), ("apple4", .apple4), ("apple5", .apple5),
        ("apple6", .apple6), ("apple7", .apple7), ("apple8", .apple8), ("apple9", .apple9),
        ("mac2", .mac2), ("common1", .common1), ("common2", .common2), ("common3", .common3), ("metal3", .metal3),
    ]
    lines.append("families=" + families.filter { device.supportsFamily($0.1) }.map { $0.0 }.joined(separator: ","))
    lines.append("argumentBuffersTier=\(device.argumentBuffersSupport == .tier2 ? "tier2" : "tier1") maxArgBufferSamplers=\(device.maxArgumentBufferSamplerCount)")
    lines.append("readWriteTextureTier=\(device.readWriteTextureSupport.rawValue)")
    lines.append("raytracing=\(device.supportsRaytracing) functionPointers=\(device.supportsFunctionPointers) dynamicLibraries=\(device.supportsDynamicLibraries)")
    let points: [(String, MTLCounterSamplingPoint)] = [
        ("stage", .atStageBoundary), ("draw", .atDrawBoundary), ("dispatch", .atDispatchBoundary),
        ("tileDispatch", .atTileDispatchBoundary), ("blit", .atBlitBoundary),
    ]
    lines.append("counterSampling=" + points.filter { device.supportsCounterSampling($0.1) }.map { $0.0 }.joined(separator: ","))
    lines.append("counterSets=" + (device.counterSets ?? []).map { $0.name }.joined(separator: ","))
    var residency = "unavailable"
    if #available(macOS 15.0, iOS 18.0, *) { residency = "api-available" }
    lines.append("residencySet=\(residency)")
    return (.info, lines.joined(separator: "\n    "))
}

// MARK: - pixel formats (D24)
// Vulkan number -> Metal format, the table D24 proposes. Probed with real objects, not tables.
let formatTable: [(vk: Int, name: String, mtl: MTLPixelFormat)] = [
    (9, "R8_UNORM", .r8Unorm), (37, "R8G8B8A8_UNORM", .rgba8Unorm), (38, "R8G8B8A8_SNORM", .rgba8Snorm),
    (43, "R8G8B8A8_SRGB", .rgba8Unorm_srgb), (50, "B8G8R8A8_SRGB", .bgra8Unorm_srgb),
    (76, "R16_SFLOAT", .r16Float), (83, "R16G16_SFLOAT", .rg16Float),
    (91, "R16G16B16A16_UNORM", .rgba16Unorm), (92, "R16G16B16A16_SNORM", .rgba16Snorm),
    (97, "R16G16B16A16_SFLOAT", .rgba16Float), (98, "R32_UINT", .r32Uint), (100, "R32_SFLOAT", .r32Float),
    (101, "R32G32_UINT", .rg32Uint), (109, "R32G32B32A32_SFLOAT", .rgba32Float),
    (122, "B10G11R11_UFLOAT", .rg11b10Float), (123, "E5B9G9R9_UFLOAT", .rgb9e5Float),
    (126, "D32_SFLOAT", .depth32Float), (130, "D32_SFLOAT_S8_UINT", .depth32Float_stencil8),
]

func isDepth(_ f: MTLPixelFormat) -> Bool { f == .depth32Float || f == .depth32Float_stencil8 }

test("format-usage") {
    var rows: [String] = []
    for entry in formatTable {
        let d = MTLTextureDescriptor.texture2DDescriptor(pixelFormat: entry.mtl, width: 8, height: 8, mipmapped: false)
        d.storageMode = isDepth(entry.mtl) ? .private : .shared
        var support: [String] = []
        for (label, usage) in [("sample", MTLTextureUsage.shaderRead), ("rt", .renderTarget), ("write", .shaderWrite),
                               ("rw", [.shaderRead, .shaderWrite]), ("view", .pixelFormatView)] as [(String, MTLTextureUsage)] {
            if isDepth(entry.mtl) && (label == "write" || label == "rw") { continue }
            d.usage = usage
            if device.makeTexture(descriptor: d) != nil { support.append(label) }
        }
        // Render-target-ness is only proven by a pipeline that uses the format.
        if !isDepth(entry.mtl) {
            let p = MTLRenderPipelineDescriptor()
            if let lib = try? library("#include <metal_stdlib>\nusing namespace metal;\nvertex float4 v(uint i [[vertex_id]]){return float4(0);}\nfragment float4 f(){return 1;}") {
                p.vertexFunction = lib.makeFunction(name: "v"); p.fragmentFunction = lib.makeFunction(name: "f")
                p.colorAttachments[0].pixelFormat = entry.mtl
                if (try? device.makeRenderPipelineState(descriptor: p)) != nil { support.append("rt-pipeline") }
            }
        }
        rows.append("\(entry.name)(\(entry.vk)): \(support.joined(separator: ","))")
    }
    return (.info, rows.joined(separator: "\n    "))
}

test("format-risks") {
    var notes: [String] = []
    // R32G32_UINT as a render-target pipeline format is covered above ("rt-pipeline").
    // R32_UINT texture viewed as E5B9G9R9 (surface_ice.rs:50-60).
    let base = MTLTextureDescriptor.texture2DDescriptor(pixelFormat: .r32Uint, width: 8, height: 8, mipmapped: false)
    base.usage = [.shaderRead, .pixelFormatView]; base.storageMode = .shared
    if let t = device.makeTexture(descriptor: base) {
        notes.append("r32Uint->rgb9e5Float view: \(t.makeTextureView(pixelFormat: .rgb9e5Float) != nil)")
    } else { notes.append("r32Uint texture with pixelFormatView failed") }
    // E5B9G9R9 as a 3D texture (grading LUT, resources/grading_lut_cube.rs:38).
    let lut = MTLTextureDescriptor()
    lut.textureType = .type3D; lut.pixelFormat = .rgb9e5Float; lut.width = 16; lut.height = 16; lut.depth = 16
    lut.usage = .shaderRead; lut.storageMode = .shared
    notes.append("rgb9e5Float 3D texture: \(device.makeTexture(descriptor: lut) != nil)")
    // Stencil-only view of D32F_S8 for compute reads (extract_half_res_depth_normals.cs.hlsl:19).
    let ds = MTLTextureDescriptor.texture2DDescriptor(pixelFormat: .depth32Float_stencil8, width: 8, height: 8, mipmapped: false)
    ds.usage = [.shaderRead, .renderTarget, .pixelFormatView]; ds.storageMode = .private
    if let t = device.makeTexture(descriptor: ds) {
        notes.append("depth32Float_stencil8 -> x32_stencil8 view: \(t.makeTextureView(pixelFormat: .x32_stencil8) != nil)")
    } else { notes.append("depth32Float_stencil8 texture failed") }
    return (.info, notes.joined(separator: "\n    "))
}

test("storage-write-rg11b10") {
    let lib = try library("""
    #include <metal_stdlib>
    using namespace metal;
    kernel void w(texture2d<float, access::write> t [[texture(0)]], uint2 g [[thread_position_in_grid]]) {
        t.write(float4(0.5, 0.25, 0.125, 1.0), g);
    }
    """)
    let pso = try computePipeline(lib, "w")
    let d = MTLTextureDescriptor.texture2DDescriptor(pixelFormat: .rg11b10Float, width: 8, height: 8, mipmapped: false)
    d.usage = [.shaderWrite, .shaderRead]; d.storageMode = .shared
    guard let tex = device.makeTexture(descriptor: d) else { return (.unsupported, "texture with shaderWrite not created") }
    _ = try run { cb in
        let e = cb.makeComputeCommandEncoder()!
        e.setComputePipelineState(pso); e.setTexture(tex, index: 0)
        e.dispatchThreads(MTLSize(width: 8, height: 8, depth: 1), threadsPerThreadgroup: MTLSize(width: 8, height: 8, depth: 1))
        e.endEncoding()
    }
    var pixel = [UInt32](repeating: 0, count: 1)
    tex.getBytes(&pixel, bytesPerRow: 32, from: MTLRegionMake2D(0, 0, 1, 1), mipmapLevel: 0)
    return pixel[0] != 0 ? (.pass, "wrote and read back 0x\(String(pixel[0], radix: 16))") : (.fail, "storage write left the texture zero")
}

test("depth32-linear-filter") {
    // Render depth 0.0 (left) and 1.0 (right) into a 2x1 depth texture, then sample at u=0.5 with a
    // linear sampler. 0.5 => linear filtering works on Depth32Float; 0 or 1 => nearest only.
    let lib = try library("""
    #include <metal_stdlib>
    using namespace metal;
    struct VOut { float4 pos [[position]]; };
    vertex VOut v(uint i [[vertex_id]]) {
        float2 p = float2((i << 1) & 2, i & 2);
        VOut o; o.pos = float4(p * 2.0 - 1.0, 0.0, 1.0); return o;
    }
    struct FOut { float depth [[depth(any)]]; };
    fragment FOut f(VOut in [[stage_in]]) { FOut o; o.depth = floor(in.pos.x); return o; }
    kernel void sample_depth(depth2d<float> d [[texture(0)]], device float* out [[buffer(0)]]) {
        constexpr sampler s(filter::linear, address::clamp_to_edge, coord::normalized);
        out[0] = d.sample(s, float2(0.5, 0.5));
    }
    """)
    let dd = MTLTextureDescriptor.texture2DDescriptor(pixelFormat: .depth32Float, width: 2, height: 1, mipmapped: false)
    dd.usage = [.renderTarget, .shaderRead]; dd.storageMode = .private
    guard let depth = device.makeTexture(descriptor: dd) else { return (.unsupported, "no depth texture") }
    let rp = MTLRenderPipelineDescriptor()
    rp.vertexFunction = lib.makeFunction(name: "v"); rp.fragmentFunction = lib.makeFunction(name: "f")
    rp.depthAttachmentPixelFormat = .depth32Float
    let pso = try device.makeRenderPipelineState(descriptor: rp)
    let ds = MTLDepthStencilDescriptor(); ds.depthCompareFunction = .always; ds.isDepthWriteEnabled = true
    let dstate = device.makeDepthStencilState(descriptor: ds)!
    let out = sharedBuffer(1)
    let cs = try computePipeline(lib, "sample_depth")
    _ = try run { cb in
        let pass = MTLRenderPassDescriptor()
        pass.depthAttachment.texture = depth; pass.depthAttachment.loadAction = .clear; pass.depthAttachment.storeAction = .store
        let r = cb.makeRenderCommandEncoder(descriptor: pass)!
        r.setRenderPipelineState(pso); r.setDepthStencilState(dstate)
        r.drawPrimitives(type: .triangle, vertexStart: 0, vertexCount: 3)
        r.endEncoding()
        let c = cb.makeComputeCommandEncoder()!
        c.setComputePipelineState(cs); c.setTexture(depth, index: 0); c.setBuffer(out, offset: 0, index: 0)
        c.dispatchThreads(MTLSize(width: 1, height: 1, depth: 1), threadsPerThreadgroup: MTLSize(width: 1, height: 1, depth: 1))
        c.endEncoding()
    }
    let value = out.contents().bindMemory(to: Float.self, capacity: 1)[0]
    return (abs(value - 0.5) < 0.01 ? .pass : .fail, "sampled \(value) (0.5 expected if linear filtering applies)")
}

// MARK: - compiler behaviour (D9, D25)

test("fast-math-nan") {
    let src = """
    #include <metal_stdlib>
    using namespace metal;
    kernel void k(device const float* in [[buffer(0)]], device uint* out [[buffer(1)]]) {
        out[0] = isnan(in[0]) ? 1u : 0u;
        out[1] = isinf(in[1]) ? 1u : 0u;
    }
    """
    var results: [String] = []
    for fast in [true, false] {
        let pso = try computePipeline(try library(src, fastMath: fast), "k")
        let input = device.makeBuffer(length: 8, options: .storageModeShared)!
        input.contents().bindMemory(to: Float.self, capacity: 2)[0] = Float.nan
        input.contents().bindMemory(to: Float.self, capacity: 2)[1] = Float.infinity
        let out = sharedBuffer(2)
        _ = try run { cb in
            let e = cb.makeComputeCommandEncoder()!
            e.setComputePipelineState(pso); e.setBuffer(input, offset: 0, index: 0); e.setBuffer(out, offset: 0, index: 1)
            e.dispatchThreads(MTLSize(width: 1, height: 1, depth: 1), threadsPerThreadgroup: MTLSize(width: 1, height: 1, depth: 1))
            e.endEncoding()
        }
        let v = out.contents().bindMemory(to: UInt32.self, capacity: 2)
        results.append("fastMath=\(fast): isnan(NaN)=\(v[0]) isinf(inf)=\(v[1])")
    }
    return (.info, results.joined(separator: "; ") + "  (fast math on is allowed to give 0 here; off must give 1,1)")
}

test("simd-quad-compile") {
    let cases: [(String, String)] = [
        ("fragment quad_shuffle", "fragment float4 f(float4 p [[position]]) { return quad_shuffle(p, 1); }"),
        ("fragment quad_broadcast", "fragment float4 f(float4 p [[position]]) { return quad_broadcast(p, 0); }"),
        ("fragment simd_all", "fragment float4 f(float4 p [[position]]) { return simd_all(p.x > 0.0) ? 1.0 : 0.0; }"),
        ("fragment simd_any", "fragment float4 f(float4 p [[position]]) { return simd_any(p.x > 0.0) ? 1.0 : 0.0; }"),
        ("compute simd_sum", "kernel void k(device float* o [[buffer(0)]], uint i [[thread_position_in_grid]]) { o[i] = simd_sum(float(i)); }"),
        ("compute simd_shuffle_xor", "kernel void k(device float* o [[buffer(0)]], uint i [[thread_position_in_grid]], ushort l [[thread_index_in_simdgroup]]) { o[i] = simd_shuffle_xor(float(i), ushort(2)); }"),
        ("compute simd_ballot", "kernel void k(device uint* o [[buffer(0)]], uint i [[thread_position_in_grid]]) { o[i] = uint(simd_ballot(i > 3)); }"),
        ("compute 64-bit atomic max", "kernel void k(device atomic_ulong* o [[buffer(0)]], uint i [[thread_position_in_grid]]) { atomic_max_explicit(o, ulong(i), memory_order_relaxed); }"),
        ("compute device pointer from ulong", "kernel void k(device ulong* a [[buffer(0)]], device uint* o [[buffer(1)]]) { o[0] = *(device uint*)(a[0]); }"),
    ]
    var rows: [String] = []
    var failed = 0
    for (name, body) in cases {
        let source = "#include <metal_stdlib>\nusing namespace metal;\n" + body
        do { _ = try library(source); rows.append("ok   \(name)") } catch { failed += 1; rows.append("FAIL \(name): \(String(describing: error).split(separator: "\n").first ?? "")") }
    }
    return (failed == 0 ? .pass : .fail, rows.joined(separator: "\n    "))
}

test("simd-width") {
    let lib = try library("kernel void k(device uint* o [[buffer(0)]], uint i [[thread_position_in_grid]], uint w [[threads_per_simdgroup]]) { o[i] = w; }")
    let pso = try computePipeline(lib, "k")
    return (.info, "threadExecutionWidth=\(pso.threadExecutionWidth) maxTotalThreadsPerThreadgroup=\(pso.maxTotalThreadsPerThreadgroup)")
}

// MARK: - memory (D6, D13, D22, D23)

test("heap-placement-alias") {
    let d = MTLTextureDescriptor.texture2DDescriptor(pixelFormat: .rgba16Float, width: 256, height: 256, mipmapped: false)
    d.usage = [.shaderRead, .shaderWrite, .renderTarget]; d.storageMode = .private
    let sa = device.heapTextureSizeAndAlign(descriptor: d)
    let hd = MTLHeapDescriptor()
    hd.type = .placement; hd.storageMode = .private; hd.size = sa.size * 2
    guard let heap = device.makeHeap(descriptor: hd) else { return (.unsupported, "placement heap not created") }
    let a = heap.makeTexture(descriptor: d, offset: 0)
    let b = heap.makeTexture(descriptor: d, offset: 0)  // same offset: aliasing
    let c = heap.makeTexture(descriptor: d, offset: ((sa.size + sa.align - 1) / sa.align) * sa.align)
    a?.makeAliasable();
    return (a != nil && b != nil && c != nil ? .pass : .fail,
            "size=\(sa.size) align=\(sa.align) heapSize=\(hd.size) a=\(a != nil) b(aliasing a)=\(b != nil) c=\(c != nil)")
}

test("memoryless") {
    let d = MTLTextureDescriptor.texture2DDescriptor(pixelFormat: .rg32Uint, width: 64, height: 64, mipmapped: false)
    d.usage = .renderTarget; d.storageMode = .memoryless
    let standalone = device.makeTexture(descriptor: d) != nil
    // Not probed: a memoryless *heap*. An invalid heap storage mode raises an NSException, which Swift
    // cannot catch and would abort the whole probe; check the Metal docs / try it in isolation instead.
    return (standalone ? .pass : .unsupported, "standalone memoryless texture=\(standalone) (memoryless heaps deliberately not probed)")
}

test("buffer-gpu-address") {
    let src = """
    #include <metal_stdlib>
    using namespace metal;
    kernel void k(device ulong* addr [[buffer(0)]], device uint* out [[buffer(1)]]) { out[0] = *(device uint*)(addr[0]); }
    """
    let pso = try computePipeline(try library(src), "k")
    let target = sharedBuffer(1); target.contents().bindMemory(to: UInt32.self, capacity: 1)[0] = 0xC0FFEE
    let addr = device.makeBuffer(length: 8, options: .storageModeShared)!
    addr.contents().bindMemory(to: UInt64.self, capacity: 1)[0] = target.gpuAddress
    let out = sharedBuffer(1)
    _ = try run { cb in
        let e = cb.makeComputeCommandEncoder()!
        e.setComputePipelineState(pso); e.setBuffer(addr, offset: 0, index: 0); e.setBuffer(out, offset: 0, index: 1)
        e.useResource(target, usage: .read)   // required: the kernel reaches `target` only via an address
        e.dispatchThreads(MTLSize(width: 1, height: 1, depth: 1), threadsPerThreadgroup: MTLSize(width: 1, height: 1, depth: 1))
        e.endEncoding()
    }
    let v = out.contents().bindMemory(to: UInt32.self, capacity: 1)[0]
    return (v == 0xC0FFEE ? .pass : .fail, "gpuAddress=0x\(String(target.gpuAddress, radix: 16)) read back 0x\(String(v, radix: 16))")
}

test("residency-set") {
    guard #available(macOS 15.0, iOS 18.0, *) else { return (.unsupported, "OS older than macOS 15 / iOS 18") }
    let d = MTLResidencySetDescriptor()
    let set = try device.makeResidencySet(descriptor: d)
    let b = sharedBuffer(16)
    set.addAllocation(b); set.commit()
    queue.addResidencySet(set)
    return (.pass, "created, committed and attached a residency set; allocationCount=\(set.allocationCount)")
}

// MARK: - synchronisation (D5)

test("fence-cross-commandbuffer") {
    // MTLFence is documented for use within one command buffer; the forum says same-queue use across
    // command buffers works. Run with MTL_DEBUG_LAYER=1: a validation failure shows up as an error status.
    let lib = try library("""
    #include <metal_stdlib>
    using namespace metal;
    kernel void produce(device uint* b [[buffer(0)]], uint i [[thread_position_in_grid]]) { b[i] = i + 1; }
    kernel void consume(device const uint* b [[buffer(0)]], device uint* o [[buffer(1)]], uint i [[thread_position_in_grid]]) { o[i] = b[i] * 2; }
    """)
    let p = try computePipeline(lib, "produce"), c = try computePipeline(lib, "consume")
    let n = 4096
    let desc = device.makeBuffer(length: n * 4, options: [.storageModeShared, .hazardTrackingModeUntracked])!
    let out = sharedBuffer(n)
    let fence = device.makeFence()!
    let grid = MTLSize(width: n, height: 1, depth: 1), group = MTLSize(width: 256, height: 1, depth: 1)
    guard let cb1 = queue.makeCommandBuffer(), let cb2 = queue.makeCommandBuffer() else { throw ProbeError("no command buffers") }
    let e1 = cb1.makeComputeCommandEncoder()!
    e1.setComputePipelineState(p); e1.setBuffer(desc, offset: 0, index: 0)
    e1.dispatchThreads(grid, threadsPerThreadgroup: group); e1.updateFence(fence); e1.endEncoding()
    let e2 = cb2.makeComputeCommandEncoder()!
    e2.waitForFence(fence); e2.setComputePipelineState(c); e2.setBuffer(desc, offset: 0, index: 0); e2.setBuffer(out, offset: 0, index: 1)
    e2.dispatchThreads(grid, threadsPerThreadgroup: group); e2.endEncoding()
    cb1.commit(); cb2.commit(); cb2.waitUntilCompleted()
    if cb1.status == .error || cb2.status == .error { return (.fail, "error: \(String(describing: cb1.error ?? cb2.error))") }
    let v = out.contents().bindMemory(to: UInt32.self, capacity: n)
    var bad = 0; for i in 0..<n where v[i] != UInt32(i + 1) * 2 { bad += 1 }
    return (bad == 0 ? .pass : .fail, "mismatches=\(bad) of \(n) (a pass with validation off proves little: same-queue order already holds)")
}

test("shared-event") {
    let event = device.makeSharedEvent()!
    guard let cb1 = queue.makeCommandBuffer(), let cb2 = queue.makeCommandBuffer() else { throw ProbeError("no command buffers") }
    cb2.encodeWaitForEvent(event, value: 1)
    cb1.encodeSignalEvent(event, value: 1)
    cb2.commit(); cb1.commit()   // commit order reversed on purpose: the wait must hold cb2 back
    cb2.waitUntilCompleted(); cb1.waitUntilCompleted()
    return (event.signaledValue >= 1 && cb2.status == .completed ? .pass : .fail, "signaledValue=\(event.signaledValue)")
}

test("dispatch-order") {
    // Chain of dependent kernels over untracked buffers. serial: ordered. concurrent without a barrier:
    // allowed to race. concurrent with a barrier: ordered. Mismatch counts are evidence of races, but
    // zero mismatches without a barrier does NOT prove ordering (racing is not guaranteed to show).
    let lib = try library("""
    #include <metal_stdlib>
    using namespace metal;
    kernel void step(device uint* b [[buffer(0)]], uint i [[thread_position_in_grid]]) { b[i] = b[i] + 1u; }
    """)
    let pso = try computePipeline(lib, "step")
    let n = 1 << 20, steps = 16
    var report: [String] = []
    for (label, dispatchType, barrier) in [("serial", MTLDispatchType.serial, false),
                                           ("concurrent+barrier", .concurrent, true),
                                           ("concurrent no barrier", .concurrent, false)] as [(String, MTLDispatchType, Bool)] {
        var mismatches = 0
        for _ in 0..<20 {
            let b = device.makeBuffer(length: n * 4, options: [.storageModeShared, .hazardTrackingModeUntracked])!
            memset(b.contents(), 0, n * 4)
            _ = try run { cb in
                let e = cb.makeComputeCommandEncoder(dispatchType: dispatchType)!
                e.setComputePipelineState(pso); e.setBuffer(b, offset: 0, index: 0)
                for _ in 0..<steps {
                    e.dispatchThreads(MTLSize(width: n, height: 1, depth: 1), threadsPerThreadgroup: MTLSize(width: 256, height: 1, depth: 1))
                    if barrier { e.memoryBarrier(resources: [b]) }
                }
                e.endEncoding()
            }
            let v = b.contents().bindMemory(to: UInt32.self, capacity: n)
            for i in stride(from: 0, to: n, by: 997) where v[i] != UInt32(steps) { mismatches += 1 }
        }
        report.append("\(label): sampled mismatches=\(mismatches)")
    }
    return (.info, report.joined(separator: "; "))
}

// MARK: - indirect command buffers (D19)

test("icb-gpu-range") {
    // The D19 path: a compute pass fills an ICB and writes the execution range from a GPU-side count;
    // the render pass then executes only that range via an indirect buffer.
    let lib = try library("""
    #include <metal_stdlib>
    using namespace metal;
    struct ICBContainer { command_buffer icb [[id(0)]]; };
    kernel void encode(device ICBContainer* c [[buffer(0)]], device const uint* count [[buffer(1)]],
                       device uint2* range [[buffer(2)]], uint i [[thread_position_in_grid]]) {
        if (i == 0) { range[0] = uint2(0, count[0]); }
        if (i < count[0]) {
            render_command cmd(c->icb, i);
            cmd.draw_primitives(primitive_type::triangle, 0, 3, 1, i);   // base instance = draw index
        }
    }
    struct VOut { float4 pos [[position]]; };
    vertex VOut v(uint vid [[vertex_id]], uint inst [[instance_id]], uint base [[base_instance]]) {
        float2 p = float2((vid << 1) & 2, vid & 2);
        float cell = float(inst + base);   // instance_id excludes the base instance
        VOut o; o.pos = float4((p * 0.05 - 1.0) + float2(fmod(cell, 16.0) * 0.12, floor(cell / 16.0) * 0.12), 0.0, 1.0); return o;
    }
    fragment float4 f() { return float4(1, 0, 0, 1); }
    """)
    guard #available(macOS 12.0, iOS 13.0, *) else { return (.unsupported, "ICB range execution needs a newer OS") }
    let rpd = MTLRenderPipelineDescriptor()
    rpd.vertexFunction = lib.makeFunction(name: "v"); rpd.fragmentFunction = lib.makeFunction(name: "f")
    rpd.colorAttachments[0].pixelFormat = .rgba8Unorm
    rpd.supportIndirectCommandBuffers = true
    let render = try device.makeRenderPipelineState(descriptor: rpd)
    let icbDesc = MTLIndirectCommandBufferDescriptor()
    icbDesc.commandTypes = .draw
    icbDesc.inheritPipelineState = true
    icbDesc.inheritBuffers = false
    icbDesc.maxVertexBufferBindCount = 31
    icbDesc.maxFragmentBufferBindCount = 31
    let maxCommands = 256
    guard let icb = device.makeIndirectCommandBuffer(descriptor: icbDesc, maxCommandCount: maxCommands, options: []) else { return (.unsupported, "ICB not created") }
    let encodeFn = lib.makeFunction(name: "encode")!
    let encodePSO = try device.makeComputePipelineState(function: encodeFn)
    let argEncoder = encodeFn.makeArgumentEncoder(bufferIndex: 0)
    let container = device.makeBuffer(length: argEncoder.encodedLength, options: .storageModeShared)!
    argEncoder.setArgumentBuffer(container, offset: 0)
    argEncoder.setIndirectCommandBuffer(icb, index: 0)
    let count = sharedBuffer(1); count.contents().bindMemory(to: UInt32.self, capacity: 1)[0] = 100
    let range = device.makeBuffer(length: 8, options: .storageModeShared)!
    let td = MTLTextureDescriptor.texture2DDescriptor(pixelFormat: .rgba8Unorm, width: 64, height: 64, mipmapped: false)
    td.usage = .renderTarget; td.storageMode = .shared
    let target = device.makeTexture(descriptor: td)!
    _ = try run { cb in
        let c = cb.makeComputeCommandEncoder()!
        c.setComputePipelineState(encodePSO)
        c.setBuffer(container, offset: 0, index: 0); c.setBuffer(count, offset: 0, index: 1); c.setBuffer(range, offset: 0, index: 2)
        c.useResource(icb, usage: .write)
        c.dispatchThreads(MTLSize(width: maxCommands, height: 1, depth: 1), threadsPerThreadgroup: MTLSize(width: 64, height: 1, depth: 1))
        c.endEncoding()
        let pass = MTLRenderPassDescriptor()
        pass.colorAttachments[0].texture = target; pass.colorAttachments[0].loadAction = .clear
        pass.colorAttachments[0].clearColor = MTLClearColor(red: 0, green: 0, blue: 0, alpha: 0); pass.colorAttachments[0].storeAction = .store
        let r = cb.makeRenderCommandEncoder(descriptor: pass)!
        r.setRenderPipelineState(render)
        r.executeCommandsInBuffer(icb, indirectBuffer: range, indirectBufferOffset: 0)
        r.endEncoding()
    }
    var pixels = [UInt8](repeating: 0, count: 64 * 64 * 4)
    target.getBytes(&pixels, bytesPerRow: 64 * 4, from: MTLRegionMake2D(0, 0, 64, 64), mipmapLevel: 0)
    var lit = 0; for i in stride(from: 0, to: pixels.count, by: 4) where pixels[i] > 0 { lit += 1 }
    return (lit > 0 ? .pass : .fail, "lit pixels=\(lit) after executing a GPU-chosen range of 100 draws (0 means nothing was drawn)")
}

// MARK: - counters (D18)

test("counter-timestamps") {
    guard device.supportsCounterSampling(.atStageBoundary) else { return (.unsupported, "no stage-boundary sampling") }
    guard let set = (device.counterSets ?? []).first(where: { $0.name == MTLCommonCounterSet.timestamp.rawValue }) else { return (.unsupported, "no timestamp counter set") }
    let d = MTLCounterSampleBufferDescriptor()
    d.counterSet = set; d.storageMode = .shared; d.sampleCount = 2; d.label = "x5"
    let sb = try device.makeCounterSampleBuffer(descriptor: d)
    let lib = try library("kernel void k(device uint* o [[buffer(0)]], uint i [[thread_position_in_grid]]) { o[i] = i; }")
    let pso = try computePipeline(lib, "k")
    let out = sharedBuffer(1024)
    _ = try run { cb in
        let pd = MTLComputePassDescriptor()
        pd.sampleBufferAttachments[0].sampleBuffer = sb
        pd.sampleBufferAttachments[0].startOfEncoderSampleIndex = 0
        pd.sampleBufferAttachments[0].endOfEncoderSampleIndex = 1
        let e = cb.makeComputeCommandEncoder(descriptor: pd)!
        e.setComputePipelineState(pso); e.setBuffer(out, offset: 0, index: 0)
        e.dispatchThreads(MTLSize(width: 1024, height: 1, depth: 1), threadsPerThreadgroup: MTLSize(width: 64, height: 1, depth: 1))
        e.endEncoding()
    }
    guard let data = try sb.resolveCounterRange(0..<2) else { return (.fail, "resolve returned nil") }
    let stamps = data.withUnsafeBytes { Array($0.bindMemory(to: UInt64.self)) }
    let delta = stamps.count >= 2 ? Int64(bitPattern: stamps[1] &- stamps[0]) : 0
    var failing = false
    if delta <= 0 || stamps.first == UInt64.max { failing = true }
    return (failing ? .fail : .pass, "start=\(stamps.first ?? 0) end=\(stamps.last ?? 0) delta=\(delta) ticks (all-zero or error value means the stage-boundary sampler does not work on this OS)")
}

// MARK: - output

let summary = Dictionary(grouping: outcomes, by: { $0.status.rawValue }).mapValues { $0.count }
print("\nsummary: \(summary)")
if let path = jsonPath {
    let payload: [String: Any] = [
        "device": device.name,
        "os": ProcessInfo.processInfo.operatingSystemVersionString,
        "validation": ProcessInfo.processInfo.environment["MTL_DEBUG_LAYER"] ?? "off",
        "results": outcomes.map { ["name": $0.name, "status": $0.status.rawValue, "detail": $0.detail] },
    ]
    if let data = try? JSONSerialization.data(withJSONObject: payload, options: [.prettyPrinted]) {
        try? data.write(to: URL(fileURLWithPath: path))
        print("wrote \(path)")
    }
}
