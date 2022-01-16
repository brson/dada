use dada_ir::{filename::Filename, span::FileSpan};
use dada_execute::heap_graph::HeapGraph;
use tokio::io::AsyncWriteExt;

pub struct Kernel {}

impl Kernel {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl dada_execute::kernel::Kernel for Kernel {
    async fn print(&self, text: &str) -> eyre::Result<()> {
        let mut stdout = tokio::io::stdout();
        let mut text = text.as_bytes();
        while !text.is_empty() {
            let written = stdout.write(text).await?;
            text = &text[written..];
        }
        return Ok(());
    }

    fn breakpoint_start(
        &self,
        db: &dyn dada_execute::Db,
        breakpoint_filename: Filename,
        breakpoint_index: usize,
        generate_heap_graph: &dyn Fn() -> HeapGraph,
    ) -> eyre::Result<()> {
        Ok(())
    }

    fn breakpoint_end(
        &self,
        db: &dyn dada_execute::Db,
        breakpoint_filename: Filename,
        breakpoint_index: usize,
        breakpoint_span: FileSpan,
        generate_heap_graph: &dyn Fn() -> HeapGraph,
    ) -> eyre::Result<()> {
        Ok(())
    }
}
