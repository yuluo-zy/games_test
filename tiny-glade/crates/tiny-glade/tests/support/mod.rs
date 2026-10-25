#![allow(dead_code)]
mod upload_wait;
use rhapsody::rg::SessionFrameReceipt;
use tiny_glade::render_bridge::{
    BuiltCoreGraph, CompiledCoreGraph, CoreGraphFrame, CoreRenderBridge,
};
pub use upload_wait::until_ready;

/// Integration tests drive uploads explicitly before native execution.
pub trait BridgeTestDriver {
    fn prepare_test_execution(&self) -> rhapsody::Result<()>;
    fn execute_after_uploads(
        &mut self,
        frame: CompiledCoreGraph,
    ) -> rhapsody::Result<SessionFrameReceipt>;
    fn execute_test_graph(
        &mut self,
        graph: BuiltCoreGraph,
    ) -> rhapsody::Result<SessionFrameReceipt>;
    fn render_test_frame<T>(
        &mut self,
        build: impl FnOnce(&mut CoreGraphFrame<'_>) -> rhapsody::Result<T>,
    ) -> rhapsody::Result<(T, SessionFrameReceipt)>;
}
impl BridgeTestDriver for CoreRenderBridge {
    fn prepare_test_execution(&self) -> rhapsody::Result<()> {
        until_ready(|| {
            let pipelines = self.shader_backend().poll_preparation()?;
            let uploads = self.hosts().buffers.poll_pending_uploads()?;
            let textures = self.hosts().textures.poll_pending_uploads()?;
            Ok(pipelines && uploads && textures)
        })
    }
    fn execute_after_uploads(
        &mut self,
        frame: CompiledCoreGraph,
    ) -> rhapsody::Result<SessionFrameReceipt> {
        self.prepare_test_execution()?;
        self.execute_compiled(frame)
    }
    fn execute_test_graph(
        &mut self,
        graph: BuiltCoreGraph,
    ) -> rhapsody::Result<SessionFrameReceipt> {
        let frame = self.compile_built(graph)?;
        self.execute_after_uploads(frame)
    }
    fn render_test_frame<T>(
        &mut self,
        build: impl FnOnce(&mut CoreGraphFrame<'_>) -> rhapsody::Result<T>,
    ) -> rhapsody::Result<(T, SessionFrameReceipt)> {
        let (value, graph) = self.build_frame(build)?;
        let receipt = self.execute_test_graph(graph)?;
        Ok((value, receipt))
    }
}

/// Tests drive the same pending pipeline protocol as the resident owner.
pub trait ShaderTestDriver {
    fn prepare_test_pipelines(&self, timeout: std::time::Duration) -> rhapsody::Result<()>;
}
impl ShaderTestDriver for tiny_glade::render_host::ShaderHost {
    fn prepare_test_pipelines(&self, timeout: std::time::Duration) -> rhapsody::Result<()> {
        let deadline = std::time::Instant::now()
            .checked_add(timeout)
            .ok_or(rhapsody::Error::Timeout)?;
        while !self.poll_preparation()? {
            if std::time::Instant::now() >= deadline {
                return Err(rhapsody::Error::Timeout);
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        Ok(())
    }
}
