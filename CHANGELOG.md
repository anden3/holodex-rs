# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.2.0 (2021-10-19)

### New Features

 - <csr-id-413fde120f179f4eb28eb26fc72f42b7da3aeca8/> add feature-gated streaming methods
 - <csr-id-7605aa36fd244c484c9dcc1ac2ab1b7bad03aa9e/> add convenience methods to `ChannelId`
 - <csr-id-3bc37526a10899b492f98a7f2bb274489555bef8/> add convenience methods to `VideoId`
   Add `metadata()`, `timestamps()`, and `related()` methods to `VideoId`.
 - <csr-id-bf7c62e23bc4bbe8912ca45d2184055057fcee63/> implement `Into<Vec<T>>` for `PaginatedResult`
 - <csr-id-cfd677d70f2a5d247390800852a5b5fd8f88ae2a/> add `channels` method and types
 - <csr-id-66f4d87140a42e94c64e68cf0711949c950f6653/> add `FromStr` to ID types.
 - <csr-id-019b50c29449cb856f2618737874024b1a9159bb/> add PaginatedResult::into_items
   Add a method to convert a PaginatedResult<T> into a Vec<T>.
   This consumes the result.
 - <csr-id-4e53dfeb90b2151d71f1398892e91e67345aaf60/> make Client derive Clone

### Bug Fixes

 - <csr-id-1963b860f315f6f530e72c127bdd234bdb1b67f5/> add manual impl's of some traits
   `Duration` stopped deriving `PartialEq`, `Hash` etc.
   Therefore we switched to manual implementations omitting `Duration`s.
 - <csr-id-5b52ffea05de5d5fd92c18602295d27288b84812/> accept `Video` without a duration
   BREAKING CHANGES: `Video::duration` is now wrapped in an `Option`
 - <csr-id-4db0367ed8eac79f044a7f03f0b9fcaec5b4d78f/> fix faulty channel ID regex

### New Features (BREAKING)

 - <csr-id-5ccbeac09c4d98f936862d9077e2bfeba98543df/> add support for multiple IDs
   Holodex supports multiple video IDs to be specified in some endpoints.

### refactor (BREAKING)

 - <csr-id-8c1e9b9e7c912cd28a017e492872722d64f7f46b/> rename a few types and fields
   A few types and fields were removed to make room for other types and to
   make things more consistent.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 17 commits contributed to the release.
 - 17 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - add feature-gated streaming methods ([`413fde1`](https://github.com/git///anden3/holodex-rs.git/commit/413fde120f179f4eb28eb26fc72f42b7da3aeca8))
    - add manual impl's of some traits ([`1963b86`](https://github.com/git///anden3/holodex-rs.git/commit/1963b860f315f6f530e72c127bdd234bdb1b67f5))
    - enable more clippy flags ([`1f09ff2`](https://github.com/git///anden3/holodex-rs.git/commit/1f09ff259749fe3253a9f137bb2a621cfbdcceb7))
    - accept `Video` without a duration ([`5b52ffe`](https://github.com/git///anden3/holodex-rs.git/commit/5b52ffea05de5d5fd92c18602295d27288b84812))
    - rename topics field to singular ([`d89de66`](https://github.com/git///anden3/holodex-rs.git/commit/d89de66cdebb3b95dd6b48f8f136fa16e94e6733))
    - `query_videos` now takes http directly ([`331b33b`](https://github.com/git///anden3/holodex-rs.git/commit/331b33bb8e666bb9e512291f683a9d71de7f930e))
    - `VideoFilterBuilder::status` now accepts slices ([`2fa4d15`](https://github.com/git///anden3/holodex-rs.git/commit/2fa4d159022b20c5d646c927e160d55cbecd0168))
    - fix faulty channel ID regex ([`4db0367`](https://github.com/git///anden3/holodex-rs.git/commit/4db0367ed8eac79f044a7f03f0b9fcaec5b4d78f))
    - add convenience methods to `ChannelId` ([`7605aa3`](https://github.com/git///anden3/holodex-rs.git/commit/7605aa36fd244c484c9dcc1ac2ab1b7bad03aa9e))
    - add convenience methods to `VideoId` ([`3bc3752`](https://github.com/git///anden3/holodex-rs.git/commit/3bc37526a10899b492f98a7f2bb274489555bef8))
    - implement `Into<Vec<T>>` for `PaginatedResult` ([`bf7c62e`](https://github.com/git///anden3/holodex-rs.git/commit/bf7c62e23bc4bbe8912ca45d2184055057fcee63))
    - add `channels` method and types ([`cfd677d`](https://github.com/git///anden3/holodex-rs.git/commit/cfd677d70f2a5d247390800852a5b5fd8f88ae2a))
    - rename a few types and fields ([`8c1e9b9`](https://github.com/git///anden3/holodex-rs.git/commit/8c1e9b9e7c912cd28a017e492872722d64f7f46b))
    - add `FromStr` to ID types. ([`66f4d87`](https://github.com/git///anden3/holodex-rs.git/commit/66f4d87140a42e94c64e68cf0711949c950f6653))
    - add support for multiple IDs ([`5ccbeac`](https://github.com/git///anden3/holodex-rs.git/commit/5ccbeac09c4d98f936862d9077e2bfeba98543df))
    - add PaginatedResult::into_items ([`019b50c`](https://github.com/git///anden3/holodex-rs.git/commit/019b50c29449cb856f2618737874024b1a9159bb))
    - make Client derive Clone ([`4e53dfe`](https://github.com/git///anden3/holodex-rs.git/commit/4e53dfeb90b2151d71f1398892e91e67345aaf60))
</details>

## v0.1.0 (2021-10-16)

### New Features

 - <csr-id-78d80a320fb3de7eda4e6df56e0b67d841fc3088/> add video metadata methods
   Add methods to query endpoints for metadata about a particular video.
 - <csr-id-343cfb4948f9074831089d2477b134c812a221f4/> add search methods
   Add methods to query the search videos and comment endpoints.
 - <csr-id-69c322b2fd25d7b53c3bebfe3dbf38bef0d80926/> start adding endpoints
   Added several endpoints to the client.
 - <csr-id-c7e78795b777874f84f44f952e34466bf2c2665a/> add more types
   Add more types, such as ID wrappers for videos and channels, and also
   derive more traits for all types.
 - <csr-id-ab17a39cade4ff3aa1a533f4f13d14220070cdf0/> add more error types for parsing
   Add error types for server issues and parsing problems.
 - <csr-id-d5c5c398d19a483e83f512de46b5e1d9c173733e/> add holodex models
   Add the different models that the Holodex API uses.
 - <csr-id-e7e2ce37bebd704fa78d09f02010a21b8018bf87/> add client struct
   Add client struct that contains an inner HTTP client, and all endpoints are accessible through.
 - <csr-id-3c1c610d78de11c7b205e944072367707e8b3aed/> initial commit

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 12 commits contributed to the release over the course of 2 calendar days.
 - 11 commits where understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' where seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release holodex v0.1.0 ([`f97a591`](https://github.com/git///anden3/holodex-rs.git/commit/f97a591460b9f8e75f957e2c08b67647ee59f0f8))
    - add changelog ([`1567062`](https://github.com/git///anden3/holodex-rs.git/commit/1567062b323bba2ea115a2fe1c44b7d7a2653c5d))
    - add more examples ([`0593d1d`](https://github.com/git///anden3/holodex-rs.git/commit/0593d1dbdd56dbee90f7a08d535fb313d2c8e051))
    - rename and rearrange types ([`7754dad`](https://github.com/git///anden3/holodex-rs.git/commit/7754dadc2af0b1f1b21eae3eb4904fb683371a8f))
    - add video metadata methods ([`78d80a3`](https://github.com/git///anden3/holodex-rs.git/commit/78d80a320fb3de7eda4e6df56e0b67d841fc3088))
    - add search methods ([`343cfb4`](https://github.com/git///anden3/holodex-rs.git/commit/343cfb4948f9074831089d2477b134c812a221f4))
    - start adding endpoints ([`69c322b`](https://github.com/git///anden3/holodex-rs.git/commit/69c322b2fd25d7b53c3bebfe3dbf38bef0d80926))
    - add more types ([`c7e7879`](https://github.com/git///anden3/holodex-rs.git/commit/c7e78795b777874f84f44f952e34466bf2c2665a))
    - add more error types for parsing ([`ab17a39`](https://github.com/git///anden3/holodex-rs.git/commit/ab17a39cade4ff3aa1a533f4f13d14220070cdf0))
    - add holodex models ([`d5c5c39`](https://github.com/git///anden3/holodex-rs.git/commit/d5c5c398d19a483e83f512de46b5e1d9c173733e))
    - add client struct ([`e7e2ce3`](https://github.com/git///anden3/holodex-rs.git/commit/e7e2ce37bebd704fa78d09f02010a21b8018bf87))
    - initial commit ([`3c1c610`](https://github.com/git///anden3/holodex-rs.git/commit/3c1c610d78de11c7b205e944072367707e8b3aed))
</details>

