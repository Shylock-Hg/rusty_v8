use std::convert::TryInto;
use std::ffi::c_void;

use crate::support::int;
use crate::ArrayBuffer;
use crate::ArrayBufferView;
use crate::BackingStore;
use crate::HandleScope;
use crate::Local;
use crate::SharedRef;

extern "C" {
  fn v8__ArrayBufferView__Buffer(
    this: *const ArrayBufferView,
  ) -> *const ArrayBuffer;
  fn v8__ArrayBufferView__Buffer__Data(
    this: *const ArrayBufferView,
  ) -> *mut c_void;
  fn v8__ArrayBufferView__ByteLength(this: *const ArrayBufferView) -> usize;
  fn v8__ArrayBufferView__ByteOffset(this: *const ArrayBufferView) -> usize;
  fn v8__ArrayBufferView__CopyContents(
    this: *const ArrayBufferView,
    dest: *mut c_void,
    byte_length: int,
  ) -> usize;
}

impl ArrayBufferView {
  /// Returns underlying ArrayBuffer.
  #[inline(always)]
  pub fn buffer<'s>(
    &self,
    scope: &mut HandleScope<'s>,
  ) -> Option<Local<'s, ArrayBuffer>> {
    unsafe { scope.cast_local(|_| v8__ArrayBufferView__Buffer(self)) }
  }

  /// Get a shared pointer to the backing store of this array buffer. This
  /// pointer coordinates the lifetime management of the internal storage
  /// with any live ArrayBuffers on the heap, even across isolates. The embedder
  /// should not attempt to manage lifetime of the storage through other means.
  #[inline(always)]
  pub fn get_backing_store(&self) -> Option<SharedRef<BackingStore>> {
    let buffer = unsafe { v8__ArrayBufferView__Buffer(self) };
    unsafe { buffer.as_ref().map(|buffer| buffer.get_backing_store()) }
  }

  /// Returns the underlying storage for this `ArrayBufferView`, including the built-in `byte_offset`.
  /// This is a more efficient way of calling `buffer(scope)->data()`, and may be called without a
  /// scope.
  #[inline(always)]
  pub fn data(&self) -> *mut c_void {
    unsafe {
      v8__ArrayBufferView__Buffer__Data(self)
        .add(v8__ArrayBufferView__ByteOffset(self))
    }
  }

  /// Size of a view in bytes.
  #[inline(always)]
  pub fn byte_length(&self) -> usize {
    unsafe { v8__ArrayBufferView__ByteLength(self) }
  }

  /// Byte offset in |Buffer|.
  #[inline(always)]
  pub fn byte_offset(&self) -> usize {
    unsafe { v8__ArrayBufferView__ByteOffset(self) }
  }

  /// Copy the contents of the ArrayBufferView's buffer to an embedder defined
  /// memory without additional overhead that calling ArrayBufferView::Buffer
  /// might incur.
  /// Returns the number of bytes actually written.
  #[inline(always)]
  pub fn copy_contents(&self, dest: &mut [u8]) -> usize {
    unsafe {
      v8__ArrayBufferView__CopyContents(
        self,
        dest.as_mut_ptr() as *mut c_void,
        dest.len().try_into().unwrap(),
      )
    }
  }
}
