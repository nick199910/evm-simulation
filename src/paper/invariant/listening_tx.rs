// 持续监听区块链状态
// 用ws监听特定地址发生的交易
// todo! 实验时无法监听到特定地址的不变量
use crate::errors::DatabaseError;
use crate::paper::invariant::database::Value_range;
use ethers::types::U256;
use ethers::types::{Address, BlockId};
use ethers_providers::{Middleware, Ws};
use std::collections::HashMap;
use z3::*;
// 不变量结构
pub struct Invariant {
    // 不变量数组
    // 对于不变量，其本质是一个表达式，合约执行过程中必须满足该表达式，表达式由各状态变量组成
    // 满足的不变量式子
    expression: Vec<String>,
    // 所有slot
    slots: Vec<U256>,
    // 不变量描述
    description: String,
}

pub struct RangeExpression {
    // 对于一个表达式，其本质是一个表达式组，表达式组由多个表达式组成
    // 表达式组
    expressions: Vec<String>,
    // 所有
    // 描述
    description: String,
}
// 维护不变量结构，用户的不变量保存在此结构中
// 也需要用到数据库
pub struct ProjectInfo {
    // 不变量
    pub invariants: HashMap<Address, Invariant>,
    // 表达式
    pub expressions: HashMap<Address, HashMap<[u8; 4], HashMap<usize, RangeExpression>>>,
    // 需要连接数据库
    pub sql_connect: mysql::Conn,
    // 需要连接rpc，使用ws速度更快
    pub ethers_provicer: ethers::providers::Provider<Ws>,
}

impl ProjectInfo {
    // 初始化函数
    async fn init(sql_url: &str, rpc_url: &str) -> Self {
        Self {
            invariants: HashMap::new(),
            expressions: HashMap::new(),
            sql_connect: mysql::Conn::new(sql_url).expect("error url"),
            ethers_provicer: ethers::providers::Provider::connect(rpc_url)
                .await
                .expect("error url"),
        }
    }

    // 注册不变量
    // 用户进行注册，那么需要执行以下步骤
    // 1. 不变量插入数据库
    // 2. 计算slot范围
    // 3. 将slot范围插入数据库
    pub async fn register(
        &mut self,
        _address: &str,
        _invariant: Invariant,
        _expression: RangeExpression,
    ) -> Result<(), DatabaseError> {
        // 最新的块号
        let previous_block_number: BlockId = (self
            .ethers_provicer
            .get_block_number()
            .await
            .expect("error provider")
            - 1)
        .into();
        // 项目地址
        let address: Address = _address.parse().expect("error address");
        // 确定地址为合约地址
        let code = self
            .ethers_provicer
            .get_code(address, Some(previous_block_number))
            .await
            .expect("error provider");
        if code.len() == 0 {
            // 非合约地址报错
            Err(DatabaseError::ErrorSlot("not contract address"))
        } else {
            // 保存不变量
            //todo!插入数据库
            self.invariants.insert(address, _invariant);
            Ok(())
        }

        // 计算表达式
    }
}

impl RangeExpression {
    pub fn new(expressions: Vec<String>, description: String) -> Self {
        Self {
            expressions,
            description,
        }
    }

    pub fn getRange(&self) -> Value_range {
        let config = z3::Config::new();
        let ctx = z3::Context::new(&config);
        let solver = Solver::new(&ctx);

        for _exp in &self.expressions {
            // 这里默认表达式都是smtlib2类型，将这些内容都加到solver中
            solver.from_string(_exp.as_bytes());
        }
        // 有解
        match solver.check() {
            // 有解
            SatResult::Sat => {
                println!("{:?}", solver.get_model());
            }
            SatResult::Unknown => {
                println!("unknown");
            }
            SatResult::Unsat => {
                println!("unsat");
            }
            _ => {
                println!("error");
            }
        }
        Value_range {
            min: U256::from(0),
            max: U256::from(0),
        }
    }
}

#[test]
fn test1() {
    let expressions = vec![
        "(declare-const x Int) (assert (> x 0))".to_string(),
        "(declare-const x Int) (assert (> x 100))".to_string(),
    ];
    let mut exp = RangeExpression::new(expressions, "is alright".to_string());
    exp.getRange();
}
