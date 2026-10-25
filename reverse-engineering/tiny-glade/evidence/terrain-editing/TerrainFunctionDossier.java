import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.Function;
import ghidra.program.model.address.AddressRange;
import ghidra.program.model.symbol.Reference;
import java.nio.file.*;
import java.util.*;
import com.google.gson.GsonBuilder;

/** Read-only terrain dossiers; no transactions or database updates. */
public class TerrainFunctionDossier extends GhidraScript {
    public void run() throws Exception {
        String[] args = getScriptArgs();
        Path out = Paths.get(args[0]);
        Files.createDirectories(out);
        Map<String,Object> report = new LinkedHashMap<>();
        report.put("database_read_only", true);
        report.put("target_execution", false);
        report.put("executable_sha256", currentProgram.getExecutableSHA256());
        List<Object> rows = new ArrayList<>();
        for (int i = 1; i < args.length; i++) {
            Function fn = getFunctionAt(toAddr(args[i]));
            if (fn == null) throw new Exception("Missing exact entry: " + args[i]);
            Map<String,Object> row = new LinkedHashMap<>();
            row.put("address", args[i]);
            row.put("name", fn.getName(true));
            row.put("body_bytes", fn.getBody().getNumAddresses());
            List<Object> ranges = new ArrayList<>();
            for (AddressRange range : fn.getBody().getAddressRanges(true)) {
                Map<String,Object> r = new LinkedHashMap<>();
                r.put("start", range.getMinAddress().toString());
                r.put("end_inclusive", range.getMaxAddress().toString());
                r.put("bytes", range.getLength());
                byte[] bytes = new byte[(int)range.getLength()];
                currentProgram.getMemory().getBytes(range.getMinAddress(), bytes);
                String file = "range-" + range.getMinAddress() + ".bin";
                Files.write(out.resolve(file), bytes);
                r.put("file", file);
                ranges.add(r);
            }
            row.put("ranges", ranges);
            List<Object> callers = new ArrayList<>();
            for (Reference ref : currentProgram.getReferenceManager().getReferencesTo(fn.getEntryPoint())) {
                Map<String,Object> c = new LinkedHashMap<>();
                c.put("from", ref.getFromAddress().toString());
                c.put("type", ref.getReferenceType().toString());
                Function owner = getFunctionContaining(ref.getFromAddress());
                c.put("owner", owner == null ? null : owner.getName(true));
                callers.add(c);
            }
            row.put("entry_references", callers);
            rows.add(row);
        }
        report.put("functions", rows);
        Files.writeString(out.resolve("dossier.json"), new GsonBuilder().setPrettyPrinting().create().toJson(report));
    }
}
