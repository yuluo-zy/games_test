# Collision Rectangle3d field-order correction

This corrects one descriptive offset sentence in frozen schema909d; do not edit that old contract/manifest. Its fields Triangles/Arc/Rectangle3d and width/height conclusions stay unchanged. The prior sentence inverted footprint axis and center.

Complete original Rectangle3d::new140c98380/305B (PDB768:252) now binds the producer directly: at9843d/441 it writes actual axisX/axisZ into result+0/+4; 98446/44b writes actual centerX/centerZ into+8/+c; 98450/455 stores sizeX/sizeZ into+10/+14; 9845a/460 stores centerY-halfheight and halfheight+centerY into+18/+1c. Thus the native collision record has bounds at+20: axis at+20/+24, center at+28/+2c, size at+30/+34, bottom/top at+38/+3c. Current safe Rectangle2d has axis,center,size fields and Rectangle3d has footprint,bottom,top, matching these logical roles. Rust layout is still not copied to native pointers.

Exact location inputs are `record.bounds.footprint.size.x` for the direct+30 width (not another min/max subtraction), and `record.bounds.top-record.bounds.bottom` for+3c-minus+38 height. Original8143B locations14080f715/71b/721 bind the consumer, and the independently retained17386B loader callbacks transfer the complete Rectangle3d into record+20. Empty collision constructor583B calls this originalRectangle3d::new with ZERO center, ONE dimensions and axis(1,0), so its true unitBBox conclusion is unchanged.

No rectangle source/claim or production changes. This pack is solely a bounded necessary field producer, reusing old decomp only as navigation and binding the full305B originalPE bytes. Scoped schema review must use this corrected offset statement, not the old transposed labels.
