// Copyright (c) 2014 Marshall A. Greenblatt. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are
// met:
//
//    * Redistributions of source code must retain the above copyright
// notice, this list of conditions and the following disclaimer.
//    * Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following disclaimer
// in the documentation and/or other materials provided with the
// distribution.
//    * Neither the name of Google Inc. nor the name Chromium Embedded
// Framework nor the names of its contributors may be used to endorse
// or promote products derived from this software without specific prior
// written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
//
// ---------------------------------------------------------------------------
//
// This file was generated by the CEF translator tool and should not be edited
// by hand. See the translator.README.txt file in the tools directory for
// more information.
//

#![allow(non_snake_case, unused_imports)]

use eutil;
use interfaces;
use types;
use wrappers::CefWrap;

use libc;
use std::collections::HashMap;
use std::ptr;

//
// Callback structure used to asynchronously continue a download.
//
#[repr(C)]
pub struct _cef_before_download_callback_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Call to continue the download. Set |download_path| to the full file path
  // for the download including the file name or leave blank to use the
  // suggested name and the default temp directory. Set |show_dialog| to true
  // (1) if you do wish to show the default "Save As" dialog.
  //
  pub cont: Option<extern "C" fn(this: *mut cef_before_download_callback_t,
      download_path: *const types::cef_string_t, show_dialog: libc::c_int) -> (
      )>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
} 

pub type cef_before_download_callback_t = _cef_before_download_callback_t;


//
// Callback structure used to asynchronously continue a download.
//
pub struct CefBeforeDownloadCallback {
  c_object: *mut cef_before_download_callback_t,
}

impl Clone for CefBeforeDownloadCallback {
  fn clone(&self) -> CefBeforeDownloadCallback{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefBeforeDownloadCallback {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefBeforeDownloadCallback {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefBeforeDownloadCallback {
  pub unsafe fn from_c_object(c_object: *mut cef_before_download_callback_t) -> CefBeforeDownloadCallback {
    CefBeforeDownloadCallback {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_before_download_callback_t) -> CefBeforeDownloadCallback {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefBeforeDownloadCallback {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_before_download_callback_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_before_download_callback_t {
    unsafe {
      if !self.c_object.is_null() {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null()
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null()
  }

  //
  // Call to continue the download. Set |download_path| to the full file path
  // for the download including the file name or leave blank to use the
  // suggested name and the default temp directory. Set |show_dialog| to true
  // (1) if you do wish to show the default "Save As" dialog.
  //
  pub fn cont(&self, download_path: &[u16], show_dialog: libc::c_int) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).cont.unwrap())(
          self.c_object,
          CefWrap::to_c(download_path),
          CefWrap::to_c(show_dialog)))
    }
  }
} 

impl CefWrap<*mut cef_before_download_callback_t> for CefBeforeDownloadCallback {
  fn to_c(rust_object: CefBeforeDownloadCallback) -> *mut cef_before_download_callback_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_before_download_callback_t) -> CefBeforeDownloadCallback {
    CefBeforeDownloadCallback::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_before_download_callback_t> for Option<CefBeforeDownloadCallback> {
  fn to_c(rust_object: Option<CefBeforeDownloadCallback>) -> *mut cef_before_download_callback_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_before_download_callback_t) -> Option<CefBeforeDownloadCallback> {
    if c_object.is_null() {
      None
    } else {
      Some(CefBeforeDownloadCallback::from_c_object_addref(c_object))
    }
  }
}


//
// Callback structure used to asynchronously cancel a download.
//
#[repr(C)]
pub struct _cef_download_item_callback_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Call to cancel the download.
  //
  pub cancel: Option<extern "C" fn(this: *mut cef_download_item_callback_t) -> (
      )>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
} 

pub type cef_download_item_callback_t = _cef_download_item_callback_t;


//
// Callback structure used to asynchronously cancel a download.
//
pub struct CefDownloadItemCallback {
  c_object: *mut cef_download_item_callback_t,
}

impl Clone for CefDownloadItemCallback {
  fn clone(&self) -> CefDownloadItemCallback{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefDownloadItemCallback {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefDownloadItemCallback {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefDownloadItemCallback {
  pub unsafe fn from_c_object(c_object: *mut cef_download_item_callback_t) -> CefDownloadItemCallback {
    CefDownloadItemCallback {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_download_item_callback_t) -> CefDownloadItemCallback {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefDownloadItemCallback {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_download_item_callback_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_download_item_callback_t {
    unsafe {
      if !self.c_object.is_null() {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null()
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null()
  }

  //
  // Call to cancel the download.
  //
  pub fn cancel(&self) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).cancel.unwrap())(
          self.c_object))
    }
  }
} 

impl CefWrap<*mut cef_download_item_callback_t> for CefDownloadItemCallback {
  fn to_c(rust_object: CefDownloadItemCallback) -> *mut cef_download_item_callback_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_download_item_callback_t) -> CefDownloadItemCallback {
    CefDownloadItemCallback::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_download_item_callback_t> for Option<CefDownloadItemCallback> {
  fn to_c(rust_object: Option<CefDownloadItemCallback>) -> *mut cef_download_item_callback_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_download_item_callback_t) -> Option<CefDownloadItemCallback> {
    if c_object.is_null() {
      None
    } else {
      Some(CefDownloadItemCallback::from_c_object_addref(c_object))
    }
  }
}


//
// Structure used to handle file downloads. The functions of this structure will
// called on the browser process UI thread.
//
#[repr(C)]
pub struct _cef_download_handler_t {
  //
  // Base structure.
  //
  pub base: types::cef_base_t,

  //
  // Called before a download begins. |suggested_name| is the suggested name for
  // the download file. By default the download will be canceled. Execute
  // |callback| either asynchronously or in this function to continue the
  // download if desired. Do not keep a reference to |download_item| outside of
  // this function.
  //
  pub on_before_download: Option<extern "C" fn(
      this: *mut cef_download_handler_t,
      browser: *mut interfaces::cef_browser_t,
      download_item: *mut interfaces::cef_download_item_t,
      suggested_name: *const types::cef_string_t,
      callback: *mut interfaces::cef_before_download_callback_t) -> ()>,

  //
  // Called when a download's status or progress information has been updated.
  // This may be called multiple times before and after on_before_download().
  // Execute |callback| either asynchronously or in this function to cancel the
  // download if desired. Do not keep a reference to |download_item| outside of
  // this function.
  //
  pub on_download_updated: Option<extern "C" fn(
      this: *mut cef_download_handler_t,
      browser: *mut interfaces::cef_browser_t,
      download_item: *mut interfaces::cef_download_item_t,
      callback: *mut interfaces::cef_download_item_callback_t) -> ()>,

  //
  // The reference count. This will only be present for Rust instances!
  //
  pub ref_count: uint,

  //
  // Extra data. This will only be present for Rust instances!
  //
  pub extra: u8,
} 

pub type cef_download_handler_t = _cef_download_handler_t;


//
// Structure used to handle file downloads. The functions of this structure will
// called on the browser process UI thread.
//
pub struct CefDownloadHandler {
  c_object: *mut cef_download_handler_t,
}

impl Clone for CefDownloadHandler {
  fn clone(&self) -> CefDownloadHandler{
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.add_ref.unwrap())(&mut (*self.c_object).base);
      }
      CefDownloadHandler {
        c_object: self.c_object,
      }
    }
  }
}

impl Drop for CefDownloadHandler {
  fn drop(&mut self) {
    unsafe {
      if !self.c_object.is_null() {
        ((*self.c_object).base.release.unwrap())(&mut (*self.c_object).base);
      }
    }
  }
}

impl CefDownloadHandler {
  pub unsafe fn from_c_object(c_object: *mut cef_download_handler_t) -> CefDownloadHandler {
    CefDownloadHandler {
      c_object: c_object,
    }
  }

  pub unsafe fn from_c_object_addref(c_object: *mut cef_download_handler_t) -> CefDownloadHandler {
    if !c_object.is_null() {
      ((*c_object).base.add_ref.unwrap())(&mut (*c_object).base);
    }
    CefDownloadHandler {
      c_object: c_object,
    }
  }

  pub fn c_object(&self) -> *mut cef_download_handler_t {
    self.c_object
  }

  pub fn c_object_addrefed(&self) -> *mut cef_download_handler_t {
    unsafe {
      if !self.c_object.is_null() {
        eutil::add_ref(self.c_object as *mut types::cef_base_t);
      }
      self.c_object
    }
  }

  pub fn is_null_cef_object(&self) -> bool {
    self.c_object.is_null()
  }
  pub fn is_not_null_cef_object(&self) -> bool {
    !self.c_object.is_null()
  }

  //
  // Called before a download begins. |suggested_name| is the suggested name for
  // the download file. By default the download will be canceled. Execute
  // |callback| either asynchronously or in this function to continue the
  // download if desired. Do not keep a reference to |download_item| outside of
  // this function.
  //
  pub fn on_before_download(&self, browser: interfaces::CefBrowser,
      download_item: interfaces::CefDownloadItem, suggested_name: &[u16],
      callback: interfaces::CefBeforeDownloadCallback) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_before_download.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(download_item),
          CefWrap::to_c(suggested_name),
          CefWrap::to_c(callback)))
    }
  }

  //
  // Called when a download's status or progress information has been updated.
  // This may be called multiple times before and after on_before_download().
  // Execute |callback| either asynchronously or in this function to cancel the
  // download if desired. Do not keep a reference to |download_item| outside of
  // this function.
  //
  pub fn on_download_updated(&self, browser: interfaces::CefBrowser,
      download_item: interfaces::CefDownloadItem,
      callback: interfaces::CefDownloadItemCallback) -> () {
    if self.c_object.is_null() {
      panic!("called a CEF method on a null object")
    }
    unsafe {
      CefWrap::to_rust(
        ((*self.c_object).on_download_updated.unwrap())(
          self.c_object,
          CefWrap::to_c(browser),
          CefWrap::to_c(download_item),
          CefWrap::to_c(callback)))
    }
  }
} 

impl CefWrap<*mut cef_download_handler_t> for CefDownloadHandler {
  fn to_c(rust_object: CefDownloadHandler) -> *mut cef_download_handler_t {
    rust_object.c_object_addrefed()
  }
  unsafe fn to_rust(c_object: *mut cef_download_handler_t) -> CefDownloadHandler {
    CefDownloadHandler::from_c_object_addref(c_object)
  }
}
impl CefWrap<*mut cef_download_handler_t> for Option<CefDownloadHandler> {
  fn to_c(rust_object: Option<CefDownloadHandler>) -> *mut cef_download_handler_t {
    match rust_object {
      None => ptr::null_mut(),
      Some(rust_object) => rust_object.c_object_addrefed(),
    }
  }
  unsafe fn to_rust(c_object: *mut cef_download_handler_t) -> Option<CefDownloadHandler> {
    if c_object.is_null() {
      None
    } else {
      Some(CefDownloadHandler::from_c_object_addref(c_object))
    }
  }
}
