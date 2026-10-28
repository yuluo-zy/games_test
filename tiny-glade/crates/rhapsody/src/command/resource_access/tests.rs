use super::*;
use crate::tracking::range_state::RangeOwner;
use crate::tracking::range_state::{Frontier, RangeStateConfig, RangeStateTable};

fn image(state: NativeState) -> RangeCell {
    RangeCell {
        range: OwnershipRange::Image {
            aspects: 1,
            base_mip: 0,
            mip_count: 1,
            base_layer: 0,
            layer_count: 1,
        },
        state,
    }
}
fn scope(stage: vk::PipelineStageFlags, access: vk::AccessFlags, write: bool) -> AccessScope {
    AccessScope {
        stages: stage,
        access,
        write,
    }
}
#[test]
fn write_barrier_uses_local_readers_writer_and_host_without_foreign_queue_stages() {
    let queue = QueueId::new(0).unwrap();
    let mut state = NativeState {
        layout: vk::ImageLayout::GENERAL,
        owner: RangeOwner::Owned(0),
        ..NativeState::default()
    };
    state.writer.scopes[0] = scope(
        vk::PipelineStageFlags::COMPUTE_SHADER,
        vk::AccessFlags::SHADER_WRITE,
        true,
    );
    state.writer.host = scope(
        vk::PipelineStageFlags::HOST,
        vk::AccessFlags::HOST_WRITE,
        true,
    );
    state.readers.scopes[0] = scope(
        vk::PipelineStageFlags::VERTEX_SHADER | vk::PipelineStageFlags::FRAGMENT_SHADER,
        vk::AccessFlags::SHADER_READ,
        false,
    );
    state.readers.scopes[1] = scope(
        vk::PipelineStageFlags::TRANSFER,
        vk::AccessFlags::TRANSFER_READ,
        false,
    );
    let planned = step(
        image(state),
        ResourceSharing::Exclusive,
        queue,
        0,
        scope(
            vk::PipelineStageFlags::TRANSFER,
            vk::AccessFlags::TRANSFER_WRITE,
            true,
        ),
        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        None,
    )
    .unwrap();
    assert_eq!(
        planned.source.stages,
        vk::PipelineStageFlags::HOST
            | vk::PipelineStageFlags::COMPUTE_SHADER
            | vk::PipelineStageFlags::VERTEX_SHADER
            | vk::PipelineStageFlags::FRAGMENT_SHADER
    );
    assert_eq!(
        planned.source.access,
        vk::AccessFlags::HOST_WRITE
            | vk::AccessFlags::SHADER_WRITE
            | vk::AccessFlags::SHADER_READ
    );
    assert!(
        !planned
            .source
            .stages
            .contains(vk::PipelineStageFlags::TRANSFER)
    );
}
#[test]
fn unknown_layout_old_scalar_mismatch_and_unacquired_owner_are_rejected() {
    let queue = QueueId::new(0).unwrap();
    let destination = scope(
        vk::PipelineStageFlags::TRANSFER,
        vk::AccessFlags::TRANSFER_WRITE,
        true,
    );
    let mut state = NativeState {
        layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        ..NativeState::default()
    };
    assert!(matches!(
        step(
            image(state),
            ResourceSharing::Exclusive,
            queue,
            0,
            destination,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            Some(vk::ImageLayout::UNDEFINED)
        ),
        Err(Error::InvalidUsage { .. })
    ));
    state.layout_known = false;
    assert!(matches!(
        step(
            image(state),
            ResourceSharing::Exclusive,
            queue,
            0,
            destination,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            None
        ),
        Err(Error::Unsupported(_))
    ));
    state.layout_known = true;
    state.owner = RangeOwner::Owned(1);
    assert!(matches!(
        step(
            image(state),
            ResourceSharing::Exclusive,
            queue,
            0,
            destination,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            None
        ),
        Err(Error::Unsupported(_))
    ));
}
#[test]
fn staging_initial_and_second_barriers_use_real_prepared_layout_and_revision() {
    let queue = QueueId::new(0).unwrap();
    let destination = scope(
        vk::PipelineStageFlags::TRANSFER,
        vk::AccessFlags::TRANSFER_WRITE,
        true,
    );
    let initial = image(NativeState::default());
    let first = step(
        initial,
        ResourceSharing::Exclusive,
        queue,
        0,
        destination,
        vk::ImageLayout::TRANSFER_DST_OPTIMAL,
        Some(vk::ImageLayout::UNDEFINED),
    )
    .unwrap();
    assert!(first.source.stages.is_empty());
    assert!(first.source.access.is_empty());
    let budget = Arc::new(crate::accounting::AllocatorStats::default());
    let mut table = RangeStateTable::new(
        initial.range,
        ResourceSharing::Exclusive,
        initial.state,
        RangeStateConfig::default(),
        budget,
    )
    .unwrap();
    table
        .prepare(
            &[StatePatch {
                range: first.cell.range,
                expected_revision: 0,
                action: PatchAction::Access {
                    scope: destination,
                    layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    queue,
                    family: 0,
                },
            }],
            queue,
            0,
        )
        .unwrap();
    let prepared = table.prepared_cells().next().unwrap();
    assert_eq!(prepared.state.revision, 1);
    let last = step(
        prepared,
        ResourceSharing::Exclusive,
        queue,
        0,
        scope(
            vk::PipelineStageFlags::ALL_COMMANDS,
            vk::AccessFlags::SHADER_READ,
            false,
        ),
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        Some(vk::ImageLayout::TRANSFER_DST_OPTIMAL),
    )
    .unwrap();
    assert_eq!(last.source.access, vk::AccessFlags::TRANSFER_WRITE);
    assert_eq!(last.source.stages, vk::PipelineStageFlags::TRANSFER);
    assert_eq!(
        last.cell.state.layout,
        vk::ImageLayout::TRANSFER_DST_OPTIMAL
    );
    // Preview and cancellation cannot turn the not-yet-submitted image into
    // an externally visible shader-read image.
    table.cancel_prepared();
    assert_eq!(table.cells()[0].state.layout, vk::ImageLayout::UNDEFINED);
    assert_eq!(table.cells()[0].state.writer, Frontier::EMPTY);
}
