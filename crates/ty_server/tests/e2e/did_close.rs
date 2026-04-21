use std::time::Duration;

use anyhow::Result;
use lsp_types::notification::ShowMessage;
use ruff_db::system::SystemPath;

use crate::TestServerBuilder;

#[test]
fn close_unopened_document_is_no_op() -> Result<()> {
    let file = SystemPath::new("src/api.py");

    let mut server = TestServerBuilder::new()?
        .with_workspace(SystemPath::new("src"), None)?
        .with_file(file, "x = 1\n")?
        .build()
        .wait_until_workspaces_are_initialized();

    server.close_text_document(file);

    let result = server.try_await_notification::<ShowMessage>(Some(Duration::from_millis(200)));
    assert!(
        result.is_err(),
        "expected no error closing unopened file, got: {result:?}",
    );

    Ok(())
}
