// Flat C exports observed in MPR.dll. Unknown ABIs are intentionally not declared.
// Validated dialects: ISO C17/C23/C26 draft and Windissect C20 (C ABI under C++20).
#pragma once
#ifndef MPR_C_H
#define MPR_C_H
#if defined(__cplusplus)
#  if __cplusplus < 202002L
#    define WINDISSECT_C20_PROFILE 0
#  else
#    define WINDISSECT_C20_PROFILE 1
#  endif
extern "C" {
#else
#  if defined(__STDC_VERSION__) && __STDC_VERSION__ < 201710L
#    error "Windissect C output requires C17 or newer"
#  endif
#  define WINDISSECT_C20_PROFILE 0
#  define WINDISSECT_C_STANDARD __STDC_VERSION__
#endif

// Export: WNetAddConnectionA (ABI unverified)
// Export: WNetClearConnections (ABI unverified)
// Export: WNetCloseEnum (ABI unverified)
// Export: WNetEnumResourceA (ABI unverified)
// Export: WNetGetUniversalNameA (ABI unverified)
// Export: WNetOpenEnumA (ABI unverified)
// Export: WNetGetConnectionA (ABI unverified)
// Export: WNetEnumResourceW (ABI unverified)
// Export: WNetGetConnectionW (ABI unverified)
// Export: WNetOpenEnumW (ABI unverified)
// Export: WNetGetNetworkInformationW (ABI unverified)
// Export: WNetGetResourceInformationW (ABI unverified)
// Export: WNetGetConnection3W (ABI unverified)
// Export: WNetGetUniversalNameW (ABI unverified)
// Export: WNetFormatNetworkNameW (ABI unverified)
// Export: WNetGetProviderNameW (ABI unverified)
// Export: WNetGetProviderTypeW (ABI unverified)
// Export: WNetUseConnectionW (ABI unverified)
// Export: WNetAddConnection2W (ABI unverified)
// Export: WNetRestoreAllConnectionsW (ABI unverified)
// Export: ShowReconnectDialog (ABI unverified)
// Export: WNetSetLastErrorW (ABI unverified)
// Export: WNetGetUserW (ABI unverified)
// Export: MultinetGetConnectionPerformanceW (ABI unverified)
// Export: WNetCancelConnection2W (ABI unverified)
// Export: WNetAddConnection3W (ABI unverified)
// Export: WNetAddConnection4W (ABI unverified)
// Export: WNetAddConnectionW (ABI unverified)
// Export: WNetCancelConnectionW (ABI unverified)
// Export: WNetGetConnection2W (ABI unverified)
// Export: WNetRestoreSingleConnectionW (ABI unverified)
// Export: WNetSetConnectionW (ABI unverified)
// Export: WNetUseConnection4W (ABI unverified)
// Export: WNetLogonNotify (ABI unverified)
// Export: WNetPasswordChangeNotify (ABI unverified)
// Export: MultinetGetErrorTextW (ABI unverified)
// Export: WNetGetLastErrorW (ABI unverified)
// Export: MultinetGetConnectionPerformanceA (ABI unverified)
// Export: MultinetGetErrorTextA (ABI unverified)
// Export: WNetAddConnection2A (ABI unverified)
// Export: WNetAddConnection3A (ABI unverified)
// Export: WNetAddConnection4A (ABI unverified)
// Export: WNetCancelConnection2A (ABI unverified)
// Export: WNetCancelConnectionA (ABI unverified)
// Export: WNetConnectionDialog1A (ABI unverified)
// Export: WNetDirectoryNotifyA (ABI unverified)
// Export: WNetDisconnectDialog1A (ABI unverified)
// Export: WNetFormatNetworkNameA (ABI unverified)
// Export: WNetGetConnection2A (ABI unverified)
// Export: WNetGetConnection3A (ABI unverified)
// Export: WNetGetDirectoryTypeA (ABI unverified)
// Export: WNetGetLastErrorA (ABI unverified)
// Export: WNetGetNetworkInformationA (ABI unverified)
// Export: WNetGetPropertyTextA (ABI unverified)
// Export: WNetGetProviderNameA (ABI unverified)
// Export: WNetGetProviderTypeA (ABI unverified)
// Export: WNetGetResourceInformationA (ABI unverified)
// Export: WNetGetResourceParentA (ABI unverified)
// Export: WNetGetUserA (ABI unverified)
// Export: WNetPropertyDialogA (ABI unverified)
// Export: WNetSetConnectionA (ABI unverified)
// Export: WNetSetLastErrorA (ABI unverified)
// Export: WNetUseConnection4A (ABI unverified)
// Export: WNetUseConnectionA (ABI unverified)
// Export: WNetGetPropertyTextW (ABI unverified)
// Export: WNetPropertyDialogW (ABI unverified)
// Export: I_MprSaveConn (ABI unverified)
// Export: DoBroadcastSystemMessage (ABI unverified)
// Export: DoCommandLinePrompt (ABI unverified)
// Export: DoPasswordDialog (ABI unverified)
// Export: DoProfileErrorDialog (ABI unverified)
// Export: ShowReconnectDialogEnd (ABI unverified)
// Export: ShowReconnectDialogUI (ABI unverified)
// Export: WNetConnectionDialog2 (ABI unverified)
// Export: WNetDisconnectDialog2 (ABI unverified)
// Export: WNetConnectionDialog (ABI unverified)
// Export: WNetConnectionDialog1W (ABI unverified)
// Export: WNetDisconnectDialog (ABI unverified)
// Export: WNetDisconnectDialog1W (ABI unverified)
// Export: WNetGetSearchDialog (ABI unverified)
// Export: WNetSupportGlobalEnum (ABI unverified)
// Export: WNetGetResourceParentW (ABI unverified)
// Export: WNetDirectoryNotifyW (ABI unverified)
// Export: WNetGetDirectoryTypeW (ABI unverified)
// Export: WNetGetHomeDirectoryW (ABI unverified)

#ifdef __cplusplus
} // extern "C"
#endif
#endif // MPR_C_H
