import { $ } from "bun";
import { test, expect } from "bun:test";

test("wac", async () => {
  let res = await $`just wac`.nothrow();

  expect({
    stdout: res.stdout.toString(),
    stderr: res.stderr.toString(),
  }).toMatchSnapshot();
});

test("wac-no-python", async () => {
  let res = await $`just wac`.nothrow();

  expect({
    stdout: res.stdout.toString(),
    stderr: res.stderr.toString(),
  }).toMatchSnapshot();
});
