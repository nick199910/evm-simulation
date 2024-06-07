use crate::paper::strategy::simiarity::read_op_from_file;
use crate::paper::tx_origin_data::get_origin_oplist::get_opcode_list;
use std::fs::File;
use std::io::Write; // Import the Write trait
pub enum PathStrategy {
    FullPathMatch,
    ControlFlowMatch,
}

pub struct ControlFlowMatchOpcode {
    contain_opcode: Vec<&'static str>,
}

impl ControlFlowMatchOpcode {
    pub fn default() -> Self {
        Self {
            contain_opcode: vec![
                "CALL",
                "DELEGATECALL",
                "JUMP",
                "JUMPI",
                "RETURN",
                "CREATE",
                "CREATE2",
            ],
        }
    }
}

pub async fn get_path_strategy(
    _rpc: &str,
    _tx_hash: &str,
    _path_strategy: PathStrategy,
) -> Vec<String> {
    let opcode_list = get_opcode_list(_rpc, _tx_hash).await;

    let ret_path_strategy = match _path_strategy {
        PathStrategy::FullPathMatch => opcode_list,
        PathStrategy::ControlFlowMatch => {
            let control_flow_opcode = ControlFlowMatchOpcode::default().contain_opcode;
            let ret = opcode_list
                .into_iter()
                .filter(|opcode| control_flow_opcode.contains(&opcode.as_str()) == true)
                .collect();
            // ret
            ret
        }
    };
    ret_path_strategy
}

// #[tokio::test]
pub async fn test_get_path_strategy() {
    // let rpc = "https://lb.nodies.app/v1/181a5ebf4c954f8496ae7cbc1ac8d03b";
    let rpc = "https://blue-quiet-surf.quiknode.pro/ac44441c600862066c752e9d83b7aefb0532f03b/";
    let attackhash_1 = "0xe28ca1f43036f4768776805fb50906f8172f75eba3bf1d9866bcd64361fda834";
    let attackhash_2 = "0x8e1b0ab098c4cc5f632e00b0842b5f825bbd15ded796d4a59880bb724f6c5372";

    let mut path_strategy1 = PathStrategy::FullPathMatch;
    let mut path_strategy2 = PathStrategy::ControlFlowMatch;

    let mut opcode_list1: Vec<String> = Vec::new();
    let mut opcode_list2: Vec<String> = Vec::new();

    let mut controlFlow_opcode1: Vec<String> = Vec::new();
    let mut controlFlow_opcode2: Vec<String> = Vec::new();

    opcode_list2 = get_path_strategy(&rpc, &attackhash_2, path_strategy1).await;

    // 创建并打开一个文件，用于写入 opcode_list1 和 opcode_list2
    let mut FullPathMatch_file_path = "src/paper/tx_oplist/HeavenGate/attack2_full.txt";
    let mut file = match File::create(FullPathMatch_file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            return;
        }
    };

    // 将 opcode_list1 的内容写入文件
    for opcode in &opcode_list2 {
        if let Err(e) = writeln!(file, "{}", opcode) {
            eprintln!("Error writing to file: {}", e);
            return;
        }
    }

    // 写入完成后关闭文件
    if let Err(e) = file.sync_all() {
        eprintln!("Error syncing file: {}", e);
        return;
    }
}
// #[tokio::test]
pub async fn test_get_path_strategy1() {
    // let rpc = "https://lb.nodies.app/v1/181a5ebf4c954f8496ae7cbc1ac8d03b";
    let rpc = "https://blue-quiet-surf.quiknode.pro/ac44441c600862066c752e9d83b7aefb0532f03b/";
    let attackhash_1 = "0xe28ca1f43036f4768776805fb50906f8172f75eba3bf1d9866bcd64361fda834";
    let attackhash_2 = "0x8e1b0ab098c4cc5f632e00b0842b5f825bbd15ded796d4a59880bb724f6c5372";

    let mut path_strategy1 = PathStrategy::FullPathMatch;
    let mut path_strategy2 = PathStrategy::ControlFlowMatch;

    let mut opcode_list1: Vec<String> = Vec::new();
    let mut opcode_list2: Vec<String> = Vec::new();

    let mut controlFlow_opcode1: Vec<String> = Vec::new();
    let mut controlFlow_opcode2: Vec<String> = Vec::new();

    opcode_list2 = get_path_strategy(&rpc, &attackhash_2, path_strategy1).await;

    // 创建并打开一个文件，用于写入 opcode_list1 和 opcode_list2
    let mut FullPathMatch_file_path = "src/paper/tx_oplist/HeavenGate/attack2_full.txt";
    let mut file = match File::create(FullPathMatch_file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error creating file: {}", e);
            return;
        }
    };

    // 将 opcode_list1 的内容写入文件
    for opcode in &opcode_list2 {
        if let Err(e) = writeln!(file, "{}", opcode) {
            eprintln!("Error writing to file: {}", e);
            return;
        }
    }

    // 写入完成后关闭文件
    if let Err(e) = file.sync_all() {
        eprintln!("Error syncing file: {}", e);
        return;
    }
}
