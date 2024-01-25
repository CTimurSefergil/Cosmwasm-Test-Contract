use crate::msg::ValueResp;

pub mod query {
    use crate::msg::ValueResp;

    pub fn value() -> ValueResp {
        ValueResp { value: 0 }
    }

    pub fn incremented(val: u64) -> ValueResp {
        ValueResp { value: val + 1 }
    }
}
