# test-common

This is intended to be a stand-alone, published crate.. probably in it's own
repo. In particular it is for generating mock data for types that are
`serde::JsonSchema + serde::Deserialize`. It could be used for both test data
and for fuzzing.