# Blue Book Ed.16 Class ID Mapping

Source: IEC 62056-6-2 (DLMS/COSEM Blue Book) Edition 16

## Core Interface Classes (Section 4.3)

| Class ID | Interface Class | File |
|----------|----------------|------|
| 1 | Data | `data.rs` |
| 3 | Register | `register.rs` |
| 4 | Extended Register | `extended_register.rs` |
| 5 | Demand Register | `demand_register.rs` |
| 6 | Register Activation | `register_activation.rs` |
| 7 | Profile Generic | `profile_generic.rs` |
| 26 | Utility Tables | `utility_tables.rs` |
| 61 | Register Table | `register_table.rs` |
| 62 | Compact Data | `compact_data.rs` |
| 63 | Status Mapping | `status_mapping.rs` |
| 66 | Measurement Data Monitoring | `measurement_data_monitoring.rs` |

## Time & Control (Section 4.5)

| Class ID | Interface Class | File |
|----------|----------------|------|
| 8 | Clock | `clock.rs` |
| 9 | Script Table | `script_table.rs` |
| 10 | Schedule | `schedule.rs` |
| 11 | Special Days Table | `special_day_table.rs` |
| 20 | Activity Calendar | `activity_calendar.rs` |
| 21 | Register Monitor | `register_monitor.rs` |
| 22 | Single Action Schedule | `single_action_schedule.rs` |
| 65 | Parameter Monitor | `parameter_monitor.rs` |
| 67 | Sensor Manager | `sensor_manager.rs` |
| 68 | Arbitrator | `arbitrator.rs` |
| 70 | Disconnect Control | `disconnect_control.rs` |
| 71 | Limiter | `limiter.rs` |

## Access Control (Section 4.4)

| Class ID | Interface Class | File |
|----------|----------------|------|
| 12 | Association SN | `association_sn.rs` |
| 15 | Association LN | `association_ln.rs` |
| 17 | SAP Assignment | `sap_assignment.rs` |
| 18 | Image Transfer | `image_transfer.rs` |
| 30 | COSEM Data Protection | `cosem_data_protection.rs` |
| 64 | Security Setup | `security_setup.rs` |
| 122 | Function Control | `function_control.rs` |
| 123 | Array Manager | `array_manager.rs` |
| 124 | Communication Port Protection | `comm_port_protection.rs` |

## Payment (Section 4.6)

| Class ID | Interface Class | File |
|----------|----------------|------|
| 111 | Account | `account.rs` |
| 112 | Credit | `credit.rs` |
| 113 | Charge | `charge.rs` |
| 115 | Token Gateway | `token_gateway.rs` |

## Local Ports & Modems (Section 4.7)

| Class ID | Interface Class | File |
|----------|----------------|------|
| 19 | IEC Local Port Setup | `iec_local_port_setup.rs` |
| 23 | IEC HDLC Setup | `iec_hdlc_setup.rs` |
| 24 | IEC Twisted Pair Setup | `iec_twisted_pair_setup.rs` |
| 27 | Modem Configuration | `modem_configuration.rs` |
| 28 | Auto Answer | `auto_answer.rs` |
| 29 | Auto Connect | `auto_connect.rs` |
| 45 | GPRS Modem Setup | `gprs_modem_setup.rs` |
| 47 | GSM Diagnostic | `gsm_diagnostic.rs` |
| 151 | LTE Monitoring | `lte_monitoring.rs` |

## M-Bus (Section 4.8)

| Class ID | Interface Class | File |
|----------|----------------|------|
| 25 | M-Bus Slave Port Setup | `mbus_slave_port_setup.rs` |
| 72 | M-Bus Client | `mbus_client.rs` |
| 73 | Wireless Mode Q Channel | `wireless_mode_q_channel.rs` |
| 74 | M-Bus Master Port Setup | `mbus_master_port_setup.rs` |
| 76 | DLMS M-Bus Port Setup | `dlms_mbus_port_setup.rs` |
| 77 | M-Bus Diagnostic | `mbus_diagnostic.rs` |

## Internet Protocols (Section 4.9)

| Class ID | Interface Class | File |
|----------|----------------|------|
| 41 | TCP-UDP Setup | `tcp_udp_setup.rs` |
| 42 | IPv4 Setup | `ipv4_setup.rs` |
| 43 | MAC Address Setup | `mac_address_setup.rs` |
| 44 | PPP Setup | `ppp_setup.rs` |
| 46 | SMTP Setup | `smtp_setup.rs` |
| 48 | IPv6 Setup | `ipv6_setup.rs` |
| 100 | NTP Setup | `ntp_setup.rs` |

## S-FSK PLC (Section 4.10)

| Class ID | Interface Class | File |
|----------|----------------|------|
| 50 | S-FSK Phy MAC Setup | `sfsk_phy_mac_setup.rs` |
| 51 | S-FSK Active Initiator | `sfsk_active_initiator.rs` |
| 52 | S-FSK MAC Sync Timeouts | `sfsk_mac_sync_timeouts.rs` |
| 53 | S-FSK MAC Counters | `sfsk_mac_counters.rs` |
| 55 | IEC 61334 LLC Setup | `iec61334_llc_setup.rs` |
| 56 | S-FSK Reporting System List | `sfsk_reporting_system_list.rs` |

## LLC (Section 4.11)

| Class ID | Interface Class | File |
|----------|----------------|------|
| 57 | LLC Type 1 Setup | `llc_type1_setup.rs` |
| 58 | LLC Type 2 Setup | `llc_type2_setup.rs` |
| 59 | LLC Type 3 Setup | `llc_type3_setup.rs` |

## PRIME PLC (Section 4.12)

| Class ID | Interface Class | File |
|----------|----------------|------|
| 80 | PRIME LLC SCS S Setup | `prime_llc_scs_s_setup.rs` |
| 81 | PRIME Physical Counters | `prime_physical_counters.rs` |
| 82 | PRIME MAC Setup | `prime_mac_setup.rs` |
| 83 | PRIME MAC Function Parameters | `prime_mac_func_params.rs` |
| 84 | PRIME MAC Counters | `prime_mac_counters.rs` |
| 85 | PRIME MAC Network Administration | `prime_mac_network_admin.rs` |
| 86 | PRIME Application Identification | `prime_app_identification.rs` |

## G3-PLC (Section 4.13)

| Class ID | Interface Class | File |
|----------|----------------|------|
| 90 | G3 MAC Layer Counters | `g3_mac_layer_counters.rs` |
| 91 | G3 MAC Setup | `g3_mac_setup.rs` |
| 92 | G3 6LoWPAN Setup | `g3_6lowpan_setup.rs` |
| 160 | G3 Hybrid RF Counters | `g3_hybrid_rf_counters.rs` |
| 161 | G3 Hybrid RF Setup | `g3_hybrid_rf_setup.rs` |
| 162 | G3 Hybrid 6LoWPAN Setup | `g3_hybrid_6lowpan_setup.rs` |

## HS-PLC (Section 4.14)

| Class ID | Interface Class | File |
|----------|----------------|------|
| 140 | HS-PLC MAC Setup | `hsplc_mac_setup.rs` |
| 141 | HS-PLC CPAS Setup | `hsplc_cpas_setup.rs` |
| 142 | HS-PLC IP-S SAS Setup | `hsplc_ipssas_setup.rs` |
| 143 | HS-PLC HDLC-S SAS Setup | `hsplc_hdlcssas_setup.rs` |

## ZigBee (Section 4.15)

| Class ID | Interface Class | File |
|----------|----------------|------|
| 101 | ZigBee SAS Startup | `zigbee_sas_startup.rs` |
| 102 | ZigBee SAS Join | `zigbee_sas_join.rs` |
| 103 | ZigBee SAS APS Fragmentation | `zigbee_sas_aps_fragmentation.rs` |
| 104 | ZigBee Network Control | `zigbee_network_control.rs` |
| 105 | ZigBee Tunnel Setup | `zigbee_tunnel_setup.rs` |

## LPWAN (Section 4.16)

| Class ID | Interface Class | File |
|----------|----------------|------|
| 126 | SCHC LPWAN Setup | `schclpwan_setup.rs` |
| 127 | SCHC LPWAN Diagnostic | `schclpwan_diagnostic.rs` |
| 128 | LoRaWAN Setup | `lorawan_setup.rs` |
| 129 | LoRaWAN Diagnostic | `lorawan_diagnostic.rs` |

## IEC 14908 (Section 4.19)

| Class ID | Interface Class | File |
|----------|----------------|------|
| 130 | IEC 14908 Identification | `iec14908_identification.rs` |
| 131 | IEC 14908 Protocol Setup | `iec14908_protocol_setup.rs` |
| 132 | IEC 14908 Protocol Status | `iec14908_protocol_status.rs` |
| 133 | IEC 14908 Diagnostic | `iec14908_diagnostic.rs` |

## CoAP

| Class ID | Interface Class | File |
|----------|----------------|------|
| 152 | CoAP Setup | `coap_setup.rs` |
| 153 | CoAP Diagnostic | `coap_diagnostic.rs` |
