# Changelog

## [1.0.0](https://github.com/tmcinerney/ddog/compare/v0.1.0...v1.0.0) (2025-12-17)


### ⚠ BREAKING CHANGES

* Command structure has changed to domain-first pattern. All commands now require both a domain and action verb.

### Features

* restructure CLI from dd-search to ddog with domain-first commands ([5abf176](https://github.com/tmcinerney/ddog/commit/5abf1766831972bc22a99f21b4f6d7744a3f7427))


### Bug Fixes

* add missing logs module files ignored by global gitignore ([7245b1a](https://github.com/tmcinerney/ddog/commit/7245b1a14495687d0da46a8ac462a6f02a05aaf8))

## 0.1.0 (2025-12-16)


### Features

* add verbose logging capabilities and URL construction for Datadog UI ([f3aa928](https://github.com/tmcinerney/ddog/commit/f3aa9283bfac79d44dc60ad589bda9813034ac01))
* enhance time handling and add integration tests ([6eb1950](https://github.com/tmcinerney/ddog/commit/6eb1950e3efdb6e870db77e0c5b3d9ad5d1bad6c))
* initial implementation of Datadog CLI search tool ([2dc9ba7](https://github.com/tmcinerney/ddog/commit/2dc9ba7e3d1bfea9a81655fb2ef9681c40c451b7))
* integrate cargo-husky for pre-commit hooks and add serial test support ([9ece799](https://github.com/tmcinerney/ddog/commit/9ece79914a7e5d6bc08054d28c54ac741c533cb1))
* **metrics:** add metrics querying and listing commands ([6b5bfe4](https://github.com/tmcinerney/ddog/commit/6b5bfe4034e769d1fdd70a86b56ef21fc3930f39))


### Bug Fixes

* **lint:** resolve CI formatting and clippy errors ([fcfeda7](https://github.com/tmcinerney/ddog/commit/fcfeda7da79cf8e47820da29b0e0f3064b41c290))
