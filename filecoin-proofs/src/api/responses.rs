use api::errors::SectorBuilderErr;
use api::sector_builder::SectorBuilder;
use api::{API_POREP_PROOF_BYTES, API_POST_PROOF_BYTES};
use failure::Error;
use ffi_toolkit::c_str_to_rust_str;
use ffi_toolkit::FFIResponseStatus;
use libc;
use sector_base::api::errors::SectorManagerErr;
use std::ffi::CString;
use std::mem;
use std::ptr;

///////////////////////////////////////////////////////////////////////////////
/// SealResponse
////////////////

#[repr(C)]
pub struct SealResponse {
    pub status_code: FFIResponseStatus,
    pub error_msg: *const libc::c_char,
    pub comm_d: [u8; 32],
    pub comm_r: [u8; 32],
    pub comm_r_star: [u8; 32],
    pub proof: [u8; API_POREP_PROOF_BYTES],
}

impl Default for SealResponse {
    fn default() -> SealResponse {
        SealResponse {
            status_code: FFIResponseStatus::NoError,
            error_msg: ptr::null(),
            comm_d: [0; 32],
            comm_r: [0; 32],
            comm_r_star: [0; 32],
            proof: [0; API_POREP_PROOF_BYTES],
        }
    }
}

impl Drop for SealResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_seal_response(ptr: *mut SealResponse) {
    let _ = Box::from_raw(ptr);
}

///////////////////////////////////////////////////////////////////////////////
/// VerifySealResponse
//////////////////////

#[repr(C)]
pub struct VerifySealResponse {
    pub status_code: FFIResponseStatus,
    pub error_msg: *const libc::c_char,
    pub is_valid: bool,
}

impl Default for VerifySealResponse {
    fn default() -> VerifySealResponse {
        VerifySealResponse {
            status_code: FFIResponseStatus::NoError,
            error_msg: ptr::null(),
            is_valid: false,
        }
    }
}

impl Drop for VerifySealResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_verify_seal_response(ptr: *mut VerifySealResponse) {
    let _ = Box::from_raw(ptr);
}

///////////////////////////////////////////////////////////////////////////////
/// GetUnsealedRangeResponse
////////////////////////////

#[repr(C)]
pub struct GetUnsealedRangeResponse {
    pub status_code: FFIResponseStatus,
    pub error_msg: *const libc::c_char,
    pub num_bytes_written: u64,
}

impl Default for GetUnsealedRangeResponse {
    fn default() -> GetUnsealedRangeResponse {
        GetUnsealedRangeResponse {
            status_code: FFIResponseStatus::NoError,
            error_msg: ptr::null(),
            num_bytes_written: 0,
        }
    }
}

impl Drop for GetUnsealedRangeResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_get_unsealed_range_response(ptr: *mut GetUnsealedRangeResponse) {
    let _ = Box::from_raw(ptr);
}

///////////////////////////////////////////////////////////////////////////////
/// GetUnsealedResponse
///////////////////////

#[repr(C)]
pub struct GetUnsealedResponse {
    pub status_code: FFIResponseStatus,
    pub error_msg: *const libc::c_char,
}

impl Default for GetUnsealedResponse {
    fn default() -> GetUnsealedResponse {
        GetUnsealedResponse {
            status_code: FFIResponseStatus::NoError,
            error_msg: ptr::null(),
        }
    }
}

impl Drop for GetUnsealedResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_get_unsealed_response(ptr: *mut GetUnsealedResponse) {
    let _ = Box::from_raw(ptr);
}

///////////////////////////////////////////////////////////////////////////////
/// GeneratePoSTResult
//////////////////////

#[repr(C)]
pub struct GeneratePoSTResponse {
    pub status_code: FFIResponseStatus,
    pub error_msg: *const libc::c_char,
    pub faults_len: libc::size_t,
    pub faults_ptr: *const u64,
    pub proof: [u8; API_POST_PROOF_BYTES],
}

impl Default for GeneratePoSTResponse {
    fn default() -> GeneratePoSTResponse {
        GeneratePoSTResponse {
            status_code: FFIResponseStatus::NoError,
            error_msg: ptr::null(),
            faults_len: 0,
            faults_ptr: ptr::null(),
            proof: [0; API_POST_PROOF_BYTES],
        }
    }
}

impl Drop for GeneratePoSTResponse {
    fn drop(&mut self) {
        unsafe {
            drop(Vec::from_raw_parts(
                self.faults_ptr as *mut u8,
                self.faults_len,
                self.faults_len,
            ));

            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_generate_post_response(ptr: *mut GeneratePoSTResponse) {
    let _ = Box::from_raw(ptr);
}

///////////////////////////////////////////////////////////////////////////////
/// VerifyPoSTResult
////////////////////

#[repr(C)]
pub struct VerifyPoSTResponse {
    pub status_code: FFIResponseStatus,
    pub error_msg: *const libc::c_char,
    pub is_valid: bool,
}

impl Default for VerifyPoSTResponse {
    fn default() -> VerifyPoSTResponse {
        VerifyPoSTResponse {
            status_code: FFIResponseStatus::NoError,
            error_msg: ptr::null(),
            is_valid: false,
        }
    }
}

impl Drop for VerifyPoSTResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_verify_post_response(ptr: *mut VerifyPoSTResponse) {
    let _ = Box::from_raw(ptr);
}

// err_code_and_msg accepts an Error struct and produces a tuple of response
// status code and a pointer to a C string, both of which can be used to set
// fields in a response struct to be returned from an FFI call.
pub fn err_code_and_msg(err: &Error) -> (FFIResponseStatus, *const libc::c_char) {
    use ffi_toolkit::FFIResponseStatus::*;

    let msg = CString::new(format!("{}", err)).unwrap();
    let ptr = msg.as_ptr();
    mem::forget(msg);

    match err.downcast_ref() {
        Some(SectorBuilderErr::OverflowError { .. }) => return (CallerError, ptr),
        Some(SectorBuilderErr::IncompleteWriteError { .. }) => return (ReceiverError, ptr),
        Some(SectorBuilderErr::InvalidInternalStateError(_)) => return (ReceiverError, ptr),
        None => (),
    }

    match err.downcast_ref() {
        Some(SectorManagerErr::UnclassifiedError(_)) => return (UnclassifiedError, ptr),
        Some(SectorManagerErr::CallerError(_)) => return (CallerError, ptr),
        Some(SectorManagerErr::ReceiverError(_)) => return (ReceiverError, ptr),
        None => (),
    }

    (UnclassifiedError, ptr)
}

///////////////////////////////////////////////////////////////////////////////
/// InitSectorBuilderResponse
/////////////////////////////

#[repr(C)]
pub struct InitSectorBuilderResponse {
    pub status_code: FFIResponseStatus,
    pub error_msg: *const libc::c_char,
    pub sector_builder: *mut SectorBuilder,
}

impl Default for InitSectorBuilderResponse {
    fn default() -> InitSectorBuilderResponse {
        InitSectorBuilderResponse {
            status_code: FFIResponseStatus::NoError,
            error_msg: ptr::null(),
            sector_builder: ptr::null_mut(),
        }
    }
}

impl Drop for InitSectorBuilderResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_init_sector_builder_response(ptr: *mut InitSectorBuilderResponse) {
    let _ = Box::from_raw(ptr);
}

///////////////////////////////////////////////////////////////////////////////
/// AddPieceResponse
////////////////////

#[repr(C)]
pub struct AddPieceResponse {
    pub status_code: FFIResponseStatus,
    pub error_msg: *const libc::c_char,
    pub sector_id: u64,
}

impl Default for AddPieceResponse {
    fn default() -> AddPieceResponse {
        AddPieceResponse {
            status_code: FFIResponseStatus::NoError,
            error_msg: ptr::null(),
            sector_id: 0,
        }
    }
}

impl Drop for AddPieceResponse {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_add_piece_response(ptr: *mut AddPieceResponse) {
    let _ = Box::from_raw(ptr);
}

///////////////////////////////////////////////////////////////////////////////
/// GetMaxStagedBytesPerSector
//////////////////////////////

#[repr(C)]
pub struct GetMaxStagedBytesPerSector {
    pub status_code: FFIResponseStatus,
    pub error_msg: *const libc::c_char,
    pub max_staged_bytes_per_sector: u64,
}

impl Default for GetMaxStagedBytesPerSector {
    fn default() -> GetMaxStagedBytesPerSector {
        GetMaxStagedBytesPerSector {
            status_code: FFIResponseStatus::NoError,
            error_msg: ptr::null(),
            max_staged_bytes_per_sector: 0,
        }
    }
}

impl Drop for GetMaxStagedBytesPerSector {
    fn drop(&mut self) {
        unsafe {
            drop(c_str_to_rust_str(self.error_msg));
        };
    }
}

#[no_mangle]
pub unsafe extern "C" fn destroy_get_max_user_bytes_per_staged_sector_response(
    ptr: *mut GetMaxStagedBytesPerSector,
) {
    let _ = Box::from_raw(ptr);
}
