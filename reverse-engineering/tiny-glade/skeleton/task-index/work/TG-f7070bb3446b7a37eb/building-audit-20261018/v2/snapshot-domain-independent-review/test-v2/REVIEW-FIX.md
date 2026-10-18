# Test API correction after snapshot61 freeze

Only the new encoder regression used the private import tg_terrain::stroke::StrokeId. The public existing type is tg_terrain::schema::StrokeId (also reexported by core resources::terrain_editing). This delta changes that single test constructor path, with no assertion, expected JSON, producer/consumer behavior or original evidence change. Snapshot61 remains immutable; apply this after its candidate.patch. No Cargo/native execution or production change.
