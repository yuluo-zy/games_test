# canonical API已由root同链owner恢复

lead已明确通知world SharedShape/Clone源码独审通过：native RaycastMeshData:Clone，shape:Option<SharedShape>，shape_ref()->Option<&dyn Shape>不改。当前trap单次new_cuboid_at→data.clone到Arc1→move原data到Arc2的已证body无需伪适配；与该world候选原子集成。旧pending manifest/patch保留审计追溯，新delivery-dynamic-trap.json为有效最终候选。完整游戏/visual余下欠账未据此消失。
