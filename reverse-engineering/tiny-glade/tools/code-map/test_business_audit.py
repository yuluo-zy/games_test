"""Guard against inflated function counts and false source associations."""
from pathlib import Path
import sys
import unittest

from settings import WORKSPACE
from source_index import lexer,functions,associate

sys.path.insert(0,str(WORKSPACE/'tiny-glade/tools'))
import audit_all_business_functions as audit

class SourceAuditTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):cls.lex=staticmethod(lexer(WORKSPACE))

    def parse(self,text,path='tiny-glade/crates/test/src/foo.rs'):
        return functions(text,path,self.lex)

    def test_test_module_does_not_hide_later_production_functions(self):
        rows=self.parse('fn before() {} #[cfg(test)] mod tests { fn fake() {todo!()} } fn after() {1;}')
        self.assertEqual([r['name'] for r in rows],['before','after'])

    def test_raw_strings_and_nested_comments_cannot_create_stubs(self):
        rows=self.parse('fn real() { let s = r##"todo!() /* }"##; /* a /* unimplemented!() */ b */ }')
        self.assertEqual(rows[0]['state'],'body_candidate')

    def test_literal_only_body_is_not_empty(self):
        self.assertEqual(self.parse('fn name() -> &\'static str { "value" }')[0]['state'],'body_candidate')

    def test_trait_declarations_tests_and_macro_templates_are_not_functions(self):
        rows=self.parse('trait T { fn decl(&self); } #[test] fn fake() {} macro_rules! f { () => { fn generated() {} } } fn live() {}')
        self.assertEqual([r['name'] for r in rows],['live'])

    def test_nominal_owners_and_const_generics(self):
        rows=self.parse("impl<'a> Trait for World<'a> { fn run<const N: usize>(&self) {todo!{}} }")
        self.assertEqual((rows[0]['owner'],rows[0]['trait'],rows[0]['state']),('World','Trait','explicit_stub'))

    def test_wrong_owner_is_not_matched_from_same_filename(self):
        rows=self.parse('impl Other { fn update(&self) {1;} }')
        candidates,_=associate('crate::Wanted::update','crates/test/src/foo.rs',rows)
        self.assertEqual(candidates,[])

    def test_cross_module_owner_mapping_resolves_shared_terrain_resource(self):
        rows=self.parse('impl ScheduledTerrainReadback { fn on_gpu_copy_issued(&self) {1;} }',
                        'tiny-glade/crates/core/src/render/read_terrain_data.rs')
        candidates,method=associate('core::terrain::ScheduledTerrainReadback::on_gpu_copy_issued','crates/core/src/terrain/mod.rs',rows)
        self.assertEqual(len(candidates),1);self.assertEqual(method,'owner_candidate_across_files')

    def test_generic_lib_filename_cannot_associate_unrelated_free_functions(self):
        rows=self.parse('fn configure() {}','tiny-glade/crates/other/src/lib.rs')
        self.assertEqual(associate('original::configure','crates/original/src/lib.rs',rows)[0],[])

    def test_filtered_and_removed_symbols_never_enter_business_denominator(self):
        self.assertEqual(audit.scope('T::clone','crates/foo/src/lib.rs'),'mechanical_excluded')
        self.assertEqual(audit.scope('T::new','crates/foo/src/lib.rs'),'mechanical_excluded')
        self.assertEqual(audit.scope('theme_fetcher::fetch','crates/theme-fetcher/src/lib.rs'),'daily_deferred')
        self.assertEqual(audit.scope('feedback_reporter::send','crates/feedback-reporter/src/lib.rs'),'user_removed')

    def test_wall_constructor_stair_leaf_keeps_real_wall_ownership(self):
        self.assertEqual(audit.primary('crates/systems/wall-constructor/src/stairs_signifiers.rs'),'M06')
        self.assertEqual(audit.primary('crates/systems/wall-constructor/src/construct_platform_stairs/mod.rs'),'M08')

if __name__=='__main__':unittest.main()
