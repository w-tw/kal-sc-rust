#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "native_functions")]
extern "C" {
    pub fn submit_txn(
        payload_pointer: *const u8,
        payload_length: i32,
        target_pointer: *mut u8,
    ) -> i32;

    // state
    pub fn store_state(
        key_pointer: *const u8,
        key_length: i32,
        value_pointer: *const u8,
        value_length: i32,
    ) -> i32;
    pub fn retrieve_state(
        key_pointer: *const u8,
        key_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn clear_state(key_pointer: *const u8, key_length: i32) -> i32;
    pub fn delegate_store_state(
        key_pointer: *const u8,
        key_length: i32,
        value_pointer: *const u8,
        value_length: i32,
        contract_pointer: *const u8,
        contract_length: i32,
    ) -> i32;
    pub fn delegate_retrieve_state(
        key_pointer: *const u8,
        key_length: i32,
        contract_pointer: *const u8,
        contract_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn delegate_clear_state(
        key_pointer: *const u8,
        key_length: i32,
        contract_pointer: *const u8,
        contract_length: i32,
    ) -> i32;
    pub fn get_all_states(
        contract_key_pointer: *const u8,
        contract_key_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn get_all_states_by_prefix(
        contract_key_pointer: *const u8,
        contract_key_length: i32,
        prefix_pointer: *const u8,
        prefix_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn current_get_all_states(target_pointer: *const u8) -> i32;
    pub fn current_get_all_states_by_prefix(
        prefix_pointer: *const u8,
        prefix_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn state_exists(key_pointer: *const u8, key_length: i32) -> i32;
    pub fn delegate_state_exists(
        key_pointer: *const u8,
        key_length: i32,
        contract_pointer: *const u8,
        contract_length: i32,
    ) -> i32;

    //call
    pub fn call(
        contract_name_pointer: *const u8,
        contract_name_length: i32,
        instance_pointer: *const u8,
        instance_length: i32,
        function_name_pointer: *const u8,
        function_name_length: i32,
        parameters_pointer: *const u8,
        parameters_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn delegatecall(
        contract_name_pointer: *const u8,
        contract_name_length: i32,
        instance_pointer: *const u8,
        instance_length: i32,
        function_name_pointer: *const u8,
        function_name_length: i32,
        parameters_pointer: *const u8,
        parameters_length: i32,
        target_pointer: *const u8,
    ) -> i32;

    // transfer: send
    pub fn transfer(
        contract_id: *const u8,
        contract_id_length: i32,
        amount: *const u8,
        amount_length: i32,
        address: *const u8,
        address_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn send(
        contract_id: *const u8,
        contract_id_length: i32,
        amount: *const u8,
        amount_length: i32,
        address: *const u8,
        address_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn current_send(
        contract_id: *const u8,
        contract_id_length: i32,
        amount: *const u8,
        amount_length: i32,
        address: *const u8,
        address_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn delegate_send(
        contract_id: *const u8,
        contract_id_length: i32,
        amount: *const u8,
        amount_length: i32,
        address: *const u8,
        address_length: i32,
        sc_wallet: *const u8,
        sc_wallet_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn derived_send(
        contract_id: *const u8,
        contract_id_length: i32,
        amount: *const u8,
        amount_length: i32,
        address: *const u8,
        address_length: i32,
        derived_wallet: *const u8,
        derived_wallet_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn derived_current_send(
        contract_id: *const u8,
        contract_id_length: i32,
        amount: *const u8,
        amount_length: i32,
        address: *const u8,
        address_length: i32,
        derived_wallet: *const u8,
        derived_wallet_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn derived_delegate_send(
        contract_id: *const u8,
        contract_id_length: i32,
        amount: *const u8,
        amount_length: i32,
        address: *const u8,
        address_length: i32,
        derived_wallet: *const u8,
        derived_wallet_length: i32,
        sc_contract: *const u8,
        sc_contract_length: i32,
        sc_instance: *const u8,
        sc_instance_length: i32,
        target_pointer: *const u8,
    ) -> i32;

    // transfer: send_multi
    pub fn transfer_multi(
        contract_id: *const u8,
        contract_id_length: i32,
        input_amounts: *const u8,
        input_amounts_length: i32,
        amounts: *const u8,
        amounts_length: i32,
        addresses: *const u8,
        addresses_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn send_multi(
        contract_id: *const u8,
        contract_id_length: i32,
        input_amounts: *const u8,
        input_amounts_length: i32,
        amounts: *const u8,
        amounts_length: i32,
        addresses: *const u8,
        addresses_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn current_send_multi(
        contract_id: *const u8,
        contract_id_length: i32,
        input_amounts: *const u8,
        input_amounts_length: i32,
        amounts: *const u8,
        amounts_length: i32,
        addresses: *const u8,
        addresses_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn delegate_send_multi(
        contract_id: *const u8,
        contract_id_length: i32,
        input_amounts: *const u8,
        input_amounts_length: i32,
        amounts: *const u8,
        amounts_length: i32,
        addresses: *const u8,
        addresses_length: i32,
        sc_wallet: *const u8,
        sc_wallet_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn derived_send_multi(
        contract_id: *const u8,
        contract_id_length: i32,
        input_amounts: *const u8,
        input_amounts_length: i32,
        amounts: *const u8,
        amounts_length: i32,
        addresses: *const u8,
        addresses_length: i32,
        derived_wallet: *const u8,
        derived_wallet_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn derived_current_send_multi(
        contract_id: *const u8,
        contract_id_length: i32,
        input_amounts: *const u8,
        input_amounts_length: i32,
        amounts: *const u8,
        amounts_length: i32,
        addresses: *const u8,
        addresses_length: i32,
        derived_wallet: *const u8,
        derived_wallet_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn derived_delegate_send_multi(
        contract_id: *const u8,
        contract_id_length: i32,
        input_amounts: *const u8,
        input_amounts_length: i32,
        amounts: *const u8,
        amounts_length: i32,
        addresses: *const u8,
        addresses_length: i32,
        derived_wallet: *const u8,
        derived_wallet_length: i32,
        sc_contract: *const u8,
        sc_contract_length: i32,
        sc_instance: *const u8,
        sc_instance_length: i32,
        target_pointer: *const u8,
    ) -> i32;

    // transfer: send_all
    pub fn transfer_all(address: *const u8, address_length: i32, target_pointer: *const u8) -> i32;
    pub fn send_all(address: *const u8, address_length: i32, target_pointer: *const u8) -> i32;
    pub fn current_send_all(
        address: *const u8,
        address_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn delegate_send_all(
        address: *const u8,
        address_length: i32,
        sc_wallet: *const u8,
        sc_wallet_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn derived_send_all(
        address: *const u8,
        address_length: i32,
        derived_wallet: *const u8,
        derived_wallet_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn derived_current_send_all(
        address: *const u8,
        address_length: i32,
        derived_wallet: *const u8,
        derived_wallet_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn derived_delegate_send_all(
        address: *const u8,
        address_length: i32,
        derived_wallet: *const u8,
        derived_wallet_length: i32,
        sc_contract: *const u8,
        sc_contract_length: i32,
        sc_instance: *const u8,
        sc_instance_length: i32,
        target_pointer: *const u8,
    ) -> i32;

    // transfer: deposit (hold)
    pub fn hold(
        contract_id: *const u8,
        contract_id_length: i32,
        amount: *const u8,
        amount_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn current_hold(
        contract_id: *const u8,
        contract_id_length: i32,
        amount: *const u8,
        amount_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn delegate_hold(
        contract_id: *const u8,
        contract_id_length: i32,
        amount: *const u8,
        amount_length: i32,
        sc_wallet: *const u8,
        sc_wallet_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn derived_hold(
        contract_id: *const u8,
        contract_id_length: i32,
        amount: *const u8,
        amount_length: i32,
        derived_wallet: *const u8,
        derived_wallet_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn derived_current_hold(
        contract_id: *const u8,
        contract_id_length: i32,
        amount: *const u8,
        amount_length: i32,
        derived_wallet: *const u8,
        derived_wallet_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn derived_delegate_hold(
        contract_id: *const u8,
        contract_id_length: i32,
        amount: *const u8,
        amount_length: i32,
        derived_wallet: *const u8,
        derived_wallet_length: i32,
        sc_contract: *const u8,
        sc_contract_length: i32,
        sc_instance: *const u8,
        sc_instance_length: i32,
        target_pointer: *const u8,
    ) -> i32;

    // transfer: mint
    pub fn mint(
        contract_id: *const u8,
        contract_id_length: i32,
        amount: *const u8,
        amount_length: i32,
        address: *const u8,
        address_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn current_mint(
        contract_id: *const u8,
        contract_id_length: i32,
        amount: *const u8,
        amount_length: i32,
        address: *const u8,
        address_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn delegate_mint(
        contract_id: *const u8,
        contract_id_length: i32,
        amount: *const u8,
        amount_length: i32,
        address: *const u8,
        address_length: i32,
        sc_wallet: *const u8,
        sc_wallet_length: i32,
        target_pointer: *const u8,
    ) -> i32;

    //context
    pub fn public_key(target_pointer: *const u8) -> i32;
    pub fn txn_hash(target_pointer: *const u8) -> i32;
    pub fn wallet_address(target_pointer: *const u8) -> i32;
    pub fn last_block_time(target_pointer: *const u8) -> i32;
    pub fn smart_contract_wallet(target_pointer: *const u8) -> i32;
    pub fn current_smart_contract_wallet(target_pointer: *const u8) -> i32;
    pub fn called_smart_contract_wallet(target_pointer: *const u8) -> i32;
    pub fn current_smart_contract(target_pointer: *const u8) -> i32;

    //crypto
    pub fn hash(
        data_pointer: *const u8,
        data_length: i32,
        hash_type_pointer: *const u8,
        hash_type_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn verify_signature(
        message_pointer: *const u8,
        message_length: i32,
        signatures_pointer: *const u8,
        signatures_length: i32,
        public_key_pointer: *const u8,
        public_key_length: i32,
        target_pointer: *const u8,
    ) -> i32;

    // query
    pub fn wallet_balance(
        contract_name_pointer: *const u8,
        contract_name_length: i32,
        wallet_pointer: *const u8,
        wallet_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn wallet_tokens(
        address_pointer: *const u8,
        address_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn contract_exists(
        contract_name_pointer: *const u8,
        contract_name_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn smart_contract_exists(
        contract_name_pointer: *const u8,
        contract_name_length: i32,
        instance_pointer: *const u8,
        instance_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn wallet_exists(
        wallet_pointer: *const u8,
        wallet_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn contract_denomination(
        contract_name_pointer: *const u8,
        contract_name_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn contract_name(
        contract_id_pointer: *const u8,
        contract_id_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn contract_symbol(
        contract_id_pointer: *const u8,
        contract_id_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn validator_info(
        wallet_pointer: *const u8,
        wallet_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn contract_wallet(
        sc_key_pointer: *const u8,
        sc_key_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn circulating_supply(
        contract_name_pointer: *const u8,
        contract_name_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn smart_contract_balance(
        contract_name_pointer: *const u8,
        contract_name_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn current_smart_contract_balance(
        contract_name_pointer: *const u8,
        contract_name_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn compliance(
        contract_name_pointer: *const u8,
        contract_name_length: i32,
        wallet_pointer: *const u8,
        wallet_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn compliance_levels(
        contract_name_pointer: *const u8,
        contract_name_length: i32,
        wallet_pointer: *const u8,
        wallet_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn get_ace_data(
        contract_pointer: *const u8,
        contract_length: i32,
        target_pointer: *const u8,
    ) -> i32;

    // wallet
    pub fn derive_wallet(
        seed_pointer: *const u8,
        seed_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn derive_wallet_current(
        seed_pointer: *const u8,
        seed_length: i32,
        target_pointer: *const u8,
    ) -> i32;
    pub fn derive_wallet_delegate(
        seed_pointer: *const u8,
        seed_length: i32,
        contract_pointer: *const u8,
        contract_length: i32,
        instance_pointer: *const u8,
        instance_length: i32,
        target_pointer: *const u8,
    ) -> i32;

    // emit
    pub fn emit(value_pointer: *const u8, value_length: i32) -> i32;
    pub fn emit_block(value_pointer: *const u8, value_length: i32) -> i32;
}

#[cfg(not(target_arch = "wasm32"))]
mod stubs {
    const MSG: &str = "kal-sc: FFI functions are only available when compiled to wasm32";

    pub unsafe extern "C" fn submit_txn(_: *const u8, _: i32, _: *mut u8) -> i32 {
        panic!("{}", MSG)
    }

    pub unsafe extern "C" fn store_state(_: *const u8, _: i32, _: *const u8, _: i32) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn retrieve_state(_: *const u8, _: i32, _: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn clear_state(_: *const u8, _: i32) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn delegate_store_state(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn delegate_retrieve_state(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn delegate_clear_state(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn get_all_states(_: *const u8, _: i32, _: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn get_all_states_by_prefix(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn current_get_all_states(_: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn current_get_all_states_by_prefix(
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn state_exists(_: *const u8, _: i32) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn delegate_state_exists(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
    ) -> i32 {
        panic!("{}", MSG)
    }

    pub unsafe extern "C" fn call(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn delegatecall(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }

    pub unsafe extern "C" fn transfer(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn send(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn current_send(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn delegate_send(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn derived_send(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn derived_current_send(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn derived_delegate_send(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }

    pub unsafe extern "C" fn transfer_multi(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn send_multi(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn current_send_multi(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn delegate_send_multi(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn derived_send_multi(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn derived_current_send_multi(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn derived_delegate_send_multi(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }

    pub unsafe extern "C" fn transfer_all(_: *const u8, _: i32, _: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn send_all(_: *const u8, _: i32, _: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn current_send_all(_: *const u8, _: i32, _: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn delegate_send_all(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn derived_send_all(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn derived_current_send_all(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn derived_delegate_send_all(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }

    pub unsafe extern "C" fn hold(_: *const u8, _: i32, _: *const u8, _: i32, _: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn current_hold(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn delegate_hold(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn derived_hold(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn derived_current_hold(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn derived_delegate_hold(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }

    pub unsafe extern "C" fn mint(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn current_mint(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn delegate_mint(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }

    pub unsafe extern "C" fn public_key(_: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn txn_hash(_: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn wallet_address(_: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn last_block_time(_: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn smart_contract_wallet(_: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn current_smart_contract_wallet(_: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn called_smart_contract_wallet(_: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn current_smart_contract(_: *const u8) -> i32 {
        panic!("{}", MSG)
    }

    pub unsafe extern "C" fn hash(_: *const u8, _: i32, _: *const u8, _: i32, _: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn verify_signature(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }

    pub unsafe extern "C" fn wallet_balance(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn wallet_tokens(_: *const u8, _: i32, _: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn contract_exists(_: *const u8, _: i32, _: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn smart_contract_exists(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn wallet_exists(_: *const u8, _: i32, _: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn contract_denomination(_: *const u8, _: i32, _: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn contract_name(_: *const u8, _: i32, _: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn contract_symbol(_: *const u8, _: i32, _: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn validator_info(_: *const u8, _: i32, _: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn contract_wallet(_: *const u8, _: i32, _: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn circulating_supply(_: *const u8, _: i32, _: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn smart_contract_balance(_: *const u8, _: i32, _: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn current_smart_contract_balance(
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn compliance(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn compliance_levels(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn get_ace_data(_: *const u8, _: i32, _: *const u8) -> i32 {
        panic!("{}", MSG)
    }

    pub unsafe extern "C" fn derive_wallet(_: *const u8, _: i32, _: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn derive_wallet_current(_: *const u8, _: i32, _: *const u8) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn derive_wallet_delegate(
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
        _: i32,
        _: *const u8,
    ) -> i32 {
        panic!("{}", MSG)
    }

    pub unsafe extern "C" fn emit(_: *const u8, _: i32) -> i32 {
        panic!("{}", MSG)
    }
    pub unsafe extern "C" fn emit_block(_: *const u8, _: i32) -> i32 {
        panic!("{}", MSG)
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub use stubs::*;
