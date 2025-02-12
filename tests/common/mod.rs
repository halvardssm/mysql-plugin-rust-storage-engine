use futures_util::stream::StreamExt;
use futures_util::TryStreamExt;
use podman_api::api::{Container, Exec};
use podman_api::models::{ContainerCreateCreatedBody, ContainerMount, Schema2HealthConfig};
use podman_api::opts::{ContainerCreateOpts, ExecCreateOpts, ExecStartOpts, PullOpts};
use podman_api::Podman;
use std::error::Error;
use std::time::Duration;

const IMAGE_NAME: &str = "mariadb";
const IMAGE_TAG: &str = "11.4";
const IMAGE: &str = "mariadb:11.4";

fn get_podman() -> Podman {
    Podman::unix("/var/run/docker.sock")
}

pub async fn container_remove(id: &str) -> Result<(), Box<dyn Error>> {
    let _ = get_podman().containers().get(id).kill().await;
    let _ = get_podman().containers().get(id).remove().await;

    Ok(())
}

pub async fn container_create(name: &str) -> Result<ContainerCreateCreatedBody, Box<dyn Error>> {
    let current_dir = std::env::current_dir()?
        .into_os_string()
        .into_string()
        .expect("Could not get current dir");

    let events = get_podman()
        .images()
        .pull(
            &PullOpts::builder()
                .reference(format!(
                    "docker.io/library/{}/{}",
                    IMAGE_NAME, IMAGE_TAG
                ))
                .build(),
        )
        .map(|report| {
            report.and_then(|report| match report.error {
                Some(error) => Err(podman_api::Error::InvalidResponse(error)),
                None => Ok(report),
            })
        })
        .try_collect::<Vec<_>>()
        .await;

    if let Err(e) = events {
        eprintln!("{}", e);
    }

    let container = get_podman()
        .containers()
        .create(
            &ContainerCreateOpts::builder()
                .image(IMAGE)
                .name(name)
                .work_dir("/code")
                .mounts([ContainerMount {
                    destination: Some("/code".to_string()),
                    options: None,
                    source: Some(current_dir),
                    _type: Some("bind".to_string()),
                    uid_mappings: None,
                    gid_mappings: None,
                }])
                .remove(true)
                .env([("MARIADB_ALLOW_EMPTY_ROOT_PASSWORD", "true")])
                .health_config(Schema2HealthConfig {
                    timeout: Some(Duration::from_secs(5).as_nanos() as i64),
                    interval: Some(Duration::from_secs(5).as_nanos() as i64),
                    retries: Some(Duration::from_secs(5).as_nanos() as i64),
                    start_period: None,
                    test: Some(
                        [
                            "CMD-SHELL",
                            "healthcheck.sh",
                            "--connect",
                            "--innodb_initialized",
                        ]
                        .iter()
                        .map(|s| s.to_string())
                        .collect(),
                    ),
                })
                .build(),
        )
        .await?;

    Ok(container)
}

pub async fn container_start(id: &str) -> Result<Container, Box<dyn Error>> {
    let container = get_podman().containers().get(id);

    container.start(None).await?;

    Ok(container)
}

pub async fn container_wait_for_healthy(id: &str) -> Result<Container, Box<dyn Error>> {
    let container = get_podman().containers().get(id);

    loop {
        let res = container.healthcheck().await?;
        println!("Healthcheck for container: {}: {:?}", id, res);
        if res.status == Some("healthy".to_string()) {
            break;
        }
        tokio::time::sleep(Duration::from_secs(2)).await;
    }

    Ok(container)
}

pub async fn container_exec_copy_plugin(container: &Container) -> Result<(), Box<dyn Error>> {
    let exec_result = container
        .create_exec(
            &ExecCreateOpts::builder()
                .attach_stdin(false)
                .attach_stderr(false)
                .attach_stdout(false)
                .command([
                    "cp",
                    "target/plugin/libcrustdb.so",
                    "/usr/lib/mysql/plugin/crustdb.so",
                ])
                .build(),
        )
        .await;
    assert!(exec_result.is_ok());
    let exec = exec_result.unwrap();
    let opts = ExecStartOpts::builder().detach(true).build();
    let exec_stream = exec.start(&opts).await.unwrap();
    assert!(exec_stream.is_none());

    let exec_inspect_result = exec.inspect().await;
    assert!(exec_inspect_result.is_ok());
    let exec_inspect_data = exec_inspect_result.unwrap();
    assert_eq!(exec_inspect_data.get("ExitCode").unwrap(), 0);

    Ok(())
}

pub async fn container_create_start_and_wait_for_healthy(
    name: &str,
) -> Result<Container, Box<dyn Error>> {
    let id = format!("mariadb-test-{}", name);
    println!("Removing existing container: {}", &id);
    container_remove(&id).await?;
    println!("Creating container: {}", &id);
    container_create(&id).await?;
    println!("Starting container: {}", &id);
    let container = container_start(&id).await?;
    println!("Waiting for container to start: {}", &id);
    container_wait_for_healthy(&id).await?;

    container_exec_copy_plugin(&container).await?;

    Ok(container)
}

pub async fn container_exec(
    container: &Container,
    command: Vec<&str>,
) -> Result<Exec, Box<dyn Error>> {
    let exec_result = container
        .create_exec(
            &ExecCreateOpts::builder()
                .attach_stderr(true)
                .attach_stdout(true)
                .command(command)
                .build(),
        )
        .await;
    assert!(exec_result.is_ok());
    let exec = exec_result.unwrap();

    Ok(exec)
}

// let exec = container
// .create_exec(
//     &ExecCreateOpts::builder()
//         .command([
//             "cp",
//             "target/plugin/libcrustdb.so",
//             "/usr/lib/mysql/plugin/crustdb.so",
//         ])
//         .attach_stdout(true)
//         .attach_stderr(true)
//         .build(),
// )
// .await?;

// let opts = Default::default();
// let mut exec_stream = exec.start(&opts).await.unwrap().unwrap();
// let chunk = exec_stream.next().await;
// assert!(chunk.is_some());
// match chunk.unwrap() {
// Ok(TtyChunk::StdOut(chunk)) => {
//     let testfile_content = String::from_utf8_lossy(&chunk);
//     assert_eq!(testfile_content, "1234\n");
// }
// Ok(chunk) => {
//     let fd = match chunk {
//         TtyChunk::StdIn(_) => "stdin",
//         TtyChunk::StdOut(_) => "stdOut",
//         TtyChunk::StdErr(_) => "stderr",
//     };
//     let chunk = String::from_utf8_lossy(&chunk);
//     eprintln!("invalid chunk, fd: {fd}, content: `{chunk:?}`");
//     std::process::exit(1);
// }
// chunk => {
//     eprintln!("invalid chunk {chunk:?}");
//     std::process::exit(1);
// }
// }
