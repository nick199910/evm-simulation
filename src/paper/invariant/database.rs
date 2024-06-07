use std::str::FromStr;

use crate::core_module::utils::errors::DatabaseError;
use ethers::types::{Address, Bytes, U256};
use mysql::*;
use prelude::Queryable;
use revm_primitives::HashMap;
#[derive(Debug)]
pub struct Value_range {
    // 范围的最小值
    pub min: U256,
    // 范围的最大值
    pub max: U256,
}

pub struct registry {
    // 所有被保护的地址
    addresses: Vec<Address>,
    // 每个保护地址的slot
    slots: HashMap<Address, U256>,
    // 每个保护地址是slot的范围
    slot_ranges: HashMap<Address, HashMap<U256, Value_range>>,
    sql_connect: mysql::Conn,
}

impl registry {
    // 创建一个新的注册表
    pub fn new(mysql_url: &str) -> Self {
        Self {
            addresses: Vec::new(),
            slots: HashMap::new(),
            slot_ranges: HashMap::new(),
            sql_connect: mysql::Conn::new(mysql_url).expect("error url"),
        }
    }

    // 注册信息
    pub fn register(
        &mut self,
        _address: &str,
        _slot: U256,
        _range: Value_range,
    ) -> Result<(), DatabaseError> {
        let mut tx = self
            .sql_connect
            .start_transaction(TxOpts::default())
            .expect("get transaction error");

        let _ = tx.exec_drop(
            "insert into addresses(address) values(:address)",
            params! {
                "address"=>_address,
            },
        );

        let _ = tx.exec_drop(
            "insert into slots(address,slot) values(:address,:slot)",
            params! {
                "address"=>_address,
                "slot"=>_slot.to_string(),
            },
        );

        let _ =  tx.exec_drop("insert into slot_ranges(address,slot,slot_min,slot_max) values(:address,:slot,:slot_min,:slot_max)", params! {
            "address"=>_address,
            "slot"=>_slot.to_string(),
            "slot_min"=>_range.min.to_string(),
            "slot_max"=>_range.max.to_string()
        });

        Ok(tx.commit().unwrap())
    }

    // 更新范围
    pub fn updateRange(
        &mut self,
        _address: &str,
        _slot: U256,
        _range: Value_range,
    ) -> Result<(), DatabaseError> {
        if self.protect_exist(_address, _slot) {
            let _ =self.sql_connect.exec_drop(
                "UPDATE slot_ranges SET slot_min=:slot_min,slot_max=:slot_max where address=:address and slot=:slot",
                params! {
                    "slot_min"=>_range.min.to_string(),
                    "slot_max"=>_range.max.to_string(),
                    "address"=>_address,
                    "slot"=>_slot.to_string(),
                },
            );
            Ok(())
        } else {
            // error
            Err(DatabaseError::UpdateError)
        }
    }

    // 删除slot的范围，在slots和slot_ranges表中同时删除
    pub fn removeSlotRange(&mut self, _address: &str, _slot: U256) -> Result<bool, DatabaseError> {
        if self.protect_exist(_address, _slot) {
            let _ = self.sql_connect.exec_drop(
                "DELETE FROM slot_ranges WHERE address=:address and slot=:slot",
                params! {
                    "address" => _address,
                    "slot"=>_slot.to_string()
                },
            );
            let _ = self.sql_connect.exec_drop(
                "DELETE FROM slots WHERE address=:address and slot=:slot",
                params! {
                    "address"=>_address,
                    "slot"=>_slot.to_string(),
                },
            );
            Ok(true)
        } else {
            Err(DatabaseError::DeleteError)
        }
    }

    // 根据address和slot查询范围
    pub fn query_range(
        &mut self,
        _address: &str,
        _slot: U256,
    ) -> Result<Value_range, DatabaseError> {
        let result: Vec<(String, String, String, String)> = self
            .sql_connect
            .exec(
                "SELECT * FROM slot_ranges WHERE (address=:address AND slot=:slot)",
                params! {
                    "address" => _address,
                    "slot" => _slot.to_string()
                },
            )
            .unwrap();
        if result.len() > 0 {
            Ok(Value_range {
                max: U256::from_str(&result[0].3).unwrap(),
                min: U256::from_str(&result[0].2).unwrap(),
            })
        } else {
            Err(DatabaseError::QueryError)
        }
    }

    // 根据address查询所有检测的slot
    pub fn query_all_slot(&mut self, _address: &str) {}

    // 根据address查询所有检测的范围
    pub fn query_all_range(&mut self, _address: &str) {}

    // 根据范围找到所有符合的slot
    pub fn query_slot_with_range(&mut self, _address: &str, _range: Value_range) {}

    // 判断保护信息是否存在
    pub fn protect_exist(&mut self, _address: &str, _slot: U256) -> bool {
        // 首先判断是否存在
        let result: Vec<(String, String)> = self
            .sql_connect
            .exec(
                "SELECT * FROM slots WHERE (address=:address AND slot=:slot)",
                params! {
                    "address" => _address,
                    "slot" => _slot.to_string()
                },
            )
            .unwrap();
        return result.len() > 0;
    }
}
