load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

def rules_fluent_types_dependencies():
  http_archive(
      name = "rules_rust",
      sha256 = "2466e5b2514772e84f9009010797b9cd4b51c1e6445bbd5b5e24848d90e6fb2e",
      urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.18.0/rules_rust-v0.18.0.tar.gz"],
  )
