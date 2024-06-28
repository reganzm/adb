//! 处理表格数据
use aparser::parse_value::Value;
use aparser::Column;
use std::collections::HashMap;

/// 定义新类型 表示数据行
type StoredRow = HashMap<String,String>;

/// 所有的列信息
type ColumnsInfo = Vec<Column>;