# Example of Microservice Rust, gRPC and Apache Cassandra

This project contains an example of a microservice using Rust, gRPC and Apache Cassandra.
The main libraries used in the project are:
- [tonic](https://github.com/hyperium/tonic)
- [cdrs](https://github.com/AlexPikalov/cdrs)

## Getting Started

### Prerequisites
1. The stable installation of Rust:

```
rustup 1.21.1 (7832b2ebe 2019-12-20) or greater
cargo 1.40.0 (bc8e4c8be 2019-11-22) or greater 
rustc 1.40.0 (73528e339 2019-12-16) or greater
```
2. Amazon Managed Apache Cassandra Service deployment
[Getting Started] (https://docs.aws.amazon.com/mcs/latest/devguide/getting-started.html) - Getting Started

3. Set this environment variables: 

```bash
export CASSANDRA_URI="cassandra.us-east-2.amazonaws.com:9142"
export CASSANDRA_SSL_CERT_PATH="certs/AmazonRootCA1.pem"
export CASSANDRA_USER="YOUR CASSANDRA USER"
export CASSANDRA_PASSWORD="YOUR CASSANDRA PASSWORD"
```

The CASSANDRA_URI env var depends on the region where your Cassandra is deployed, please look at the AWS documentation: [AWS MCS] (https://docs.aws.amazon.com/mcs/latest/devguide/cqlsh.html) - Using cqlsh and Cassandra Drivers

### Running

```bash
cargo run
```

### Testing

`blocking_service` request examples using [grpcurl](https://github.com/fullstorydev/grpcurl)
```bash
# BlockUser request example:

grpcurl -import-path proto/ -proto block_service.proto -plaintext -d '{"user_id": "e411156b-9256-4bb2-a356-e4b9cec1b0c4", "blocked_user_id": "e411156b-9256-4bb2-a356-e4b9cec1b0c4", "reason": "Some reason."}' localhost:50051 blocking_service.BlockingService.BlockUser

# UnblockUser request example:

grpcurl -import-path proto/ -proto block_service.proto -plaintext -d '{"user_id": "e411156b-9256-4bb2-a356-e4b9cec1b0c4", "blocked_user_id": "e411156b-9256-4bb2-a356-e4b9cec1b0c4"}' localhost:50051 blocking_service.BlockingService.UnblockUser

# AreThisUsersBlocked service request example:

grpcurl -import-path proto/ -proto block_service.proto -plaintext -d '{"user_id": "e411156b-9256-4bb2-a356-e4b9cec1b0c4", "blocked_user_id": ["e411156b-9256-4bb2-a356-e4b9cec1b0c4"]}' localhost:50051 blocking_service.BlockingService.AreThisUsersBlocked
```

`reporting_service` request examples using [grpcurl](https://github.com/fullstorydev/grpcurl)
```bash
# ReportUser service request example:

grpcurl -import-path proto/ -proto reporting_service.proto -plaintext -d '{"user_id": "e411156b-9256-4bb2-a356-e4b9cec1b0c4", "reported_user_id": "e411156b-9256-4bb2-a356-e4b9cec1b0c4", "reported_promo_id": "e411156b-9256-4bb2-a356-e4b9cec1b0c2", "reason": "Some reason."}' localhost:50051 reporting_service.ReportingService.ReportUser
```