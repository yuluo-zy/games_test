// X4/P8 telemetry for an iOS (or macOS) run: thermal state, memory, and a CSV writer.
// Independent of the renderer: link it into the harness app, call `Telemetry.sample(label:)` once per
// second (and at scene milestones) and write the CSV out when the run ends. It exists so the 30-minute
// sustained-load test in METAL_DESIGN.md D15 has a defined, comparable record.
//
// STATUS: never compiled or run (written without a Swift toolchain). The mach/os APIs used below are the
// standard ones; fix any SDK drift on first build.
import Foundation
#if canImport(os)
import os
#endif

struct TelemetrySample {
    let label: String
    let uptime: TimeInterval          // seconds since Telemetry.start()
    let thermalState: String          // nominal | fair | serious | critical
    let lowPowerMode: Bool
    let physFootprintMB: Double       // what the OS uses for jetsam decisions
    let availableMemoryMB: Double?    // os_proc_available_memory(); iOS only
}

enum Telemetry {
    private static var start = Date()
    private static var samples: [TelemetrySample] = []

    static func startRun() { start = Date(); samples.removeAll() }

    static func thermalName(_ state: ProcessInfo.ThermalState) -> String {
        switch state {
        case .nominal: return "nominal"
        case .fair: return "fair"
        case .serious: return "serious"
        case .critical: return "critical"
        @unknown default: return "unknown"
        }
    }

    /// phys_footprint is the figure iOS compares against the per-process memory limit.
    static func physFootprintBytes() -> UInt64? {
        var info = task_vm_info_data_t()
        var count = mach_msg_type_number_t(MemoryLayout<task_vm_info_data_t>.size / MemoryLayout<integer_t>.size)
        let result = withUnsafeMutablePointer(to: &info) {
            $0.withMemoryRebound(to: integer_t.self, capacity: Int(count)) {
                task_info(mach_task_self_, task_flavor_t(TASK_VM_INFO), $0, &count)
            }
        }
        return result == KERN_SUCCESS ? info.phys_footprint : nil
    }

    @discardableResult
    static func sample(label: String) -> TelemetrySample {
        var available: Double? = nil
        #if os(iOS) || os(tvOS) || os(visionOS)
        available = Double(os_proc_available_memory()) / 1_048_576.0
        #endif
        let s = TelemetrySample(
            label: label,
            uptime: Date().timeIntervalSince(start),
            thermalState: thermalName(ProcessInfo.processInfo.thermalState),
            lowPowerMode: ProcessInfo.processInfo.isLowPowerModeEnabled,
            physFootprintMB: Double(physFootprintBytes() ?? 0) / 1_048_576.0,
            availableMemoryMB: available)
        samples.append(s)
        return s
    }

    static func csv() -> String {
        var lines = ["label,uptime_s,thermal,low_power,phys_footprint_mb,available_mb"]
        for s in samples {
            lines.append("\(s.label),\(String(format: "%.1f", s.uptime)),\(s.thermalState),\(s.lowPowerMode),"
                         + "\(String(format: "%.1f", s.physFootprintMB)),\(s.availableMemoryMB.map { String(format: "%.1f", $0) } ?? "")")
        }
        return lines.joined(separator: "\n") + "\n"
    }

    static func write(to url: URL) throws { try csv().write(to: url, atomically: true, encoding: .utf8) }
}
