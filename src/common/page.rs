use serde::{Deserialize, Serialize};


const DEFAULT_PAGESIZE: u64 = 15;
#[derive(Debug,Clone, Serialize,Deserialize)]
pub struct PageQuery {
     page_no:  Option<u64>,
     page_size: Option<u64>
}

impl PageQuery {
    pub fn page_size(&self) -> u64 {
        let ps = self.page_size.unwrap_or(0);
        if ps <= 0 {
            return DEFAULT_PAGESIZE;
        }
        ps
    }
    pub fn page_no(&self) -> u64 {
        self.page_no.unwrap_or(0)
    }
}


#[derive(Debug,Clone, Serialize,Deserialize)]
pub struct PageResult<T> {
    pub params: PageQuery,
    pub total: u64,
    pub data: Vec<T>
}