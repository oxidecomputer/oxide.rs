// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

// Copyright 2023 Oxide Computer Company

use assert_cmd::Command;
use httpmock::MockServer;
use oxide::types::{IdpMetadataSource, SamlIdentityProvider, SamlIdentityProviderCreate};
use oxide_httpmock::MockServerExt;
use rand::SeedableRng;
use test_common::JsonMock;

#[test]
fn test_silo_idp_saml_create() {
    let mut src = rand::rngs::SmallRng::seed_from_u64(42);
    let server = MockServer::start();

    let mock = server.saml_identity_provider_create(|when, then| {
        let body = SamlIdentityProviderCreate::builder()
            .name("samlrific")
            .description("for all the security I guess")
            .acs_url("http://nope.nope")
            .slo_url("http://nope.nope")
            .sp_client_id("17")
            .idp_entity_id("17")
            .idp_metadata_source(IdpMetadataSource::Url {
                url: "http://nope.nope".to_string(),
            })
            .technical_contact_email("anyone@but.adam")
            .try_into()
            .unwrap();
        when.body(&body);
        then.created(&SamlIdentityProvider {
            acs_url: body.acs_url.clone(),
            description: body.description.clone(),
            idp_entity_id: body.idp_entity_id.clone(),
            name: body.name.clone(),
            slo_url: body.slo_url.clone(),
            sp_client_id: body.sp_client_id.clone(),
            technical_contact_email: body.technical_contact_email,
            ..JsonMock::mock_value(&mut src).unwrap()
        });
    });

    Command::cargo_bin("oxide")
        .unwrap()
        .env("RUST_BACKTRACE", "1")
        .env("OXIDE_HOST", server.url(""))
        .env("OXIDE_TOKEN", "fake-token")
        .arg("silo")
        .arg("idp")
        .arg("saml")
        .arg("create")
        .arg("--name")
        .arg("samlrific")
        .arg("--description")
        .arg("for all the security I guess")
        .arg("--silo")
        .arg("mysilo")
        .arg("--acs-url")
        .arg("http://nope.nope")
        .arg("--slo-url")
        .arg("http://nope.nope")
        .arg("--sp-client-id")
        .arg("17")
        .arg("--idp-entity-id")
        .arg("17")
        .arg("--metadata-url")
        .arg("http://nope.nope")
        .arg("--technical-contact-email")
        .arg("anyone@but.adam")
        .assert()
        .success()
        .stdout(expectorate::eq_file_or_panic(
            "tests/data/test_silo_idp_saml_create.stdout",
        ));

    mock.assert();
}
