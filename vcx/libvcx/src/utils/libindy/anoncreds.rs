extern crate libc;
use self::libc::c_char;
use settings;
use std::ffi::CString;
use std::ptr::null;
use utils::constants::LIBINDY_CRED_OFFER;
use utils::libindy::{indy_function_eval, check_str, mock_libindy_rc};
use utils::libindy::return_types::{ Return_I32_STR, Return_I32_BOOL, Return_I32_STR_STR, Return_I32 };
use utils::libindy::SigTypes;
use utils::libindy::error_codes::{map_indy_error_code, map_string_error};
use utils::timeout::TimeoutUtils;
use utils::libindy::option_cstring_as_ptn;

extern {
    fn indy_issuer_create_and_store_credential_def(command_handle: i32,
                                                   wallet_handle: i32,
                                                   issuer_did: *const c_char,
                                                   schema_json: *const c_char,
                                                   tag: *const c_char,
                                                   type_: *const c_char,
                                                   config_json: *const c_char,
                                                   cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                                        cred_def_id: *const c_char,
                                                                        cred_def_json: *const c_char)>) -> i32;
//    fn indy_issuer_create_and_store_credential_def(command_handle: i32,
//                                              wallet_handle: i32,
//                                              issuer_did: *const c_char,
//                                              schema_json: *const c_char,
//                                              signature_type: *const c_char,
//                                              create_non_revoc: bool,
//                                              cb: Option<extern fn(xcommand_handle: i32,
//                                                                   err: i32,
//                                                                   credential_def_json: *const c_char)>) -> i32;

    fn indy_issuer_create_credential_offer(command_handle: i32,
                                           wallet_handle: i32,
                                           cred_def_id: *const c_char,
                                           cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                                cred_offer_json: *const c_char
                                           )>) -> i32;

//    fn indy_issuer_create_credential_offer(command_handle: i32,
//                                           wallet_handle: i32,
//                                           schema_json: *const c_char,
//                                           issuer_did: *const c_char,
//                                           prover_did: *const c_char,
//                                           cb: Option<extern fn(xcommand_handle: i32,
//                                                                err: i32,
//                                                                credential_offer_json: *const c_char)>) -> i32;

    fn indy_issuer_create_credential(command_handle: i32,
                                     wallet_handle: i32,
                                     cred_offer_json: *const c_char,
                                     cred_req_json: *const c_char,
                                     cred_values_json: *const c_char,
                                     rev_reg_id: *const c_char,
                                     blob_storage_reader_handle: i32,
                                     cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                          cred_json: *const c_char,
                                                          cred_revoc_id: *const c_char,
                                                          revoc_reg_delta_json: *const c_char)>) -> i32;
//    fn indy_issuer_create_credential(command_handle: i32,
//                                wallet_handle: i32,
//                                credential_req_json: *const c_char,
//                                credential_json: *const c_char,
//                                user_revoc_index: i32,
//                                cb: Option<extern fn(xcommand_handle: i32,
//                                                     err: i32,
//                                                     revoc_reg_update_json: *const c_char, //TODO must be OPTIONAL
//                                                     xcredential_json: *const c_char)>
//    )-> i32;
    fn indy_prover_create_credential_req(command_handle: i32,
                                         wallet_handle: i32,
                                         prover_did: *const c_char,
                                         cred_offer_json: *const c_char,
                                         cred_def_json: *const c_char,
                                         master_secret_id: *const c_char,
                                         cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                              cred_req_json: *const c_char,
                                                              cred_req_metadata_json: *const c_char
                                         )>) -> i32;
//    fn indy_prover_create_credential_req(command_handle: i32,
//                                         wallet_handle: i32,
//                                         prover_did: *const c_char,
//                                         credential_offer_json: *const c_char,
//                                         credential_def_json: *const c_char,
//                                         master_secret_name: *const c_char,
//                                         cb: Option<extern fn(xcommand_handle: i32,
//                                                              err: i32,
//                                                              credential_req_json: *const c_char)>
//    ) -> i32;

    fn indy_prover_store_credential(command_handle: i32,
                                    wallet_handle: i32,
                                    cred_id: *const c_char,
                                    cred_req_json: *const c_char,
                                    cred_req_metadata_json: *const c_char,
                                    cred_json: *const c_char,
                                    cred_def_json: *const c_char,
                                    rev_reg_def_json: *const c_char,
                                    cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                         out_cred_id: *const c_char)>) -> i32;
    //    fn indy_prover_store_credential(command_handle: i32,
    //                               wallet_handle: i32,
    //                               credentials_json: *const c_char,
    //                               rev_reg_json: *const c_char,
    //                               cb: Option<extern fn(xcommand_handle: i32,
    //                                                    err: i32)>
    //    ) -> i32;

    fn indy_prover_get_credentials_for_proof_req(command_handle: i32,
                                                 wallet_handle: i32,
                                                 proof_request_json: *const c_char,
                                                 cb: Option<extern fn(xcommand_handle: i32,
                                                                      err: i32,
                                                                      credentials_json: *const c_char)>
    ) -> i32;

    fn indy_verifier_verify_proof(command_handle: i32,
                                  proof_request_json: *const c_char,
                                  proof_json: *const c_char,
                                  schemas_json: *const c_char,
                                  credential_defs_json: *const c_char,
                                  rev_reg_defs_json: *const c_char,
                                  rev_regs_json: *const c_char,
                                  cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                       valid: bool)>) -> i32;
//    fn indy_verifier_verify_proof(command_handle: i32,
//                                  proof_request_json: *const c_char,
//                                  proof_json: *const c_char,
//                                  schemas_json: *const c_char,
//                                  credential_defs_jsons: *const c_char,
//                                  revoc_regs_json: *const c_char,
//                                  cb: Option<extern fn(xcommand_handle: i32,
//                                                       err: i32,
//                                                       valid: bool)>) -> i32;

    fn indy_prover_create_proof(command_handle: i32,
                                wallet_handle: i32,
                                proof_req_json: *const c_char,
                                requested_credentials_json: *const c_char,
                                master_secret_id: *const c_char,
                                schemas_json: *const c_char,
                                credential_defs_json: *const c_char,
                                rev_states_json: *const c_char,
                                cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                     proof_json: *const c_char)>) -> i32;
//    fn indy_prover_create_proof(command_handle: i32,
//                                wallet_handle: i32,
//                                proof_req_json: *const c_char,
//                                requested_credentials_json: *const c_char,
//                                schemas_json: *const c_char,
//                                master_secret_name: *const c_char,
//                                credential_defs_json: *const c_char,
//                                revoc_regs_json: *const c_char,
//                                cb: Option<extern fn(xcommand_handle: i32,
//                                                     err: i32,
//                                                     proof_json: *const c_char)>
//    ) -> i32;



    fn indy_prover_create_master_secret(command_handle: i32,
                                        wallet_handle: i32,
                                        master_secret_id: *const c_char,
                                        cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                             out_master_secret_id: *const c_char
                                        )>) -> i32;
//    fn indy_prover_create_master_secret(command_handle: i32,
//                                        wallet_handle: i32,
//                                        master_secret_name: *const c_char,
//                                        cb: Option<extern fn(xcommand_handle: i32,
//                                                             err: i32)>
//    ) -> i32;

/*
    fn indy_prover_store_credential_offer(command_handle: i32,
                                     wallet_handle: i32,
                                     credential_offer_json: *const c_char,
                                     cb: Option<extern fn(xcommand_handle: i32,
                                                          err: i32)>
    ) -> i32;
*/


    //Todo: Add to schema object
    fn indy_issuer_create_schema(command_handle: i32,
                                 issuer_did: *const c_char,
                                 name: *const c_char,
                                 version: *const c_char,
                                 attrs: *const c_char,
                                 cb: Option<extern fn(xcommand_handle: i32, err: i32,
                                                      schema_id: *const c_char,
                                                      schema_json: *const c_char)>) -> i32;
}

pub fn libindy_verifier_verify_proof(proof_req_json: &str,
                                     proof_json: &str,
                                     schemas_json: &str,
                                     credential_defs_json: &str,
                                     revoc_regs_json: &str)  -> Result<bool, u32>{

    let rtn_obj = Return_I32_BOOL::new()?;
    let proof_req_json = CString::new(proof_req_json.to_string()).map_err(map_string_error)?;
    let proof_json = CString::new(proof_json.to_string()).map_err(map_string_error)?;
    let schemas_json = CString::new(schemas_json.to_string()).map_err(map_string_error)?;
    let credential_defs_json = CString::new(credential_defs_json.to_string()).map_err(map_string_error)?;
    let revoc_regs_json = CString::new(revoc_regs_json.to_string()).map_err(map_string_error)?;
//    unsafe {
//        indy_function_eval(
//            indy_verifier_verify_proof(rtn_obj.command_handle,
//                                       proof_req_json.as_ptr(),
//                                       proof_json.as_ptr(),
//                                       schemas_json.as_ptr(),
//                                       credential_defs_json.as_ptr(),
//                                       revoc_regs_json.as_ptr(),
//                                       Some(rtn_obj.get_callback()))
//        ).map_err(map_indy_error_code)?;
//    }

//    rtn_obj.receive(TimeoutUtils::some_long())
    Err(0)
}

pub fn libindy_create_and_store_credential_def(wallet_handle: i32,
                                               issuer_did: &str,
                                               schema_json: &str,
                                               sig_type: Option<SigTypes>,
                                               create_non_revoc: bool)  -> Result<String, u32>{

    let rtn_obj = Return_I32_STR::new()?;
    let schema_json = CString::new(schema_json).map_err(map_string_error)?;
    let i_did = CString::new(issuer_did).map_err(map_string_error)?;
    let s_type = CString::new(sig_type.unwrap_or(SigTypes::CL).to_string()).map_err(map_string_error)?;
//    unsafe {
//        indy_function_eval(
//            indy_issuer_create_and_store_credential_def(rtn_obj.command_handle,
//                                                   wallet_handle,
//                                                   i_did.as_ptr(),
//                                                   schema_json.as_ptr(),
//                                                   s_type.as_ptr(),
//                                                   create_non_revoc,
//                                                   Some(rtn_obj.get_callback()))
//        ).map_err(map_indy_error_code)?;
//    }

    Err(0)
//    rtn_obj.receive(TimeoutUtils::some_long()).and_then(check_str)
}

pub fn libindy_issuer_create_credential_offer(wallet_handle: i32,
                                          schema_json: &str,
                                          issuer_did: &str,
                                          prover_did: &str) -> Result<String, u32> {
    if settings::test_indy_mode_enabled() {
        let rc = mock_libindy_rc();
        if rc != 0 { return Err(rc) };
        return Ok(LIBINDY_CRED_OFFER.to_string());
    }
    let rtn_obj = Return_I32_STR::new()?;
    let schema_json = CString::new(schema_json).map_err(map_string_error)?;
    let i_did = CString::new(issuer_did).map_err(map_string_error)?;
    let p_did = CString::new(prover_did).map_err(map_string_error)?;
//    unsafe {
//        indy_function_eval(
//            indy_issuer_create_credential_offer(rtn_obj.command_handle,
//                                               wallet_handle,
//                                               schema_json.as_ptr(),
//                                                   i_did.as_ptr(),
//                                                   p_did.as_ptr(),
//                                                   Some(rtn_obj.get_callback()))
//        ).map_err(map_indy_error_code)?;
//    }

    Err(0)
//    rtn_obj.receive(TimeoutUtils::some_long()).and_then(check_str)
}

pub fn libindy_issuer_create_credential(wallet_handle: i32,
                                        credential_req_json: &str,
                                        credential_json: &str,
                                        user_revoc_index: i32)  -> Result<(String, String), u32>{
    let rtn_obj = Return_I32_STR_STR::new()?;
    let credential_req_json = CString::new(credential_req_json).map_err(map_string_error)?;
    let credential_json = CString::new(credential_json).map_err(map_string_error)?;
//    unsafe {
//        indy_function_eval(
//            indy_issuer_create_credential(rtn_obj.command_handle,
//                                     wallet_handle,
//                                     credential_req_json.as_ptr(),
//                                     credential_json.as_ptr(),
//                                     user_revoc_index,
//                                     Some(rtn_obj.get_callback()))
//        ).map_err(map_indy_error_code)?;
//    }

//    let (opt_str1, opt_str2) = rtn_obj.receive(TimeoutUtils::some_long())?;
//    let str1 = check_str(opt_str1)?;
//    let str2 = check_str(opt_str2)?;
//    Ok((str1, str2))
    Err(0)
}

pub fn libindy_prover_create_proof(wallet_handle: i32,
                                   proof_req_json: &str,
                                   requested_credentials_json: &str,
                                   schemas_json: &str,
                                   master_secret_name: &str,
                                   credential_defs_json: &str,
                                   revoc_regs_json: Option<&str>) -> Result<String, u32> {
    let rtn_obj = Return_I32_STR::new()?;

    let proof_req_json = CString::new(proof_req_json).map_err(map_string_error)?;
    let requested_credentials_json = CString::new(requested_credentials_json).map_err(map_string_error)?;
    let schemas_json = CString::new(schemas_json).map_err(map_string_error)?;
    let master_secret_name = CString::new(master_secret_name).map_err(map_string_error)?;
    let credential_defs_json = CString::new(credential_defs_json).map_err(map_string_error)?;
    let revoc_regs_json = match revoc_regs_json {
        Some(s) => Some(CString::new(s).map_err(map_string_error)?),
        None => None
    };

//    unsafe {
//        indy_function_eval(
//            indy_prover_create_proof(rtn_obj.command_handle,
//                                     wallet_handle,
//                                     proof_req_json.as_ptr(),
//                                     requested_credentials_json.as_ptr(),
//                                     schemas_json.as_ptr(),
//                                     master_secret_name.as_ptr(),
//                                     credential_defs_json.as_ptr(),
//                                     option_cstring_as_ptn(&revoc_regs_json),
//                                     Some(rtn_obj.get_callback()))
//        ).map_err(map_indy_error_code)?;
//    }

    Err(0)
//    rtn_obj.receive(TimeoutUtils::some_long()).and_then(check_str)
}

pub fn libindy_prover_get_credentials(wallet_handle: i32,
                                      proof_req: &str) -> Result<String, u32> {

    let rtn_obj = Return_I32_STR::new()?;

    let proof_req = CString::new(proof_req).map_err(map_string_error)?;

//    unsafe {
//        indy_function_eval(
//            indy_prover_get_credentials_for_proof_req(rtn_obj.command_handle,
//                                                 wallet_handle,
//                                                 proof_req.as_ptr(),
//                                                 Some(rtn_obj.get_callback()))
//        ).map_err(map_indy_error_code)?;
//    }

    Err(0)
//    rtn_obj.receive(TimeoutUtils::some_medium()).and_then(check_str)
}

pub fn libindy_prover_create_credential_req(wallet_handle: i32,
                                                      prover_did: &str,
                                                      credential_offer_json: &str,
                                                      credential_def_json: &str) -> Result<String, u32>
{
    if settings::test_indy_mode_enabled() { return Ok(::utils::constants::CREDENTIAL_REQ_STRING.to_owned()); }

    let rtn_obj = Return_I32_STR::new()?;

    let prover_did = CString::new(prover_did).map_err(map_string_error)?;
    let credential_offer_json = CString::new(credential_offer_json).map_err(map_string_error)?;
    let credential_def_json = CString::new(credential_def_json).map_err(map_string_error)?;
    let master_secret_name = CString::new(settings::get_config_value(settings::CONFIG_LINK_SECRET_ALIAS).unwrap()).map_err(map_string_error)?;

//    unsafe {
//        indy_function_eval(
//            indy_prover_create_credential_req(rtn_obj.command_handle,
//                                                   wallet_handle,
//                                                   prover_did.as_ptr(),
//                                                   credential_offer_json.as_ptr(),
//                                                   credential_def_json.as_ptr(),
//                                                   master_secret_name.as_ptr(),
//                                                   Some(rtn_obj.get_callback()))
//        ).map_err(map_indy_error_code)?;
//    }

    Err(0)
//    rtn_obj.receive(TimeoutUtils::some_medium()).and_then(check_str)
}

pub fn libindy_prover_store_credential(wallet_handle: i32,
                                       credential_json: &str) -> Result<(), u32>
{
    if settings::test_indy_mode_enabled() { return Ok(()); }

    let rtn_obj = Return_I32::new()?;

    let credential_json = CString::new(credential_json).map_err(map_string_error)?;

//    unsafe {
//        indy_function_eval(
//            indy_prover_store_credential(rtn_obj.command_handle,
//                                    wallet_handle,
//                                    credential_json.as_ptr(),
//                                    null(),
//                                    Some(rtn_obj.get_callback()))
//        ).map_err(map_indy_error_code)?;
//    }

    Err(0)
//    rtn_obj.receive(TimeoutUtils::some_medium())
}

/*
pub fn libindy_prover_store_credential_offer(wallet_handle: i32,
                                             credential_offer_json: &str) -> Result<(), u32>
{
    if settings::test_indy_mode_enabled() { return Ok(()); }

    let rtn_obj = Return_I32::new()?;

    let credential_offer_json = CString::new(credential_offer_json).map_err(map_string_error)?;

    unsafe {
        indy_function_eval(
            indy_prover_store_credential_offer(rtn_obj.command_handle,
                                          wallet_handle,
                                          credential_offer_json.as_ptr(),
                                          Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    rtn_obj.receive(TimeoutUtils::some_medium())
}
*/

pub fn libindy_prover_create_master_secret(wallet_handle: i32,
                                           master_secret_name: &str) -> Result<(), u32>
{
    if settings::test_indy_mode_enabled() { return Ok(()); }

    let rtn_obj = Return_I32::new()?;

    let master_secret_name = CString::new(master_secret_name).map_err(map_string_error)?;

//    unsafe {
//        indy_function_eval(
//            indy_prover_create_master_secret(rtn_obj.command_handle,
//                                             wallet_handle,
//                                             master_secret_name.as_ptr(),
//                                             Some(rtn_obj.get_callback()))
//        ).map_err(map_indy_error_code)?;
//    }

    Err(0)
//    rtn_obj.receive(TimeoutUtils::some_medium())
}

pub fn libindy_issuer_create_schema(issuer_did: &str,
                                    name: &str,
                                    version: &str,
                                    attrs: &str) -> Result<(String, String), u32>{
    let rtn_obj = Return_I32_STR_STR::new()?;
    let issuer_did = CString::new(issuer_did).map_err(map_string_error)?;
    let name = CString::new(name).map_err(map_string_error)?;
    let version = CString::new(version).map_err(map_string_error)?;
    let attrs = CString::new(attrs).map_err(map_string_error)?;
    unsafe {
        indy_function_eval(
            indy_issuer_create_schema(rtn_obj.command_handle,
                                      issuer_did.as_ptr(),
                                      name.as_ptr(),
                                      version.as_ptr(),
                                      attrs.as_ptr(),
                                     Some(rtn_obj.get_callback()))
        ).map_err(map_indy_error_code)?;
    }

    let (opt_str1, opt_str2) = rtn_obj.receive(TimeoutUtils::some_long())?;
    let str1 = check_str(opt_str1)?;
    let str2 = check_str(opt_str2)?;
    Ok((str1, str2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use settings;
    use utils::libindy::wallet::{ init_wallet, get_wallet_handle, delete_wallet};
    use utils::constants::{ INDY_PROOF_REQ_JSON,
                            INDY_PROOF_JSON,
                            INDY_SCHEMAS_JSON,
                            INDY_CREDENTIAL_DEFS_JSON,
                            INDY_REVOC_REGS_JSON,
                            SCHEMAS_JSON,
    };

    #[test]
    fn simple_libindy_create_and_store_credential_def_test() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        init_wallet("wallet_simple").unwrap();
        let result = libindy_create_and_store_credential_def(get_wallet_handle(),
                                                             "GGBDg1j8bsKmr4h5T9XqYf",
                                                             SCHEMAS_JSON,
                                                             None,
                                                             false);
        delete_wallet("wallet_simple").unwrap();
        assert!(result.is_ok());
        println!("{}", result.unwrap());
    }

    #[test]
    fn simple_libindy_create_credential_offer_test() {
        ::utils::logger::LoggerUtils::init();
        settings::set_defaults();
        let wallet_name = "test_libindy_create_cred_offer";
        ::utils::devsetup::setup_wallet(wallet_name);
        init_wallet(wallet_name).unwrap();
        let schema_no = 1487;
        let schema_json = r#"{"dest":"2hoqvcwupRTUNkXn6ArYzs","seqNo":1487,"txnTime":1522769798,"type":"101","data":{"name":"Home Address","version":"1.4","attr_names":["address1","address2","city","zip","state"]}}"#;
        let result = libindy_issuer_create_credential_offer(get_wallet_handle(),
                                                            &schema_json,
                                                            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
                                                           "DunkM3x1y7S4ECgSL4Wkru");
        delete_wallet(wallet_name).unwrap();
        assert!(result.is_ok());
        println!("{}", result.unwrap());
    }

    #[test]
    fn simple_libindy_issuer_create_credential_test() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "test_libindy_create_credential";
        ::utils::devsetup::setup_wallet(wallet_name);
        init_wallet(wallet_name).unwrap();

        let libindy_cred_def = ::utils::constants::LIBINDY_CRED_DEF;
        let schema_json = r#"{"dest":"2hoqvcwupRTUNkXn6ArYzs","seqNo":1487,"txnTime":1522769798,"type":"101","data":{"name":"Home Address","version":"1.4","attr_names":["address1","address2","city","zip","state"]}}"#;
        let encoded_cred_data = r#"{"address1":["101TelaLane","63690509275174663089934667471948380740244018358024875547775652380902762701972"],"address2":["101WilsonLane","68086943237164982734333428280784300550565381723532936263016368251445461241953"],"city":["SLC","101327353979588246869873249766058188995681113722618593621043638294296500696424"],"state":["UT","93856629670657830351991220989031130499313559332549427637940645777813964461231"],"zip":["87121","87121"]}"#;
        let wallet_h = get_wallet_handle();
        let schema_no = 1487;

        let libindy_offer = libindy_issuer_create_credential_offer(get_wallet_handle(),
                                                                   &schema_json,
                                                                   &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
                                                                   "DunkM3x1y7S4ECgSL4Wkru").unwrap();
        println!("CredOffer: \n{:?}", libindy_offer);

        libindy_prover_create_master_secret(wallet_h, settings::DEFAULT_LINK_SECRET_ALIAS).unwrap();
        let libindy_cred_req = libindy_prover_create_credential_req(wallet_h,
                                                                              "DunkM3x1y7S4ECgSL4Wkru",
                                                                              &libindy_offer,
                                                                              &libindy_cred_def).unwrap();
        println!("CredReq: \n{:?}", libindy_cred_req);
        let result = libindy_issuer_create_credential(wallet_h,
                                                      &libindy_cred_req,
                                                      encoded_cred_data,
                                                      -1);
        delete_wallet(wallet_name).unwrap();
        assert!(result.is_ok());
        let (str1, str2) = result.unwrap();
        println!("{}\n{}", str1, str2);
    }

    #[test]
    fn simple_libindy_verifier_verify_proof() {
        settings::set_defaults();
        init_wallet("wallet_simple").unwrap();
        let result = libindy_verifier_verify_proof(INDY_PROOF_REQ_JSON,
                                                   INDY_PROOF_JSON,
                                                   INDY_SCHEMAS_JSON,
                                                   INDY_CREDENTIAL_DEFS_JSON,
                                                   INDY_REVOC_REGS_JSON);
        delete_wallet("wallet_simple").unwrap();
        assert!(result.is_ok());
        println!("{}", result.unwrap());
    }

    //Todo: Fix test. Works when run independently but errors out when run with other tests
//    #[test]
//    fn simple_libindy_prover_get_credentials() {
//        settings::set_defaults();
//        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
//        let wallet_name = "test_libindy_create_credential";
//        ::utils::devsetup::setup_wallet(wallet_name);
//        let wallet_h = init_wallet(wallet_name).unwrap();
//
//        let libindy_cred_def = ::utils::constants::LIBINDY_CRED_DEF;
//        let schema_json = r#"{"dest":"2hoqvcwupRTUNkXn6ArYzs","seqNo":1487,"txnTime":1522769798,"type":"101","data":{"name":"Home Address","version":"1.4","attr_names":["address1","address2","city","zip","state"]}}"#;
//        let encoded_cred_data = r#"{"address1":["101TelaLane","63690509275174663089934667471948380740244018358024875547775652380902762701972"],"address2":["101WilsonLane","68086943237164982734333428280784300550565381723532936263016368251445461241953"],"city":["SLC","101327353979588246869873249766058188995681113722618593621043638294296500696424"],"state":["UT","93856629670657830351991220989031130499313559332549427637940645777813964461231"],"zip":["87121","87121"]}"#;
//        let proof_req = r#"{  "nonce":"123432421212", "name":"proof_req_1", "version":"0.1", "requested_attrs":{  "address1_1":{  "name":"address1", "restrictions":[  {  "issuer_did":"2hoqvcwupRTUNkXn6ArYzs", "schema_key":{  "name":"Home Address", "version":"1.4", "did":"2hoqvcwupRTUNkXn6ArYzs" } } ] }, "state_2":{  "name":"state", "restrictions":[  {  "issuer_did":"2hoqvcwupRTUNkXn6ArYzs", "schema_key":{  "name":"Home Address", "version":"1.4", "did":"2hoqvcwupRTUNkXn6ArYzs" } } ] } }, "requested_predicates":{  } }"#;
//        let schema_no = 1487;
//
//        Store Cred In wallet
//        let libindy_offer = libindy_issuer_create_credential_offer(wallet_h,
//                                                                   &schema_json,
//                                                                   &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
//                                                                   "DunkM3x1y7S4ECgSL4Wkru").unwrap();
//        println!("CredOffer: \n{:?}", libindy_offer);
//
//        libindy_prover_create_master_secret(wallet_h, settings::DEFAULT_LINK_SECRET_ALIAS).unwrap();
//        let libindy_cred_req = libindy_prover_create_credential_req(wallet_h,
//                                                                              "DunkM3x1y7S4ECgSL4Wkru",
//                                                                              &libindy_offer,
//                                                                              &libindy_cred_def).unwrap();
//        println!("CredReq: \n{:?}", libindy_cred_req);
//        let (str1, cred) = libindy_issuer_create_credential(wallet_h,
//                                                      &libindy_cred_req,
//                                                      encoded_cred_data,
//                                                      -1).unwrap();
//        println!("Cred: \n{}", cred);
//
//        libindy_prover_store_credential(wallet_h, &cred).unwrap();
//
//        Get Credentials
//        let credentials = libindy_prover_get_credentials(wallet_h, proof_req).unwrap();
//        println!("Prover Credentials: \n{}", credentials);
//        delete_wallet(wallet_name).unwrap();
//    }

    #[test]
    fn simple_libindy_create_schema() {
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        let wallet_name = "test_create_schema";
        ::utils::devsetup::setup_wallet(wallet_name);
        init_wallet(wallet_name).unwrap();

        let schema_data = r#"["name", "age", "sex", "height"]"#;
        let result = libindy_issuer_create_schema(
            &settings::get_config_value(settings::CONFIG_INSTITUTION_DID).unwrap(),
            "schema_nam",
            "2.2.2",
            schema_data);
        delete_wallet("test_create_schema").unwrap();
        assert!(result.is_ok());
        let (id, schema) = result.unwrap();
        println!("{}, {}", id, schema);
    }

}
