wit_bindgen::generate!({ generate_all });

use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use bashkit::{Bash, FileSystem, FileType};

use crate::exports::example::bash::bash::{Guest, GuestBashRunner};
use tokio::runtime::{Builder, Runtime};

struct BashRunner {
    bash: Mutex<Bash>,
    runtime: Runtime,
}

#[cfg(not(feature = "python"))]
fn make_bash() -> Bash {
    Bash::builder().build()
}

#[cfg(feature = "python")]
fn make_bash() -> Bash {
    Bash::builder()
        .python()
        .env("BASHKIT_ALLOW_INPROCESS_PYTHON", "1")
        .build()
}

impl GuestBashRunner for BashRunner {
    fn new() -> Self {
        let runtime = Builder::new_current_thread().enable_time().build().unwrap();
        let bash = make_bash();
        BashRunner {
            bash: Mutex::new(bash),
            runtime,
        }
    }

    fn read_string(&self, path: String) -> Result<String, String> {
        let bash = self.bash.lock().unwrap();
        let fs = bash.fs();
        let path = Path::new(&path);
        let content = self
            .runtime
            .block_on(async { fs.read_file(path).await.map_err(|e| e.to_string()) })?;

        Ok(String::from_utf8_lossy(&content).into_owned())
    }

    fn write_string(&self, path: String, content: String) -> Result<(), String> {
        let bash = self.bash.lock().unwrap();
        let fs = bash.fs();
        let path = Path::new(&path);
        self.runtime.block_on(async {
            if let Some(parent) = path.parent() {
                fs.mkdir(parent, true).await.map_err(|e| e.to_string())?;
            };
            fs.write_file(path, content.as_bytes())
                .await
                .map_err(|e| e.to_string())?;
            Ok(())
        })
    }

    fn list_files(&self) -> Result<Vec<String>, String> {
        let bash = self.bash.lock().unwrap();
        let fs = bash.fs();
        let items = self.runtime.block_on(async {
            let mut items = Vec::new();
            inner_list_dir(fs, Path::new("/"), &mut items)
                .await
                .map_err(|e| e.to_string())?;
            items.sort_unstable();
            Ok::<_, String>(items)
        })?;

        Ok(items)
    }

    fn execute(&self, bash_script: String) -> Result<String, String> {
        let mut bash = self.bash.lock().unwrap();
        self.runtime.block_on(async {
            let res = bash.exec(&bash_script).await.map_err(|e| e.to_string())?;
            if res.is_success() {
                Ok(res.stdout.to_string())
            } else {
                Err(res.stderr.to_string())
            }
        })
    }
}

async fn inner_list_dir(
    fs: Arc<dyn FileSystem>,
    start: &Path,
    collected: &mut Vec<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let items = fs.read_dir(start).await?;
    for item in items {
        let mut current = PathBuf::from(start);
        current.push(item.name);
        if item.metadata.file_type == FileType::Directory {
            Box::pin(inner_list_dir(fs.clone(), current.as_path(), collected)).await?;
        }
        if item.metadata.file_type == FileType::File {
            collected.push(current.to_string_lossy().into_owned())
        }
    }

    Ok(())
}

struct BashExecutor {}

impl Guest for BashExecutor {
    type BashRunner = BashRunner;

    fn run_simple(bash_script: String) -> Result<String, String> {
        let runner = BashRunner::new();
        runner.execute(bash_script)
    }
}

export!(BashExecutor);
