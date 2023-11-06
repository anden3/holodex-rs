# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## v0.3.2 (2023-11-06)

### Chore

 - <csr-id-f3f575a61d23e06a9e200b96e290e135f7fd8d51/> update cargo_with
 - <csr-id-44a51a26d49032f9c553e86a6647623c2a5b0838/> update dependencies

### Documentation

 - <csr-id-140eaeda09fc75396c8fe81d8facc9acfb2204f1/> add cargo-deny config

### New Features

 - <csr-id-36c935e91197c1a51bef70630bbbc8fabd3d8944/> add support for `from` parameter

### Bug Fixes

 - <csr-id-7cfb18feec716ca4d7bcc3c20d2a118b65842823/> don't fail requests if missing content-length

### Refactor

 - <csr-id-c87a47725c91b0787c89ef8c7b5630de6ba832b5/> stop using async_stream internals
 - <csr-id-063545db6fb528f749f23bddf5f35a508400625b/> fix clippy warnings

### Style

 - <csr-id-6cf6ad798067d72f55f643487ea44e026a6bd78e/> fix clippy warnings

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 555 calendar days.
 - 720 days passed between releases.
 - 8 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add cargo-deny config ([`140eaed`](https://github.com/anden3/holodex-rs/commit/140eaeda09fc75396c8fe81d8facc9acfb2204f1))
    - Update cargo_with ([`f3f575a`](https://github.com/anden3/holodex-rs/commit/f3f575a61d23e06a9e200b96e290e135f7fd8d51))
    - Update dependencies ([`44a51a2`](https://github.com/anden3/holodex-rs/commit/44a51a26d49032f9c553e86a6647623c2a5b0838))
    - Fix clippy warnings ([`6cf6ad7`](https://github.com/anden3/holodex-rs/commit/6cf6ad798067d72f55f643487ea44e026a6bd78e))
    - Don't fail requests if missing content-length ([`7cfb18f`](https://github.com/anden3/holodex-rs/commit/7cfb18feec716ca4d7bcc3c20d2a118b65842823))
    - Stop using async_stream internals ([`c87a477`](https://github.com/anden3/holodex-rs/commit/c87a47725c91b0787c89ef8c7b5630de6ba832b5))
    - Fix clippy warnings ([`063545d`](https://github.com/anden3/holodex-rs/commit/063545db6fb528f749f23bddf5f35a508400625b))
    - Add support for `from` parameter ([`36c935e`](https://github.com/anden3/holodex-rs/commit/36c935e91197c1a51bef70630bbbc8fabd3d8944))
</details>

## v0.3.1 (2021-11-15)

### Bug Fixes

 - <csr-id-869e91e8da6273d71d14891dcaae792c27d1c161/> queries with multiple parameters failed deserialization

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 1 day passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release holodex v0.3.1 ([`1e262fc`](https://github.com/anden3/holodex-rs/commit/1e262fc07ae27a6552fefd4e5ffa080828d4cf55))
    - Queries with multiple parameters failed deserialization ([`869e91e`](https://github.com/anden3/holodex-rs/commit/869e91e8da6273d71d14891dcaae792c27d1c161))
</details>

## v0.3.0 (2021-11-14)

### Refactor

 - <csr-id-8dbcc1ac9ff8d0aade35719e158c2b9659488576/> remove missed `itertools` usage
 - <csr-id-f3ff4a2a71eb542a5dcb7cf3b806717faed676d0/> remove unneeded `regex` perf feature
 - <csr-id-264f199754de5d8afa9be9fa5ff1801a39c12fe3/> replace `futures` with `futures-core`
   `futures` was added to the dev-dependencies so doctests can pass.
 - <csr-id-d3ba2a45fdcf4599673e6b919548a859aa6023da/> remove dependency on `async-stream` proc macros
 - <csr-id-0689e30cc62bd2bd93a110f6a95e74ba9713bbbc/> replace `thiserror` and `miette` with `quick_error`
 - <csr-id-c0fc601ce6a22214f71f6a52346b0c2bdac4748e/> remove mostly unused `tracing` dependency
 - <csr-id-fab609114890d9ef5f0ea99531dcd3c65cc2abea/> remove dependency on `serde-enum-str`
 - <csr-id-f2001047da9b4a89a721a2a133b6a7c349f10650/> remove dependency on `strum`
 - <csr-id-cfe38782cc3587ddbd3b2672e85d07939a8d0b22/> remove `itertools` dependency
 - <csr-id-c28ff91d4ccceb9772f68013cf4965213277308f/> remove dependency on `serde_with` proc-macros

### Other

 - <csr-id-4ff2966fb5f8737c0e711c39f98f5fcee09ebad6/> re-enable disabled test
 - <csr-id-8ea93418cbd4b97c212ee37168ee9c880c761399/> add logging to `Client` streaming

### New Features

 - <csr-id-632b886231907be4b6e9ed547e5b8e5d97eb96ba/> replace several `Into` impls with `From`

### New Features (BREAKING)

 - <csr-id-d8245fd04b89a6d50620a8c60516b2a616c88a9a/> replace `reqwest` with `ureq`
   To bring down the dependency count and complexity, the HTTP client has
   been replaced by a simpler sync one.
 - <csr-id-fd2038851aebae1e36f126345161c6aa6a335c6c/> add sso feature and change id traits
   Add an opt-out feature to store `VideoId` and `ChannelId` in a `smartstring` type.
   Remove `From` impls for IDs, and impl `TryFrom` instead, to force the use of valid IDs.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 23 calendar days.
 - 24 days passed between releases.
 - 15 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release holodex v0.3.0 ([`ec097fa`](https://github.com/anden3/holodex-rs/commit/ec097fa8e55f588b8f69c7c54e48397ba9988db6))
    - Re-enable disabled test ([`4ff2966`](https://github.com/anden3/holodex-rs/commit/4ff2966fb5f8737c0e711c39f98f5fcee09ebad6))
    - Remove missed `itertools` usage ([`8dbcc1a`](https://github.com/anden3/holodex-rs/commit/8dbcc1ac9ff8d0aade35719e158c2b9659488576))
    - Remove unneeded `regex` perf feature ([`f3ff4a2`](https://github.com/anden3/holodex-rs/commit/f3ff4a2a71eb542a5dcb7cf3b806717faed676d0))
    - Replace `reqwest` with `ureq` ([`d8245fd`](https://github.com/anden3/holodex-rs/commit/d8245fd04b89a6d50620a8c60516b2a616c88a9a))
    - Replace `futures` with `futures-core` ([`264f199`](https://github.com/anden3/holodex-rs/commit/264f199754de5d8afa9be9fa5ff1801a39c12fe3))
    - Remove dependency on `async-stream` proc macros ([`d3ba2a4`](https://github.com/anden3/holodex-rs/commit/d3ba2a45fdcf4599673e6b919548a859aa6023da))
    - Replace `thiserror` and `miette` with `quick_error` ([`0689e30`](https://github.com/anden3/holodex-rs/commit/0689e30cc62bd2bd93a110f6a95e74ba9713bbbc))
    - Remove mostly unused `tracing` dependency ([`c0fc601`](https://github.com/anden3/holodex-rs/commit/c0fc601ce6a22214f71f6a52346b0c2bdac4748e))
    - Remove dependency on `serde-enum-str` ([`fab6091`](https://github.com/anden3/holodex-rs/commit/fab609114890d9ef5f0ea99531dcd3c65cc2abea))
    - Remove dependency on `strum` ([`f200104`](https://github.com/anden3/holodex-rs/commit/f2001047da9b4a89a721a2a133b6a7c349f10650))
    - Remove `itertools` dependency ([`cfe3878`](https://github.com/anden3/holodex-rs/commit/cfe38782cc3587ddbd3b2672e85d07939a8d0b22))
    - Remove dependency on `serde_with` proc-macros ([`c28ff91`](https://github.com/anden3/holodex-rs/commit/c28ff91d4ccceb9772f68013cf4965213277308f))
    - Add sso feature and change id traits ([`fd20388`](https://github.com/anden3/holodex-rs/commit/fd2038851aebae1e36f126345161c6aa6a335c6c))
    - Replace several `Into` impls with `From` ([`632b886`](https://github.com/anden3/holodex-rs/commit/632b886231907be4b6e9ed547e5b8e5d97eb96ba))
    - Add logging to `Client` streaming ([`8ea9341`](https://github.com/anden3/holodex-rs/commit/8ea93418cbd4b97c212ee37168ee9c880c761399))
</details>

## v0.2.1 (2021-10-20)

### Other

 - <csr-id-43c26872f693c4fe5ef8c59f2bc36055af949742/> reduce redundant error messages

### Bug Fixes

 - <csr-id-f261bae57c37ba6a19e4f8d35a4a63bd90519146/> default filter limit lowered
 - <csr-id-16cf9aeb38432b19abed33fd4caface212491c59/> fix compile error as crate
   The stream methods failed to compile when used as a crate due to bad type inference.
   Extra type information has been added.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 4 commits contributed to the release.
 - 3 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release holodex v0.2.1 ([`d90977f`](https://github.com/anden3/holodex-rs/commit/d90977f0abfb571575aaaf06b6ea1014f279f88b))
    - Reduce redundant error messages ([`43c2687`](https://github.com/anden3/holodex-rs/commit/43c26872f693c4fe5ef8c59f2bc36055af949742))
    - Default filter limit lowered ([`f261bae`](https://github.com/anden3/holodex-rs/commit/f261bae57c37ba6a19e4f8d35a4a63bd90519146))
    - Fix compile error as crate ([`16cf9ae`](https://github.com/anden3/holodex-rs/commit/16cf9aeb38432b19abed33fd4caface212491c59))
</details>

## v0.2.0 (2021-10-19)

### Style

 - <csr-id-1f09ff259749fe3253a9f137bb2a621cfbdcceb7/> enable more clippy flags

### Refactor

 - <csr-id-d89de66cdebb3b95dd6b48f8f136fa16e94e6733/> rename topics field to singular
   BREAKING CHANGES: `Video::topics` renamed to `Video::topic`
 - <csr-id-331b33bb8e666bb9e512291f683a9d71de7f930e/> `query_videos` now takes http directly
 - <csr-id-2fa4d159022b20c5d646c927e160d55cbecd0168/> `VideoFilterBuilder::status` now accepts slices

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

### Refactor (BREAKING)

 - <csr-id-8c1e9b9e7c912cd28a017e492872722d64f7f46b/> rename a few types and fields
   A few types and fields were removed to make room for other types and to
   make things more consistent.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 18 commits contributed to the release.
 - 3 days passed between releases.
 - 17 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Adjusting changelogs prior to release of holodex v0.2.0 ([`15c2495`](https://github.com/anden3/holodex-rs/commit/15c24957b1ab571ab917cef68bbbdee98a01aef9))
    - Add feature-gated streaming methods ([`413fde1`](https://github.com/anden3/holodex-rs/commit/413fde120f179f4eb28eb26fc72f42b7da3aeca8))
    - Add manual impl's of some traits ([`1963b86`](https://github.com/anden3/holodex-rs/commit/1963b860f315f6f530e72c127bdd234bdb1b67f5))
    - Enable more clippy flags ([`1f09ff2`](https://github.com/anden3/holodex-rs/commit/1f09ff259749fe3253a9f137bb2a621cfbdcceb7))
    - Accept `Video` without a duration ([`5b52ffe`](https://github.com/anden3/holodex-rs/commit/5b52ffea05de5d5fd92c18602295d27288b84812))
    - Rename topics field to singular ([`d89de66`](https://github.com/anden3/holodex-rs/commit/d89de66cdebb3b95dd6b48f8f136fa16e94e6733))
    - `query_videos` now takes http directly ([`331b33b`](https://github.com/anden3/holodex-rs/commit/331b33bb8e666bb9e512291f683a9d71de7f930e))
    - `VideoFilterBuilder::status` now accepts slices ([`2fa4d15`](https://github.com/anden3/holodex-rs/commit/2fa4d159022b20c5d646c927e160d55cbecd0168))
    - Fix faulty channel ID regex ([`4db0367`](https://github.com/anden3/holodex-rs/commit/4db0367ed8eac79f044a7f03f0b9fcaec5b4d78f))
    - Add convenience methods to `ChannelId` ([`7605aa3`](https://github.com/anden3/holodex-rs/commit/7605aa36fd244c484c9dcc1ac2ab1b7bad03aa9e))
    - Add convenience methods to `VideoId` ([`3bc3752`](https://github.com/anden3/holodex-rs/commit/3bc37526a10899b492f98a7f2bb274489555bef8))
    - Implement `Into<Vec<T>>` for `PaginatedResult` ([`bf7c62e`](https://github.com/anden3/holodex-rs/commit/bf7c62e23bc4bbe8912ca45d2184055057fcee63))
    - Add `channels` method and types ([`cfd677d`](https://github.com/anden3/holodex-rs/commit/cfd677d70f2a5d247390800852a5b5fd8f88ae2a))
    - Rename a few types and fields ([`8c1e9b9`](https://github.com/anden3/holodex-rs/commit/8c1e9b9e7c912cd28a017e492872722d64f7f46b))
    - Add `FromStr` to ID types. ([`66f4d87`](https://github.com/anden3/holodex-rs/commit/66f4d87140a42e94c64e68cf0711949c950f6653))
    - Add support for multiple IDs ([`5ccbeac`](https://github.com/anden3/holodex-rs/commit/5ccbeac09c4d98f936862d9077e2bfeba98543df))
    - Add PaginatedResult::into_items ([`019b50c`](https://github.com/anden3/holodex-rs/commit/019b50c29449cb856f2618737874024b1a9159bb))
    - Make Client derive Clone ([`4e53dfe`](https://github.com/anden3/holodex-rs/commit/4e53dfeb90b2151d71f1398892e91e67345aaf60))
</details>

## v0.1.0 (2021-10-16)

### Refactor

 - <csr-id-7754dadc2af0b1f1b21eae3eb4904fb683371a8f/> rename and rearrange types

### Other

 - <csr-id-1567062b323bba2ea115a2fe1c44b7d7a2653c5d/> add changelog
 - <csr-id-0593d1dbdd56dbee90f7a08d535fb313d2c8e051/> add more examples

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
 - 11 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Release holodex v0.1.0 ([`f97a591`](https://github.com/anden3/holodex-rs/commit/f97a591460b9f8e75f957e2c08b67647ee59f0f8))
    - Add changelog ([`1567062`](https://github.com/anden3/holodex-rs/commit/1567062b323bba2ea115a2fe1c44b7d7a2653c5d))
    - Add more examples ([`0593d1d`](https://github.com/anden3/holodex-rs/commit/0593d1dbdd56dbee90f7a08d535fb313d2c8e051))
    - Rename and rearrange types ([`7754dad`](https://github.com/anden3/holodex-rs/commit/7754dadc2af0b1f1b21eae3eb4904fb683371a8f))
    - Add video metadata methods ([`78d80a3`](https://github.com/anden3/holodex-rs/commit/78d80a320fb3de7eda4e6df56e0b67d841fc3088))
    - Add search methods ([`343cfb4`](https://github.com/anden3/holodex-rs/commit/343cfb4948f9074831089d2477b134c812a221f4))
    - Start adding endpoints ([`69c322b`](https://github.com/anden3/holodex-rs/commit/69c322b2fd25d7b53c3bebfe3dbf38bef0d80926))
    - Add more types ([`c7e7879`](https://github.com/anden3/holodex-rs/commit/c7e78795b777874f84f44f952e34466bf2c2665a))
    - Add more error types for parsing ([`ab17a39`](https://github.com/anden3/holodex-rs/commit/ab17a39cade4ff3aa1a533f4f13d14220070cdf0))
    - Add holodex models ([`d5c5c39`](https://github.com/anden3/holodex-rs/commit/d5c5c398d19a483e83f512de46b5e1d9c173733e))
    - Add client struct ([`e7e2ce3`](https://github.com/anden3/holodex-rs/commit/e7e2ce37bebd704fa78d09f02010a21b8018bf87))
    - Initial commit ([`3c1c610`](https://github.com/anden3/holodex-rs/commit/3c1c610d78de11c7b205e944072367707e8b3aed))
</details>

