#!/bin/bash
cd /Users/yimiliya/.openclaw/workspace/dlms-cosem-rust/dlms-cosem/src

rename() {
  if [ -f "$1" ]; then
    if [ -f "$2" ]; then
      echo "SKIP: $2 already exists"
    else
      git mv "$1" "$2"
      echo "RENAMED: $1 -> $2"
    fi
  else
    echo "MISSING: $1"
  fi
}

rename clock.rs C8_Clock.rs
rename register.rs C3_Register.rs
rename extended_register.rs C4_ExtendedRegister.rs
rename demand_register.rs C5_DemandRegister.rs
rename register_activation.rs C6_RegisterActivation.rs
rename profile_generic.rs C7_ProfileGeneric.rs
rename script_table.rs C9_ScriptTable.rs
rename schedule.rs C10_Schedule.rs
rename special_day_table.rs C11_SpecialDaysTable.rs
rename association_sn.rs C12_AssociationSN.rs
rename association_ln.rs C15_AssociationLN.rs
rename disconnect_control.rs C70_DisconnectControl.rs
rename security_setup.rs C64_SecuritySetup.rs
rename activity_calendar.rs C20_ActivityCalendar.rs
rename register_monitor.rs C21_RegisterMonitor.rs
rename single_action_schedule.rs C22_SingleActionSchedule.rs
rename limiter.rs C71_Limiter.rs
rename push_setup.rs C40_PushSetup.rs
rename image_transfer.rs C18_ImageTransfer.rs
rename sap_assignment.rs C17_SAPAssignment.rs
rename tcp_udp_setup.rs C41_TCPUDPSetup.rs
rename ipv4_setup.rs C42_IPv4Setup.rs
rename ppp_setup.rs C44_PPPSetup.rs
rename gprs_modem_setup.rs C45_GPRSModemSetup.rs
rename smtp_setup.rs C46_SMTPSetup.rs
rename ipv6_setup.rs C48_IPv6Setup.rs
rename mac_address_setup.rs C43_MACAddressSetup.rs
rename register_table.rs C61_RegisterTable.rs
rename compact_data.rs C62_CompactData.rs
rename status_mapping.rs C63_StatusMapping.rs
rename parameter_monitor.rs C65_ParameterMonitor.rs
rename sensor_manager.rs C67_SensorManager.rs
rename arbitrator.rs C68_Arbitrator.rs
rename mbus_client.rs C72_MBusClient.rs
rename mbus_diagnostic.rs C77_MBusDiagnostic.rs
rename mbus_master_port_setup.rs C74_MBusMasterPortSetup.rs
rename ntp_setup.rs C100_NTPSetup.rs
rename account.rs C111_Account.rs
rename credit.rs C112_Credit.rs
rename charge.rs C113_Charge.rs
rename token_gateway.rs C115_TokenGateway.rs
rename function_control.rs C122_FunctionControl.rs
rename array_manager.rs C123_ArrayManager.rs
rename comm_port_protection.rs C124_CommPortProtection.rs
rename iec_hdlc_setup.rs C23_IECHDLCSetup.rs
rename modem_configuration.rs C27_ModemConfiguration.rs
rename auto_connect.rs C29_AutoConnect.rs
rename cosem_data_protection.rs C30_COSEMDataProtection.rs
rename measurement_data_monitoring.rs C66_MeasurementDataMonitoring.rs
rename lte_setup.rs C151_LTEMonitoring.rs
rename lorawan_setup.rs C128_LoRaWANSetup.rs
