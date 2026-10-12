use std::io::{self, Read};
use std::panic::{catch_unwind, AssertUnwindSafe};
use tiny_glade_graph_compiler_r2::{compile_graph, dsl::parse, snapshot::json_string};
fn main() {
    std::panic::set_hook(Box::new(|_| {}));
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("stdin");
    let mut fixture = match parse(&input) {
        Ok(fixture) => fixture,
        Err(error) => {
            println!(
                "{{\"status\":\"dsl_error\",\"message\":{}}}",
                json_string(&error)
            );
            std::process::exit(2);
        }
    };
    match catch_unwind(AssertUnwindSafe(|| compile_graph(fixture.graph, &mut fixture.pool, &mut fixture.backend, &mut fixture.dependency))) {
        Ok(Ok(compiled)) => println!("{}", compiled.snapshot.to_json()),
        Ok(Err(failure)) => println!("{{\"status\":\"backend_error\",\"tag\":{},\"consumed_origins\":{},\"partial_backing_count\":{}}}", failure.error.discriminant, failure.partial.remaining_origins.iter().filter(|origin| origin.is_none()).count(), failure.partial.registry.backing_resources.len()),
        Err(_) => println!("{{\"status\":\"panic\"}}"),
    }
}
