# Protobuf
Language agnostic serialization format. Uses an Interface Definition Language (IDL) to describe services and messages.
A compilation step builds the neccessary boilerplate code for encoding/decoding data to/from bytes for a bunch of languages.
Is designed to allow "schema evolution". This means changing message schemas and service capabilities. Allows changes to be backwards compatible.

# gRPC
Opensource framework for Remote Procedure Calls (RPC). Uses protobuf for message passing.
Due to `reflection` a service can be self-describing.

# Tooling
## `grpcurl` 
CLI tool for inspecting a service and sending messages to it and receiving results

```bash
grpcurl -plaintext [::1]:50051 list
grpcurl -plaintext -proto ./proto/calculator.proto \
         -d '{"a": 2, "b": 3}' \
         '[::1]:50051' calculator.Calculator.Add
```

## `grpcui`
Opens a WebGUI to inspect the properties of a service and its corresponding messages
```bash
grpcui -plaintext -proto ./proto/calculator.proto '[::1]:50051'
```