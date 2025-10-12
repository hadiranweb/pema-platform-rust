extern "C" {
    // Host function for logging messages from the plugin
    pub fn log_message(message_ptr: *const u8, message_len: usize);

    // Host function for executing a read-only database query (restricted)
    // This is a placeholder and should be replaced with more specific, controlled APIs.
    pub fn db_execute_read_query(query_ptr: *const u8, query_len: usize) -> i32;

    // Host function for getting product count for a tenant (example of a controlled API)
    pub fn get_product_count_for_tenant(tenant_id_ptr: *const u8, tenant_id_len: usize) -> i32;
}

pub fn log(message: &str) {
    unsafe {
        log_message(message.as_ptr(), message.len());
    }
}

pub fn db_read(query: &str) -> i32 {
    unsafe {
        db_execute_read_query(query.as_ptr(), query.len())
    }
}

pub fn get_product_count(tenant_id: &str) -> i32 {
    unsafe {
        get_product_count_for_tenant(tenant_id.as_ptr(), tenant_id.len())
    }
}

