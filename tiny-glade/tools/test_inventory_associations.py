"""Do not recreate mechanical/name-only owner associations on later scans."""
import unittest
from audit_b_module_inventory import association_exclusion

class AssociationTests(unittest.TestCase):
    def test_mechanical_and_visitor_names_are_excluded(self):
        for name in ('arches::impl$3::default','decorator::impl$104::fmt',
                     'Visitor::visit_byte_buf','Visitor::visit_map','T::clone_from','T::from'):
            self.assertEqual(association_exclusion(name),'mechanical_name_no_business_association')

    def test_common_methods_need_owner_evidence(self):
        for name in ('World::new','Storage::get','Map::insert','Receipts::remove'):
            self.assertEqual(association_exclusion(name),'generic_method_requires_owner_evidence')

    def test_business_named_functions_are_retained(self):
        for name in ('arches::construct_arches','World::raycast','World::remove_collider',
                     'Wall::get_inner','Height::apply_height','T::new_receipt'):
            self.assertIsNone(association_exclusion(name))

if __name__=='__main__':unittest.main()
