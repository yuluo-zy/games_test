# Required window-extra accessor

This adopts the original complete getter and two actual window consumer contracts frozen by scene_startup in CONTRACT_MANIFEST_V1.json (39546ffce90b8115394e8d531b9f79766429078bd881951073def5deadddbd37). Independent local read-only verification checked every saved instruction against original B PE bytes, with full 105-byte getter coverage and both contiguous caller windows. Current canonical DecoratorInfo source is 7da61633c1ec7be6662533fa7cd61ab9229942fa2e03e5f3221641425aeb8976.

The only proposed source API is `DecoratorInfo::window_extra(&self) -> WindowExtra`. Require actual `extra_style() == 6`, otherwise panic; on success return existing `WindowExtra(self.u30 as u32)`. Native callers pass Info+0x30 to the original getter, whose +0x11 discriminator is Info+0x41. The successful returned address is dereferenced as u32 at Info+0x30. No destination/type/rank/state/finite check is added. No separate payload enum or decoder is introduced.

The existing generic `extra`, `extra_disc`, `set_extra`, default bytes and integrated destination codec remain unchanged. This does not validate upstream writes of actual +0x30/+0x41 or any other extra family. Both window generator consumers must use this required accessor after their original style gates; WindowLight key/id reads are a different contract.

Candidate validation should include actual discriminator versus legacy discriminator disagreement in both directions, exact low-u32 extraction with irrelevant upper words set, and rejection for every actual discriminator except 6. Tests are not run here. No build, GPU, production source or claim mutation accompanies this design.
