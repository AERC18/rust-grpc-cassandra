use cdrs::query::*;
use cdrs::query_values;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use cdrs::query::QueryParamsBuilder;
use cdrs::consistency::Consistency;

use cdrs_helpers_derive::{TryFromRow};

use uuid::Uuid;

use crate::blocking_service::*;
use crate::cassandra_session::{create_session};

pub fn create_blocking_ks() {
    let session = create_session();
    //Create the blocked keyspace
    let create_ks: &'static str = "CREATE KEYSPACE IF NOT EXISTS blocking_service WITH REPLICATION ={ 'class' : 'SimpleStrategy', 'replication_factor' : 1};
";
    session.query(create_ks).expect("Keyspace create error");
}

pub fn create_blocking_table() {
    let session = create_session();
    let create_table_cql = "CREATE TABLE IF NOT EXISTS blocking_service.blocked_users(
	user_id UUID,
	blocked_id UUID,
	reason text,
	date timestamp,
	PRIMARY KEY (user_id, blocked_id)
	); 
	";
    session
    .query(create_table_cql)
    .expect("Table creation error");
}

pub fn block_user(block_user_info: &BlockUserInfo) {
    const INSERT_BLOCKED_USER: &'static str = "INSERT INTO blocking_service.blocked_users (user_id, blocked_id, reason, date) VALUES (?, ?, ?, toTimeStamp(now()));
";
    let user_id: Uuid = Uuid::parse_str(&block_user_info.user_id).unwrap();
    let blocked_id = Uuid::parse_str(&block_user_info.blocked_user_id).unwrap();
    let reason = String::from(&block_user_info.reason);

    let values = query_values!(user_id, blocked_id, reason);
    let query_params = QueryParamsBuilder::new()
        .consistency(Consistency::LocalQuorum)
        .values(values)
        .finalize();
    let session = create_session();
    session.query_with_params(INSERT_BLOCKED_USER, query_params).unwrap();
}

pub fn unblock_user(unblock_user_info: &UnblockUserInfo) {
    const DELETE_BLOCKED_USER: &'static str = "DELETE FROM blocking_service.blocked_users WHERE user_id = ? AND blocked_id = ?;";
    let user_id: Uuid = Uuid::parse_str(&unblock_user_info.user_id).unwrap();
    let blocked_id = Uuid::parse_str(&unblock_user_info.blocked_user_id).unwrap();
    let values = query_values!(user_id, blocked_id);
    let query_params = QueryParamsBuilder::new()
        .consistency(Consistency::LocalQuorum)
        .values(values)
        .finalize();
    let session = create_session();
    session.query_with_params(DELETE_BLOCKED_USER, query_params).unwrap();
}

pub fn are_this_users_blocked(this_users_are_blocked_request: &AreThisUsersBlockedRequest) -> AreThisUsersBlockedResponse {
    let user_id: Uuid = Uuid::parse_str(&this_users_are_blocked_request.user_id).unwrap();
    let mut possible_blocked_user_ids: Vec<Uuid> = parse_vec_str_to_uuid(&this_users_are_blocked_request.blocked_user_id);
    // Deduplicate the possible blocked users ids, it doesn't make sense to query duplicate 
    // ids because the result will be always the same
    possible_blocked_user_ids.sort();
    possible_blocked_user_ids.dedup();

    let session = create_session();
    
    let mut response_vec: Vec<BlockedUsersType> = Vec::new();
    for possible_block_user_id in possible_blocked_user_ids {
        let select_blocked_users_cql = "SELECT * FROM blocking_service.blocked_users WHERE user_id = ? AND blocked_id = ?;";
        let values = query_values!(user_id, possible_block_user_id);

        let rows = session
            .query_with_values(select_blocked_users_cql, values)
            .expect("Query")
            .get_body()
            .expect("GetBody")
            .into_rows()
            .expect("Into Rows");
        
        let response_blocked_users_type = BlockedUsersType {
            is_blocked: !rows.is_empty(),
            blocked_user_id: possible_block_user_id.to_hyphenated().to_string()
        };
        response_vec.push(response_blocked_users_type);
    };

    let response = AreThisUsersBlockedResponse {
        blocked_users: response_vec
    };
    response
}

#[derive(Clone, Debug, TryFromRow, PartialEq)]
struct BlockedUsersStruct {
    user_id: Uuid,
    blocked_id: Uuid,
    reason: String
}

impl BlockedUsersStruct {
    /*fn into_query_values(self) -> QueryValues {
        query_values!("user_id" => self.user_id, "blocked_id" => self.blocked_id, "reason" => self.reason)
    }*/
}

fn parse_vec_str_to_uuid(str_uuid_vec: &Vec<String>) -> Vec<Uuid> {
    let mut uuid_vec = Vec::new();
    for str_uuid in str_uuid_vec {
        uuid_vec.push(Uuid::parse_str(&str_uuid).unwrap());
    }
    uuid_vec
}