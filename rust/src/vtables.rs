//! Evidence-labelled vftable metadata and raw slot access.
//! Slot calls remain the consumer's responsibility until a signature is unique.

use core::ffi::c_void;

#[derive(Clone, Copy, Debug)]
pub struct VTableInfo { pub name: &'static str, pub rva: u32, pub first_slot: usize, pub slot_count: usize, pub confidence: &'static str }

#[derive(Clone, Copy, Debug)]
pub struct VTableSlot { pub table_rva: u32, pub slot: u32, pub byte_offset: u32, pub target_rva: u32, pub target_id: Option<&'static str>, pub target_name: Option<&'static str>, pub ambiguous: bool, pub this_adjustment: Option<i32> }

pub static VTABLES: &[VTableInfo] = &[
    VTableInfo { name: "const CCancelConnection2::`vftable'", rva: 0x15000, first_slot: 0, slot_count: 5, confidence: "likely" },
    VTableInfo { name: "const CGetResourceInformation::`vftable'", rva: 0x15028, first_slot: 5, slot_count: 5, confidence: "likely" },
    VTableInfo { name: "const CGetConnection3::`vftable'", rva: 0x15050, first_slot: 10, slot_count: 5, confidence: "likely" },
    VTableInfo { name: "const CUseConnection::`vftable'", rva: 0x15078, first_slot: 15, slot_count: 5, confidence: "likely" },
    VTableInfo { name: "const CGetConnection::`vftable'", rva: 0x150A0, first_slot: 20, slot_count: 5, confidence: "likely" },
    VTableInfo { name: "const CGetConnectionPerformance::`vftable'", rva: 0x150C8, first_slot: 25, slot_count: 5, confidence: "likely" },
    VTableInfo { name: "const CProviderOpenEnum::`vftable'", rva: 0x150F0, first_slot: 30, slot_count: 5, confidence: "likely" },
    VTableInfo { name: "const CGetResourceParent::`vftable'", rva: 0x15138, first_slot: 35, slot_count: 5, confidence: "likely" },
];

pub static VTABLE_SLOTS: &[VTableSlot] = &[
    VTableSlot { table_rva: 0x15000, slot: 0, byte_offset: 0, target_rva: 0x6300, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x15000, slot: 1, byte_offset: 8, target_rva: 0x82F0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x15000, slot: 2, byte_offset: 16, target_rva: 0x9CC0, target_id: Some("?GetResult@CCancelConnection2@@MEAAKXZ"), target_name: Some("protected: virtual unsigned long __cdecl CCancelConnection2::GetResult(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x15000, slot: 3, byte_offset: 24, target_rva: 0xBE00, target_id: Some("?ValidateRoutedParameters@CCancelConnection2@@MEAAKPEAPEBG00@Z"), target_name: Some("protected: virtual unsigned long __cdecl CCancelConnection2::ValidateRoutedParameters(unsigned short const * *, unsigned short const * *, unsigned short const * *)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x15000, slot: 4, byte_offset: 32, target_rva: 0xD630, target_id: Some("?TestProvider@CCancelConnection2@@MEAAKPEBU_PROVIDER@@@Z"), target_name: Some("protected: virtual unsigned long __cdecl CCancelConnection2::TestProvider(struct _PROVIDER const *)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x15028, slot: 0, byte_offset: 0, target_rva: 0x6300, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x15028, slot: 1, byte_offset: 8, target_rva: 0x82F0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x15028, slot: 2, byte_offset: 16, target_rva: 0xA1D0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x15028, slot: 3, byte_offset: 24, target_rva: 0xBF90, target_id: Some("?ValidateRoutedParameters@CGetResourceInformation@@MEAAKPEAPEBG00@Z"), target_name: Some("protected: virtual unsigned long __cdecl CGetResourceInformation::ValidateRoutedParameters(unsigned short const * *, unsigned short const * *, unsigned short const * *)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x15028, slot: 4, byte_offset: 32, target_rva: 0xBDD0, target_id: Some("?TestProvider@CGetResourceInformation@@MEAAKPEBU_PROVIDER@@@Z"), target_name: Some("protected: virtual unsigned long __cdecl CGetResourceInformation::TestProvider(struct _PROVIDER const *)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x15050, slot: 0, byte_offset: 0, target_rva: 0x6300, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x15050, slot: 1, byte_offset: 8, target_rva: 0x82F0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x15050, slot: 2, byte_offset: 16, target_rva: 0xA1D0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x15050, slot: 3, byte_offset: 24, target_rva: 0xB190, target_id: Some("?ValidateRoutedParameters@CGetConnection3@@MEAAKPEAPEBG00@Z"), target_name: Some("protected: virtual unsigned long __cdecl CGetConnection3::ValidateRoutedParameters(unsigned short const * *, unsigned short const * *, unsigned short const * *)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x15050, slot: 4, byte_offset: 32, target_rva: 0xBA50, target_id: Some("?TestProvider@CGetConnection3@@MEAAKPEBU_PROVIDER@@@Z"), target_name: Some("protected: virtual unsigned long __cdecl CGetConnection3::TestProvider(struct _PROVIDER const *)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x15078, slot: 0, byte_offset: 0, target_rva: 0x6300, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x15078, slot: 1, byte_offset: 8, target_rva: 0x82F0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x15078, slot: 2, byte_offset: 16, target_rva: 0x96D0, target_id: Some("?GetResult@CUseConnection@@MEAAKXZ"), target_name: Some("protected: virtual unsigned long __cdecl CUseConnection::GetResult(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x15078, slot: 3, byte_offset: 24, target_rva: 0xD890, target_id: Some("?ValidateRoutedParameters@CUseConnection@@MEAAKPEAPEBG00@Z"), target_name: Some("protected: virtual unsigned long __cdecl CUseConnection::ValidateRoutedParameters(unsigned short const * *, unsigned short const * *, unsigned short const * *)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x15078, slot: 4, byte_offset: 32, target_rva: 0xEF50, target_id: Some("?TestProvider@CUseConnection@@MEAAKPEBU_PROVIDER@@@Z"), target_name: Some("protected: virtual unsigned long __cdecl CUseConnection::TestProvider(struct _PROVIDER const *)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x150A0, slot: 0, byte_offset: 0, target_rva: 0x6300, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x150A0, slot: 1, byte_offset: 8, target_rva: 0x82F0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x150A0, slot: 2, byte_offset: 16, target_rva: 0xA1D0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x150A0, slot: 3, byte_offset: 24, target_rva: 0xB130, target_id: Some("?ValidateRoutedParameters@CGetConnection@@MEAAKPEAPEBG00@Z"), target_name: Some("protected: virtual unsigned long __cdecl CGetConnection::ValidateRoutedParameters(unsigned short const * *, unsigned short const * *, unsigned short const * *)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x150A0, slot: 4, byte_offset: 32, target_rva: 0xDCC0, target_id: Some("?TestProvider@CGetConnection@@MEAAKPEBU_PROVIDER@@@Z"), target_name: Some("protected: virtual unsigned long __cdecl CGetConnection::TestProvider(struct _PROVIDER const *)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x150C8, slot: 0, byte_offset: 0, target_rva: 0x6300, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x150C8, slot: 1, byte_offset: 8, target_rva: 0x82F0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x150C8, slot: 2, byte_offset: 16, target_rva: 0xA1D0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x150C8, slot: 3, byte_offset: 24, target_rva: 0xD450, target_id: Some("?ValidateRoutedParameters@CGetConnectionPerformance@@MEAAKPEAPEBG00@Z"), target_name: Some("protected: virtual unsigned long __cdecl CGetConnectionPerformance::ValidateRoutedParameters(unsigned short const * *, unsigned short const * *, unsigned short const * *)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x150C8, slot: 4, byte_offset: 32, target_rva: 0xDD30, target_id: Some("?TestProvider@CGetConnectionPerformance@@MEAAKPEBU_PROVIDER@@@Z"), target_name: Some("protected: virtual unsigned long __cdecl CGetConnectionPerformance::TestProvider(struct _PROVIDER const *)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x150F0, slot: 0, byte_offset: 0, target_rva: 0x6300, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x150F0, slot: 1, byte_offset: 8, target_rva: 0x82F0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x150F0, slot: 2, byte_offset: 16, target_rva: 0xA0D0, target_id: Some("?GetResult@CProviderOpenEnum@@MEAAKXZ"), target_name: Some("protected: virtual unsigned long __cdecl CProviderOpenEnum::GetResult(void)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x150F0, slot: 3, byte_offset: 24, target_rva: 0xD670, target_id: Some("?ValidateRoutedParameters@CProviderOpenEnum@@MEAAKPEAPEBG00@Z"), target_name: Some("protected: virtual unsigned long __cdecl CProviderOpenEnum::ValidateRoutedParameters(unsigned short const * *, unsigned short const * *, unsigned short const * *)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x150F0, slot: 4, byte_offset: 32, target_rva: 0xD5C0, target_id: Some("?TestProvider@CProviderOpenEnum@@MEAAKPEBU_PROVIDER@@@Z"), target_name: Some("protected: virtual unsigned long __cdecl CProviderOpenEnum::TestProvider(struct _PROVIDER const *)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x15138, slot: 0, byte_offset: 0, target_rva: 0x6300, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x15138, slot: 1, byte_offset: 8, target_rva: 0x82F0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x15138, slot: 2, byte_offset: 16, target_rva: 0xA1D0, target_id: None, target_name: None, ambiguous: true, this_adjustment: None },
    VTableSlot { table_rva: 0x15138, slot: 3, byte_offset: 24, target_rva: 0xD760, target_id: Some("?ValidateRoutedParameters@CGetResourceParent@@MEAAKPEAPEBG00@Z"), target_name: Some("protected: virtual unsigned long __cdecl CGetResourceParent::ValidateRoutedParameters(unsigned short const * *, unsigned short const * *, unsigned short const * *)"), ambiguous: false, this_adjustment: None },
    VTableSlot { table_rva: 0x15138, slot: 4, byte_offset: 32, target_rva: 0x12C30, target_id: Some("?TestProvider@CGetResourceParent@@MEAAKPEBU_PROVIDER@@@Z"), target_name: Some("protected: virtual unsigned long __cdecl CGetResourceParent::TestProvider(struct _PROVIDER const *)"), ambiguous: false, this_adjustment: None },
];

/// Reads a raw function pointer from an object's primary vftable.
///
/// # Safety
/// `object` must point to a live object with a readable primary vftable,
/// and `slot` must be valid for that concrete object. This function does
/// not invent or transmute a callable signature.
pub unsafe fn raw_object_slot(object: *const c_void, slot: usize) -> Option<*const ()> {
if object.is_null() { return None; }
let table = unsafe { *(object.cast::<*const *const ()>()) };
if table.is_null() { return None; }
let target = unsafe { *table.add(slot) };
(!target.is_null()).then_some(target)
}
