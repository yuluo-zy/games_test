# B actual loading UI producer

Original RNE SHA f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03. This is B scope despite C-default task inventory.

Producer registration: loading plugin tuple1400cd450 includes SplashScreenPlugin. build14068e570 writes dl24hex and registers splash ui141ec1390 in Update set36. Its four ECS inputs are ResMut<MeiContext>, Local<f32>, Res<GameTime>, Res<LogoAnimation>.

ui computes alpha1-clamp(progress4/durations4,0,1), full-window root named logos, padding50 on all four edges, main SpaceBetween3, cross End3. It invokes closure140f99420 with this alpha and height150. Afterwards Local<f32> += GameTime.delta (the local does not otherwise feed this B UI).

Closure sets root local opacity, makes devs child with default horizontal layout, and image children pounce light at150, empty pad0 width20, heart at75, empty pad1 width10, noknok at120. Separate root image fmod at60. All four images ignore input. Exact image asset names: menu/pounce_light_logo, menu/heart, menu/noknok_logo, menu/fmod_logo. IDs use zero-key SipHash and original TypeId high word, not current compiler TypeId.

Image constructor140f9fca0 creates typed ImageContent (native TypeId high01f6d60c8858b8ff), sets name/height/fit-parent-height=false. Actual ImageContent vtable142baf630 points prepare14067eba0/render140680330; impl25 functions are another type and MUST NOT be used. Named image preparation gets real TextureLibrary handle+texture, allocates bindless index, uses actual extent, computes dimensions=(w,h)*(requested_height/h), encodes Image(index,Stretch)+FillColor(stored white), returns absolute intrinsic dimensions. Draw centers this computed extent in the node; optional fit-parent-height and stretch-width follow the original bool fields.

Explicit separate boundary: original file: dynamic image cache/upload path also exists in ImageContent::prepare_layout. Startup uses four named assets only. That unrelated file-image upload/cache branch is not claimed restored by this producer.

Consumer: existing mei_post_update prepares these images, performs rooted layout and draws real RawShapeStream rectangles; existing flush_mei_frame uploads once in App.update then graph consumers reuse output through retries.

Required integration: toolbar lib exports menu; loading App registers splash::ui in set36. mei_pre_update must precede rebuilding nodes at its original set. Four real named textures must be registered in the same per-App TextureLibrary and actual TextureHost. No diagnostic/dummy texture substitutes.

Pre schedule: core build14068732a calls chain-ignore-deferred140f98b20 whose system name pool142baef3d identifies mei_pre_update. build1406873a4 writes6, in_set1406873d2 andUpdate registration14068742a. The source consumes digital19..23 from reconstructed authoritative actions, correcting old independent projection defaults.
