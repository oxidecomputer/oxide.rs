// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use crate::cli_builder::NewCli;

mod port_config;
mod port_status;

pub fn make_cli(cli: NewCli<'static>) -> NewCli<'static> {
    cli.add_custom::<port_status::CmdPortStatus>("net port status")
        .add_custom::<port_config::CmdPortConfig>("net port config")
}
