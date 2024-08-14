impl wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi {
                            pub fn release(&mut self) -> () {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_Release)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi)
                                        }
                            }
                            pub fn init(&mut self) -> () {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_Init)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi)
                                        }
                            }
                            pub fn join(&mut self) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_Join)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi)
                                        }
                            }
                            pub fn register_front(&mut self, psz_front_address: &str) -> () {
                                    let psz_front_address = CString::new(psz_front_address).unwrap();

                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_RegisterFront)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             psz_front_address.as_ptr() as *mut c_char)
                                        }
                            }
                            pub fn register_name_server(&mut self, psz_ns_address: &str) -> () {
                                    let psz_ns_address = CString::new(psz_ns_address).unwrap();

                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_RegisterNameServer)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             psz_ns_address.as_ptr() as *mut c_char)
                                        }
                            }
                                pub fn register_spi(&mut self, p_spi: *const dyn wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpi_trait) -> () {
                                    let p_spi = Box::into_raw(Box::new(( &WRAPPER_HPP_TORASPAPI_CTORA_TSTP_SP_TRADER_SPI_VTABLE, p_spi)));            
                                        unsafe {
                                               ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_RegisterSpi)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_spi as * mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpi)
                                            }
                                }
                            pub fn subscribe_private_topic(&mut self, n_resume_type: wrapper.hpp_TORASPAPI_TORA_TE_RESUME_TYPE) -> () {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_SubscribePrivateTopic)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             n_resume_type)
                                        }
                            }
                            pub fn subscribe_public_topic(&mut self, n_resume_type: wrapper.hpp_TORASPAPI_TORA_TE_RESUME_TYPE) -> () {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_SubscribePublicTopic)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             n_resume_type)
                                        }
                            }
                            pub fn req_user_login(&mut self, p_req_user_login_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPReqUserLoginField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqUserLogin)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_req_user_login_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPReqUserLoginField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_user_logout(&mut self, p_user_logout_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPUserLogoutField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqUserLogout)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_user_logout_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPUserLogoutField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_user_password_update(&mut self, p_user_password_update_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPUserPasswordUpdateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqUserPasswordUpdate)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_user_password_update_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPUserPasswordUpdateField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_input_device_serial(&mut self, p_req_input_device_serial_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPReqInputDeviceSerialField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqInputDeviceSerial)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_req_input_device_serial_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPReqInputDeviceSerialField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_order_insert(&mut self, p_input_order_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqOrderInsert)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_order_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_order_action(&mut self, p_input_order_action_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqOrderAction)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_order_action_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_exercise_insert(&mut self, p_input_exercise_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqExerciseInsert)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_exercise_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_exercise_action(&mut self, p_input_exercise_action_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqExerciseAction)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_exercise_action_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_lock_insert(&mut self, p_input_lock_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPInputLockField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqLockInsert)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_lock_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPInputLockField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_lock_action(&mut self, p_input_lock_action_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPInputLockActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqLockAction)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_lock_action_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPInputLockActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_comb_order_insert(&mut self, p_input_comb_order_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqCombOrderInsert)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_comb_order_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_comb_order_action(&mut self, p_input_comb_order_action_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqCombOrderAction)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_comb_order_action_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_cond_order_insert(&mut self, p_input_cond_order_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqCondOrderInsert)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_cond_order_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_cond_order_action(&mut self, p_input_cond_order_action_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqCondOrderAction)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_cond_order_action_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_comb_exercise_insert(&mut self, p_input_comb_exercise_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqCombExerciseInsert)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_comb_exercise_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_comb_exercise_action(&mut self, p_input_comb_exercise_action_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqCombExerciseAction)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_comb_exercise_action_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_max_lock_volume(&mut self, p_req_inquiry_max_lock_volume_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPReqInquiryMaxLockVolumeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqInquiryMaxLockVolume)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_req_inquiry_max_lock_volume_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPReqInquiryMaxLockVolumeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_max_cover_volume(&mut self, p_req_inquiry_max_cover_volume_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPReqInquiryMaxCoverVolumeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqInquiryMaxCoverVolume)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_req_inquiry_max_cover_volume_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPReqInquiryMaxCoverVolumeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_split_comb_margin_difference(&mut self, p_req_inquiry_split_comb_margin_difference_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPReqInquirySplitCombMarginDifferenceField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqInquirySplitCombMarginDifference)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_req_inquiry_split_comb_margin_difference_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPReqInquirySplitCombMarginDifferenceField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_transfer_fund(&mut self, p_input_transfer_fund_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferFundField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqTransferFund)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_transfer_fund_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferFundField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_transfer_stock_position(&mut self, p_input_transfer_stock_position_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferStockPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqTransferStockPosition)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_input_transfer_stock_position_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferStockPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_jz_fund(&mut self, p_req_inquiry_jz_fund_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPReqInquiryJZFundField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqInquiryJZFund)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_req_inquiry_jz_fund_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPReqInquiryJZFundField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_bank_account_fund(&mut self, p_req_inquiry_bank_account_fund_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPReqInquiryBankAccountFundField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqInquiryBankAccountFund)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_req_inquiry_bank_account_fund_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPReqInquiryBankAccountFundField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_stock_position(&mut self, p_req_inquiry_stock_position_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPReqInquiryStockPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqInquiryStockPosition)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_req_inquiry_stock_position_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPReqInquiryStockPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_exchange(&mut self, p_qry_exchange_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryExchangeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryExchange)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_exchange_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryExchangeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_market_data(&mut self, p_qry_market_data_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryMarketDataField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryMarketData)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_market_data_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryMarketDataField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_security(&mut self, p_qry_security_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQrySecurityField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQrySecurity)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_security_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQrySecurityField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_bu_proxy(&mut self, p_qry_bu_proxy_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryBUProxyField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryBUProxy)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_bu_proxy_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryBUProxyField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_user(&mut self, p_qry_user_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryUserField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryUser)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_user_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryUserField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor(&mut self, p_qry_investor_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryInvestorField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryInvestor)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_investor_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryInvestorField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_shareholder_account(&mut self, p_qry_shareholder_account_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryShareholderAccountField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryShareholderAccount)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_shareholder_account_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryShareholderAccountField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_trading_account(&mut self, p_qry_trading_account_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryTradingAccountField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryTradingAccount)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_trading_account_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryTradingAccountField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_order(&mut self, p_qry_order_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryOrder)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_order_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_trade(&mut self, p_qry_trade_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryTradeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryTrade)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_trade_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryTradeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_position(&mut self, p_qry_position_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryPosition)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_position_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_trading_fee(&mut self, p_qry_trading_fee_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryTradingFeeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryTradingFee)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_trading_fee_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryTradingFeeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_trading_fee(&mut self, p_qry_investor_trading_fee_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryInvestorTradingFeeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryInvestorTradingFee)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_investor_trading_fee_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryInvestorTradingFeeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_margin_fee(&mut self, p_qry_investor_margin_fee_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryInvestorMarginFeeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryInvestorMarginFee)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_investor_margin_fee_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryInvestorMarginFeeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_order_fund_detail(&mut self, p_qry_order_fund_detail_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryOrderFundDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryOrderFundDetail)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_order_fund_detail_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryOrderFundDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_fund_transfer_detail(&mut self, p_qry_fund_transfer_detail_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryFundTransferDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryFundTransferDetail)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_fund_transfer_detail_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryFundTransferDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_position_transfer_detail(&mut self, p_qry_position_transfer_detail_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryPositionTransferDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryPositionTransferDetail)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_position_transfer_detail_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryPositionTransferDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_order_action(&mut self, p_qry_order_action_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryOrderAction)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_order_action_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_stock_position(&mut self, p_qry_stock_position_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryStockPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryStockPosition)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_stock_position_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryStockPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_lock(&mut self, p_qry_lock_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryLockField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryLock)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_lock_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryLockField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_exercise(&mut self, p_qry_exercise_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryExerciseField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryExercise)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_exercise_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryExerciseField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_lock_position(&mut self, p_qry_lock_position_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryLockPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryLockPosition)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_lock_position_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryLockPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_exercise_action(&mut self, p_qry_exercise_action_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryExerciseActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryExerciseAction)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_exercise_action_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryExerciseActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_lock_action(&mut self, p_qry_lock_action_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryLockActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryLockAction)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_lock_action_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryLockActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_stock_position_transfer_detail(&mut self, p_qry_stock_position_transfer_detail_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryStockPositionTransferDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryStockPositionTransferDetail)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_stock_position_transfer_detail_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryStockPositionTransferDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_trading_notice(&mut self, p_qry_trading_notice_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryTradingNoticeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryTradingNotice)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_trading_notice_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryTradingNoticeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_stock_disposal(&mut self, p_qry_stock_disposal_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryStockDisposalField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryStockDisposal)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_stock_disposal_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryStockDisposalField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_stock_disposal_action(&mut self, p_qry_stock_disposal_action_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryStockDisposalActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryStockDisposalAction)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_stock_disposal_action_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryStockDisposalActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_cond_order(&mut self, p_qry_cond_order_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryCondOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryCondOrder)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_cond_order_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryCondOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_cond_order_action(&mut self, p_qry_cond_order_action_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryCondOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryCondOrderAction)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_cond_order_action_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryCondOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_limit_position(&mut self, p_qry_investor_limit_position_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryInvestorLimitPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryInvestorLimitPosition)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_investor_limit_position_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryInvestorLimitPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_limit_amount(&mut self, p_qry_investor_limit_amount_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryInvestorLimitAmountField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryInvestorLimitAmount)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_investor_limit_amount_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryInvestorLimitAmountField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_comb_order_action(&mut self, p_qry_comb_order_action_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryCombOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryCombOrderAction)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_comb_order_action_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryCombOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_comb_order(&mut self, p_qry_comb_order_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryCombOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryCombOrder)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_comb_order_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryCombOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_comb_position(&mut self, p_qry_comb_position_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryCombPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryCombPosition)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_comb_position_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryCombPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_comb_pos_detail(&mut self, p_qry_comb_pos_detail_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryCombPosDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryCombPosDetail)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_comb_pos_detail_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryCombPosDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_exercise_appointment(&mut self, p_qry_exercise_appointment_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryExerciseAppointmentField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryExerciseAppointment)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_exercise_appointment_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryExerciseAppointmentField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_insufficient_covered_stock_position(&mut self, p_qry_insufficient_covered_stock_position_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryInsufficientCoveredStockPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryInsufficientCoveredStockPosition)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_insufficient_covered_stock_position_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryInsufficientCoveredStockPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_comb_security(&mut self, p_qry_comb_security_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryCombSecurityField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryCombSecurity)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_comb_security_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryCombSecurityField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_comb_exercise(&mut self, p_qry_comb_exercise_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryCombExerciseField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryCombExercise)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_comb_exercise_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryCombExerciseField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_comb_exercise_action(&mut self, p_qry_comb_exercise_action_field: &mut wrapper.hpp_TORASPAPI_CTORATstpSPQryCombExerciseActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi_ReqQryCombExerciseAction)(self as *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi,
                                             p_qry_comb_exercise_action_field as * mut wrapper.hpp_TORASPAPI_CTORATstpSPQryCombExerciseActionField,
                                             n_request_id)
                                        }
                            }} 
                unsafe impl Send for wrapper.hpp_TORASPAPI_CTORATstpSPTraderApi {}pub trait wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpi_trait: Send {fn on_front_connected(&mut self, ) {}
fn on_front_disconnected(&mut self, n_reason : std::os::raw::c_int) {}
fn on_rsp_error(&mut self, p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_user_login(&mut self, p_rsp_user_login_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspUserLoginField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_user_logout(&mut self, p_user_logout_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPUserLogoutField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_user_password_update(&mut self, p_user_password_update_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPUserPasswordUpdateField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_input_device_serial(&mut self, p_rsp_input_device_serial_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInputDeviceSerialField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_order_insert(&mut self, p_input_order_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_order(&mut self, p_order : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPOrderField>) {}
fn on_err_rtn_order_insert(&mut self, p_input_order : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_order_action(&mut self, p_input_order_action_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_err_rtn_order_action(&mut self, p_input_order_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_trade(&mut self, p_trade : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPTradeField>) {}
fn on_rsp_exercise_insert(&mut self, p_input_exercise_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_exercise(&mut self, p_exercise : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPExerciseField>) {}
fn on_err_rtn_exercise_insert(&mut self, p_input_exercise : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_exercise_action(&mut self, p_input_exercise_action_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_err_rtn_exercise_action(&mut self, p_input_exercise_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_lock_insert(&mut self, p_input_lock_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputLockField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_lock(&mut self, p_lock : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPLockField>) {}
fn on_err_rtn_lock_insert(&mut self, p_input_lock : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputLockField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_lock_action(&mut self, p_input_lock_action_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputLockActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_err_rtn_lock_action(&mut self, p_input_lock_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputLockActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_stock_disposal(&mut self, p_stock_disposal : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPStockDisposalField>) {}
fn on_rsp_comb_order_insert(&mut self, p_input_comb_order_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_comb_order(&mut self, p_comb_order : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPCombOrderField>) {}
fn on_err_rtn_comb_order_insert(&mut self, p_input_comb_order : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_comb_order_action(&mut self, p_input_comb_order_action_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_err_rtn_comb_order_action(&mut self, p_input_comb_order_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_cond_order_insert(&mut self, p_input_cond_order_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_cond_order(&mut self, p_condition_order : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPConditionOrderField>) {}
fn on_err_rtn_cond_order_insert(&mut self, p_input_cond_order : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_cond_order_action(&mut self, p_input_cond_order_action_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_err_rtn_cond_order_action(&mut self, p_input_cond_order_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_comb_exercise_insert(&mut self, p_input_comb_exercise_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_comb_exercise(&mut self, p_comb_exercise : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPCombExerciseField>) {}
fn on_err_rtn_comb_exercise_insert(&mut self, p_input_comb_exercise : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_comb_exercise_action(&mut self, p_input_comb_exercise_action_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_err_rtn_comb_exercise_action(&mut self, p_input_comb_exercise_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_inquiry_max_lock_volume(&mut self, p_rsp_inquiry_max_lock_volume_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryMaxLockVolumeField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_inquiry_max_cover_volume(&mut self, p_rsp_inquiry_max_cover_volume_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryMaxCoverVolumeField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_inquiry_split_comb_margin_difference(&mut self, p_rsp_inquiry_split_comb_margin_difference_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInquirySplitCombMarginDifferenceField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_transfer_fund(&mut self, p_input_transfer_fund_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferFundField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_transfer_fund(&mut self, p_transfer_fund : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPTransferFundField>) {}
fn on_err_rtn_transfer_fund(&mut self, p_input_transfer_fund : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferFundField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_transfer_position(&mut self, p_transfer_position : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPTransferPositionField>) {}
fn on_err_rtn_transfer_position(&mut self, p_input_transfer_position : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferPositionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_transfer_stock_position(&mut self, p_input_transfer_stock_position_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferStockPositionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_transfer_stock_position(&mut self, p_transfer_stock_position : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPTransferStockPositionField>) {}
fn on_err_rtn_transfer_stock_position(&mut self, p_input_transfer_stock_position : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferStockPositionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_inquiry_jz_fund(&mut self, p_rsp_inquiry_jz_fund_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryJZFundField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_inquiry_bank_account_fund(&mut self, p_rsp_inquiry_bank_account_fund_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryBankAccountFundField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_inquiry_stock_position(&mut self, p_rsp_inquiry_stock_position_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryStockPositionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_market_status(&mut self, p_market_status : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPMarketStatusField>) {}
fn on_rtn_trading_notice(&mut self, p_trading_notice : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPTradingNoticeField>) {}
fn on_rsp_qry_exchange(&mut self, p_exchange : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPExchangeField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_market_data(&mut self, p_market_data : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPMarketDataField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_security(&mut self, p_security : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPSecurityField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_bu_proxy(&mut self, p_bu_proxy : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPBUProxyField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_user(&mut self, p_user : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPUserField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor(&mut self, p_investor : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInvestorField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_shareholder_account(&mut self, p_shareholder_account : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPShareholderAccountField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_trading_account(&mut self, p_trading_account : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPTradingAccountField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_order(&mut self, p_order : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPOrderField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_trade(&mut self, p_trade : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPTradeField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_position(&mut self, p_position : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPPositionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_trading_fee(&mut self, p_trading_fee : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPTradingFeeField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_trading_fee(&mut self, p_investor_trading_fee : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInvestorTradingFeeField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_margin_fee(&mut self, p_investor_margin_fee : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInvestorMarginFeeField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_order_fund_detail(&mut self, p_order_fund_detail : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPOrderFundDetailField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_fund_transfer_detail(&mut self, p_fund_transfer_detail : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPFundTransferDetailField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_position_transfer_detail(&mut self, p_position_transfer_detail : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPPositionTransferDetailField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_order_action(&mut self, p_order_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPOrderActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_stock_position(&mut self, p_stock_position : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPStockPositionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_lock(&mut self, p_lock : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPLockField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_exercise(&mut self, p_exercise : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPExerciseField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_lock_position(&mut self, p_lock_position : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPLockPositionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_exercise_action(&mut self, p_exercise_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPExerciseActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_lock_action(&mut self, p_lock_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPLockActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_stock_position_transfer_detail(&mut self, p_stock_position_transfer_detail : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPStockPositionTransferDetailField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_trading_notice(&mut self, p_trading_notice : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPTradingNoticeField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_stock_disposal(&mut self, p_stock_disposal : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPStockDisposalField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_stock_disposal_action(&mut self, p_stock_disposal_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPStockDisposalActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_cond_order(&mut self, p_cond_order : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPCondOrderField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_cond_order_action(&mut self, p_cond_order_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPCondOrderActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_limit_position(&mut self, p_investor_limit_position : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInvestorLimitPositionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_limit_amount(&mut self, p_investor_limit_amount : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInvestorLimitAmountField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_comb_order_action(&mut self, p_comb_order_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPCombOrderActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_comb_order(&mut self, p_comb_order : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPCombOrderField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_comb_position(&mut self, p_comb_position : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPCombPositionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_comb_pos_detail(&mut self, p_comb_pos_detail : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPCombPosDetailField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_exercise_appointment(&mut self, p_exercise_appointment : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPExerciseAppointmentField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_insufficient_covered_stock_position(&mut self, p_insufficient_covered_stock_position : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInsufficientCoveredStockPositionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_comb_security(&mut self, p_comb_security : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPCombSecurityField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_comb_exercise(&mut self, p_comb_exercise : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPCombExerciseField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_comb_exercise_action(&mut self, p_comb_exercise_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPCombExerciseActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
 }

        #[repr(C)]
        #[derive(Debug)]
        struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiVTable {
        on_front_connected: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat,  ),
                on_front_disconnected: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, n_reason : std::os::raw::c_int ),
                on_rsp_error: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_user_login: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_user_login_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspUserLoginField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_user_logout: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_user_logout_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPUserLogoutField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_user_password_update: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_user_password_update_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPUserPasswordUpdateField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_input_device_serial: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_input_device_serial_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInputDeviceSerialField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_order_insert: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_order_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_order: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_order : * const wrapper.hpp_TORASPAPI_CTORATstpSPOrderField ),
                on_err_rtn_order_insert: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_order : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_order_action: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_order_action_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_err_rtn_order_action: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_order_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_trade: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_trade : * const wrapper.hpp_TORASPAPI_CTORATstpSPTradeField ),
                on_rsp_exercise_insert: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_exercise_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_exercise: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_exercise : * const wrapper.hpp_TORASPAPI_CTORATstpSPExerciseField ),
                on_err_rtn_exercise_insert: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_exercise : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_exercise_action: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_exercise_action_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_err_rtn_exercise_action: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_exercise_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_lock_insert: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_lock_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputLockField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_lock: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_lock : * const wrapper.hpp_TORASPAPI_CTORATstpSPLockField ),
                on_err_rtn_lock_insert: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_lock : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputLockField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_lock_action: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_lock_action_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputLockActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_err_rtn_lock_action: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_lock_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputLockActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_stock_disposal: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_stock_disposal : * const wrapper.hpp_TORASPAPI_CTORATstpSPStockDisposalField ),
                on_rsp_comb_order_insert: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_order_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_comb_order: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_order : * const wrapper.hpp_TORASPAPI_CTORATstpSPCombOrderField ),
                on_err_rtn_comb_order_insert: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_order : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_comb_order_action: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_order_action_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_err_rtn_comb_order_action: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_order_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_cond_order_insert: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_cond_order_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_cond_order: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_condition_order : * const wrapper.hpp_TORASPAPI_CTORATstpSPConditionOrderField ),
                on_err_rtn_cond_order_insert: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_cond_order : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_cond_order_action: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_cond_order_action_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_err_rtn_cond_order_action: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_cond_order_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_comb_exercise_insert: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_exercise_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_comb_exercise: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_exercise : * const wrapper.hpp_TORASPAPI_CTORATstpSPCombExerciseField ),
                on_err_rtn_comb_exercise_insert: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_exercise : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_comb_exercise_action: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_exercise_action_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_err_rtn_comb_exercise_action: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_exercise_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_inquiry_max_lock_volume: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_max_lock_volume_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryMaxLockVolumeField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_inquiry_max_cover_volume: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_max_cover_volume_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryMaxCoverVolumeField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_inquiry_split_comb_margin_difference: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_split_comb_margin_difference_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInquirySplitCombMarginDifferenceField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_transfer_fund: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_transfer_fund_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferFundField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_transfer_fund: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_transfer_fund : * const wrapper.hpp_TORASPAPI_CTORATstpSPTransferFundField ),
                on_err_rtn_transfer_fund: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_transfer_fund : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferFundField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_transfer_position: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_transfer_position : * const wrapper.hpp_TORASPAPI_CTORATstpSPTransferPositionField ),
                on_err_rtn_transfer_position: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_transfer_position : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferPositionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_transfer_stock_position: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_transfer_stock_position_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferStockPositionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_transfer_stock_position: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_transfer_stock_position : * const wrapper.hpp_TORASPAPI_CTORATstpSPTransferStockPositionField ),
                on_err_rtn_transfer_stock_position: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_transfer_stock_position : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferStockPositionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_inquiry_jz_fund: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_jz_fund_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryJZFundField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_inquiry_bank_account_fund: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_bank_account_fund_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryBankAccountFundField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_inquiry_stock_position: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_stock_position_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryStockPositionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_market_status: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_market_status : * const wrapper.hpp_TORASPAPI_CTORATstpSPMarketStatusField ),
                on_rtn_trading_notice: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_trading_notice : * const wrapper.hpp_TORASPAPI_CTORATstpSPTradingNoticeField ),
                on_rsp_qry_exchange: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_exchange : * const wrapper.hpp_TORASPAPI_CTORATstpSPExchangeField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_market_data: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_market_data : * const wrapper.hpp_TORASPAPI_CTORATstpSPMarketDataField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_security: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_security : * const wrapper.hpp_TORASPAPI_CTORATstpSPSecurityField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_bu_proxy: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_bu_proxy : * const wrapper.hpp_TORASPAPI_CTORATstpSPBUProxyField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_user: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_user : * const wrapper.hpp_TORASPAPI_CTORATstpSPUserField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_investor : * const wrapper.hpp_TORASPAPI_CTORATstpSPInvestorField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_shareholder_account: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_shareholder_account : * const wrapper.hpp_TORASPAPI_CTORATstpSPShareholderAccountField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_trading_account: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_trading_account : * const wrapper.hpp_TORASPAPI_CTORATstpSPTradingAccountField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_order: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_order : * const wrapper.hpp_TORASPAPI_CTORATstpSPOrderField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_trade: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_trade : * const wrapper.hpp_TORASPAPI_CTORATstpSPTradeField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_position: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_position : * const wrapper.hpp_TORASPAPI_CTORATstpSPPositionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_trading_fee: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_trading_fee : * const wrapper.hpp_TORASPAPI_CTORATstpSPTradingFeeField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_trading_fee: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_investor_trading_fee : * const wrapper.hpp_TORASPAPI_CTORATstpSPInvestorTradingFeeField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_margin_fee: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_investor_margin_fee : * const wrapper.hpp_TORASPAPI_CTORATstpSPInvestorMarginFeeField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_order_fund_detail: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_order_fund_detail : * const wrapper.hpp_TORASPAPI_CTORATstpSPOrderFundDetailField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_fund_transfer_detail: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_fund_transfer_detail : * const wrapper.hpp_TORASPAPI_CTORATstpSPFundTransferDetailField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_position_transfer_detail: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_position_transfer_detail : * const wrapper.hpp_TORASPAPI_CTORATstpSPPositionTransferDetailField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_order_action: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_order_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPOrderActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_stock_position: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_stock_position : * const wrapper.hpp_TORASPAPI_CTORATstpSPStockPositionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_lock: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_lock : * const wrapper.hpp_TORASPAPI_CTORATstpSPLockField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_exercise: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_exercise : * const wrapper.hpp_TORASPAPI_CTORATstpSPExerciseField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_lock_position: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_lock_position : * const wrapper.hpp_TORASPAPI_CTORATstpSPLockPositionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_exercise_action: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_exercise_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPExerciseActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_lock_action: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_lock_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPLockActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_stock_position_transfer_detail: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_stock_position_transfer_detail : * const wrapper.hpp_TORASPAPI_CTORATstpSPStockPositionTransferDetailField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_trading_notice: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_trading_notice : * const wrapper.hpp_TORASPAPI_CTORATstpSPTradingNoticeField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_stock_disposal: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_stock_disposal : * const wrapper.hpp_TORASPAPI_CTORATstpSPStockDisposalField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_stock_disposal_action: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_stock_disposal_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPStockDisposalActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_cond_order: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_cond_order : * const wrapper.hpp_TORASPAPI_CTORATstpSPCondOrderField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_cond_order_action: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_cond_order_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPCondOrderActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_limit_position: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_investor_limit_position : * const wrapper.hpp_TORASPAPI_CTORATstpSPInvestorLimitPositionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_limit_amount: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_investor_limit_amount : * const wrapper.hpp_TORASPAPI_CTORATstpSPInvestorLimitAmountField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_comb_order_action: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_order_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPCombOrderActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_comb_order: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_order : * const wrapper.hpp_TORASPAPI_CTORATstpSPCombOrderField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_comb_position: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_position : * const wrapper.hpp_TORASPAPI_CTORATstpSPCombPositionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_comb_pos_detail: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_pos_detail : * const wrapper.hpp_TORASPAPI_CTORATstpSPCombPosDetailField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_exercise_appointment: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_exercise_appointment : * const wrapper.hpp_TORASPAPI_CTORATstpSPExerciseAppointmentField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_insufficient_covered_stock_position: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_insufficient_covered_stock_position : * const wrapper.hpp_TORASPAPI_CTORATstpSPInsufficientCoveredStockPositionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_comb_security: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_security : * const wrapper.hpp_TORASPAPI_CTORATstpSPCombSecurityField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_comb_exercise: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_exercise : * const wrapper.hpp_TORASPAPI_CTORATstpSPCombExerciseField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_comb_exercise_action: extern "C" fn(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_exercise_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPCombExerciseActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                 } 

        #[derive(Clone, Debug)]
        pub enum wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput {OnFrontConnected(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnFrontConnectedPacket),OnFrontDisconnected(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnFrontDisconnectedPacket),OnRspError(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspErrorPacket),OnRspUserLogin(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspUserLoginPacket),OnRspUserLogout(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspUserLogoutPacket),OnRspUserPasswordUpdate(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspUserPasswordUpdatePacket),OnRspInputDeviceSerial(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspInputDeviceSerialPacket),OnRspOrderInsert(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspOrderInsertPacket),OnRtnOrder(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnOrderPacket),OnErrRtnOrderInsert(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnOrderInsertPacket),OnRspOrderAction(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspOrderActionPacket),OnErrRtnOrderAction(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnOrderActionPacket),OnRtnTrade(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnTradePacket),OnRspExerciseInsert(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspExerciseInsertPacket),OnRtnExercise(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnExercisePacket),OnErrRtnExerciseInsert(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnExerciseInsertPacket),OnRspExerciseAction(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspExerciseActionPacket),OnErrRtnExerciseAction(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnExerciseActionPacket),OnRspLockInsert(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspLockInsertPacket),OnRtnLock(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnLockPacket),OnErrRtnLockInsert(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnLockInsertPacket),OnRspLockAction(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspLockActionPacket),OnErrRtnLockAction(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnLockActionPacket),OnRtnStockDisposal(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnStockDisposalPacket),OnRspCombOrderInsert(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspCombOrderInsertPacket),OnRtnCombOrder(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnCombOrderPacket),OnErrRtnCombOrderInsert(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombOrderInsertPacket),OnRspCombOrderAction(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspCombOrderActionPacket),OnErrRtnCombOrderAction(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombOrderActionPacket),OnRspCondOrderInsert(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspCondOrderInsertPacket),OnRtnCondOrder(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnCondOrderPacket),OnErrRtnCondOrderInsert(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCondOrderInsertPacket),OnRspCondOrderAction(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspCondOrderActionPacket),OnErrRtnCondOrderAction(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCondOrderActionPacket),OnRspCombExerciseInsert(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspCombExerciseInsertPacket),OnRtnCombExercise(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnCombExercisePacket),OnErrRtnCombExerciseInsert(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombExerciseInsertPacket),OnRspCombExerciseAction(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspCombExerciseActionPacket),OnErrRtnCombExerciseAction(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombExerciseActionPacket),OnRspInquiryMaxLockVolume(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryMaxLockVolumePacket),OnRspInquiryMaxCoverVolume(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryMaxCoverVolumePacket),OnRspInquirySplitCombMarginDifference(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspInquirySplitCombMarginDifferencePacket),OnRspTransferFund(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspTransferFundPacket),OnRtnTransferFund(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnTransferFundPacket),OnErrRtnTransferFund(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnTransferFundPacket),OnRtnTransferPosition(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnTransferPositionPacket),OnErrRtnTransferPosition(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnTransferPositionPacket),OnRspTransferStockPosition(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspTransferStockPositionPacket),OnRtnTransferStockPosition(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnTransferStockPositionPacket),OnErrRtnTransferStockPosition(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnTransferStockPositionPacket),OnRspInquiryJZFund(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryJZFundPacket),OnRspInquiryBankAccountFund(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryBankAccountFundPacket),OnRspInquiryStockPosition(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryStockPositionPacket),OnRtnMarketStatus(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnMarketStatusPacket),OnRtnTradingNotice(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnTradingNoticePacket),OnRspQryExchange(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryExchangePacket),OnRspQryMarketData(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryMarketDataPacket),OnRspQrySecurity(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQrySecurityPacket),OnRspQryBUProxy(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryBUProxyPacket),OnRspQryUser(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryUserPacket),OnRspQryInvestor(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorPacket),OnRspQryShareholderAccount(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryShareholderAccountPacket),OnRspQryTradingAccount(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradingAccountPacket),OnRspQryOrder(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryOrderPacket),OnRspQryTrade(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradePacket),OnRspQryPosition(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryPositionPacket),OnRspQryTradingFee(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradingFeePacket),OnRspQryInvestorTradingFee(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorTradingFeePacket),OnRspQryInvestorMarginFee(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorMarginFeePacket),OnRspQryOrderFundDetail(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryOrderFundDetailPacket),OnRspQryFundTransferDetail(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryFundTransferDetailPacket),OnRspQryPositionTransferDetail(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryPositionTransferDetailPacket),OnRspQryOrderAction(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryOrderActionPacket),OnRspQryStockPosition(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockPositionPacket),OnRspQryLock(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryLockPacket),OnRspQryExercise(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryExercisePacket),OnRspQryLockPosition(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryLockPositionPacket),OnRspQryExerciseAction(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryExerciseActionPacket),OnRspQryLockAction(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryLockActionPacket),OnRspQryStockPositionTransferDetail(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockPositionTransferDetailPacket),OnRspQryTradingNotice(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradingNoticePacket),OnRspQryStockDisposal(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockDisposalPacket),OnRspQryStockDisposalAction(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockDisposalActionPacket),OnRspQryCondOrder(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCondOrderPacket),OnRspQryCondOrderAction(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCondOrderActionPacket),OnRspQryInvestorLimitPosition(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorLimitPositionPacket),OnRspQryInvestorLimitAmount(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorLimitAmountPacket),OnRspQryCombOrderAction(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombOrderActionPacket),OnRspQryCombOrder(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombOrderPacket),OnRspQryCombPosition(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombPositionPacket),OnRspQryCombPosDetail(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombPosDetailPacket),OnRspQryExerciseAppointment(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryExerciseAppointmentPacket),OnRspQryInsufficientCoveredStockPosition(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryInsufficientCoveredStockPositionPacket),OnRspQryCombSecurity(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombSecurityPacket),OnRspQryCombExercise(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombExercisePacket),OnRspQryCombExerciseAction(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombExerciseActionPacket), } 

            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnFrontConnectedPacket {
                
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnFrontDisconnectedPacket {
                pub n_reason : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspErrorPacket {
                pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspUserLoginPacket {
                pub p_rsp_user_login_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspUserLoginField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspUserLogoutPacket {
                pub p_user_logout_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPUserLogoutField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspUserPasswordUpdatePacket {
                pub p_user_password_update_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPUserPasswordUpdateField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspInputDeviceSerialPacket {
                pub p_rsp_input_device_serial_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInputDeviceSerialField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspOrderInsertPacket {
                pub p_input_order_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnOrderPacket {
                pub p_order : Option<wrapper.hpp_TORASPAPI_CTORATstpSPOrderField>,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnOrderInsertPacket {
                pub p_input_order : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspOrderActionPacket {
                pub p_input_order_action_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderActionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnOrderActionPacket {
                pub p_input_order_action : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderActionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnTradePacket {
                pub p_trade : Option<wrapper.hpp_TORASPAPI_CTORATstpSPTradeField>,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspExerciseInsertPacket {
                pub p_input_exercise_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnExercisePacket {
                pub p_exercise : Option<wrapper.hpp_TORASPAPI_CTORATstpSPExerciseField>,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnExerciseInsertPacket {
                pub p_input_exercise : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspExerciseActionPacket {
                pub p_input_exercise_action_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseActionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnExerciseActionPacket {
                pub p_input_exercise_action : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseActionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspLockInsertPacket {
                pub p_input_lock_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputLockField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnLockPacket {
                pub p_lock : Option<wrapper.hpp_TORASPAPI_CTORATstpSPLockField>,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnLockInsertPacket {
                pub p_input_lock : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputLockField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspLockActionPacket {
                pub p_input_lock_action_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputLockActionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnLockActionPacket {
                pub p_input_lock_action : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputLockActionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnStockDisposalPacket {
                pub p_stock_disposal : Option<wrapper.hpp_TORASPAPI_CTORATstpSPStockDisposalField>,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspCombOrderInsertPacket {
                pub p_input_comb_order_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnCombOrderPacket {
                pub p_comb_order : Option<wrapper.hpp_TORASPAPI_CTORATstpSPCombOrderField>,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombOrderInsertPacket {
                pub p_input_comb_order : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspCombOrderActionPacket {
                pub p_input_comb_order_action_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderActionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombOrderActionPacket {
                pub p_input_comb_order_action : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderActionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspCondOrderInsertPacket {
                pub p_input_cond_order_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnCondOrderPacket {
                pub p_condition_order : Option<wrapper.hpp_TORASPAPI_CTORATstpSPConditionOrderField>,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCondOrderInsertPacket {
                pub p_input_cond_order : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspCondOrderActionPacket {
                pub p_input_cond_order_action_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderActionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCondOrderActionPacket {
                pub p_input_cond_order_action : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderActionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspCombExerciseInsertPacket {
                pub p_input_comb_exercise_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnCombExercisePacket {
                pub p_comb_exercise : Option<wrapper.hpp_TORASPAPI_CTORATstpSPCombExerciseField>,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombExerciseInsertPacket {
                pub p_input_comb_exercise : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspCombExerciseActionPacket {
                pub p_input_comb_exercise_action_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseActionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombExerciseActionPacket {
                pub p_input_comb_exercise_action : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseActionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryMaxLockVolumePacket {
                pub p_rsp_inquiry_max_lock_volume_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryMaxLockVolumeField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryMaxCoverVolumePacket {
                pub p_rsp_inquiry_max_cover_volume_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryMaxCoverVolumeField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspInquirySplitCombMarginDifferencePacket {
                pub p_rsp_inquiry_split_comb_margin_difference_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInquirySplitCombMarginDifferenceField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspTransferFundPacket {
                pub p_input_transfer_fund_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferFundField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnTransferFundPacket {
                pub p_transfer_fund : Option<wrapper.hpp_TORASPAPI_CTORATstpSPTransferFundField>,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnTransferFundPacket {
                pub p_input_transfer_fund : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferFundField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnTransferPositionPacket {
                pub p_transfer_position : Option<wrapper.hpp_TORASPAPI_CTORATstpSPTransferPositionField>,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnTransferPositionPacket {
                pub p_input_transfer_position : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferPositionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspTransferStockPositionPacket {
                pub p_input_transfer_stock_position_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferStockPositionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnTransferStockPositionPacket {
                pub p_transfer_stock_position : Option<wrapper.hpp_TORASPAPI_CTORATstpSPTransferStockPositionField>,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnTransferStockPositionPacket {
                pub p_input_transfer_stock_position : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferStockPositionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryJZFundPacket {
                pub p_rsp_inquiry_jz_fund_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryJZFundField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryBankAccountFundPacket {
                pub p_rsp_inquiry_bank_account_fund_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryBankAccountFundField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryStockPositionPacket {
                pub p_rsp_inquiry_stock_position_field : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryStockPositionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnMarketStatusPacket {
                pub p_market_status : Option<wrapper.hpp_TORASPAPI_CTORATstpSPMarketStatusField>,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnTradingNoticePacket {
                pub p_trading_notice : Option<wrapper.hpp_TORASPAPI_CTORATstpSPTradingNoticeField>,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryExchangePacket {
                pub p_exchange : Option<wrapper.hpp_TORASPAPI_CTORATstpSPExchangeField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryMarketDataPacket {
                pub p_market_data : Option<wrapper.hpp_TORASPAPI_CTORATstpSPMarketDataField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQrySecurityPacket {
                pub p_security : Option<wrapper.hpp_TORASPAPI_CTORATstpSPSecurityField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryBUProxyPacket {
                pub p_bu_proxy : Option<wrapper.hpp_TORASPAPI_CTORATstpSPBUProxyField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryUserPacket {
                pub p_user : Option<wrapper.hpp_TORASPAPI_CTORATstpSPUserField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorPacket {
                pub p_investor : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInvestorField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryShareholderAccountPacket {
                pub p_shareholder_account : Option<wrapper.hpp_TORASPAPI_CTORATstpSPShareholderAccountField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradingAccountPacket {
                pub p_trading_account : Option<wrapper.hpp_TORASPAPI_CTORATstpSPTradingAccountField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryOrderPacket {
                pub p_order : Option<wrapper.hpp_TORASPAPI_CTORATstpSPOrderField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradePacket {
                pub p_trade : Option<wrapper.hpp_TORASPAPI_CTORATstpSPTradeField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryPositionPacket {
                pub p_position : Option<wrapper.hpp_TORASPAPI_CTORATstpSPPositionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradingFeePacket {
                pub p_trading_fee : Option<wrapper.hpp_TORASPAPI_CTORATstpSPTradingFeeField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorTradingFeePacket {
                pub p_investor_trading_fee : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInvestorTradingFeeField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorMarginFeePacket {
                pub p_investor_margin_fee : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInvestorMarginFeeField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryOrderFundDetailPacket {
                pub p_order_fund_detail : Option<wrapper.hpp_TORASPAPI_CTORATstpSPOrderFundDetailField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryFundTransferDetailPacket {
                pub p_fund_transfer_detail : Option<wrapper.hpp_TORASPAPI_CTORATstpSPFundTransferDetailField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryPositionTransferDetailPacket {
                pub p_position_transfer_detail : Option<wrapper.hpp_TORASPAPI_CTORATstpSPPositionTransferDetailField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryOrderActionPacket {
                pub p_order_action : Option<wrapper.hpp_TORASPAPI_CTORATstpSPOrderActionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockPositionPacket {
                pub p_stock_position : Option<wrapper.hpp_TORASPAPI_CTORATstpSPStockPositionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryLockPacket {
                pub p_lock : Option<wrapper.hpp_TORASPAPI_CTORATstpSPLockField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryExercisePacket {
                pub p_exercise : Option<wrapper.hpp_TORASPAPI_CTORATstpSPExerciseField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryLockPositionPacket {
                pub p_lock_position : Option<wrapper.hpp_TORASPAPI_CTORATstpSPLockPositionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryExerciseActionPacket {
                pub p_exercise_action : Option<wrapper.hpp_TORASPAPI_CTORATstpSPExerciseActionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryLockActionPacket {
                pub p_lock_action : Option<wrapper.hpp_TORASPAPI_CTORATstpSPLockActionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockPositionTransferDetailPacket {
                pub p_stock_position_transfer_detail : Option<wrapper.hpp_TORASPAPI_CTORATstpSPStockPositionTransferDetailField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradingNoticePacket {
                pub p_trading_notice : Option<wrapper.hpp_TORASPAPI_CTORATstpSPTradingNoticeField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockDisposalPacket {
                pub p_stock_disposal : Option<wrapper.hpp_TORASPAPI_CTORATstpSPStockDisposalField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockDisposalActionPacket {
                pub p_stock_disposal_action : Option<wrapper.hpp_TORASPAPI_CTORATstpSPStockDisposalActionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCondOrderPacket {
                pub p_cond_order : Option<wrapper.hpp_TORASPAPI_CTORATstpSPCondOrderField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCondOrderActionPacket {
                pub p_cond_order_action : Option<wrapper.hpp_TORASPAPI_CTORATstpSPCondOrderActionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorLimitPositionPacket {
                pub p_investor_limit_position : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInvestorLimitPositionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorLimitAmountPacket {
                pub p_investor_limit_amount : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInvestorLimitAmountField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombOrderActionPacket {
                pub p_comb_order_action : Option<wrapper.hpp_TORASPAPI_CTORATstpSPCombOrderActionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombOrderPacket {
                pub p_comb_order : Option<wrapper.hpp_TORASPAPI_CTORATstpSPCombOrderField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombPositionPacket {
                pub p_comb_position : Option<wrapper.hpp_TORASPAPI_CTORATstpSPCombPositionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombPosDetailPacket {
                pub p_comb_pos_detail : Option<wrapper.hpp_TORASPAPI_CTORATstpSPCombPosDetailField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryExerciseAppointmentPacket {
                pub p_exercise_appointment : Option<wrapper.hpp_TORASPAPI_CTORATstpSPExerciseAppointmentField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryInsufficientCoveredStockPositionPacket {
                pub p_insufficient_covered_stock_position : Option<wrapper.hpp_TORASPAPI_CTORATstpSPInsufficientCoveredStockPositionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombSecurityPacket {
                pub p_comb_security : Option<wrapper.hpp_TORASPAPI_CTORATstpSPCombSecurityField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombExercisePacket {
                pub p_comb_exercise : Option<wrapper.hpp_TORASPAPI_CTORATstpSPCombExerciseField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug)]
            pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombExerciseActionPacket {
                pub p_comb_exercise_action : Option<wrapper.hpp_TORASPAPI_CTORATstpSPCombExerciseActionField>,pub p_rsp_info : Option<wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }  
static WRAPPER_HPP_TORASPAPI_CTORA_TSTP_SP_TRADER_SPI_VTABLE: wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiVTable = wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiVTable {
                on_front_connected: spi_on_front_connected,
            on_front_disconnected: spi_on_front_disconnected,
            on_rsp_error: spi_on_rsp_error,
            on_rsp_user_login: spi_on_rsp_user_login,
            on_rsp_user_logout: spi_on_rsp_user_logout,
            on_rsp_user_password_update: spi_on_rsp_user_password_update,
            on_rsp_input_device_serial: spi_on_rsp_input_device_serial,
            on_rsp_order_insert: spi_on_rsp_order_insert,
            on_rtn_order: spi_on_rtn_order,
            on_err_rtn_order_insert: spi_on_err_rtn_order_insert,
            on_rsp_order_action: spi_on_rsp_order_action,
            on_err_rtn_order_action: spi_on_err_rtn_order_action,
            on_rtn_trade: spi_on_rtn_trade,
            on_rsp_exercise_insert: spi_on_rsp_exercise_insert,
            on_rtn_exercise: spi_on_rtn_exercise,
            on_err_rtn_exercise_insert: spi_on_err_rtn_exercise_insert,
            on_rsp_exercise_action: spi_on_rsp_exercise_action,
            on_err_rtn_exercise_action: spi_on_err_rtn_exercise_action,
            on_rsp_lock_insert: spi_on_rsp_lock_insert,
            on_rtn_lock: spi_on_rtn_lock,
            on_err_rtn_lock_insert: spi_on_err_rtn_lock_insert,
            on_rsp_lock_action: spi_on_rsp_lock_action,
            on_err_rtn_lock_action: spi_on_err_rtn_lock_action,
            on_rtn_stock_disposal: spi_on_rtn_stock_disposal,
            on_rsp_comb_order_insert: spi_on_rsp_comb_order_insert,
            on_rtn_comb_order: spi_on_rtn_comb_order,
            on_err_rtn_comb_order_insert: spi_on_err_rtn_comb_order_insert,
            on_rsp_comb_order_action: spi_on_rsp_comb_order_action,
            on_err_rtn_comb_order_action: spi_on_err_rtn_comb_order_action,
            on_rsp_cond_order_insert: spi_on_rsp_cond_order_insert,
            on_rtn_cond_order: spi_on_rtn_cond_order,
            on_err_rtn_cond_order_insert: spi_on_err_rtn_cond_order_insert,
            on_rsp_cond_order_action: spi_on_rsp_cond_order_action,
            on_err_rtn_cond_order_action: spi_on_err_rtn_cond_order_action,
            on_rsp_comb_exercise_insert: spi_on_rsp_comb_exercise_insert,
            on_rtn_comb_exercise: spi_on_rtn_comb_exercise,
            on_err_rtn_comb_exercise_insert: spi_on_err_rtn_comb_exercise_insert,
            on_rsp_comb_exercise_action: spi_on_rsp_comb_exercise_action,
            on_err_rtn_comb_exercise_action: spi_on_err_rtn_comb_exercise_action,
            on_rsp_inquiry_max_lock_volume: spi_on_rsp_inquiry_max_lock_volume,
            on_rsp_inquiry_max_cover_volume: spi_on_rsp_inquiry_max_cover_volume,
            on_rsp_inquiry_split_comb_margin_difference: spi_on_rsp_inquiry_split_comb_margin_difference,
            on_rsp_transfer_fund: spi_on_rsp_transfer_fund,
            on_rtn_transfer_fund: spi_on_rtn_transfer_fund,
            on_err_rtn_transfer_fund: spi_on_err_rtn_transfer_fund,
            on_rtn_transfer_position: spi_on_rtn_transfer_position,
            on_err_rtn_transfer_position: spi_on_err_rtn_transfer_position,
            on_rsp_transfer_stock_position: spi_on_rsp_transfer_stock_position,
            on_rtn_transfer_stock_position: spi_on_rtn_transfer_stock_position,
            on_err_rtn_transfer_stock_position: spi_on_err_rtn_transfer_stock_position,
            on_rsp_inquiry_jz_fund: spi_on_rsp_inquiry_jz_fund,
            on_rsp_inquiry_bank_account_fund: spi_on_rsp_inquiry_bank_account_fund,
            on_rsp_inquiry_stock_position: spi_on_rsp_inquiry_stock_position,
            on_rtn_market_status: spi_on_rtn_market_status,
            on_rtn_trading_notice: spi_on_rtn_trading_notice,
            on_rsp_qry_exchange: spi_on_rsp_qry_exchange,
            on_rsp_qry_market_data: spi_on_rsp_qry_market_data,
            on_rsp_qry_security: spi_on_rsp_qry_security,
            on_rsp_qry_bu_proxy: spi_on_rsp_qry_bu_proxy,
            on_rsp_qry_user: spi_on_rsp_qry_user,
            on_rsp_qry_investor: spi_on_rsp_qry_investor,
            on_rsp_qry_shareholder_account: spi_on_rsp_qry_shareholder_account,
            on_rsp_qry_trading_account: spi_on_rsp_qry_trading_account,
            on_rsp_qry_order: spi_on_rsp_qry_order,
            on_rsp_qry_trade: spi_on_rsp_qry_trade,
            on_rsp_qry_position: spi_on_rsp_qry_position,
            on_rsp_qry_trading_fee: spi_on_rsp_qry_trading_fee,
            on_rsp_qry_investor_trading_fee: spi_on_rsp_qry_investor_trading_fee,
            on_rsp_qry_investor_margin_fee: spi_on_rsp_qry_investor_margin_fee,
            on_rsp_qry_order_fund_detail: spi_on_rsp_qry_order_fund_detail,
            on_rsp_qry_fund_transfer_detail: spi_on_rsp_qry_fund_transfer_detail,
            on_rsp_qry_position_transfer_detail: spi_on_rsp_qry_position_transfer_detail,
            on_rsp_qry_order_action: spi_on_rsp_qry_order_action,
            on_rsp_qry_stock_position: spi_on_rsp_qry_stock_position,
            on_rsp_qry_lock: spi_on_rsp_qry_lock,
            on_rsp_qry_exercise: spi_on_rsp_qry_exercise,
            on_rsp_qry_lock_position: spi_on_rsp_qry_lock_position,
            on_rsp_qry_exercise_action: spi_on_rsp_qry_exercise_action,
            on_rsp_qry_lock_action: spi_on_rsp_qry_lock_action,
            on_rsp_qry_stock_position_transfer_detail: spi_on_rsp_qry_stock_position_transfer_detail,
            on_rsp_qry_trading_notice: spi_on_rsp_qry_trading_notice,
            on_rsp_qry_stock_disposal: spi_on_rsp_qry_stock_disposal,
            on_rsp_qry_stock_disposal_action: spi_on_rsp_qry_stock_disposal_action,
            on_rsp_qry_cond_order: spi_on_rsp_qry_cond_order,
            on_rsp_qry_cond_order_action: spi_on_rsp_qry_cond_order_action,
            on_rsp_qry_investor_limit_position: spi_on_rsp_qry_investor_limit_position,
            on_rsp_qry_investor_limit_amount: spi_on_rsp_qry_investor_limit_amount,
            on_rsp_qry_comb_order_action: spi_on_rsp_qry_comb_order_action,
            on_rsp_qry_comb_order: spi_on_rsp_qry_comb_order,
            on_rsp_qry_comb_position: spi_on_rsp_qry_comb_position,
            on_rsp_qry_comb_pos_detail: spi_on_rsp_qry_comb_pos_detail,
            on_rsp_qry_exercise_appointment: spi_on_rsp_qry_exercise_appointment,
            on_rsp_qry_insufficient_covered_stock_position: spi_on_rsp_qry_insufficient_covered_stock_position,
            on_rsp_qry_comb_security: spi_on_rsp_qry_comb_security,
            on_rsp_qry_comb_exercise: spi_on_rsp_qry_comb_exercise,
            on_rsp_qry_comb_exercise_action: spi_on_rsp_qry_comb_exercise_action,
             };
extern "C" fn spi_on_front_connected(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, ) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_front_connected()
                    }
                }extern "C" fn spi_on_front_disconnected(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, n_reason : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_front_disconnected(n_reason)
                    }
                }extern "C" fn spi_on_rsp_error(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_error(p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_user_login(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_user_login_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspUserLoginField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_user_login(p_rsp_user_login_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_user_logout(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_user_logout_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPUserLogoutField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_user_logout(p_user_logout_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_user_password_update(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_user_password_update_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPUserPasswordUpdateField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_user_password_update(p_user_password_update_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_input_device_serial(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_input_device_serial_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInputDeviceSerialField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_input_device_serial(p_rsp_input_device_serial_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_order_insert(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_order_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_order_insert(p_input_order_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_order(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_order : * const wrapper.hpp_TORASPAPI_CTORATstpSPOrderField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_order(p_order.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_order_insert(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_order : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_order_insert(p_input_order.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_order_action(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_order_action_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_order_action(p_input_order_action_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_err_rtn_order_action(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_order_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_order_action(p_input_order_action.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_trade(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_trade : * const wrapper.hpp_TORASPAPI_CTORATstpSPTradeField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_trade(p_trade.as_ref())
                    }
                }extern "C" fn spi_on_rsp_exercise_insert(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_exercise_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_exercise_insert(p_input_exercise_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_exercise(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_exercise : * const wrapper.hpp_TORASPAPI_CTORATstpSPExerciseField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_exercise(p_exercise.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_exercise_insert(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_exercise : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_exercise_insert(p_input_exercise.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_exercise_action(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_exercise_action_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_exercise_action(p_input_exercise_action_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_err_rtn_exercise_action(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_exercise_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_exercise_action(p_input_exercise_action.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_lock_insert(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_lock_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputLockField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_lock_insert(p_input_lock_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_lock(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_lock : * const wrapper.hpp_TORASPAPI_CTORATstpSPLockField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_lock(p_lock.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_lock_insert(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_lock : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputLockField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_lock_insert(p_input_lock.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_lock_action(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_lock_action_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputLockActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_lock_action(p_input_lock_action_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_err_rtn_lock_action(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_lock_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputLockActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_lock_action(p_input_lock_action.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_stock_disposal(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_stock_disposal : * const wrapper.hpp_TORASPAPI_CTORATstpSPStockDisposalField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_stock_disposal(p_stock_disposal.as_ref())
                    }
                }extern "C" fn spi_on_rsp_comb_order_insert(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_order_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_comb_order_insert(p_input_comb_order_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_comb_order(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_order : * const wrapper.hpp_TORASPAPI_CTORATstpSPCombOrderField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_comb_order(p_comb_order.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_comb_order_insert(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_order : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_comb_order_insert(p_input_comb_order.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_comb_order_action(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_order_action_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_comb_order_action(p_input_comb_order_action_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_err_rtn_comb_order_action(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_order_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_comb_order_action(p_input_comb_order_action.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_cond_order_insert(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_cond_order_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_cond_order_insert(p_input_cond_order_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_cond_order(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_condition_order : * const wrapper.hpp_TORASPAPI_CTORATstpSPConditionOrderField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_cond_order(p_condition_order.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_cond_order_insert(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_cond_order : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_cond_order_insert(p_input_cond_order.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_cond_order_action(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_cond_order_action_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_cond_order_action(p_input_cond_order_action_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_err_rtn_cond_order_action(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_cond_order_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_cond_order_action(p_input_cond_order_action.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_comb_exercise_insert(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_exercise_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_comb_exercise_insert(p_input_comb_exercise_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_comb_exercise(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_exercise : * const wrapper.hpp_TORASPAPI_CTORATstpSPCombExerciseField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_comb_exercise(p_comb_exercise.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_comb_exercise_insert(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_exercise : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_comb_exercise_insert(p_input_comb_exercise.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_comb_exercise_action(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_exercise_action_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_comb_exercise_action(p_input_comb_exercise_action_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_err_rtn_comb_exercise_action(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_comb_exercise_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_comb_exercise_action(p_input_comb_exercise_action.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_inquiry_max_lock_volume(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_max_lock_volume_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryMaxLockVolumeField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_max_lock_volume(p_rsp_inquiry_max_lock_volume_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_inquiry_max_cover_volume(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_max_cover_volume_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryMaxCoverVolumeField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_max_cover_volume(p_rsp_inquiry_max_cover_volume_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_inquiry_split_comb_margin_difference(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_split_comb_margin_difference_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInquirySplitCombMarginDifferenceField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_split_comb_margin_difference(p_rsp_inquiry_split_comb_margin_difference_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_transfer_fund(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_transfer_fund_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferFundField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_transfer_fund(p_input_transfer_fund_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_transfer_fund(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_transfer_fund : * const wrapper.hpp_TORASPAPI_CTORATstpSPTransferFundField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_transfer_fund(p_transfer_fund.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_transfer_fund(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_transfer_fund : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferFundField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_transfer_fund(p_input_transfer_fund.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_transfer_position(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_transfer_position : * const wrapper.hpp_TORASPAPI_CTORATstpSPTransferPositionField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_transfer_position(p_transfer_position.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_transfer_position(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_transfer_position : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferPositionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_transfer_position(p_input_transfer_position.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_transfer_stock_position(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_transfer_stock_position_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferStockPositionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_transfer_stock_position(p_input_transfer_stock_position_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_transfer_stock_position(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_transfer_stock_position : * const wrapper.hpp_TORASPAPI_CTORATstpSPTransferStockPositionField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_transfer_stock_position(p_transfer_stock_position.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_transfer_stock_position(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_input_transfer_stock_position : * const wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferStockPositionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_transfer_stock_position(p_input_transfer_stock_position.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_inquiry_jz_fund(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_jz_fund_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryJZFundField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_jz_fund(p_rsp_inquiry_jz_fund_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_inquiry_bank_account_fund(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_bank_account_fund_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryBankAccountFundField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_bank_account_fund(p_rsp_inquiry_bank_account_fund_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_inquiry_stock_position(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_rsp_inquiry_stock_position_field : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryStockPositionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_stock_position(p_rsp_inquiry_stock_position_field.as_ref(),p_rsp_info.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_market_status(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_market_status : * const wrapper.hpp_TORASPAPI_CTORATstpSPMarketStatusField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_market_status(p_market_status.as_ref())
                    }
                }extern "C" fn spi_on_rtn_trading_notice(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_trading_notice : * const wrapper.hpp_TORASPAPI_CTORATstpSPTradingNoticeField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_trading_notice(p_trading_notice.as_ref())
                    }
                }extern "C" fn spi_on_rsp_qry_exchange(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_exchange : * const wrapper.hpp_TORASPAPI_CTORATstpSPExchangeField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_exchange(p_exchange.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_market_data(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_market_data : * const wrapper.hpp_TORASPAPI_CTORATstpSPMarketDataField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_market_data(p_market_data.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_security(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_security : * const wrapper.hpp_TORASPAPI_CTORATstpSPSecurityField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_security(p_security.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_bu_proxy(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_bu_proxy : * const wrapper.hpp_TORASPAPI_CTORATstpSPBUProxyField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_bu_proxy(p_bu_proxy.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_user(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_user : * const wrapper.hpp_TORASPAPI_CTORATstpSPUserField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_user(p_user.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_investor : * const wrapper.hpp_TORASPAPI_CTORATstpSPInvestorField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor(p_investor.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_shareholder_account(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_shareholder_account : * const wrapper.hpp_TORASPAPI_CTORATstpSPShareholderAccountField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_shareholder_account(p_shareholder_account.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_trading_account(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_trading_account : * const wrapper.hpp_TORASPAPI_CTORATstpSPTradingAccountField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_trading_account(p_trading_account.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_order(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_order : * const wrapper.hpp_TORASPAPI_CTORATstpSPOrderField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_order(p_order.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_trade(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_trade : * const wrapper.hpp_TORASPAPI_CTORATstpSPTradeField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_trade(p_trade.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_position(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_position : * const wrapper.hpp_TORASPAPI_CTORATstpSPPositionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_position(p_position.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_trading_fee(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_trading_fee : * const wrapper.hpp_TORASPAPI_CTORATstpSPTradingFeeField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_trading_fee(p_trading_fee.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_trading_fee(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_investor_trading_fee : * const wrapper.hpp_TORASPAPI_CTORATstpSPInvestorTradingFeeField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_trading_fee(p_investor_trading_fee.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_margin_fee(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_investor_margin_fee : * const wrapper.hpp_TORASPAPI_CTORATstpSPInvestorMarginFeeField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_margin_fee(p_investor_margin_fee.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_order_fund_detail(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_order_fund_detail : * const wrapper.hpp_TORASPAPI_CTORATstpSPOrderFundDetailField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_order_fund_detail(p_order_fund_detail.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_fund_transfer_detail(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_fund_transfer_detail : * const wrapper.hpp_TORASPAPI_CTORATstpSPFundTransferDetailField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_fund_transfer_detail(p_fund_transfer_detail.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_position_transfer_detail(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_position_transfer_detail : * const wrapper.hpp_TORASPAPI_CTORATstpSPPositionTransferDetailField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_position_transfer_detail(p_position_transfer_detail.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_order_action(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_order_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPOrderActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_order_action(p_order_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_stock_position(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_stock_position : * const wrapper.hpp_TORASPAPI_CTORATstpSPStockPositionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_stock_position(p_stock_position.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_lock(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_lock : * const wrapper.hpp_TORASPAPI_CTORATstpSPLockField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_lock(p_lock.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_exercise(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_exercise : * const wrapper.hpp_TORASPAPI_CTORATstpSPExerciseField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_exercise(p_exercise.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_lock_position(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_lock_position : * const wrapper.hpp_TORASPAPI_CTORATstpSPLockPositionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_lock_position(p_lock_position.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_exercise_action(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_exercise_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPExerciseActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_exercise_action(p_exercise_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_lock_action(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_lock_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPLockActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_lock_action(p_lock_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_stock_position_transfer_detail(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_stock_position_transfer_detail : * const wrapper.hpp_TORASPAPI_CTORATstpSPStockPositionTransferDetailField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_stock_position_transfer_detail(p_stock_position_transfer_detail.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_trading_notice(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_trading_notice : * const wrapper.hpp_TORASPAPI_CTORATstpSPTradingNoticeField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_trading_notice(p_trading_notice.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_stock_disposal(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_stock_disposal : * const wrapper.hpp_TORASPAPI_CTORATstpSPStockDisposalField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_stock_disposal(p_stock_disposal.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_stock_disposal_action(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_stock_disposal_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPStockDisposalActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_stock_disposal_action(p_stock_disposal_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_cond_order(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_cond_order : * const wrapper.hpp_TORASPAPI_CTORATstpSPCondOrderField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_cond_order(p_cond_order.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_cond_order_action(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_cond_order_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPCondOrderActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_cond_order_action(p_cond_order_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_limit_position(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_investor_limit_position : * const wrapper.hpp_TORASPAPI_CTORATstpSPInvestorLimitPositionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_limit_position(p_investor_limit_position.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_limit_amount(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_investor_limit_amount : * const wrapper.hpp_TORASPAPI_CTORATstpSPInvestorLimitAmountField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_limit_amount(p_investor_limit_amount.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_comb_order_action(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_order_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPCombOrderActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_comb_order_action(p_comb_order_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_comb_order(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_order : * const wrapper.hpp_TORASPAPI_CTORATstpSPCombOrderField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_comb_order(p_comb_order.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_comb_position(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_position : * const wrapper.hpp_TORASPAPI_CTORATstpSPCombPositionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_comb_position(p_comb_position.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_comb_pos_detail(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_pos_detail : * const wrapper.hpp_TORASPAPI_CTORATstpSPCombPosDetailField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_comb_pos_detail(p_comb_pos_detail.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_exercise_appointment(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_exercise_appointment : * const wrapper.hpp_TORASPAPI_CTORATstpSPExerciseAppointmentField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_exercise_appointment(p_exercise_appointment.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_insufficient_covered_stock_position(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_insufficient_covered_stock_position : * const wrapper.hpp_TORASPAPI_CTORATstpSPInsufficientCoveredStockPositionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_insufficient_covered_stock_position(p_insufficient_covered_stock_position.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_comb_security(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_security : * const wrapper.hpp_TORASPAPI_CTORATstpSPCombSecurityField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_comb_security(p_comb_security.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_comb_exercise(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_exercise : * const wrapper.hpp_TORASPAPI_CTORATstpSPCombExerciseField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_comb_exercise(p_comb_exercise.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_comb_exercise_action(spi: *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat, p_comb_exercise_action : * const wrapper.hpp_TORASPAPI_CTORATstpSPCombExerciseActionField,p_rsp_info : * const wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_comb_exercise_action(p_comb_exercise_action.as_ref(),p_rsp_info.as_ref(),n_request_id,b_is_last)
                    }
                }

        #[repr(C)]
        pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiFat {
            vtable: *const wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiVTable,
            pub md_spi_ptr: *mut dyn wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpi_trait,
        }
        

        use futures::stream::Stream;
        use std::{
            pin::Pin,
            sync::{Arc, Mutex},
            task::Waker,
        };
        
        struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiInner {
            buf: std::collections::VecDeque<wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput>,
            waker: Option<Waker>,
        }
        
        impl wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiInner {
            fn push(&mut self, msg: wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput) {
                self.buf.push_back(msg);
                if let Some(ref waker) = &self.waker {
                    waker.clone().wake()
                }
            }
        }
        
        pub struct wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiStream {
            inner: Arc<Mutex<wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiInner>>,
        }
        
        impl Stream for wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiStream {
            type Item = wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput;
        
            fn poll_next(
                self: Pin<&mut Self>,
                cx: &mut futures::task::Context<'_>,
            ) -> futures::task::Poll<Option<Self::Item>> {
                use futures::task::Poll;
                let mut inner = self.inner.lock().unwrap();
                if let Some(i) = inner.buf.pop_front() {
                    Poll::Ready(Some(i))
                } else {
                    inner.waker = Some(cx.waker().clone());
                    Poll::Pending
                }
            }
        
            fn size_hint(&self) -> (usize, Option<usize>) {
                (0, None)
            }
        }
        
        pub fn create_spi() -> (Box<wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiStream>, *mut wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiStream) {
            let i = wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiInner {
                buf: std::collections::VecDeque::new(),
                waker: None,
            };
            let xspi = wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiStream {
                inner: Arc::new(Mutex::new(i)),
            };
            let myspi = Box::new(xspi);
            let pp = Box::into_raw(myspi);
            let pp2 = pp.clone();
            (unsafe { Box::from_raw(pp2) }, pp)
        }
        
impl wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpi_trait for wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiStream {fn on_front_connected(&mut self, ) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnFrontConnected( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnFrontConnectedPacket {  } ))
                }
            fn on_front_disconnected(&mut self, n_reason : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnFrontDisconnected( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnFrontDisconnectedPacket { n_reason:n_reason } ))
                }
            fn on_rsp_error(&mut self, p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspError( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspErrorPacket { p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_user_login(&mut self, p_rsp_user_login_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspUserLoginField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspUserLogin( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspUserLoginPacket { p_rsp_user_login_field:p_rsp_user_login_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_user_logout(&mut self, p_user_logout_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPUserLogoutField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspUserLogout( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspUserLogoutPacket { p_user_logout_field:p_user_logout_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_user_password_update(&mut self, p_user_password_update_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPUserPasswordUpdateField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspUserPasswordUpdate( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspUserPasswordUpdatePacket { p_user_password_update_field:p_user_password_update_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_input_device_serial(&mut self, p_rsp_input_device_serial_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInputDeviceSerialField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspInputDeviceSerial( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspInputDeviceSerialPacket { p_rsp_input_device_serial_field:p_rsp_input_device_serial_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_order_insert(&mut self, p_input_order_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspOrderInsert( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspOrderInsertPacket { p_input_order_field:p_input_order_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_order(&mut self, p_order : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPOrderField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnOrder( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnOrderPacket { p_order:p_order.cloned() } ))
                }
            fn on_err_rtn_order_insert(&mut self, p_input_order : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnOrderInsert( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnOrderInsertPacket { p_input_order:p_input_order.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_order_action(&mut self, p_input_order_action_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspOrderAction( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspOrderActionPacket { p_input_order_action_field:p_input_order_action_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_err_rtn_order_action(&mut self, p_input_order_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputOrderActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnOrderAction( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnOrderActionPacket { p_input_order_action:p_input_order_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_trade(&mut self, p_trade : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPTradeField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnTrade( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnTradePacket { p_trade:p_trade.cloned() } ))
                }
            fn on_rsp_exercise_insert(&mut self, p_input_exercise_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspExerciseInsert( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspExerciseInsertPacket { p_input_exercise_field:p_input_exercise_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_exercise(&mut self, p_exercise : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPExerciseField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnExercise( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnExercisePacket { p_exercise:p_exercise.cloned() } ))
                }
            fn on_err_rtn_exercise_insert(&mut self, p_input_exercise : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnExerciseInsert( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnExerciseInsertPacket { p_input_exercise:p_input_exercise.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_exercise_action(&mut self, p_input_exercise_action_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspExerciseAction( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspExerciseActionPacket { p_input_exercise_action_field:p_input_exercise_action_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_err_rtn_exercise_action(&mut self, p_input_exercise_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputExerciseActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnExerciseAction( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnExerciseActionPacket { p_input_exercise_action:p_input_exercise_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_lock_insert(&mut self, p_input_lock_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputLockField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspLockInsert( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspLockInsertPacket { p_input_lock_field:p_input_lock_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_lock(&mut self, p_lock : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPLockField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnLock( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnLockPacket { p_lock:p_lock.cloned() } ))
                }
            fn on_err_rtn_lock_insert(&mut self, p_input_lock : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputLockField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnLockInsert( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnLockInsertPacket { p_input_lock:p_input_lock.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_lock_action(&mut self, p_input_lock_action_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputLockActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspLockAction( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspLockActionPacket { p_input_lock_action_field:p_input_lock_action_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_err_rtn_lock_action(&mut self, p_input_lock_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputLockActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnLockAction( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnLockActionPacket { p_input_lock_action:p_input_lock_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_stock_disposal(&mut self, p_stock_disposal : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPStockDisposalField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnStockDisposal( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnStockDisposalPacket { p_stock_disposal:p_stock_disposal.cloned() } ))
                }
            fn on_rsp_comb_order_insert(&mut self, p_input_comb_order_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspCombOrderInsert( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspCombOrderInsertPacket { p_input_comb_order_field:p_input_comb_order_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_comb_order(&mut self, p_comb_order : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPCombOrderField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnCombOrder( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnCombOrderPacket { p_comb_order:p_comb_order.cloned() } ))
                }
            fn on_err_rtn_comb_order_insert(&mut self, p_input_comb_order : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnCombOrderInsert( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombOrderInsertPacket { p_input_comb_order:p_input_comb_order.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_comb_order_action(&mut self, p_input_comb_order_action_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspCombOrderAction( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspCombOrderActionPacket { p_input_comb_order_action_field:p_input_comb_order_action_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_err_rtn_comb_order_action(&mut self, p_input_comb_order_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCombOrderActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnCombOrderAction( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombOrderActionPacket { p_input_comb_order_action:p_input_comb_order_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_cond_order_insert(&mut self, p_input_cond_order_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspCondOrderInsert( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspCondOrderInsertPacket { p_input_cond_order_field:p_input_cond_order_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_cond_order(&mut self, p_condition_order : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPConditionOrderField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnCondOrder( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnCondOrderPacket { p_condition_order:p_condition_order.cloned() } ))
                }
            fn on_err_rtn_cond_order_insert(&mut self, p_input_cond_order : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnCondOrderInsert( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCondOrderInsertPacket { p_input_cond_order:p_input_cond_order.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_cond_order_action(&mut self, p_input_cond_order_action_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspCondOrderAction( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspCondOrderActionPacket { p_input_cond_order_action_field:p_input_cond_order_action_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_err_rtn_cond_order_action(&mut self, p_input_cond_order_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCondOrderActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnCondOrderAction( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCondOrderActionPacket { p_input_cond_order_action:p_input_cond_order_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_comb_exercise_insert(&mut self, p_input_comb_exercise_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspCombExerciseInsert( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspCombExerciseInsertPacket { p_input_comb_exercise_field:p_input_comb_exercise_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_comb_exercise(&mut self, p_comb_exercise : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPCombExerciseField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnCombExercise( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnCombExercisePacket { p_comb_exercise:p_comb_exercise.cloned() } ))
                }
            fn on_err_rtn_comb_exercise_insert(&mut self, p_input_comb_exercise : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnCombExerciseInsert( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombExerciseInsertPacket { p_input_comb_exercise:p_input_comb_exercise.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_comb_exercise_action(&mut self, p_input_comb_exercise_action_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspCombExerciseAction( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspCombExerciseActionPacket { p_input_comb_exercise_action_field:p_input_comb_exercise_action_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_err_rtn_comb_exercise_action(&mut self, p_input_comb_exercise_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputCombExerciseActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnCombExerciseAction( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnCombExerciseActionPacket { p_input_comb_exercise_action:p_input_comb_exercise_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_inquiry_max_lock_volume(&mut self, p_rsp_inquiry_max_lock_volume_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryMaxLockVolumeField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspInquiryMaxLockVolume( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryMaxLockVolumePacket { p_rsp_inquiry_max_lock_volume_field:p_rsp_inquiry_max_lock_volume_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_inquiry_max_cover_volume(&mut self, p_rsp_inquiry_max_cover_volume_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryMaxCoverVolumeField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspInquiryMaxCoverVolume( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryMaxCoverVolumePacket { p_rsp_inquiry_max_cover_volume_field:p_rsp_inquiry_max_cover_volume_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_inquiry_split_comb_margin_difference(&mut self, p_rsp_inquiry_split_comb_margin_difference_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInquirySplitCombMarginDifferenceField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspInquirySplitCombMarginDifference( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspInquirySplitCombMarginDifferencePacket { p_rsp_inquiry_split_comb_margin_difference_field:p_rsp_inquiry_split_comb_margin_difference_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_transfer_fund(&mut self, p_input_transfer_fund_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferFundField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspTransferFund( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspTransferFundPacket { p_input_transfer_fund_field:p_input_transfer_fund_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_transfer_fund(&mut self, p_transfer_fund : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPTransferFundField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnTransferFund( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnTransferFundPacket { p_transfer_fund:p_transfer_fund.cloned() } ))
                }
            fn on_err_rtn_transfer_fund(&mut self, p_input_transfer_fund : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferFundField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnTransferFund( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnTransferFundPacket { p_input_transfer_fund:p_input_transfer_fund.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_transfer_position(&mut self, p_transfer_position : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPTransferPositionField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnTransferPosition( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnTransferPositionPacket { p_transfer_position:p_transfer_position.cloned() } ))
                }
            fn on_err_rtn_transfer_position(&mut self, p_input_transfer_position : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferPositionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnTransferPosition( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnTransferPositionPacket { p_input_transfer_position:p_input_transfer_position.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_transfer_stock_position(&mut self, p_input_transfer_stock_position_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferStockPositionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspTransferStockPosition( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspTransferStockPositionPacket { p_input_transfer_stock_position_field:p_input_transfer_stock_position_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_transfer_stock_position(&mut self, p_transfer_stock_position : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPTransferStockPositionField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnTransferStockPosition( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnTransferStockPositionPacket { p_transfer_stock_position:p_transfer_stock_position.cloned() } ))
                }
            fn on_err_rtn_transfer_stock_position(&mut self, p_input_transfer_stock_position : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInputTransferStockPositionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnErrRtnTransferStockPosition( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnErrRtnTransferStockPositionPacket { p_input_transfer_stock_position:p_input_transfer_stock_position.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_inquiry_jz_fund(&mut self, p_rsp_inquiry_jz_fund_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryJZFundField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspInquiryJZFund( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryJZFundPacket { p_rsp_inquiry_jz_fund_field:p_rsp_inquiry_jz_fund_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_inquiry_bank_account_fund(&mut self, p_rsp_inquiry_bank_account_fund_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryBankAccountFundField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspInquiryBankAccountFund( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryBankAccountFundPacket { p_rsp_inquiry_bank_account_fund_field:p_rsp_inquiry_bank_account_fund_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_inquiry_stock_position(&mut self, p_rsp_inquiry_stock_position_field : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInquiryStockPositionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspInquiryStockPosition( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspInquiryStockPositionPacket { p_rsp_inquiry_stock_position_field:p_rsp_inquiry_stock_position_field.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_market_status(&mut self, p_market_status : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPMarketStatusField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnMarketStatus( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnMarketStatusPacket { p_market_status:p_market_status.cloned() } ))
                }
            fn on_rtn_trading_notice(&mut self, p_trading_notice : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPTradingNoticeField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRtnTradingNotice( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRtnTradingNoticePacket { p_trading_notice:p_trading_notice.cloned() } ))
                }
            fn on_rsp_qry_exchange(&mut self, p_exchange : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPExchangeField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryExchange( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryExchangePacket { p_exchange:p_exchange.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_market_data(&mut self, p_market_data : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPMarketDataField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryMarketData( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryMarketDataPacket { p_market_data:p_market_data.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_security(&mut self, p_security : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPSecurityField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQrySecurity( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQrySecurityPacket { p_security:p_security.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_bu_proxy(&mut self, p_bu_proxy : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPBUProxyField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryBUProxy( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryBUProxyPacket { p_bu_proxy:p_bu_proxy.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_user(&mut self, p_user : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPUserField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryUser( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryUserPacket { p_user:p_user.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor(&mut self, p_investor : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInvestorField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryInvestor( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorPacket { p_investor:p_investor.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_shareholder_account(&mut self, p_shareholder_account : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPShareholderAccountField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryShareholderAccount( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryShareholderAccountPacket { p_shareholder_account:p_shareholder_account.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_trading_account(&mut self, p_trading_account : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPTradingAccountField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryTradingAccount( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradingAccountPacket { p_trading_account:p_trading_account.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_order(&mut self, p_order : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPOrderField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryOrder( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryOrderPacket { p_order:p_order.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_trade(&mut self, p_trade : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPTradeField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryTrade( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradePacket { p_trade:p_trade.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_position(&mut self, p_position : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPPositionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryPosition( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryPositionPacket { p_position:p_position.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_trading_fee(&mut self, p_trading_fee : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPTradingFeeField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryTradingFee( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradingFeePacket { p_trading_fee:p_trading_fee.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_trading_fee(&mut self, p_investor_trading_fee : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInvestorTradingFeeField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryInvestorTradingFee( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorTradingFeePacket { p_investor_trading_fee:p_investor_trading_fee.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_margin_fee(&mut self, p_investor_margin_fee : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInvestorMarginFeeField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryInvestorMarginFee( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorMarginFeePacket { p_investor_margin_fee:p_investor_margin_fee.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_order_fund_detail(&mut self, p_order_fund_detail : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPOrderFundDetailField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryOrderFundDetail( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryOrderFundDetailPacket { p_order_fund_detail:p_order_fund_detail.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_fund_transfer_detail(&mut self, p_fund_transfer_detail : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPFundTransferDetailField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryFundTransferDetail( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryFundTransferDetailPacket { p_fund_transfer_detail:p_fund_transfer_detail.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_position_transfer_detail(&mut self, p_position_transfer_detail : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPPositionTransferDetailField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryPositionTransferDetail( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryPositionTransferDetailPacket { p_position_transfer_detail:p_position_transfer_detail.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_order_action(&mut self, p_order_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPOrderActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryOrderAction( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryOrderActionPacket { p_order_action:p_order_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_stock_position(&mut self, p_stock_position : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPStockPositionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryStockPosition( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockPositionPacket { p_stock_position:p_stock_position.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_lock(&mut self, p_lock : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPLockField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryLock( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryLockPacket { p_lock:p_lock.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_exercise(&mut self, p_exercise : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPExerciseField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryExercise( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryExercisePacket { p_exercise:p_exercise.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_lock_position(&mut self, p_lock_position : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPLockPositionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryLockPosition( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryLockPositionPacket { p_lock_position:p_lock_position.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_exercise_action(&mut self, p_exercise_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPExerciseActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryExerciseAction( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryExerciseActionPacket { p_exercise_action:p_exercise_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_lock_action(&mut self, p_lock_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPLockActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryLockAction( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryLockActionPacket { p_lock_action:p_lock_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_stock_position_transfer_detail(&mut self, p_stock_position_transfer_detail : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPStockPositionTransferDetailField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryStockPositionTransferDetail( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockPositionTransferDetailPacket { p_stock_position_transfer_detail:p_stock_position_transfer_detail.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_trading_notice(&mut self, p_trading_notice : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPTradingNoticeField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryTradingNotice( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryTradingNoticePacket { p_trading_notice:p_trading_notice.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_stock_disposal(&mut self, p_stock_disposal : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPStockDisposalField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryStockDisposal( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockDisposalPacket { p_stock_disposal:p_stock_disposal.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_stock_disposal_action(&mut self, p_stock_disposal_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPStockDisposalActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryStockDisposalAction( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryStockDisposalActionPacket { p_stock_disposal_action:p_stock_disposal_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_cond_order(&mut self, p_cond_order : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPCondOrderField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryCondOrder( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCondOrderPacket { p_cond_order:p_cond_order.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_cond_order_action(&mut self, p_cond_order_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPCondOrderActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryCondOrderAction( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCondOrderActionPacket { p_cond_order_action:p_cond_order_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_limit_position(&mut self, p_investor_limit_position : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInvestorLimitPositionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryInvestorLimitPosition( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorLimitPositionPacket { p_investor_limit_position:p_investor_limit_position.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_limit_amount(&mut self, p_investor_limit_amount : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInvestorLimitAmountField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryInvestorLimitAmount( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryInvestorLimitAmountPacket { p_investor_limit_amount:p_investor_limit_amount.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_comb_order_action(&mut self, p_comb_order_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPCombOrderActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryCombOrderAction( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombOrderActionPacket { p_comb_order_action:p_comb_order_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_comb_order(&mut self, p_comb_order : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPCombOrderField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryCombOrder( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombOrderPacket { p_comb_order:p_comb_order.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_comb_position(&mut self, p_comb_position : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPCombPositionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryCombPosition( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombPositionPacket { p_comb_position:p_comb_position.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_comb_pos_detail(&mut self, p_comb_pos_detail : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPCombPosDetailField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryCombPosDetail( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombPosDetailPacket { p_comb_pos_detail:p_comb_pos_detail.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_exercise_appointment(&mut self, p_exercise_appointment : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPExerciseAppointmentField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryExerciseAppointment( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryExerciseAppointmentPacket { p_exercise_appointment:p_exercise_appointment.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_insufficient_covered_stock_position(&mut self, p_insufficient_covered_stock_position : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPInsufficientCoveredStockPositionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryInsufficientCoveredStockPosition( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryInsufficientCoveredStockPositionPacket { p_insufficient_covered_stock_position:p_insufficient_covered_stock_position.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_comb_security(&mut self, p_comb_security : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPCombSecurityField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryCombSecurity( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombSecurityPacket { p_comb_security:p_comb_security.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_comb_exercise(&mut self, p_comb_exercise : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPCombExerciseField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryCombExercise( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombExercisePacket { p_comb_exercise:p_comb_exercise.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_comb_exercise_action(&mut self, p_comb_exercise_action : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPCombExerciseActionField>,p_rsp_info : Option<&wrapper.hpp_TORASPAPI_CTORATstpSPRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOutput::OnRspQryCombExerciseAction( wrapper.hpp_TORASPAPI_CTORATstpSPTraderSpiOnRspQryCombExerciseActionPacket { p_comb_exercise_action:p_comb_exercise_action.cloned(),p_rsp_info:p_rsp_info.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
             }
