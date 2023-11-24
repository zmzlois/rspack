use std::{
  path::{Path, PathBuf},
  process::{Command, Stdio},
};

pub fn evaluate_to_json(config_path: &Path) -> Vec<u8> {
  let r = Command::new("node")
    .args(["-p", &get_evaluate_code(config_path)])
    .stdout(Stdio::piped())
    .spawn()
    .expect("ok");
  let out = r.wait_with_output().expect("ok");
  out.stdout
}

fn get_evaluate_code(config_path: &Path) -> String {
  let workspace_dir = PathBuf::from(env!("CARGO_WORKSPACE_DIR"));
  let rspack_path = workspace_dir.join("packages").join("rspack");
  fn strip_long(s: impl Into<String>) -> String {
    let s = s.into();
    if let Some(s) = s.strip_prefix("\\\\?\\") {
      return s.to_string();
    }
    s
  }
  let rspack_path = strip_long(rspack_path.to_string_lossy());
  let test_dir = strip_long(config_path.parent().expect("TODO:").to_string_lossy());
  let config_path = strip_long(config_path.to_string_lossy());
  format!(
    r#"
const path = require("path");
let rspackPath = {rspack_path:?};
let configPath = {config_path:?};
let testDir = {test_dir:?};
const rspack = require(rspackPath);
const config = require(configPath);
config.context ??= testDir;
config.output ??= {{}};
config.output.path ??= path.join(testDir, "dist");
const normalized = rspack.config.getNormalizedRspackOptions(config);
// TODO: remove until builtins are removed.
let builtins = normalized.builtins;
builtins.treeShaking ??= "false";
builtins.react ??= {{}};
builtins.noEmitAssets ??= false;
builtins.devFriendlySplitChunks ??= false;
rspack.config.applyRspackOptionsDefaults(normalized);
const raw = rspack.config.__do_not_use_getRawOptions(normalized);
JSON.stringify(raw, null, 2)
"#
  )
}

pub fn evaluate_js(input: &str) -> String {
  let r = Command::new("node")
    .args(["-e", input])
    .stdout(Stdio::piped())
    .spawn()
    .expect("ok");
  let out = r.wait_with_output().expect("ok");
  String::from_utf8(out.stdout).expect("failed to evaluate")
}
