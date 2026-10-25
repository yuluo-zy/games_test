import ghidra.app.script.GhidraScript;
import ghidra.program.model.listing.Function;
import java.nio.file.*;
import java.util.*;
import com.google.gson.GsonBuilder;

/** Read only names for a bounded business registration query. */
public class FindRegistrationTypes extends GhidraScript {
    public void run() throws Exception {
        String[] args = getScriptArgs();
        List<Object> rows = new ArrayList<>();
        Iterator<Function> iterator = currentProgram.getFunctionManager().getFunctions(true);
        while (iterator.hasNext()) {
            Function fn = iterator.next();
            String name = fn.getName(true);
            boolean matched = false;
            for (int i = 1; i < args.length; i++) matched |= name.contains(args[i]);
            if (!matched) continue;
            Map<String, Object> row = new LinkedHashMap<>();
            row.put("name", name);
            row.put("entry", fn.getEntryPoint().toString());
            row.put("body_bytes", fn.getBody().getNumAddresses());
            rows.add(row);
        }
        Map<String, Object> report = new LinkedHashMap<>();
        report.put("program", currentProgram.getName());
        report.put("database_read_only", true);
        report.put("executable_sha256", currentProgram.getExecutableSHA256());
        report.put("functions", rows);
        Files.writeString(Paths.get(args[0]), new GsonBuilder().setPrettyPrinting().create().toJson(report));
        println("Matching functions: " + rows.size());
    }
}
