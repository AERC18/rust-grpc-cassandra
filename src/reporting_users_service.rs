use cdrs::query::*;
use cdrs::query_values;
use cdrs::query::QueryParamsBuilder;
use cdrs::consistency::Consistency;

use uuid::Uuid;

use crate::reporting_service::*;
use crate::cassandra_session::{create_session};
use crate::blocking_users_service::{block_user};
use crate::blocking_service::{BlockUserInfo};

pub fn create_reporting_ks() {
    let session = create_session();
    //Create the blocked keyspace
    let create_ks: &'static str = "CREATE KEYSPACE IF NOT EXISTS blocking_service WITH REPLICATION ={ 'class' : 'SimpleStrategy', 'replication_factor' : 1};
";
    session.query(create_ks).expect("Keyspace create error");
}

pub fn create_reporting_table() {
    let session = create_session();
    let create_table_cql = "CREATE TABLE IF NOT EXISTS blocking_service.reported_users(
	user_id UUID,
	reported_id UUID,
    reported_promo_id UUID,
	reason text,
	date timestamp,
	PRIMARY KEY (user_id, reported_id)
	);
	";
    session
        .query(create_table_cql)
        .expect("Table creation error");
}

pub fn report_user(report_user_request: &ReportUserRequest) {
    // Insert the reported user into the DB 
    const INSERT_BLOCKED_USER: &'static str = "INSERT INTO blocking_service.reported_users (user_id, reported_id, reported_promo_id, reason, date) VALUES (?, ?, ?, ?, toTimeStamp(now()));
";
    let user_id: Uuid = Uuid::parse_str(&report_user_request.user_id).unwrap();
    let reported_id = Uuid::parse_str(&report_user_request.reported_user_id).unwrap();
    let reported_promo_id = Uuid::parse_str(&report_user_request.reported_promo_id).unwrap();
    let reason = String::from(&report_user_request.reason);

    let values = query_values!(user_id, reported_id, reported_promo_id, reason);
    let query_params = QueryParamsBuilder::new()
        .consistency(Consistency::LocalQuorum)
        .values(values)
        .finalize();   let session = create_session();
    // session.query_with_values(INSERT_BLOCKED_USER, values).unwrap();
    session.query_with_params(INSERT_BLOCKED_USER, query_params).unwrap();
    
    // Block the reported user
    let block_user_info = BlockUserInfo {
         user_id:  String::from(&report_user_request.user_id),
         blocked_user_id:  String::from(&report_user_request.reported_user_id),
         reason: String::from(&report_user_request.reason)
    };

    block_user(&block_user_info);
}
