wit_bindgen::generate!({ generate_all });

use std::sync::Mutex;

use bashkit::Bash;

use crate::exports::example::bash::bash::{Guest, GuestBashRunner};
use tokio::runtime::{Builder, Runtime};

struct BashRunner {
    bash: Mutex<Bash>,
    runtime: Runtime,
}

impl GuestBashRunner for BashRunner {
    fn new() -> Self {
        let runtime = Builder::new_current_thread().enable_time().build().unwrap();
        let bash = Bash::new();
        BashRunner {
            bash: Mutex::new(bash),
            runtime
        }
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

struct BashExecutor {}

impl Guest for BashExecutor {
    type BashRunner = BashRunner;

    fn run_simple(bash_script: String) -> Result<String, String>{
        let runner = BashRunner::new();
        runner.execute(bash_script)
    }
}

export!(BashExecutor);
