# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-08-26
### Changed
- Implement `validate_iban`, `validate_phone` (E.164), and `validate_ssn` (US).
- Implement `types::extras::try_cast` using `Any::downcast_ref`.
- Ensure `strings::truncate` is UTF-8 safe via internal `prefix_by_bytes` helper.
- Document background thread behavior in `functions::debounce`.
- Add attribution note for MurmurHash3 implementation.

### Added
- Initial utility modules: Strings, Math, Deep, Functions, Types, Collections, Time, Random, Hash, IO (fs feature), Validation, Encoding.
- Tests for `validate_iban`, `validate_phone` (E.164), and `validate_ssn`.
- Documentation examples for the new validators in `validation::mod`.
