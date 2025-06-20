// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2024 Oxide Computer Company

//! This crate exists only for stand-alone integration tests that act as SDK
//! consumers. This is particularly necessary for tests involving the
//! `oxide-httpmock` crate: it depends on `oxide` so cannot be used by the
//! tests directly in the `oxide` crate.
