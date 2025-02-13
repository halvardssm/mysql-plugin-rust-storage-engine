use std::error::Error;
use podman_api::conn::TtyChunk;
use tokio;
use futures_util::stream::StreamExt;

use common::{container_exec, SetupTest};

mod common;

#[tokio::test]
async fn test_mariadb_query() -> Result<(), Box<dyn Error>> {
    let setup = SetupTest::init("1").await;
    
    // let exec = container_exec(&container, vec!["mariadb", "--help"]).await?;
    let exec = container_exec(&setup.container, vec!["mariadb", "--verbose","-e", "INSTALL PLUGIN rust_storage SONAME 'crustdb.so';"]).await?;

    let opts = Default::default();
    let mut exec_stream = exec.start(&opts).await.unwrap().unwrap();
    let chunk = exec_stream.next().await;
    assert!(chunk.is_some());
    match chunk.unwrap() {
        Ok(TtyChunk::StdOut(chunk)) => {
            let testfile_content = String::from_utf8_lossy(&chunk);
            assert_eq!(testfile_content, "1234\n");
        }
        Ok(chunk) => {
            let fd = match chunk {
                TtyChunk::StdIn(_) => "stdin",
                TtyChunk::StdOut(_) => "stdOut",
                TtyChunk::StdErr(_) => "stderr",
            };
            let chunk = String::from_utf8_lossy(&chunk);
            eprintln!("invalid chunk, fd: {fd}, content: `{chunk:?}`");
            std::process::exit(1);
        }
        chunk => {
            eprintln!("invalid chunk {chunk:?}");
            std::process::exit(1);
        }
    }

    Ok(())
}
