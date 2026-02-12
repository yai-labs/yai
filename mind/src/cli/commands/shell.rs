use crate::cli::config::RuntimeConfig;
use crate::transport::rpc::protocol::{Request, Response};
use crate::transport::rpc::uds_client;
use anyhow::Result;

pub fn exec(
    cfg: &RuntimeConfig,
    ws: &str,
    cmd: &str,
    args: &[String],
    cwd: Option<String>,
) -> Result<()> {
    let req = Request::ShellExec {
        cmd: cmd.to_string(),
        args: args.to_vec(),
        cwd,
    };
    match uds_client::send_request(&cfg.run_dir, ws, &req)? {
        Response::ShellExec {
            exit_code,
            stdout,
            stderr,
        } => {
            if !stdout.is_empty() {
                print!("{stdout}");
            }
            if !stderr.is_empty() {
                eprint!("{stderr}");
            }
            if exit_code != 0 {
                anyhow::bail!("shell exit code: {exit_code}");
            }
            Ok(())
        }
        Response::Error { message } => anyhow::bail!("{message}"),
        other => anyhow::bail!("unexpected response: {other:?}"),
    }
}
