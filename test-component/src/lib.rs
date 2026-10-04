wit_bindgen::generate!({ generate_all });

struct BashTester;

use crate::{example::bash::bash::BashRunner, exports::example::testing_bash::doit::Guest};

impl Guest for BashTester {
    fn run() -> Result<String, String> {
        let runner = BashRunner::new();

        dbg!(runner.list_files()?);
        runner.write_string("./hallo.txt", "this is some text, nice")?;
        runner.write_string("/home/user/hallo.txt", "this is some text in user folder")?;
        dbg!(runner.read_string("./hallo.txt")?);
        dbg!(runner.list_files()?);

        dbg!(runner.execute("pwd && ls && cat hallo.txt")?);
        #[cfg(feature = "python")]
        dbg!(runner.execute(
            r#"python3 -c "
with open('/tmp/data.txt', 'w') as f:
    f.write('hello from python')
""#
        )?);
        #[cfg(feature = "python")]
        dbg!(runner.execute("cat /tmp/data.txt | grep python")?);

        dbg!(runner.execute_exit_response("echo hello && >&2 echo error && exit 24")?);

        Ok(String::new())
    }
}

export!(BashTester);
