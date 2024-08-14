impl wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi {
                            pub fn release(&mut self) -> () {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_Release)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi)
                                        }
                            }
                            pub fn init(&mut self) -> () {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_Init)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi)
                                        }
                            }
                            pub fn join(&mut self) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_Join)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi)
                                        }
                            }
                            pub fn register_front(&mut self, psz_front_address: &str) -> () {
                                    let psz_front_address = CString::new(psz_front_address).unwrap();

                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_RegisterFront)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             psz_front_address.as_ptr() as *mut c_char)
                                        }
                            }
                            pub fn register_name_server(&mut self, psz_ns_address: &str) -> () {
                                    let psz_ns_address = CString::new(psz_ns_address).unwrap();

                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_RegisterNameServer)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             psz_ns_address.as_ptr() as *mut c_char)
                                        }
                            }
                                pub fn register_spi(&mut self, p_spi: *const dyn wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpi_trait) -> () {
                                    let p_spi = Box::into_raw(Box::new(( &WRAPPER_HPP_TORASTOCKAPI_CTORA_TSTP_TRADER_SPI_VTABLE, p_spi)));            
                                        unsafe {
                                               ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_RegisterSpi)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_spi as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpi)
                                            }
                                }
                            pub fn subscribe_private_topic(&mut self, n_resume_type: wrapper.hpp_TORASTOCKAPI_TORA_TE_RESUME_TYPE) -> () {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_SubscribePrivateTopic)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             n_resume_type)
                                        }
                            }
                            pub fn subscribe_public_topic(&mut self, n_resume_type: wrapper.hpp_TORASTOCKAPI_TORA_TE_RESUME_TYPE) -> () {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_SubscribePublicTopic)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             n_resume_type)
                                        }
                            }
                            pub fn req_get_connection_info(&mut self, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqGetConnectionInfo)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             n_request_id)
                                        }
                            }
                            pub fn req_user_login(&mut self, p_req_user_login_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpReqUserLoginField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqUserLogin)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_req_user_login_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpReqUserLoginField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_user_logout(&mut self, p_user_logout_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpUserLogoutField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqUserLogout)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_user_logout_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpUserLogoutField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_user_password_update(&mut self, p_user_password_update_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpUserPasswordUpdateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqUserPasswordUpdate)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_user_password_update_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpUserPasswordUpdateField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_input_device_serial(&mut self, p_req_input_device_serial_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpReqInputDeviceSerialField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqInputDeviceSerial)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_req_input_device_serial_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpReqInputDeviceSerialField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_order_insert(&mut self, p_input_order_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqOrderInsert)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_input_order_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_order_action(&mut self, p_input_order_action_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqOrderAction)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_input_order_action_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_cond_order_insert(&mut self, p_input_cond_order_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqCondOrderInsert)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_input_cond_order_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_cond_order_action(&mut self, p_input_cond_order_action_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqCondOrderAction)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_input_cond_order_action_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_nego_order_insert(&mut self, p_input_nego_order_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqNegoOrderInsert)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_input_nego_order_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_nego_order_action(&mut self, p_input_nego_order_action_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqNegoOrderAction)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_input_nego_order_action_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_transfer_fund(&mut self, p_input_transfer_fund_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferFundField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqTransferFund)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_input_transfer_fund_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferFundField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_transfer_position(&mut self, p_input_transfer_position_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqTransferPosition)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_input_transfer_position_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_jz_fund(&mut self, p_req_inquiry_jz_fund_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpReqInquiryJZFundField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqInquiryJZFund)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_req_inquiry_jz_fund_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpReqInquiryJZFundField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_bank_account_fund(&mut self, p_req_inquiry_bank_account_fund_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpReqInquiryBankAccountFundField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqInquiryBankAccountFund)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_req_inquiry_bank_account_fund_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpReqInquiryBankAccountFundField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_max_order_volume(&mut self, p_req_inquiry_max_order_volume_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpReqInquiryMaxOrderVolumeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqInquiryMaxOrderVolume)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_req_inquiry_max_order_volume_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpReqInquiryMaxOrderVolumeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_trade_concentration(&mut self, p_inquiry_trade_concentration_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpInquiryTradeConcentrationField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqInquiryTradeConcentration)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_inquiry_trade_concentration_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpInquiryTradeConcentrationField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_modify_open_pos_cost(&mut self, p_req_modify_open_pos_cost_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpReqModifyOpenPosCostField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqModifyOpenPosCost)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_req_modify_open_pos_cost_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpReqModifyOpenPosCostField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_input_node_fund_assignment(&mut self, p_input_node_fund_assignment_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpInputNodeFundAssignmentField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqInputNodeFundAssignment)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_input_node_fund_assignment_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpInputNodeFundAssignmentField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_inquiry_node_fund_assignment(&mut self, p_req_inquiry_node_fund_assignment_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpReqInquiryNodeFundAssignmentField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqInquiryNodeFundAssignment)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_req_inquiry_node_fund_assignment_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpReqInquiryNodeFundAssignmentField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_exchange(&mut self, p_qry_exchange_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryExchangeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryExchange)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_exchange_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryExchangeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_security(&mut self, p_qry_security_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQrySecurityField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQrySecurity)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_security_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQrySecurityField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_ipo_info(&mut self, p_qry_ipo_info_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryIPOInfoField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryIPOInfo)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_ipo_info_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryIPOInfoField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_user(&mut self, p_qry_user_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryUserField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryUser)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_user_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryUserField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor(&mut self, p_qry_investor_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryInvestorField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryInvestor)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_investor_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryInvestorField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_shareholder_account(&mut self, p_qry_shareholder_account_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryShareholderAccountField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryShareholderAccount)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_shareholder_account_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryShareholderAccountField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_rational_info(&mut self, p_qry_rational_info_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryRationalInfoField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryRationalInfo)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_rational_info_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryRationalInfoField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_order(&mut self, p_qry_order_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryOrder)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_order_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_order_action(&mut self, p_qry_order_action_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryOrderAction)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_order_action_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_trade(&mut self, p_qry_trade_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryTradeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryTrade)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_trade_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryTradeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_trading_account(&mut self, p_qry_trading_account_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryTradingAccountField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryTradingAccount)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_trading_account_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryTradingAccountField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_position(&mut self, p_qry_position_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryPosition)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_position_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_trading_fee(&mut self, p_qry_trading_fee_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryTradingFeeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryTradingFee)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_trading_fee_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryTradingFeeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_trading_fee(&mut self, p_qry_investor_trading_fee_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryInvestorTradingFeeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryInvestorTradingFee)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_investor_trading_fee_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryInvestorTradingFeeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_ipo_quota(&mut self, p_qry_ipo_quota_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryIPOQuotaField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryIPOQuota)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_ipo_quota_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryIPOQuotaField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_order_fund_detail(&mut self, p_qry_order_fund_detail_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryOrderFundDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryOrderFundDetail)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_order_fund_detail_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryOrderFundDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_fund_transfer_detail(&mut self, p_qry_fund_transfer_detail_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryFundTransferDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryFundTransferDetail)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_fund_transfer_detail_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryFundTransferDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_position_transfer_detail(&mut self, p_qry_position_transfer_detail_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryPositionTransferDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryPositionTransferDetail)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_position_transfer_detail_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryPositionTransferDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_periphery_position_transfer_detail(&mut self, p_qry_periphery_position_transfer_detail_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryPeripheryPositionTransferDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryPeripheryPositionTransferDetail)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_periphery_position_transfer_detail_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryPeripheryPositionTransferDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_periphery_fund_transfer_detail(&mut self, p_qry_periphery_fund_transfer_detail_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryPeripheryFundTransferDetailField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryPeripheryFundTransferDetail)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_periphery_fund_transfer_detail_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryPeripheryFundTransferDetailField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_bond_conversion_info(&mut self, p_qry_bond_conversion_info_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryBondConversionInfoField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryBondConversionInfo)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_bond_conversion_info_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryBondConversionInfoField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_bond_putback_info(&mut self, p_qry_bond_putback_info_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryBondPutbackInfoField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryBondPutbackInfo)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_bond_putback_info_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryBondPutbackInfoField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_cond_order_limit_param(&mut self, p_qry_investor_cond_order_limit_param_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryInvestorCondOrderLimitParamField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryInvestorCondOrderLimitParam)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_investor_cond_order_limit_param_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryInvestorCondOrderLimitParamField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_condition_order(&mut self, p_qry_condition_order_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryConditionOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryConditionOrder)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_condition_order_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryConditionOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_cond_order_action(&mut self, p_qry_cond_order_action_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryCondOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryCondOrderAction)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_cond_order_action_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryCondOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_trading_notice(&mut self, p_qry_trading_notice_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryTradingNoticeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryTradingNotice)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_trading_notice_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryTradingNoticeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_ipo_number_result(&mut self, p_qry_ipo_number_result_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryIPONumberResultField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryIPONumberResult)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_ipo_number_result_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryIPONumberResultField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_ipo_match_number_result(&mut self, p_qry_ipo_match_number_result_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryIPOMatchNumberResultField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryIPOMatchNumberResult)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_ipo_match_number_result_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryIPOMatchNumberResultField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_shareholder_spec_privilege(&mut self, p_qry_shareholder_spec_privilege_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryShareholderSpecPrivilegeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryShareholderSpecPrivilege)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_shareholder_spec_privilege_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryShareholderSpecPrivilegeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_market(&mut self, p_qry_market_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryMarketField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryMarket)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_market_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryMarketField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_etf_file(&mut self, p_qry_etf_file_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryETFFileField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryETFFile)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_etf_file_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryETFFileField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_etf_basket(&mut self, p_qry_etf_basket_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryETFBasketField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryETFBasket)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_etf_basket_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryETFBasketField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_investor_position_limit(&mut self, p_qry_investor_position_limit_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryInvestorPositionLimitField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryInvestorPositionLimit)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_investor_position_limit_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryInvestorPositionLimitField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_szse_imc_params(&mut self, p_qry_szse_imc_params_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQrySZSEImcParamsField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQrySZSEImcParams)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_szse_imc_params_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQrySZSEImcParamsField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_szse_imc_exchange_rate(&mut self, p_qry_szse_imc_exchange_rate_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQrySZSEImcExchangeRateField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQrySZSEImcExchangeRate)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_szse_imc_exchange_rate_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQrySZSEImcExchangeRateField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_szsehk_price_tick_info(&mut self, p_qry_szsehk_price_tick_info_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQrySZSEHKPriceTickInfoField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQrySZSEHKPriceTickInfo)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_szsehk_price_tick_info_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQrySZSEHKPriceTickInfoField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_lof_fund_info(&mut self, p_qry_lof_fund_info_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryLofFundInfoField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryLofFundInfo)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_lof_fund_info_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryLofFundInfoField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_pledge_position(&mut self, p_qry_pledge_position_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryPledgePositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryPledgePosition)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_pledge_position_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryPledgePositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_pledge_info(&mut self, p_qry_pledge_info_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryPledgeInfoField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryPledgeInfo)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_pledge_info_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryPledgeInfoField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_system_node_info(&mut self, p_qry_system_node_info_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQrySystemNodeInfoField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQrySystemNodeInfo)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_system_node_info_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQrySystemNodeInfoField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_standard_bond_position(&mut self, p_qry_standard_bond_position_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryStandardBondPositionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryStandardBondPosition)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_standard_bond_position_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryStandardBondPositionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_prematurity_repo_order(&mut self, p_qry_prematurity_repo_order_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryPrematurityRepoOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryPrematurityRepoOrder)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_prematurity_repo_order_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryPrematurityRepoOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_nego_order(&mut self, p_qry_nego_order_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryNegoOrderField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryNegoOrder)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_nego_order_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryNegoOrderField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_nego_order_action(&mut self, p_qry_nego_order_action_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryNegoOrderActionField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryNegoOrderAction)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_nego_order_action_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryNegoOrderActionField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_nego_trade(&mut self, p_qry_nego_trade_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryNegoTradeField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryNegoTrade)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_nego_trade_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryNegoTradeField,
                                             n_request_id)
                                        }
                            }
                            pub fn req_qry_negotiation_param(&mut self, p_qry_negotiation_param_field: &mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryNegotiationParamField, n_request_id: std::os::raw::c_int) -> std::os::raw::c_int {
                                    
                                    unsafe {
                                           ((*(*self).vtable_).wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi_ReqQryNegotiationParam)(self as *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi,
                                             p_qry_negotiation_param_field as * mut wrapper.hpp_TORASTOCKAPI_CTORATstpQryNegotiationParamField,
                                             n_request_id)
                                        }
                            }} 
                unsafe impl Send for wrapper.hpp_TORASTOCKAPI_CTORATstpTraderApi {}pub trait wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpi_trait: Send {fn on_front_connected(&mut self, ) {}
fn on_front_disconnected(&mut self, n_reason : std::os::raw::c_int) {}
fn on_rsp_error(&mut self, p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_get_connection_info(&mut self, p_connection_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpConnectionInfoField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_user_login(&mut self, p_rsp_user_login_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspUserLoginField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_user_logout(&mut self, p_user_logout_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpUserLogoutField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_user_password_update(&mut self, p_user_password_update_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpUserPasswordUpdateField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_input_device_serial(&mut self, p_rsp_input_device_serial_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInputDeviceSerialField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_order_insert(&mut self, p_input_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_order(&mut self, p_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpOrderField>) {}
fn on_err_rtn_order_insert(&mut self, p_input_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_trade(&mut self, p_trade_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpTradeField>) {}
fn on_rsp_order_action(&mut self, p_input_order_action_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderActionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_err_rtn_order_action(&mut self, p_input_order_action_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderActionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_cond_order_insert(&mut self, p_input_cond_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_cond_order(&mut self, p_condition_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpConditionOrderField>) {}
fn on_err_rtn_cond_order_insert(&mut self, p_input_cond_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_cond_order_action(&mut self, p_input_cond_order_action_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderActionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_err_rtn_cond_order_action(&mut self, p_input_cond_order_action_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderActionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_nego_order_insert(&mut self, p_input_nego_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_nego_order(&mut self, p_nego_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpNegoOrderField>) {}
fn on_err_rtn_nego_order_insert(&mut self, p_input_nego_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_nego_trade(&mut self, p_nego_trade_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpNegoTradeField>) {}
fn on_rsp_nego_order_action(&mut self, p_input_nego_order_action_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderActionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_err_rtn_nego_order_action(&mut self, p_input_nego_order_action_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderActionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_market_status(&mut self, p_market_status_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpMarketStatusField>) {}
fn on_rsp_transfer_fund(&mut self, p_input_transfer_fund_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferFundField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_err_rtn_transfer_fund(&mut self, p_input_transfer_fund_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferFundField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_transfer_fund(&mut self, p_transfer_fund_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpTransferFundField>) {}
fn on_rsp_transfer_position(&mut self, p_input_transfer_position_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferPositionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_err_rtn_transfer_position(&mut self, p_input_transfer_position_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferPositionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_transfer_position(&mut self, p_transfer_position_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpTransferPositionField>) {}
fn on_rtn_periphery_transfer_position(&mut self, p_periphery_transfer_position_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpPeripheryTransferPositionField>) {}
fn on_rtn_periphery_transfer_fund(&mut self, p_periphery_transfer_fund_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpPeripheryTransferFundField>) {}
fn on_rsp_inquiry_jz_fund(&mut self, p_rsp_inquiry_jz_fund_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInquiryJZFundField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_inquiry_bank_account_fund(&mut self, p_rsp_inquiry_bank_account_fund_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInquiryBankAccountFundField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rtn_trading_notice(&mut self, p_trading_notice_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpTradingNoticeField>) {}
fn on_rsp_inquiry_max_order_volume(&mut self, p_rsp_inquiry_max_order_volume_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInquiryMaxOrderVolumeField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_inquiry_trade_concentration(&mut self, p_inquiry_trade_concentration_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInquiryTradeConcentrationField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_modify_open_pos_cost(&mut self, p_req_modify_open_pos_cost_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpReqModifyOpenPosCostField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_input_node_fund_assignment(&mut self, p_input_node_fund_assignment_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputNodeFundAssignmentField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_inquiry_node_fund_assignment(&mut self, p_rsp_inquiry_node_fund_assignment_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInquiryNodeFundAssignmentField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) {}
fn on_rsp_qry_exchange(&mut self, p_exchange_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpExchangeField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_security(&mut self, p_security_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpSecurityField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_ipo_info(&mut self, p_ipo_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpIPOInfoField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_user(&mut self, p_user_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpUserField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor(&mut self, p_investor_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInvestorField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_shareholder_account(&mut self, p_shareholder_account_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpShareholderAccountField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_rational_info(&mut self, p_rational_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRationalInfoField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_order(&mut self, p_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpOrderField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_order_action(&mut self, p_order_action_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpOrderActionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_trade(&mut self, p_trade_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpTradeField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_trading_account(&mut self, p_trading_account_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpTradingAccountField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_position(&mut self, p_position_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpPositionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_trading_fee(&mut self, p_trading_fee_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpTradingFeeField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_trading_fee(&mut self, p_investor_trading_fee_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInvestorTradingFeeField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_ipo_quota(&mut self, p_ipo_quota_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpIPOQuotaField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_order_fund_detail(&mut self, p_order_fund_detail_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpOrderFundDetailField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_fund_transfer_detail(&mut self, p_fund_transfer_detail_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpFundTransferDetailField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_position_transfer_detail(&mut self, p_position_transfer_detail_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpPositionTransferDetailField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_periphery_position_transfer_detail(&mut self, p_periphery_position_transfer_detail_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpPeripheryPositionTransferDetailField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_periphery_fund_transfer_detail(&mut self, p_periphery_fund_transfer_detail_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpPeripheryFundTransferDetailField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_bond_conversion_info(&mut self, p_bond_conversion_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpBondConversionInfoField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_bond_putback_info(&mut self, p_bond_putback_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpBondPutbackInfoField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_cond_order_limit_param(&mut self, p_investor_cond_order_limit_param_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInvestorCondOrderLimitParamField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_condition_order(&mut self, p_condition_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpConditionOrderField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_cond_order_action(&mut self, p_cond_order_action_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpCondOrderActionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_trading_notice(&mut self, p_trading_notice_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpTradingNoticeField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_ipo_number_result(&mut self, p_ipo_number_result_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpIPONumberResultField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_ipo_match_number_result(&mut self, p_ipo_match_number_result_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpIPOMatchNumberResultField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_shareholder_spec_privilege(&mut self, p_shareholder_spec_privilege_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpShareholderSpecPrivilegeField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_market(&mut self, p_market_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpMarketField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_etf_file(&mut self, p_etf_file_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpETFFileField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_etf_basket(&mut self, p_etf_basket_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpETFBasketField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_investor_position_limit(&mut self, p_investor_position_limit_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInvestorPositionLimitField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_szse_imc_params(&mut self, p_szse_imc_params_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpSZSEImcParamsField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_szse_imc_exchange_rate(&mut self, p_szse_imc_exchange_rate_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpSZSEImcExchangeRateField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_szsehk_price_tick_info(&mut self, p_szsehk_price_tick_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpSZSEHKPriceTickInfoField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_lof_fund_info(&mut self, p_lof_fund_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpLofFundInfoField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_pledge_position(&mut self, p_pledge_position_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpPledgePositionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_pledge_info(&mut self, p_pledge_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpPledgeInfoField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_system_node_info(&mut self, p_system_node_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpSystemNodeInfoField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_standard_bond_position(&mut self, p_standard_bond_position_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpStandardBondPositionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_prematurity_repo_order(&mut self, p_prematurity_repo_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpPrematurityRepoOrderField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_nego_order(&mut self, p_nego_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpNegoOrderField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_nego_order_action(&mut self, p_nego_order_action_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpNegoOrderActionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_nego_trade(&mut self, p_nego_trade_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpNegoTradeField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
fn on_rsp_qry_negotiation_param(&mut self, p_negotiation_param_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpNegotiationParamField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) {}
 }

        #[repr(C)]
        #[derive(Debug)]
        struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiVTable {
        on_front_connected: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat,  ),
                on_front_disconnected: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, n_reason : std::os::raw::c_int ),
                on_rsp_error: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_get_connection_info: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_connection_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpConnectionInfoField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_user_login: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_user_login_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspUserLoginField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_user_logout: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_user_logout_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpUserLogoutField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_user_password_update: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_user_password_update_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpUserPasswordUpdateField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_input_device_serial: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_input_device_serial_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInputDeviceSerialField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_order_insert: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_order: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpOrderField ),
                on_err_rtn_order_insert: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_trade: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_trade_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpTradeField ),
                on_rsp_order_action: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_order_action_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderActionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_err_rtn_order_action: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_order_action_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderActionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_cond_order_insert: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_cond_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_cond_order: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_condition_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpConditionOrderField ),
                on_err_rtn_cond_order_insert: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_cond_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_cond_order_action: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_cond_order_action_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderActionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_err_rtn_cond_order_action: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_cond_order_action_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderActionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_nego_order_insert: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_nego_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_nego_order: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_nego_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpNegoOrderField ),
                on_err_rtn_nego_order_insert: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_nego_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_nego_trade: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_nego_trade_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpNegoTradeField ),
                on_rsp_nego_order_action: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_nego_order_action_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderActionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_err_rtn_nego_order_action: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_nego_order_action_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderActionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_market_status: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_market_status_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpMarketStatusField ),
                on_rsp_transfer_fund: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_transfer_fund_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferFundField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_err_rtn_transfer_fund: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_transfer_fund_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferFundField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_transfer_fund: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_transfer_fund_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpTransferFundField ),
                on_rsp_transfer_position: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_transfer_position_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferPositionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_err_rtn_transfer_position: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_transfer_position_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferPositionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_transfer_position: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_transfer_position_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpTransferPositionField ),
                on_rtn_periphery_transfer_position: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_periphery_transfer_position_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpPeripheryTransferPositionField ),
                on_rtn_periphery_transfer_fund: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_periphery_transfer_fund_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpPeripheryTransferFundField ),
                on_rsp_inquiry_jz_fund: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_inquiry_jz_fund_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInquiryJZFundField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_inquiry_bank_account_fund: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_inquiry_bank_account_fund_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInquiryBankAccountFundField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rtn_trading_notice: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_trading_notice_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpTradingNoticeField ),
                on_rsp_inquiry_max_order_volume: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_inquiry_max_order_volume_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInquiryMaxOrderVolumeField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_inquiry_trade_concentration: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_inquiry_trade_concentration_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInquiryTradeConcentrationField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_modify_open_pos_cost: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_req_modify_open_pos_cost_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpReqModifyOpenPosCostField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_input_node_fund_assignment: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_node_fund_assignment_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputNodeFundAssignmentField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_inquiry_node_fund_assignment: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_inquiry_node_fund_assignment_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInquiryNodeFundAssignmentField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int ),
                on_rsp_qry_exchange: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_exchange_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpExchangeField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_security: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_security_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpSecurityField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_ipo_info: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_ipo_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpIPOInfoField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_user: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_user_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpUserField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_investor_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInvestorField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_shareholder_account: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_shareholder_account_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpShareholderAccountField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_rational_info: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_rational_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRationalInfoField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_order: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpOrderField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_order_action: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_order_action_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpOrderActionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_trade: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_trade_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpTradeField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_trading_account: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_trading_account_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpTradingAccountField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_position: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_position_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpPositionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_trading_fee: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_trading_fee_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpTradingFeeField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_trading_fee: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_investor_trading_fee_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInvestorTradingFeeField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_ipo_quota: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_ipo_quota_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpIPOQuotaField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_order_fund_detail: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_order_fund_detail_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpOrderFundDetailField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_fund_transfer_detail: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_fund_transfer_detail_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpFundTransferDetailField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_position_transfer_detail: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_position_transfer_detail_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpPositionTransferDetailField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_periphery_position_transfer_detail: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_periphery_position_transfer_detail_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpPeripheryPositionTransferDetailField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_periphery_fund_transfer_detail: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_periphery_fund_transfer_detail_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpPeripheryFundTransferDetailField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_bond_conversion_info: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_bond_conversion_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpBondConversionInfoField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_bond_putback_info: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_bond_putback_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpBondPutbackInfoField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_cond_order_limit_param: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_investor_cond_order_limit_param_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInvestorCondOrderLimitParamField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_condition_order: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_condition_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpConditionOrderField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_cond_order_action: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_cond_order_action_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpCondOrderActionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_trading_notice: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_trading_notice_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpTradingNoticeField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_ipo_number_result: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_ipo_number_result_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpIPONumberResultField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_ipo_match_number_result: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_ipo_match_number_result_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpIPOMatchNumberResultField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_shareholder_spec_privilege: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_shareholder_spec_privilege_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpShareholderSpecPrivilegeField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_market: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_market_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpMarketField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_etf_file: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_etf_file_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpETFFileField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_etf_basket: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_etf_basket_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpETFBasketField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_investor_position_limit: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_investor_position_limit_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInvestorPositionLimitField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_szse_imc_params: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_szse_imc_params_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpSZSEImcParamsField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_szse_imc_exchange_rate: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_szse_imc_exchange_rate_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpSZSEImcExchangeRateField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_szsehk_price_tick_info: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_szsehk_price_tick_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpSZSEHKPriceTickInfoField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_lof_fund_info: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_lof_fund_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpLofFundInfoField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_pledge_position: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_pledge_position_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpPledgePositionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_pledge_info: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_pledge_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpPledgeInfoField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_system_node_info: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_system_node_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpSystemNodeInfoField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_standard_bond_position: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_standard_bond_position_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpStandardBondPositionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_prematurity_repo_order: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_prematurity_repo_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpPrematurityRepoOrderField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_nego_order: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_nego_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpNegoOrderField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_nego_order_action: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_nego_order_action_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpNegoOrderActionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_nego_trade: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_nego_trade_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpNegoTradeField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                on_rsp_qry_negotiation_param: extern "C" fn(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_negotiation_param_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpNegotiationParamField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool ),
                 } 

        #[derive(Clone, Debug, Decode, Encode)]
        pub enum wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput {OnFrontConnected(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnFrontConnectedPacket),OnFrontDisconnected(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnFrontDisconnectedPacket),OnRspError(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspErrorPacket),OnRspGetConnectionInfo(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspGetConnectionInfoPacket),OnRspUserLogin(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspUserLoginPacket),OnRspUserLogout(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspUserLogoutPacket),OnRspUserPasswordUpdate(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspUserPasswordUpdatePacket),OnRspInputDeviceSerial(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspInputDeviceSerialPacket),OnRspOrderInsert(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspOrderInsertPacket),OnRtnOrder(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnOrderPacket),OnErrRtnOrderInsert(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnOrderInsertPacket),OnRtnTrade(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnTradePacket),OnRspOrderAction(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspOrderActionPacket),OnErrRtnOrderAction(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnOrderActionPacket),OnRspCondOrderInsert(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspCondOrderInsertPacket),OnRtnCondOrder(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnCondOrderPacket),OnErrRtnCondOrderInsert(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnCondOrderInsertPacket),OnRspCondOrderAction(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspCondOrderActionPacket),OnErrRtnCondOrderAction(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnCondOrderActionPacket),OnRspNegoOrderInsert(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspNegoOrderInsertPacket),OnRtnNegoOrder(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnNegoOrderPacket),OnErrRtnNegoOrderInsert(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnNegoOrderInsertPacket),OnRtnNegoTrade(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnNegoTradePacket),OnRspNegoOrderAction(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspNegoOrderActionPacket),OnErrRtnNegoOrderAction(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnNegoOrderActionPacket),OnRtnMarketStatus(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnMarketStatusPacket),OnRspTransferFund(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspTransferFundPacket),OnErrRtnTransferFund(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnTransferFundPacket),OnRtnTransferFund(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnTransferFundPacket),OnRspTransferPosition(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspTransferPositionPacket),OnErrRtnTransferPosition(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnTransferPositionPacket),OnRtnTransferPosition(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnTransferPositionPacket),OnRtnPeripheryTransferPosition(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnPeripheryTransferPositionPacket),OnRtnPeripheryTransferFund(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnPeripheryTransferFundPacket),OnRspInquiryJZFund(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryJZFundPacket),OnRspInquiryBankAccountFund(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryBankAccountFundPacket),OnRtnTradingNotice(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnTradingNoticePacket),OnRspInquiryMaxOrderVolume(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryMaxOrderVolumePacket),OnRspInquiryTradeConcentration(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryTradeConcentrationPacket),OnRspModifyOpenPosCost(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspModifyOpenPosCostPacket),OnRspInputNodeFundAssignment(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspInputNodeFundAssignmentPacket),OnRspInquiryNodeFundAssignment(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryNodeFundAssignmentPacket),OnRspQryExchange(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryExchangePacket),OnRspQrySecurity(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySecurityPacket),OnRspQryIPOInfo(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPOInfoPacket),OnRspQryUser(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryUserPacket),OnRspQryInvestor(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorPacket),OnRspQryShareholderAccount(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryShareholderAccountPacket),OnRspQryRationalInfo(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryRationalInfoPacket),OnRspQryOrder(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryOrderPacket),OnRspQryOrderAction(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryOrderActionPacket),OnRspQryTrade(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradePacket),OnRspQryTradingAccount(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradingAccountPacket),OnRspQryPosition(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPositionPacket),OnRspQryTradingFee(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradingFeePacket),OnRspQryInvestorTradingFee(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorTradingFeePacket),OnRspQryIPOQuota(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPOQuotaPacket),OnRspQryOrderFundDetail(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryOrderFundDetailPacket),OnRspQryFundTransferDetail(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryFundTransferDetailPacket),OnRspQryPositionTransferDetail(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPositionTransferDetailPacket),OnRspQryPeripheryPositionTransferDetail(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPeripheryPositionTransferDetailPacket),OnRspQryPeripheryFundTransferDetail(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPeripheryFundTransferDetailPacket),OnRspQryBondConversionInfo(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryBondConversionInfoPacket),OnRspQryBondPutbackInfo(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryBondPutbackInfoPacket),OnRspQryInvestorCondOrderLimitParam(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorCondOrderLimitParamPacket),OnRspQryConditionOrder(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryConditionOrderPacket),OnRspQryCondOrderAction(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryCondOrderActionPacket),OnRspQryTradingNotice(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradingNoticePacket),OnRspQryIPONumberResult(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPONumberResultPacket),OnRspQryIPOMatchNumberResult(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPOMatchNumberResultPacket),OnRspQryShareholderSpecPrivilege(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryShareholderSpecPrivilegePacket),OnRspQryMarket(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryMarketPacket),OnRspQryETFFile(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryETFFilePacket),OnRspQryETFBasket(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryETFBasketPacket),OnRspQryInvestorPositionLimit(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorPositionLimitPacket),OnRspQrySZSEImcParams(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySZSEImcParamsPacket),OnRspQrySZSEImcExchangeRate(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySZSEImcExchangeRatePacket),OnRspQrySZSEHKPriceTickInfo(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySZSEHKPriceTickInfoPacket),OnRspQryLofFundInfo(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryLofFundInfoPacket),OnRspQryPledgePosition(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPledgePositionPacket),OnRspQryPledgeInfo(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPledgeInfoPacket),OnRspQrySystemNodeInfo(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySystemNodeInfoPacket),OnRspQryStandardBondPosition(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryStandardBondPositionPacket),OnRspQryPrematurityRepoOrder(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPrematurityRepoOrderPacket),OnRspQryNegoOrder(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegoOrderPacket),OnRspQryNegoOrderAction(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegoOrderActionPacket),OnRspQryNegoTrade(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegoTradePacket),OnRspQryNegotiationParam(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegotiationParamPacket), } 

            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnFrontConnectedPacket {
                
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnFrontDisconnectedPacket {
                pub n_reason : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspErrorPacket {
                pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspGetConnectionInfoPacket {
                pub p_connection_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpConnectionInfoField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspUserLoginPacket {
                pub p_rsp_user_login_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspUserLoginField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspUserLogoutPacket {
                pub p_user_logout_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpUserLogoutField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspUserPasswordUpdatePacket {
                pub p_user_password_update_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpUserPasswordUpdateField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspInputDeviceSerialPacket {
                pub p_rsp_input_device_serial_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInputDeviceSerialField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspOrderInsertPacket {
                pub p_input_order_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnOrderPacket {
                pub p_order_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpOrderField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnOrderInsertPacket {
                pub p_input_order_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnTradePacket {
                pub p_trade_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpTradeField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspOrderActionPacket {
                pub p_input_order_action_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderActionField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnOrderActionPacket {
                pub p_input_order_action_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderActionField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspCondOrderInsertPacket {
                pub p_input_cond_order_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnCondOrderPacket {
                pub p_condition_order_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpConditionOrderField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnCondOrderInsertPacket {
                pub p_input_cond_order_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspCondOrderActionPacket {
                pub p_input_cond_order_action_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderActionField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnCondOrderActionPacket {
                pub p_input_cond_order_action_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderActionField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspNegoOrderInsertPacket {
                pub p_input_nego_order_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnNegoOrderPacket {
                pub p_nego_order_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpNegoOrderField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnNegoOrderInsertPacket {
                pub p_input_nego_order_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnNegoTradePacket {
                pub p_nego_trade_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpNegoTradeField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspNegoOrderActionPacket {
                pub p_input_nego_order_action_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderActionField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnNegoOrderActionPacket {
                pub p_input_nego_order_action_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderActionField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnMarketStatusPacket {
                pub p_market_status_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpMarketStatusField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspTransferFundPacket {
                pub p_input_transfer_fund_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferFundField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnTransferFundPacket {
                pub p_input_transfer_fund_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferFundField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnTransferFundPacket {
                pub p_transfer_fund_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpTransferFundField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspTransferPositionPacket {
                pub p_input_transfer_position_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferPositionField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnTransferPositionPacket {
                pub p_input_transfer_position_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferPositionField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnTransferPositionPacket {
                pub p_transfer_position_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpTransferPositionField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnPeripheryTransferPositionPacket {
                pub p_periphery_transfer_position_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpPeripheryTransferPositionField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnPeripheryTransferFundPacket {
                pub p_periphery_transfer_fund_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpPeripheryTransferFundField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryJZFundPacket {
                pub p_rsp_inquiry_jz_fund_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInquiryJZFundField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryBankAccountFundPacket {
                pub p_rsp_inquiry_bank_account_fund_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInquiryBankAccountFundField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnTradingNoticePacket {
                pub p_trading_notice_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpTradingNoticeField>,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryMaxOrderVolumePacket {
                pub p_rsp_inquiry_max_order_volume_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInquiryMaxOrderVolumeField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryTradeConcentrationPacket {
                pub p_inquiry_trade_concentration_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpInquiryTradeConcentrationField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspModifyOpenPosCostPacket {
                pub p_req_modify_open_pos_cost_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpReqModifyOpenPosCostField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspInputNodeFundAssignmentPacket {
                pub p_input_node_fund_assignment_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpInputNodeFundAssignmentField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryNodeFundAssignmentPacket {
                pub p_rsp_inquiry_node_fund_assignment_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInquiryNodeFundAssignmentField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryExchangePacket {
                pub p_exchange_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpExchangeField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySecurityPacket {
                pub p_security_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpSecurityField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPOInfoPacket {
                pub p_ipo_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpIPOInfoField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryUserPacket {
                pub p_user_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpUserField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorPacket {
                pub p_investor_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpInvestorField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryShareholderAccountPacket {
                pub p_shareholder_account_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpShareholderAccountField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryRationalInfoPacket {
                pub p_rational_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRationalInfoField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryOrderPacket {
                pub p_order_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpOrderField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryOrderActionPacket {
                pub p_order_action_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpOrderActionField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradePacket {
                pub p_trade_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpTradeField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradingAccountPacket {
                pub p_trading_account_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpTradingAccountField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPositionPacket {
                pub p_position_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpPositionField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradingFeePacket {
                pub p_trading_fee_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpTradingFeeField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorTradingFeePacket {
                pub p_investor_trading_fee_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpInvestorTradingFeeField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPOQuotaPacket {
                pub p_ipo_quota_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpIPOQuotaField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryOrderFundDetailPacket {
                pub p_order_fund_detail_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpOrderFundDetailField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryFundTransferDetailPacket {
                pub p_fund_transfer_detail_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpFundTransferDetailField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPositionTransferDetailPacket {
                pub p_position_transfer_detail_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpPositionTransferDetailField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPeripheryPositionTransferDetailPacket {
                pub p_periphery_position_transfer_detail_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpPeripheryPositionTransferDetailField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPeripheryFundTransferDetailPacket {
                pub p_periphery_fund_transfer_detail_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpPeripheryFundTransferDetailField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryBondConversionInfoPacket {
                pub p_bond_conversion_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpBondConversionInfoField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryBondPutbackInfoPacket {
                pub p_bond_putback_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpBondPutbackInfoField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorCondOrderLimitParamPacket {
                pub p_investor_cond_order_limit_param_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpInvestorCondOrderLimitParamField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryConditionOrderPacket {
                pub p_condition_order_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpConditionOrderField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryCondOrderActionPacket {
                pub p_cond_order_action_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpCondOrderActionField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradingNoticePacket {
                pub p_trading_notice_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpTradingNoticeField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPONumberResultPacket {
                pub p_ipo_number_result_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpIPONumberResultField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPOMatchNumberResultPacket {
                pub p_ipo_match_number_result_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpIPOMatchNumberResultField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryShareholderSpecPrivilegePacket {
                pub p_shareholder_spec_privilege_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpShareholderSpecPrivilegeField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryMarketPacket {
                pub p_market_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpMarketField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryETFFilePacket {
                pub p_etf_file_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpETFFileField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryETFBasketPacket {
                pub p_etf_basket_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpETFBasketField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorPositionLimitPacket {
                pub p_investor_position_limit_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpInvestorPositionLimitField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySZSEImcParamsPacket {
                pub p_szse_imc_params_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpSZSEImcParamsField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySZSEImcExchangeRatePacket {
                pub p_szse_imc_exchange_rate_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpSZSEImcExchangeRateField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySZSEHKPriceTickInfoPacket {
                pub p_szsehk_price_tick_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpSZSEHKPriceTickInfoField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryLofFundInfoPacket {
                pub p_lof_fund_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpLofFundInfoField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPledgePositionPacket {
                pub p_pledge_position_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpPledgePositionField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPledgeInfoPacket {
                pub p_pledge_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpPledgeInfoField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySystemNodeInfoPacket {
                pub p_system_node_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpSystemNodeInfoField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryStandardBondPositionPacket {
                pub p_standard_bond_position_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpStandardBondPositionField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPrematurityRepoOrderPacket {
                pub p_prematurity_repo_order_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpPrematurityRepoOrderField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegoOrderPacket {
                pub p_nego_order_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpNegoOrderField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegoOrderActionPacket {
                pub p_nego_order_action_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpNegoOrderActionField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegoTradePacket {
                pub p_nego_trade_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpNegoTradeField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }
            #[derive(Clone, Debug, Encode, Decode)]
            pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegotiationParamPacket {
                pub p_negotiation_param_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpNegotiationParamField>,pub p_rsp_info_field : Option<wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,pub n_request_id : std::os::raw::c_int,pub b_is_last : bool,
            }  
static WRAPPER_HPP_TORASTOCKAPI_CTORA_TSTP_TRADER_SPI_VTABLE: wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiVTable = wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiVTable {
                on_front_connected: spi_on_front_connected,
            on_front_disconnected: spi_on_front_disconnected,
            on_rsp_error: spi_on_rsp_error,
            on_rsp_get_connection_info: spi_on_rsp_get_connection_info,
            on_rsp_user_login: spi_on_rsp_user_login,
            on_rsp_user_logout: spi_on_rsp_user_logout,
            on_rsp_user_password_update: spi_on_rsp_user_password_update,
            on_rsp_input_device_serial: spi_on_rsp_input_device_serial,
            on_rsp_order_insert: spi_on_rsp_order_insert,
            on_rtn_order: spi_on_rtn_order,
            on_err_rtn_order_insert: spi_on_err_rtn_order_insert,
            on_rtn_trade: spi_on_rtn_trade,
            on_rsp_order_action: spi_on_rsp_order_action,
            on_err_rtn_order_action: spi_on_err_rtn_order_action,
            on_rsp_cond_order_insert: spi_on_rsp_cond_order_insert,
            on_rtn_cond_order: spi_on_rtn_cond_order,
            on_err_rtn_cond_order_insert: spi_on_err_rtn_cond_order_insert,
            on_rsp_cond_order_action: spi_on_rsp_cond_order_action,
            on_err_rtn_cond_order_action: spi_on_err_rtn_cond_order_action,
            on_rsp_nego_order_insert: spi_on_rsp_nego_order_insert,
            on_rtn_nego_order: spi_on_rtn_nego_order,
            on_err_rtn_nego_order_insert: spi_on_err_rtn_nego_order_insert,
            on_rtn_nego_trade: spi_on_rtn_nego_trade,
            on_rsp_nego_order_action: spi_on_rsp_nego_order_action,
            on_err_rtn_nego_order_action: spi_on_err_rtn_nego_order_action,
            on_rtn_market_status: spi_on_rtn_market_status,
            on_rsp_transfer_fund: spi_on_rsp_transfer_fund,
            on_err_rtn_transfer_fund: spi_on_err_rtn_transfer_fund,
            on_rtn_transfer_fund: spi_on_rtn_transfer_fund,
            on_rsp_transfer_position: spi_on_rsp_transfer_position,
            on_err_rtn_transfer_position: spi_on_err_rtn_transfer_position,
            on_rtn_transfer_position: spi_on_rtn_transfer_position,
            on_rtn_periphery_transfer_position: spi_on_rtn_periphery_transfer_position,
            on_rtn_periphery_transfer_fund: spi_on_rtn_periphery_transfer_fund,
            on_rsp_inquiry_jz_fund: spi_on_rsp_inquiry_jz_fund,
            on_rsp_inquiry_bank_account_fund: spi_on_rsp_inquiry_bank_account_fund,
            on_rtn_trading_notice: spi_on_rtn_trading_notice,
            on_rsp_inquiry_max_order_volume: spi_on_rsp_inquiry_max_order_volume,
            on_rsp_inquiry_trade_concentration: spi_on_rsp_inquiry_trade_concentration,
            on_rsp_modify_open_pos_cost: spi_on_rsp_modify_open_pos_cost,
            on_rsp_input_node_fund_assignment: spi_on_rsp_input_node_fund_assignment,
            on_rsp_inquiry_node_fund_assignment: spi_on_rsp_inquiry_node_fund_assignment,
            on_rsp_qry_exchange: spi_on_rsp_qry_exchange,
            on_rsp_qry_security: spi_on_rsp_qry_security,
            on_rsp_qry_ipo_info: spi_on_rsp_qry_ipo_info,
            on_rsp_qry_user: spi_on_rsp_qry_user,
            on_rsp_qry_investor: spi_on_rsp_qry_investor,
            on_rsp_qry_shareholder_account: spi_on_rsp_qry_shareholder_account,
            on_rsp_qry_rational_info: spi_on_rsp_qry_rational_info,
            on_rsp_qry_order: spi_on_rsp_qry_order,
            on_rsp_qry_order_action: spi_on_rsp_qry_order_action,
            on_rsp_qry_trade: spi_on_rsp_qry_trade,
            on_rsp_qry_trading_account: spi_on_rsp_qry_trading_account,
            on_rsp_qry_position: spi_on_rsp_qry_position,
            on_rsp_qry_trading_fee: spi_on_rsp_qry_trading_fee,
            on_rsp_qry_investor_trading_fee: spi_on_rsp_qry_investor_trading_fee,
            on_rsp_qry_ipo_quota: spi_on_rsp_qry_ipo_quota,
            on_rsp_qry_order_fund_detail: spi_on_rsp_qry_order_fund_detail,
            on_rsp_qry_fund_transfer_detail: spi_on_rsp_qry_fund_transfer_detail,
            on_rsp_qry_position_transfer_detail: spi_on_rsp_qry_position_transfer_detail,
            on_rsp_qry_periphery_position_transfer_detail: spi_on_rsp_qry_periphery_position_transfer_detail,
            on_rsp_qry_periphery_fund_transfer_detail: spi_on_rsp_qry_periphery_fund_transfer_detail,
            on_rsp_qry_bond_conversion_info: spi_on_rsp_qry_bond_conversion_info,
            on_rsp_qry_bond_putback_info: spi_on_rsp_qry_bond_putback_info,
            on_rsp_qry_investor_cond_order_limit_param: spi_on_rsp_qry_investor_cond_order_limit_param,
            on_rsp_qry_condition_order: spi_on_rsp_qry_condition_order,
            on_rsp_qry_cond_order_action: spi_on_rsp_qry_cond_order_action,
            on_rsp_qry_trading_notice: spi_on_rsp_qry_trading_notice,
            on_rsp_qry_ipo_number_result: spi_on_rsp_qry_ipo_number_result,
            on_rsp_qry_ipo_match_number_result: spi_on_rsp_qry_ipo_match_number_result,
            on_rsp_qry_shareholder_spec_privilege: spi_on_rsp_qry_shareholder_spec_privilege,
            on_rsp_qry_market: spi_on_rsp_qry_market,
            on_rsp_qry_etf_file: spi_on_rsp_qry_etf_file,
            on_rsp_qry_etf_basket: spi_on_rsp_qry_etf_basket,
            on_rsp_qry_investor_position_limit: spi_on_rsp_qry_investor_position_limit,
            on_rsp_qry_szse_imc_params: spi_on_rsp_qry_szse_imc_params,
            on_rsp_qry_szse_imc_exchange_rate: spi_on_rsp_qry_szse_imc_exchange_rate,
            on_rsp_qry_szsehk_price_tick_info: spi_on_rsp_qry_szsehk_price_tick_info,
            on_rsp_qry_lof_fund_info: spi_on_rsp_qry_lof_fund_info,
            on_rsp_qry_pledge_position: spi_on_rsp_qry_pledge_position,
            on_rsp_qry_pledge_info: spi_on_rsp_qry_pledge_info,
            on_rsp_qry_system_node_info: spi_on_rsp_qry_system_node_info,
            on_rsp_qry_standard_bond_position: spi_on_rsp_qry_standard_bond_position,
            on_rsp_qry_prematurity_repo_order: spi_on_rsp_qry_prematurity_repo_order,
            on_rsp_qry_nego_order: spi_on_rsp_qry_nego_order,
            on_rsp_qry_nego_order_action: spi_on_rsp_qry_nego_order_action,
            on_rsp_qry_nego_trade: spi_on_rsp_qry_nego_trade,
            on_rsp_qry_negotiation_param: spi_on_rsp_qry_negotiation_param,
             };
extern "C" fn spi_on_front_connected(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, ) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_front_connected()
                    }
                }extern "C" fn spi_on_front_disconnected(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, n_reason : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_front_disconnected(n_reason)
                    }
                }extern "C" fn spi_on_rsp_error(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_error(p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_get_connection_info(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_connection_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpConnectionInfoField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_get_connection_info(p_connection_info_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_user_login(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_user_login_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspUserLoginField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_user_login(p_rsp_user_login_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_user_logout(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_user_logout_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpUserLogoutField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_user_logout(p_user_logout_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_user_password_update(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_user_password_update_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpUserPasswordUpdateField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_user_password_update(p_user_password_update_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_input_device_serial(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_input_device_serial_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInputDeviceSerialField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_input_device_serial(p_rsp_input_device_serial_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_order_insert(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_order_insert(p_input_order_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_order(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpOrderField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_order(p_order_field.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_order_insert(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_order_insert(p_input_order_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_trade(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_trade_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpTradeField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_trade(p_trade_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_order_action(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_order_action_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderActionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_order_action(p_input_order_action_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_err_rtn_order_action(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_order_action_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderActionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_order_action(p_input_order_action_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_cond_order_insert(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_cond_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_cond_order_insert(p_input_cond_order_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_cond_order(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_condition_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpConditionOrderField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_cond_order(p_condition_order_field.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_cond_order_insert(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_cond_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_cond_order_insert(p_input_cond_order_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_cond_order_action(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_cond_order_action_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderActionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_cond_order_action(p_input_cond_order_action_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_err_rtn_cond_order_action(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_cond_order_action_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderActionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_cond_order_action(p_input_cond_order_action_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_nego_order_insert(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_nego_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_nego_order_insert(p_input_nego_order_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_nego_order(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_nego_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpNegoOrderField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_nego_order(p_nego_order_field.as_ref())
                    }
                }extern "C" fn spi_on_err_rtn_nego_order_insert(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_nego_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_nego_order_insert(p_input_nego_order_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_nego_trade(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_nego_trade_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpNegoTradeField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_nego_trade(p_nego_trade_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_nego_order_action(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_nego_order_action_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderActionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_nego_order_action(p_input_nego_order_action_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_err_rtn_nego_order_action(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_nego_order_action_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderActionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_nego_order_action(p_input_nego_order_action_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_market_status(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_market_status_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpMarketStatusField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_market_status(p_market_status_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_transfer_fund(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_transfer_fund_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferFundField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_transfer_fund(p_input_transfer_fund_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_err_rtn_transfer_fund(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_transfer_fund_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferFundField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_transfer_fund(p_input_transfer_fund_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_transfer_fund(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_transfer_fund_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpTransferFundField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_transfer_fund(p_transfer_fund_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_transfer_position(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_transfer_position_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferPositionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_transfer_position(p_input_transfer_position_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_err_rtn_transfer_position(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_transfer_position_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferPositionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_err_rtn_transfer_position(p_input_transfer_position_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_transfer_position(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_transfer_position_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpTransferPositionField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_transfer_position(p_transfer_position_field.as_ref())
                    }
                }extern "C" fn spi_on_rtn_periphery_transfer_position(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_periphery_transfer_position_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpPeripheryTransferPositionField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_periphery_transfer_position(p_periphery_transfer_position_field.as_ref())
                    }
                }extern "C" fn spi_on_rtn_periphery_transfer_fund(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_periphery_transfer_fund_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpPeripheryTransferFundField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_periphery_transfer_fund(p_periphery_transfer_fund_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_inquiry_jz_fund(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_inquiry_jz_fund_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInquiryJZFundField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_jz_fund(p_rsp_inquiry_jz_fund_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_inquiry_bank_account_fund(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_inquiry_bank_account_fund_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInquiryBankAccountFundField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_bank_account_fund(p_rsp_inquiry_bank_account_fund_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rtn_trading_notice(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_trading_notice_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpTradingNoticeField) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rtn_trading_notice(p_trading_notice_field.as_ref())
                    }
                }extern "C" fn spi_on_rsp_inquiry_max_order_volume(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_inquiry_max_order_volume_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInquiryMaxOrderVolumeField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_max_order_volume(p_rsp_inquiry_max_order_volume_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_inquiry_trade_concentration(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_inquiry_trade_concentration_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInquiryTradeConcentrationField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_trade_concentration(p_inquiry_trade_concentration_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_modify_open_pos_cost(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_req_modify_open_pos_cost_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpReqModifyOpenPosCostField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_modify_open_pos_cost(p_req_modify_open_pos_cost_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_input_node_fund_assignment(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_input_node_fund_assignment_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInputNodeFundAssignmentField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_input_node_fund_assignment(p_input_node_fund_assignment_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_inquiry_node_fund_assignment(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_rsp_inquiry_node_fund_assignment_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInquiryNodeFundAssignmentField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_inquiry_node_fund_assignment(p_rsp_inquiry_node_fund_assignment_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id)
                    }
                }extern "C" fn spi_on_rsp_qry_exchange(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_exchange_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpExchangeField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_exchange(p_exchange_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_security(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_security_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpSecurityField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_security(p_security_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_ipo_info(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_ipo_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpIPOInfoField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_ipo_info(p_ipo_info_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_user(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_user_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpUserField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_user(p_user_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_investor_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInvestorField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor(p_investor_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_shareholder_account(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_shareholder_account_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpShareholderAccountField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_shareholder_account(p_shareholder_account_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_rational_info(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_rational_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRationalInfoField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_rational_info(p_rational_info_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_order(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpOrderField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_order(p_order_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_order_action(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_order_action_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpOrderActionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_order_action(p_order_action_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_trade(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_trade_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpTradeField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_trade(p_trade_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_trading_account(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_trading_account_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpTradingAccountField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_trading_account(p_trading_account_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_position(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_position_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpPositionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_position(p_position_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_trading_fee(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_trading_fee_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpTradingFeeField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_trading_fee(p_trading_fee_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_trading_fee(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_investor_trading_fee_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInvestorTradingFeeField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_trading_fee(p_investor_trading_fee_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_ipo_quota(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_ipo_quota_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpIPOQuotaField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_ipo_quota(p_ipo_quota_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_order_fund_detail(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_order_fund_detail_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpOrderFundDetailField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_order_fund_detail(p_order_fund_detail_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_fund_transfer_detail(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_fund_transfer_detail_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpFundTransferDetailField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_fund_transfer_detail(p_fund_transfer_detail_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_position_transfer_detail(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_position_transfer_detail_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpPositionTransferDetailField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_position_transfer_detail(p_position_transfer_detail_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_periphery_position_transfer_detail(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_periphery_position_transfer_detail_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpPeripheryPositionTransferDetailField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_periphery_position_transfer_detail(p_periphery_position_transfer_detail_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_periphery_fund_transfer_detail(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_periphery_fund_transfer_detail_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpPeripheryFundTransferDetailField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_periphery_fund_transfer_detail(p_periphery_fund_transfer_detail_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_bond_conversion_info(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_bond_conversion_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpBondConversionInfoField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_bond_conversion_info(p_bond_conversion_info_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_bond_putback_info(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_bond_putback_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpBondPutbackInfoField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_bond_putback_info(p_bond_putback_info_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_cond_order_limit_param(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_investor_cond_order_limit_param_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInvestorCondOrderLimitParamField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_cond_order_limit_param(p_investor_cond_order_limit_param_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_condition_order(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_condition_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpConditionOrderField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_condition_order(p_condition_order_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_cond_order_action(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_cond_order_action_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpCondOrderActionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_cond_order_action(p_cond_order_action_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_trading_notice(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_trading_notice_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpTradingNoticeField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_trading_notice(p_trading_notice_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_ipo_number_result(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_ipo_number_result_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpIPONumberResultField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_ipo_number_result(p_ipo_number_result_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_ipo_match_number_result(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_ipo_match_number_result_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpIPOMatchNumberResultField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_ipo_match_number_result(p_ipo_match_number_result_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_shareholder_spec_privilege(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_shareholder_spec_privilege_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpShareholderSpecPrivilegeField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_shareholder_spec_privilege(p_shareholder_spec_privilege_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_market(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_market_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpMarketField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_market(p_market_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_etf_file(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_etf_file_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpETFFileField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_etf_file(p_etf_file_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_etf_basket(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_etf_basket_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpETFBasketField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_etf_basket(p_etf_basket_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_investor_position_limit(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_investor_position_limit_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpInvestorPositionLimitField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_investor_position_limit(p_investor_position_limit_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_szse_imc_params(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_szse_imc_params_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpSZSEImcParamsField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_szse_imc_params(p_szse_imc_params_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_szse_imc_exchange_rate(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_szse_imc_exchange_rate_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpSZSEImcExchangeRateField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_szse_imc_exchange_rate(p_szse_imc_exchange_rate_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_szsehk_price_tick_info(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_szsehk_price_tick_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpSZSEHKPriceTickInfoField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_szsehk_price_tick_info(p_szsehk_price_tick_info_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_lof_fund_info(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_lof_fund_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpLofFundInfoField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_lof_fund_info(p_lof_fund_info_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_pledge_position(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_pledge_position_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpPledgePositionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_pledge_position(p_pledge_position_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_pledge_info(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_pledge_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpPledgeInfoField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_pledge_info(p_pledge_info_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_system_node_info(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_system_node_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpSystemNodeInfoField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_system_node_info(p_system_node_info_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_standard_bond_position(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_standard_bond_position_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpStandardBondPositionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_standard_bond_position(p_standard_bond_position_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_prematurity_repo_order(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_prematurity_repo_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpPrematurityRepoOrderField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_prematurity_repo_order(p_prematurity_repo_order_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_nego_order(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_nego_order_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpNegoOrderField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_nego_order(p_nego_order_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_nego_order_action(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_nego_order_action_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpNegoOrderActionField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_nego_order_action(p_nego_order_action_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_nego_trade(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_nego_trade_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpNegoTradeField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_nego_trade(p_nego_trade_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }extern "C" fn spi_on_rsp_qry_negotiation_param(spi: *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat, p_negotiation_param_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpNegotiationParamField,p_rsp_info_field : * const wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField,n_request_id : std::os::raw::c_int,b_is_last : bool) {
                    unsafe {
                        (*(*spi).md_spi_ptr).on_rsp_qry_negotiation_param(p_negotiation_param_field.as_ref(),p_rsp_info_field.as_ref(),n_request_id,b_is_last)
                    }
                }

        #[repr(C)]
        pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiFat {
            vtable: *const wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiVTable,
            pub md_spi_ptr: *mut dyn wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpi_trait,
        }
        

        use futures::stream::Stream;
        use std::{
            pin::Pin,
            sync::{Arc, Mutex},
            task::Waker,
        };
        
        struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiInner {
            buf: std::collections::VecDeque<wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput>,
            waker: Option<Waker>,
        }
        
        impl wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiInner {
            fn push(&mut self, msg: wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput) {
                self.buf.push_back(msg);
                if let Some(ref waker) = &self.waker {
                    waker.clone().wake()
                }
            }
        }
        
        pub struct wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiStream {
            inner: Arc<Mutex<wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiInner>>,
        }
        
        impl Stream for wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiStream {
            type Item = wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput;
        
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
        
        pub fn create_spi() -> (Box<wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiStream>, *mut wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiStream) {
            let i = wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiInner {
                buf: std::collections::VecDeque::new(),
                waker: None,
            };
            let xspi = wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiStream {
                inner: Arc::new(Mutex::new(i)),
            };
            let myspi = Box::new(xspi);
            let pp = Box::into_raw(myspi);
            let pp2 = pp.clone();
            (unsafe { Box::from_raw(pp2) }, pp)
        }
        
impl wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpi_trait for wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiStream {fn on_front_connected(&mut self, ) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnFrontConnected( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnFrontConnectedPacket {  } ))
                }
            fn on_front_disconnected(&mut self, n_reason : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnFrontDisconnected( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnFrontDisconnectedPacket { n_reason:n_reason } ))
                }
            fn on_rsp_error(&mut self, p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspError( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspErrorPacket { p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_get_connection_info(&mut self, p_connection_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpConnectionInfoField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspGetConnectionInfo( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspGetConnectionInfoPacket { p_connection_info_field:p_connection_info_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_user_login(&mut self, p_rsp_user_login_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspUserLoginField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspUserLogin( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspUserLoginPacket { p_rsp_user_login_field:p_rsp_user_login_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_user_logout(&mut self, p_user_logout_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpUserLogoutField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspUserLogout( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspUserLogoutPacket { p_user_logout_field:p_user_logout_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_user_password_update(&mut self, p_user_password_update_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpUserPasswordUpdateField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspUserPasswordUpdate( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspUserPasswordUpdatePacket { p_user_password_update_field:p_user_password_update_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_input_device_serial(&mut self, p_rsp_input_device_serial_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInputDeviceSerialField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspInputDeviceSerial( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspInputDeviceSerialPacket { p_rsp_input_device_serial_field:p_rsp_input_device_serial_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_order_insert(&mut self, p_input_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspOrderInsert( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspOrderInsertPacket { p_input_order_field:p_input_order_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_order(&mut self, p_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpOrderField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRtnOrder( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnOrderPacket { p_order_field:p_order_field.cloned() } ))
                }
            fn on_err_rtn_order_insert(&mut self, p_input_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnErrRtnOrderInsert( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnOrderInsertPacket { p_input_order_field:p_input_order_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_trade(&mut self, p_trade_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpTradeField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRtnTrade( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnTradePacket { p_trade_field:p_trade_field.cloned() } ))
                }
            fn on_rsp_order_action(&mut self, p_input_order_action_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderActionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspOrderAction( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspOrderActionPacket { p_input_order_action_field:p_input_order_action_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_err_rtn_order_action(&mut self, p_input_order_action_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputOrderActionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnErrRtnOrderAction( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnOrderActionPacket { p_input_order_action_field:p_input_order_action_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_cond_order_insert(&mut self, p_input_cond_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspCondOrderInsert( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspCondOrderInsertPacket { p_input_cond_order_field:p_input_cond_order_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_cond_order(&mut self, p_condition_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpConditionOrderField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRtnCondOrder( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnCondOrderPacket { p_condition_order_field:p_condition_order_field.cloned() } ))
                }
            fn on_err_rtn_cond_order_insert(&mut self, p_input_cond_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnErrRtnCondOrderInsert( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnCondOrderInsertPacket { p_input_cond_order_field:p_input_cond_order_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_cond_order_action(&mut self, p_input_cond_order_action_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderActionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspCondOrderAction( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspCondOrderActionPacket { p_input_cond_order_action_field:p_input_cond_order_action_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_err_rtn_cond_order_action(&mut self, p_input_cond_order_action_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputCondOrderActionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnErrRtnCondOrderAction( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnCondOrderActionPacket { p_input_cond_order_action_field:p_input_cond_order_action_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_nego_order_insert(&mut self, p_input_nego_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspNegoOrderInsert( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspNegoOrderInsertPacket { p_input_nego_order_field:p_input_nego_order_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_nego_order(&mut self, p_nego_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpNegoOrderField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRtnNegoOrder( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnNegoOrderPacket { p_nego_order_field:p_nego_order_field.cloned() } ))
                }
            fn on_err_rtn_nego_order_insert(&mut self, p_input_nego_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnErrRtnNegoOrderInsert( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnNegoOrderInsertPacket { p_input_nego_order_field:p_input_nego_order_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_nego_trade(&mut self, p_nego_trade_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpNegoTradeField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRtnNegoTrade( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnNegoTradePacket { p_nego_trade_field:p_nego_trade_field.cloned() } ))
                }
            fn on_rsp_nego_order_action(&mut self, p_input_nego_order_action_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderActionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspNegoOrderAction( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspNegoOrderActionPacket { p_input_nego_order_action_field:p_input_nego_order_action_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_err_rtn_nego_order_action(&mut self, p_input_nego_order_action_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputNegoOrderActionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnErrRtnNegoOrderAction( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnNegoOrderActionPacket { p_input_nego_order_action_field:p_input_nego_order_action_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_market_status(&mut self, p_market_status_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpMarketStatusField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRtnMarketStatus( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnMarketStatusPacket { p_market_status_field:p_market_status_field.cloned() } ))
                }
            fn on_rsp_transfer_fund(&mut self, p_input_transfer_fund_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferFundField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspTransferFund( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspTransferFundPacket { p_input_transfer_fund_field:p_input_transfer_fund_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_err_rtn_transfer_fund(&mut self, p_input_transfer_fund_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferFundField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnErrRtnTransferFund( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnTransferFundPacket { p_input_transfer_fund_field:p_input_transfer_fund_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_transfer_fund(&mut self, p_transfer_fund_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpTransferFundField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRtnTransferFund( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnTransferFundPacket { p_transfer_fund_field:p_transfer_fund_field.cloned() } ))
                }
            fn on_rsp_transfer_position(&mut self, p_input_transfer_position_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferPositionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspTransferPosition( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspTransferPositionPacket { p_input_transfer_position_field:p_input_transfer_position_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_err_rtn_transfer_position(&mut self, p_input_transfer_position_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputTransferPositionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnErrRtnTransferPosition( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnErrRtnTransferPositionPacket { p_input_transfer_position_field:p_input_transfer_position_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_transfer_position(&mut self, p_transfer_position_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpTransferPositionField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRtnTransferPosition( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnTransferPositionPacket { p_transfer_position_field:p_transfer_position_field.cloned() } ))
                }
            fn on_rtn_periphery_transfer_position(&mut self, p_periphery_transfer_position_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpPeripheryTransferPositionField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRtnPeripheryTransferPosition( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnPeripheryTransferPositionPacket { p_periphery_transfer_position_field:p_periphery_transfer_position_field.cloned() } ))
                }
            fn on_rtn_periphery_transfer_fund(&mut self, p_periphery_transfer_fund_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpPeripheryTransferFundField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRtnPeripheryTransferFund( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnPeripheryTransferFundPacket { p_periphery_transfer_fund_field:p_periphery_transfer_fund_field.cloned() } ))
                }
            fn on_rsp_inquiry_jz_fund(&mut self, p_rsp_inquiry_jz_fund_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInquiryJZFundField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspInquiryJZFund( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryJZFundPacket { p_rsp_inquiry_jz_fund_field:p_rsp_inquiry_jz_fund_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_inquiry_bank_account_fund(&mut self, p_rsp_inquiry_bank_account_fund_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInquiryBankAccountFundField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspInquiryBankAccountFund( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryBankAccountFundPacket { p_rsp_inquiry_bank_account_fund_field:p_rsp_inquiry_bank_account_fund_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rtn_trading_notice(&mut self, p_trading_notice_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpTradingNoticeField>) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRtnTradingNotice( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRtnTradingNoticePacket { p_trading_notice_field:p_trading_notice_field.cloned() } ))
                }
            fn on_rsp_inquiry_max_order_volume(&mut self, p_rsp_inquiry_max_order_volume_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInquiryMaxOrderVolumeField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspInquiryMaxOrderVolume( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryMaxOrderVolumePacket { p_rsp_inquiry_max_order_volume_field:p_rsp_inquiry_max_order_volume_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_inquiry_trade_concentration(&mut self, p_inquiry_trade_concentration_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInquiryTradeConcentrationField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspInquiryTradeConcentration( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryTradeConcentrationPacket { p_inquiry_trade_concentration_field:p_inquiry_trade_concentration_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_modify_open_pos_cost(&mut self, p_req_modify_open_pos_cost_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpReqModifyOpenPosCostField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspModifyOpenPosCost( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspModifyOpenPosCostPacket { p_req_modify_open_pos_cost_field:p_req_modify_open_pos_cost_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_input_node_fund_assignment(&mut self, p_input_node_fund_assignment_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInputNodeFundAssignmentField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspInputNodeFundAssignment( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspInputNodeFundAssignmentPacket { p_input_node_fund_assignment_field:p_input_node_fund_assignment_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_inquiry_node_fund_assignment(&mut self, p_rsp_inquiry_node_fund_assignment_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInquiryNodeFundAssignmentField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspInquiryNodeFundAssignment( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspInquiryNodeFundAssignmentPacket { p_rsp_inquiry_node_fund_assignment_field:p_rsp_inquiry_node_fund_assignment_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id } ))
                }
            fn on_rsp_qry_exchange(&mut self, p_exchange_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpExchangeField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryExchange( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryExchangePacket { p_exchange_field:p_exchange_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_security(&mut self, p_security_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpSecurityField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQrySecurity( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySecurityPacket { p_security_field:p_security_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_ipo_info(&mut self, p_ipo_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpIPOInfoField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryIPOInfo( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPOInfoPacket { p_ipo_info_field:p_ipo_info_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_user(&mut self, p_user_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpUserField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryUser( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryUserPacket { p_user_field:p_user_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor(&mut self, p_investor_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInvestorField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryInvestor( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorPacket { p_investor_field:p_investor_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_shareholder_account(&mut self, p_shareholder_account_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpShareholderAccountField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryShareholderAccount( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryShareholderAccountPacket { p_shareholder_account_field:p_shareholder_account_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_rational_info(&mut self, p_rational_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRationalInfoField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryRationalInfo( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryRationalInfoPacket { p_rational_info_field:p_rational_info_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_order(&mut self, p_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpOrderField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryOrder( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryOrderPacket { p_order_field:p_order_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_order_action(&mut self, p_order_action_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpOrderActionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryOrderAction( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryOrderActionPacket { p_order_action_field:p_order_action_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_trade(&mut self, p_trade_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpTradeField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryTrade( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradePacket { p_trade_field:p_trade_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_trading_account(&mut self, p_trading_account_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpTradingAccountField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryTradingAccount( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradingAccountPacket { p_trading_account_field:p_trading_account_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_position(&mut self, p_position_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpPositionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryPosition( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPositionPacket { p_position_field:p_position_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_trading_fee(&mut self, p_trading_fee_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpTradingFeeField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryTradingFee( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradingFeePacket { p_trading_fee_field:p_trading_fee_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_trading_fee(&mut self, p_investor_trading_fee_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInvestorTradingFeeField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryInvestorTradingFee( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorTradingFeePacket { p_investor_trading_fee_field:p_investor_trading_fee_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_ipo_quota(&mut self, p_ipo_quota_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpIPOQuotaField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryIPOQuota( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPOQuotaPacket { p_ipo_quota_field:p_ipo_quota_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_order_fund_detail(&mut self, p_order_fund_detail_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpOrderFundDetailField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryOrderFundDetail( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryOrderFundDetailPacket { p_order_fund_detail_field:p_order_fund_detail_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_fund_transfer_detail(&mut self, p_fund_transfer_detail_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpFundTransferDetailField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryFundTransferDetail( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryFundTransferDetailPacket { p_fund_transfer_detail_field:p_fund_transfer_detail_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_position_transfer_detail(&mut self, p_position_transfer_detail_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpPositionTransferDetailField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryPositionTransferDetail( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPositionTransferDetailPacket { p_position_transfer_detail_field:p_position_transfer_detail_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_periphery_position_transfer_detail(&mut self, p_periphery_position_transfer_detail_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpPeripheryPositionTransferDetailField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryPeripheryPositionTransferDetail( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPeripheryPositionTransferDetailPacket { p_periphery_position_transfer_detail_field:p_periphery_position_transfer_detail_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_periphery_fund_transfer_detail(&mut self, p_periphery_fund_transfer_detail_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpPeripheryFundTransferDetailField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryPeripheryFundTransferDetail( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPeripheryFundTransferDetailPacket { p_periphery_fund_transfer_detail_field:p_periphery_fund_transfer_detail_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_bond_conversion_info(&mut self, p_bond_conversion_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpBondConversionInfoField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryBondConversionInfo( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryBondConversionInfoPacket { p_bond_conversion_info_field:p_bond_conversion_info_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_bond_putback_info(&mut self, p_bond_putback_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpBondPutbackInfoField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryBondPutbackInfo( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryBondPutbackInfoPacket { p_bond_putback_info_field:p_bond_putback_info_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_cond_order_limit_param(&mut self, p_investor_cond_order_limit_param_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInvestorCondOrderLimitParamField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryInvestorCondOrderLimitParam( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorCondOrderLimitParamPacket { p_investor_cond_order_limit_param_field:p_investor_cond_order_limit_param_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_condition_order(&mut self, p_condition_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpConditionOrderField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryConditionOrder( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryConditionOrderPacket { p_condition_order_field:p_condition_order_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_cond_order_action(&mut self, p_cond_order_action_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpCondOrderActionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryCondOrderAction( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryCondOrderActionPacket { p_cond_order_action_field:p_cond_order_action_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_trading_notice(&mut self, p_trading_notice_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpTradingNoticeField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryTradingNotice( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryTradingNoticePacket { p_trading_notice_field:p_trading_notice_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_ipo_number_result(&mut self, p_ipo_number_result_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpIPONumberResultField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryIPONumberResult( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPONumberResultPacket { p_ipo_number_result_field:p_ipo_number_result_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_ipo_match_number_result(&mut self, p_ipo_match_number_result_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpIPOMatchNumberResultField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryIPOMatchNumberResult( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryIPOMatchNumberResultPacket { p_ipo_match_number_result_field:p_ipo_match_number_result_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_shareholder_spec_privilege(&mut self, p_shareholder_spec_privilege_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpShareholderSpecPrivilegeField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryShareholderSpecPrivilege( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryShareholderSpecPrivilegePacket { p_shareholder_spec_privilege_field:p_shareholder_spec_privilege_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_market(&mut self, p_market_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpMarketField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryMarket( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryMarketPacket { p_market_field:p_market_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_etf_file(&mut self, p_etf_file_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpETFFileField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryETFFile( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryETFFilePacket { p_etf_file_field:p_etf_file_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_etf_basket(&mut self, p_etf_basket_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpETFBasketField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryETFBasket( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryETFBasketPacket { p_etf_basket_field:p_etf_basket_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_investor_position_limit(&mut self, p_investor_position_limit_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpInvestorPositionLimitField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryInvestorPositionLimit( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryInvestorPositionLimitPacket { p_investor_position_limit_field:p_investor_position_limit_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_szse_imc_params(&mut self, p_szse_imc_params_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpSZSEImcParamsField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQrySZSEImcParams( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySZSEImcParamsPacket { p_szse_imc_params_field:p_szse_imc_params_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_szse_imc_exchange_rate(&mut self, p_szse_imc_exchange_rate_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpSZSEImcExchangeRateField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQrySZSEImcExchangeRate( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySZSEImcExchangeRatePacket { p_szse_imc_exchange_rate_field:p_szse_imc_exchange_rate_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_szsehk_price_tick_info(&mut self, p_szsehk_price_tick_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpSZSEHKPriceTickInfoField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQrySZSEHKPriceTickInfo( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySZSEHKPriceTickInfoPacket { p_szsehk_price_tick_info_field:p_szsehk_price_tick_info_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_lof_fund_info(&mut self, p_lof_fund_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpLofFundInfoField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryLofFundInfo( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryLofFundInfoPacket { p_lof_fund_info_field:p_lof_fund_info_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_pledge_position(&mut self, p_pledge_position_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpPledgePositionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryPledgePosition( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPledgePositionPacket { p_pledge_position_field:p_pledge_position_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_pledge_info(&mut self, p_pledge_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpPledgeInfoField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryPledgeInfo( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPledgeInfoPacket { p_pledge_info_field:p_pledge_info_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_system_node_info(&mut self, p_system_node_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpSystemNodeInfoField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQrySystemNodeInfo( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQrySystemNodeInfoPacket { p_system_node_info_field:p_system_node_info_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_standard_bond_position(&mut self, p_standard_bond_position_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpStandardBondPositionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryStandardBondPosition( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryStandardBondPositionPacket { p_standard_bond_position_field:p_standard_bond_position_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_prematurity_repo_order(&mut self, p_prematurity_repo_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpPrematurityRepoOrderField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryPrematurityRepoOrder( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryPrematurityRepoOrderPacket { p_prematurity_repo_order_field:p_prematurity_repo_order_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_nego_order(&mut self, p_nego_order_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpNegoOrderField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryNegoOrder( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegoOrderPacket { p_nego_order_field:p_nego_order_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_nego_order_action(&mut self, p_nego_order_action_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpNegoOrderActionField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryNegoOrderAction( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegoOrderActionPacket { p_nego_order_action_field:p_nego_order_action_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_nego_trade(&mut self, p_nego_trade_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpNegoTradeField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryNegoTrade( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegoTradePacket { p_nego_trade_field:p_nego_trade_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
            fn on_rsp_qry_negotiation_param(&mut self, p_negotiation_param_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpNegotiationParamField>,p_rsp_info_field : Option<&wrapper.hpp_TORASTOCKAPI_CTORATstpRspInfoField>,n_request_id : std::os::raw::c_int,b_is_last : bool) 
 {
            self.inner.lock().unwrap().push(wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOutput::OnRspQryNegotiationParam( wrapper.hpp_TORASTOCKAPI_CTORATstpTraderSpiOnRspQryNegotiationParamPacket { p_negotiation_param_field:p_negotiation_param_field.cloned(),p_rsp_info_field:p_rsp_info_field.cloned(),n_request_id:n_request_id,b_is_last:b_is_last } ))
                }
             }
