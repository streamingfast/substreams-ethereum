## Notes

Generated files in there are manually built because they require a manual transformation step.

### Generation

```
cargo run -p substreams-ethereum --example build && sed -i '' -e 's|substreams_ethereum::pb|crate::pb|g' core/src/abi/tests.rs
```
