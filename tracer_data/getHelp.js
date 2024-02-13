const fs = require('fs');

// 读取JSON文件
const readJsonFile = (filePath) => {
    try {
        const rawData = fs.readFileSync(filePath);
        const jsonData = JSON.parse(rawData);
        return jsonData.structLogs;
    } catch (error) {
        console.error('Error reading JSON file:', error.message);
        return null;
    }
};

// 获取数组中第n个数据
const getNthElement = (array, n) => {
    if (array && array.length > n) {
        return array[n];
    } else {
        console.error('Invalid index or array is empty.');
        return null;
    }
};

// 根据gas拿到index
const getElementIndex = (gas) => {
    const structLogs = readJsonFile(filePath);
    // console.log(structLogs[3].gas)
    for (let i = 0; i < structLogs.length; i++) {
        if (structLogs[i].gas === gas) {
            return i;
        }
    }
};



// 封装为一个函数
const processJsonFile = (filePath, index) => {
    if (!filePath || !index) {
        console.error('Usage: node readJsonFile.js <filePath> <index>');
        return;
    }

    const structLogs = readJsonFile(filePath);
    if (structLogs) {
        const nthElement = getNthElement(structLogs, parseInt(index, 10));
        // if (nthElement.op === "MSTORE") {
            console.log('Result:', nthElement);

        // }
    }
};

// 获取文件路径和索引
const filePath = "euler_attack_tx_op_logs.json";
const index = "8350";

// 调用封装的函数
// for (let i = parseInt(index); i >= parseInt(index) - 1000; i --) {
    processJsonFile(filePath, index);
// }

// console.log("index is: ", getElementIndex(nthElement.gas))

