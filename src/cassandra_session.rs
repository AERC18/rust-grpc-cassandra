use cdrs::authenticators::{StaticPasswordAuthenticator};
use cdrs::cluster::session::{new_ssl, Session};
use cdrs::cluster::{ClusterSslConfig, NodeSslConfigBuilder, SslConnectionPool};
use cdrs::load_balancing::RoundRobin;

use std::env;
use std::time::Duration;


use openssl::ssl::{SslConnector, SslMethod};

pub type CurrentSession = Session<RoundRobin<SslConnectionPool<StaticPasswordAuthenticator>>>;


 pub fn create_session() -> CurrentSession {
    // ------------- Get the cassandra URI from CASSANDRA_URI env var ---------------------
    let cassandra_uri: String = env::var("CASSANDRA_URI").ok().unwrap();
    let cassandra_ssl_cert_path: String = env::var("CASSANDRA_SSL_CERT_PATH").ok().unwrap();
    // ------------- Authenticator -----------------------
    let user: String = env::var("CASSANDRA_USER").ok().unwrap();
    let password: String = env::var("CASSANDRA_PASSWORD").ok().unwrap();
    let auth = StaticPasswordAuthenticator::new(&user, &password);
    // ------------- SSL connector -----------------------
    let mut ssl_connector_builder = SslConnector::builder(SslMethod::tls()).unwrap();
    ssl_connector_builder.set_ca_file(cassandra_ssl_cert_path).unwrap();

    let ssl_connector = ssl_connector_builder.build();
    let node = NodeSslConfigBuilder::new(&cassandra_uri, auth, ssl_connector)
        .connection_timeout(Duration::from_secs(15))
        .max_size(1)
        .min_idle(Some(1))
        .build();

    let cluster_ssl_config = ClusterSslConfig(vec![node]);
    let no_compression = new_ssl(&cluster_ssl_config, RoundRobin::new()).expect("session should be created");
    println!("Connected to cassandra: {}", cassandra_uri);
    no_compression
}
