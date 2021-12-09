workspace(name = "bazel-linking-issue")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "5c3814a8007c32e4ecb1d264c9a6c8b8fdd25713563882e26cf46cbd00c2e702",
    strip_prefix = "rules_rust-6d5598ccf30c8b3a37c703e8f89726a1cb2d17a9",
    urls = [
        "https://github.com/bazelbuild/rules_rust/archive/6d5598ccf30c8b3a37c703e8f89726a1cb2d17a9.tar.gz",
    ],
)

load("@rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories(version = "1.56.1")
