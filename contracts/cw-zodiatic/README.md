# Zodiatic CosmWasm Contract Example

This demo contract provides a 1:1 mapping to the Zodiatic module
bindings.

The contract messages only do some input validation and
directly call into their respective bindings outlined in the
"Messages" section below.

There are unit tests added to demonstrate how contract
developers might utilize `zodiatic-bindings-test` package
to import and use some test utilities.

## Messages

There are 3 messages:
- `ExecuteMsg::CreateLunar` maps to `ZodiaticMsg::CreateLunar`
- `ExecuteMsg::UpdateLunar` maps to `ZodiaticMsg::UpdateLunar`
- `ExecuteMsg::DeleteLunar` maps to `ZodiaticMsg::DeleteLunar`

## Query

1 query:
- `QueryMsg::GetLunar` maps to `ZodiaticQuery::Lunar`
