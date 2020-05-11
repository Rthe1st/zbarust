use ::libc;
extern {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type PyMemberDef;
    /* @} */
    /* ------------------------------------------------------------ */
    /* * @name Processor interface
     * @anchor c-processor
     * high-level self-contained image processor.
     * processes video and images for barcodes, optionally displaying
     * images to a library owned output window
     */
    /* @{ */
    pub type zbar_processor_s;
    /* access macro to the members which are floating "behind" the object */
    /* Generic type check */
    #[no_mangle]
    fn PyType_IsSubtype(_: *mut PyTypeObject, _: *mut PyTypeObject) -> libc::c_int;
    #[no_mangle]
    static mut zbar_exc: [*mut PyObject; 12];
    #[no_mangle]
    fn PyString_FromString(_: *const libc::c_char) -> *mut PyObject;
    #[no_mangle]
    fn PyString_Size(_: *mut PyObject) -> Py_ssize_t;
    #[no_mangle]
    fn PyErr_SetObject(_: *mut PyObject, _: *mut PyObject);
    #[no_mangle]
    static mut PyExc_Exception: *mut PyObject;
    #[no_mangle]
    fn PyErr_NoMemory() -> *mut PyObject;
    #[no_mangle]
    fn _PyArg_NoKeywords(funcname: *const libc::c_char, kw: *mut PyObject) -> libc::c_int;
    #[no_mangle]
    fn _zbar_error_string(
        object: *const libc::c_void,
        verbosity: libc::c_int,
    ) -> *const libc::c_char;
    #[no_mangle]
    fn _zbar_get_error_code(object: *const libc::c_void) -> zbar_error_t;
    #[no_mangle]
    static mut zbarProcessor_Type: PyTypeObject;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
/* include for defines */
/* Some versions of HP-UX & Solaris need inttypes.h for int32_t,
INT32_MAX, etc. */
/* *************************************************************************
Symbols and macros to supply platform-independent interfaces to basic
C language & library operations whose spellings vary across platforms.

Please try to make documentation here as clear as possible:  by definition,
the stuff here is trying to illuminate C's darkest corners.

Config #defines referenced here:

SIGNED_RIGHT_SHIFT_ZERO_FILLS
Meaning:  To be defined iff i>>j does not extend the sign bit when i is a
          signed integral type and i < 0.
Used in:  Py_ARITHMETIC_RIGHT_SHIFT

Py_DEBUG
Meaning:  Extra checks compiled in for debug mode.
Used in:  Py_SAFE_DOWNCAST

HAVE_UINTPTR_T
Meaning:  The C9X type uintptr_t is supported by the compiler
Used in:  Py_uintptr_t

HAVE_LONG_LONG
Meaning:  The compiler supports the C type "long long"
Used in:  PY_LONG_LONG

**************************************************************************/
/* For backward compatibility only. Obsolete, do not use. */
/* typedefs for some C9X-defined synonyms for integral types.
 *
 * The names in Python are exactly the same as the C9X names, except with a
 * Py_ prefix.  Until C9X is universally implemented, this is the only way
 * to ensure that Python gets reliable names that don't conflict with names
 * in non-Python code that are playing their own tricks to define the C9X
 * names.
 *
 * NOTE: don't go nuts here!  Python has no use for *most* of the C9X
 * integral synonyms.  Only define the ones we actually need.
 */
/* If LLONG_MAX is defined in limits.h, use that. */
/* LLONG_MAX */
/* HAVE_LONG_LONG */
/* a build with 30-bit digits for Python long integers needs an exact-width
 * 32-bit unsigned integer type to store those digits.  (We could just use
 * type 'unsigned long', but that would be wasteful on a system where longs
 * are 64-bits.)  On Unix systems, the autoconf macro AC_TYPE_UINT32_T defines
 * uint32_t to be such a type unless stdint.h or inttypes.h defines uint32_t.
 * However, it doesn't set HAVE_UINT32_T, so we do that here.
 */
/* Macros for a 64-bit unsigned integer type; used for type 'twodigits' in the
 * long integer implementation, when 30-bit digits are enabled.
 */
/* Signed variants of the above */
/* If PYLONG_BITS_IN_DIGIT is not defined then we'll use 30-bit digits if all
the necessary integer types are available, and we're on a 64-bit platform
(as determined by SIZEOF_VOID_P); otherwise we use 15-bit digits. */
/* uintptr_t is the C9X name for an unsigned integral type such that a
 * legitimate void* can be cast to uintptr_t and then back to void* again
 * without loss of information.  Similarly for intptr_t, wrt a signed
 * integral type.
 */
/* HAVE_UINTPTR_T */
/* Py_ssize_t is a signed integral type such that sizeof(Py_ssize_t) ==
 * sizeof(size_t).  C99 doesn't define such a thing directly (size_t is an
 * unsigned integral type).  See PEP 353 for details.
 */
pub type Py_ssize_t = ssize_t;
/* Object and type object interface */
/*
Objects are structures allocated on the heap.  Special rules apply to
the use of objects to ensure they are properly garbage-collected.
Objects are never allocated statically or on the stack; they must be
accessed through special macros and functions only.  (Type objects are
exceptions to the first rule; the standard types are represented by
statically initialized type objects, although work on type/class unification
for Python 2.2 made it possible to have heap-allocated type objects too).

An object has a 'reference count' that is increased or decreased when a
pointer to the object is copied or deleted; when the reference count
reaches zero there are no references to the object left and it can be
removed from the heap.

An object has a 'type' that determines what it represents and what kind
of data it contains.  An object's type is fixed when it is created.
Types themselves are represented as objects; an object contains a
pointer to the corresponding type object.  The type itself has a type
pointer pointing to the object representing the type 'type', which
contains a pointer to itself!).

Objects do not float around in memory; once allocated an object keeps
the same size and address.  Objects that must hold variable-size data
can contain pointers to variable-size parts of the object.  Not all
objects of the same type have the same size; but the size cannot change
after allocation.  (These restrictions are made so a reference to an
object can be simply a pointer -- moving an object would require
updating all the pointers, and changing an object's size would require
moving it if there was another object right next to it.)

Objects are always accessed through pointers of the type 'PyObject *'.
The type 'PyObject' is a structure that only contains the reference count
and the type pointer.  The actual memory allocated for an object
contains other data that can only be accessed after casting the pointer
to a pointer to a longer structure type.  This longer type must start
with the reference count and type fields; the macro PyObject_HEAD should be
used for this (to accommodate for future changes).  The implementation
of a particular object type can cast the object pointer to the proper
type and back.

A standard interface exists for objects that contain an array of items
whose size is determined when the object is allocated.
*/
/* Py_DEBUG implies Py_TRACE_REFS. */
/* Py_TRACE_REFS implies Py_REF_DEBUG. */
/* PyObject_HEAD defines the initial segment of every PyObject. */
/* PyObject_VAR_HEAD defines the initial segment of all variable-size
 * container objects.  These end with a declaration of an array with 1
 * element, but enough space is malloc'ed so that the array actually
 * has room for ob_size elements.  Note that ob_size is an element count,
 * not necessarily a byte count.
 */
/* Number of items in variable part */
/* Nothing is actually declared to be a PyObject, but every pointer to
 * a Python object can be cast to a PyObject*.  This is inheritance built
 * by hand.  Similarly every pointer to a variable-size Python object can,
 * in addition, be cast to PyVarObject*.
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _object {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _typeobject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_size: Py_ssize_t,
    pub tp_name: *const libc::c_char,
    pub tp_basicsize: Py_ssize_t,
    pub tp_itemsize: Py_ssize_t,
    pub tp_dealloc: destructor,
    pub tp_print: printfunc,
    pub tp_getattr: getattrfunc,
    pub tp_setattr: setattrfunc,
    pub tp_compare: cmpfunc,
    pub tp_repr: reprfunc,
    pub tp_as_number: *mut PyNumberMethods,
    pub tp_as_sequence: *mut PySequenceMethods,
    pub tp_as_mapping: *mut PyMappingMethods,
    pub tp_hash: hashfunc,
    pub tp_call: ternaryfunc,
    pub tp_str: reprfunc,
    pub tp_getattro: getattrofunc,
    pub tp_setattro: setattrofunc,
    pub tp_as_buffer: *mut PyBufferProcs,
    pub tp_flags: libc::c_long,
    pub tp_doc: *const libc::c_char,
    pub tp_traverse: traverseproc,
    pub tp_clear: inquiry,
    pub tp_richcompare: richcmpfunc,
    pub tp_weaklistoffset: Py_ssize_t,
    pub tp_iter: getiterfunc,
    pub tp_iternext: iternextfunc,
    pub tp_methods: *mut PyMethodDef,
    pub tp_members: *mut PyMemberDef,
    pub tp_getset: *mut PyGetSetDef,
    pub tp_base: *mut _typeobject,
    pub tp_dict: *mut PyObject,
    pub tp_descr_get: descrgetfunc,
    pub tp_descr_set: descrsetfunc,
    pub tp_dictoffset: Py_ssize_t,
    pub tp_init: initproc,
    pub tp_alloc: allocfunc,
    pub tp_new: newfunc,
    pub tp_free: freefunc,
    pub tp_is_gc: inquiry,
    pub tp_bases: *mut PyObject,
    pub tp_mro: *mut PyObject,
    pub tp_cache: *mut PyObject,
    pub tp_subclasses: *mut PyObject,
    pub tp_weaklist: *mut PyObject,
    pub tp_del: destructor,
    pub tp_version_tag: libc::c_uint,
}
/*
Type objects contain a string containing the type name (to help somewhat
in debugging), the allocation parameters (see PyObject_New() and
PyObject_NewVar()),
and methods for accessing objects of the type.  Methods are optional, a
nil pointer meaning that particular kind of access is not available for
this type.  The Py_DECREF() macro uses the tp_dealloc method without
checking for a nil pointer; it should always be implemented except if
the implementation can guarantee that the reference count will never
reach zero (e.g., for statically allocated type objects).

NB: the methods for certain type groups are now contained in separate
method blocks.
*/
/* int-based buffer interface */
/* ssize_t-based buffer interface */
/* Py3k buffer interface */
/* owned reference */
/* This is Py_ssize_t so it can be
pointed to by strides in simple case.*/
/* static store for shape and strides of
mono-dimensional buffers. */
/* Flags for getting buffers */
/* we used to include an E, backwards compatible alias */
/* end Py3k buffer interface */
/* For numbers without flag bit Py_TPFLAGS_CHECKTYPES set, all
arguments are guaranteed to be of the object's type (modulo
coercion hacks -- i.e. if the type's coercion function
returns other types, then these are allowed as well).  Numbers that
have the Py_TPFLAGS_CHECKTYPES flag bit set should check *both*
arguments for proper type and implement the necessary conversions
in the slot functions themselves. */
/* Added in release 2.0 */
/* Added in release 2.2 */
/* The following require the Py_TPFLAGS_HAVE_CLASS flag */
/* Added in release 2.5 */
/* Added in release 2.0 */
pub type destructor = Option<unsafe extern fn(_: *mut PyObject) -> ()>;
pub type PyObject = _object;
pub type inquiry = Option<unsafe extern fn(_: *mut PyObject) -> libc::c_int>;
pub type freefunc = Option<unsafe extern fn(_: *mut libc::c_void) -> ()>;
pub type newfunc = Option<
    unsafe extern fn(_: *mut _typeobject, _: *mut PyObject, _: *mut PyObject) -> *mut PyObject,
>;
pub type allocfunc = Option<unsafe extern fn(_: *mut _typeobject, _: Py_ssize_t) -> *mut PyObject>;
pub type initproc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject, _: *mut PyObject) -> libc::c_int>;
pub type descrsetfunc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject, _: *mut PyObject) -> libc::c_int>;
pub type descrgetfunc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject, _: *mut PyObject) -> *mut PyObject>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyGetSetDef {
    pub name: *mut libc::c_char,
    pub get: getter,
    pub set: setter,
    pub doc: *mut libc::c_char,
    pub closure: *mut libc::c_void,
}
/* Descriptors */
pub type setter = Option<
    unsafe extern fn(_: *mut PyObject, _: *mut PyObject, _: *mut libc::c_void) -> libc::c_int,
>;
pub type getter = Option<unsafe extern fn(_: *mut PyObject, _: *mut libc::c_void) -> *mut PyObject>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyMethodDef {
    pub ml_name: *const libc::c_char,
    pub ml_meth: PyCFunction,
    pub ml_flags: libc::c_int,
    pub ml_doc: *const libc::c_char,
}
/* Method object interface */
/* This is about the type 'builtin_function_or_method',
not Python methods in user-defined classes.  See classobject.h
for the latter. */
pub type PyCFunction =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject) -> *mut PyObject>;
pub type iternextfunc = Option<unsafe extern fn(_: *mut PyObject) -> *mut PyObject>;
pub type getiterfunc = Option<unsafe extern fn(_: *mut PyObject) -> *mut PyObject>;
pub type richcmpfunc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject, _: libc::c_int) -> *mut PyObject>;
pub type traverseproc =
    Option<unsafe extern fn(_: *mut PyObject, _: visitproc, _: *mut libc::c_void) -> libc::c_int>;
pub type visitproc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut libc::c_void) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyBufferProcs {
    pub bf_getreadbuffer: readbufferproc,
    pub bf_getwritebuffer: writebufferproc,
    pub bf_getsegcount: segcountproc,
    pub bf_getcharbuffer: charbufferproc,
    pub bf_getbuffer: getbufferproc,
    pub bf_releasebuffer: releasebufferproc,
}
pub type releasebufferproc = Option<unsafe extern fn(_: *mut PyObject, _: *mut Py_buffer) -> ()>;
pub type Py_buffer = bufferinfo;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bufferinfo {
    pub buf: *mut libc::c_void,
    pub obj: *mut PyObject,
    pub len: Py_ssize_t,
    pub itemsize: Py_ssize_t,
    pub readonly: libc::c_int,
    pub ndim: libc::c_int,
    pub format: *mut libc::c_char,
    pub shape: *mut Py_ssize_t,
    pub strides: *mut Py_ssize_t,
    pub suboffsets: *mut Py_ssize_t,
    pub smalltable: [Py_ssize_t; 2],
    pub internal: *mut libc::c_void,
}
pub type getbufferproc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut Py_buffer, _: libc::c_int) -> libc::c_int>;
pub type charbufferproc = Option<
    unsafe extern fn(_: *mut PyObject, _: Py_ssize_t, _: *mut *mut libc::c_char) -> Py_ssize_t,
>;
pub type segcountproc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut Py_ssize_t) -> Py_ssize_t>;
pub type writebufferproc = Option<
    unsafe extern fn(_: *mut PyObject, _: Py_ssize_t, _: *mut *mut libc::c_void) -> Py_ssize_t,
>;
pub type readbufferproc = Option<
    unsafe extern fn(_: *mut PyObject, _: Py_ssize_t, _: *mut *mut libc::c_void) -> Py_ssize_t,
>;
pub type setattrofunc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject, _: *mut PyObject) -> libc::c_int>;
pub type getattrofunc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject) -> *mut PyObject>;
pub type reprfunc = Option<unsafe extern fn(_: *mut PyObject) -> *mut PyObject>;
pub type ternaryfunc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject, _: *mut PyObject) -> *mut PyObject>;
pub type hashfunc = Option<unsafe extern fn(_: *mut PyObject) -> libc::c_long>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyMappingMethods {
    pub mp_length: lenfunc,
    pub mp_subscript: binaryfunc,
    pub mp_ass_subscript: objobjargproc,
}
pub type objobjargproc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject, _: *mut PyObject) -> libc::c_int>;
pub type binaryfunc = Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject) -> *mut PyObject>;
pub type lenfunc = Option<unsafe extern fn(_: *mut PyObject) -> Py_ssize_t>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PySequenceMethods {
    pub sq_length: lenfunc,
    pub sq_concat: binaryfunc,
    pub sq_repeat: ssizeargfunc,
    pub sq_item: ssizeargfunc,
    pub sq_slice: ssizessizeargfunc,
    pub sq_ass_item: ssizeobjargproc,
    pub sq_ass_slice: ssizessizeobjargproc,
    pub sq_contains: objobjproc,
    pub sq_inplace_concat: binaryfunc,
    pub sq_inplace_repeat: ssizeargfunc,
}
pub type ssizeargfunc = Option<unsafe extern fn(_: *mut PyObject, _: Py_ssize_t) -> *mut PyObject>;
pub type objobjproc = Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject) -> libc::c_int>;
pub type ssizessizeobjargproc = Option<
    unsafe extern fn(
        _: *mut PyObject,
        _: Py_ssize_t,
        _: Py_ssize_t,
        _: *mut PyObject,
    ) -> libc::c_int,
>;
pub type ssizeobjargproc =
    Option<unsafe extern fn(_: *mut PyObject, _: Py_ssize_t, _: *mut PyObject) -> libc::c_int>;
pub type ssizessizeargfunc =
    Option<unsafe extern fn(_: *mut PyObject, _: Py_ssize_t, _: Py_ssize_t) -> *mut PyObject>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyNumberMethods {
    pub nb_add: binaryfunc,
    pub nb_subtract: binaryfunc,
    pub nb_multiply: binaryfunc,
    pub nb_divide: binaryfunc,
    pub nb_remainder: binaryfunc,
    pub nb_divmod: binaryfunc,
    pub nb_power: ternaryfunc,
    pub nb_negative: unaryfunc,
    pub nb_positive: unaryfunc,
    pub nb_absolute: unaryfunc,
    pub nb_nonzero: inquiry,
    pub nb_invert: unaryfunc,
    pub nb_lshift: binaryfunc,
    pub nb_rshift: binaryfunc,
    pub nb_and: binaryfunc,
    pub nb_xor: binaryfunc,
    pub nb_or: binaryfunc,
    pub nb_coerce: coercion,
    pub nb_int: unaryfunc,
    pub nb_long: unaryfunc,
    pub nb_float: unaryfunc,
    pub nb_oct: unaryfunc,
    pub nb_hex: unaryfunc,
    pub nb_inplace_add: binaryfunc,
    pub nb_inplace_subtract: binaryfunc,
    pub nb_inplace_multiply: binaryfunc,
    pub nb_inplace_divide: binaryfunc,
    pub nb_inplace_remainder: binaryfunc,
    pub nb_inplace_power: ternaryfunc,
    pub nb_inplace_lshift: binaryfunc,
    pub nb_inplace_rshift: binaryfunc,
    pub nb_inplace_and: binaryfunc,
    pub nb_inplace_xor: binaryfunc,
    pub nb_inplace_or: binaryfunc,
    pub nb_floor_divide: binaryfunc,
    pub nb_true_divide: binaryfunc,
    pub nb_inplace_floor_divide: binaryfunc,
    pub nb_inplace_true_divide: binaryfunc,
    pub nb_index: unaryfunc,
}
pub type unaryfunc = Option<unsafe extern fn(_: *mut PyObject) -> *mut PyObject>;
pub type coercion =
    Option<unsafe extern fn(_: *mut *mut PyObject, _: *mut *mut PyObject) -> libc::c_int>;
pub type cmpfunc = Option<unsafe extern fn(_: *mut PyObject, _: *mut PyObject) -> libc::c_int>;
pub type setattrfunc = Option<
    unsafe extern fn(_: *mut PyObject, _: *mut libc::c_char, _: *mut PyObject) -> libc::c_int,
>;
pub type getattrfunc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut libc::c_char) -> *mut PyObject>;
pub type printfunc =
    Option<unsafe extern fn(_: *mut PyObject, _: *mut FILE, _: libc::c_int) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyVarObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_size: Py_ssize_t,
}
pub type PyTypeObject = _typeobject;
/* Tuple object interface */
/*
Another generally useful object type is a tuple of object pointers.
For Python, this is an immutable type.  C code can change the tuple items
(but not their number), and even use tuples are general-purpose arrays of
object references, but in general only brand new tuples should be mutated,
not ones that might already have been exposed to Python code.

*** WARNING *** PyTuple_SetItem does not increment the new item's reference
count, but does decrement the reference count of the item it replaces,
if not nil.  It does *decrement* the reference count if it is *not*
inserted in the tuple.  Similarly, PyTuple_GetItem does not increment the
returned item's reference count.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyTupleObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_size: Py_ssize_t,
    pub ob_item: [*mut PyObject; 1],
}
/* Error objects */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyBaseExceptionObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub dict: *mut PyObject,
    pub args: *mut PyObject,
    pub message: *mut PyObject,
}
/* * error codes. */
pub type zbar_error_e = libc::c_uint;
/* *< number of error codes */
/* *< windows system error */
pub const ZBAR_ERR_NUM: zbar_error_e = 12;
/* *< output window is closed */
pub const ZBAR_ERR_WINAPI: zbar_error_e = 11;
/* *< X11 protocol error */
pub const ZBAR_ERR_CLOSED: zbar_error_e = 10;
/* *< X11 display error */
pub const ZBAR_ERR_XPROTO: zbar_error_e = 9;
/* *< all resources busy */
pub const ZBAR_ERR_XDISPLAY: zbar_error_e = 8;
/* *< locking error */
pub const ZBAR_ERR_BUSY: zbar_error_e = 7;
/* *< system error */
pub const ZBAR_ERR_LOCKING: zbar_error_e = 6;
/* *< invalid request */
pub const ZBAR_ERR_SYSTEM: zbar_error_e = 5;
/* *< unsupported request */
pub const ZBAR_ERR_INVALID: zbar_error_e = 4;
/* *< internal library error */
pub const ZBAR_ERR_UNSUPPORTED: zbar_error_e = 3;
/* *< out of memory */
pub const ZBAR_ERR_INTERNAL: zbar_error_e = 2;
/* *< no error */
pub const ZBAR_ERR_NOMEM: zbar_error_e = 1;
pub const ZBAR_OK: zbar_error_e = 0;
pub type zbar_error_t = zbar_error_e;
/* * opaque standalone processor object. */
pub type zbar_processor_t = zbar_processor_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbarException {
    pub base: PyBaseExceptionObject,
    pub obj: *mut PyObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbarProcessor {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub zproc: *mut zbar_processor_t,
    pub handler: *mut PyObject,
    pub closure: *mut PyObject,
}
/*------------------------------------------------------------------------
 *  Copyright 2009 (c) Jeff Brown <spadix@users.sourceforge.net>
 *
 *  This file is part of the ZBar Bar Code Reader.
 *
 *  The ZBar Bar Code Reader is free software; you can redistribute it
 *  and/or modify it under the terms of the GNU Lesser Public License as
 *  published by the Free Software Foundation; either version 2.1 of
 *  the License, or (at your option) any later version.
 *
 *  The ZBar Bar Code Reader is distributed in the hope that it will be
 *  useful, but WITHOUT ANY WARRANTY; without even the implied warranty
 *  of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Lesser Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser Public License
 *  along with the ZBar Bar Code Reader; if not, write to the Free
 *  Software Foundation, Inc., 51 Franklin St, Fifth Floor,
 *  Boston, MA  02110-1301  USA
 *
 *  http://sourceforge.net/projects/zbar
 *------------------------------------------------------------------------ */
#[inline]
unsafe extern fn exc_get_message(
    mut self_0: *mut zbarException,
    mut closure: *mut libc::c_void,
) -> *mut PyObject {
    let mut super_0: *mut PyBaseExceptionObject = self_0 as *mut PyBaseExceptionObject;
    if PyString_Size((*super_0).message) == 0 {
        if !(*super_0).message.is_null() {
            let mut _py_tmp: *mut PyObject = (*super_0).message;
            (*super_0).message = 0 as *mut PyObject;
            (*_py_tmp).ob_refcnt -= 1;
            if !((*_py_tmp).ob_refcnt != 0 as libc::c_int as libc::c_long) {
                Some((*(*_py_tmp).ob_type).tp_dealloc.expect("non-null function pointer"))
                    .expect("non-null function pointer")(_py_tmp);
            }
        }
        if (*self_0).obj.is_null()
            || !((*(*self_0).obj).ob_type == &mut zbarProcessor_Type as *mut PyTypeObject
                || PyType_IsSubtype((*(*self_0).obj).ob_type, &mut zbarProcessor_Type) != 0)
        {
            (*super_0).message =
                PyString_FromString(b"unknown zbar error\x00" as *const u8 as *const libc::c_char)
        } else {
            let mut zobj: *const libc::c_void =
                (*((*self_0).obj as *mut zbarProcessor)).zproc as *const libc::c_void;
            (*super_0).message = PyString_FromString(_zbar_error_string(zobj, 1 as libc::c_int))
        }
    }
    (*(*super_0).message).ob_refcnt += 1;
    return (*super_0).message;
}
unsafe extern fn exc_init(
    mut self_0: *mut zbarException,
    mut args: *mut PyObject,
    mut kwds: *mut PyObject,
) -> libc::c_int {
    if _PyArg_NoKeywords((*(*self_0).base.ob_type).tp_name, kwds) == 0 {
        return -(1 as libc::c_int);
    }
    let mut super_0: *mut PyBaseExceptionObject = self_0 as *mut PyBaseExceptionObject;
    if !(*super_0).args.is_null() {
        let mut _py_tmp: *mut PyObject = (*super_0).args;
        (*super_0).args = 0 as *mut PyObject;
        (*_py_tmp).ob_refcnt -= 1;
        if !((*_py_tmp).ob_refcnt != 0 as libc::c_int as libc::c_long) {
            Some((*(*_py_tmp).ob_type).tp_dealloc.expect("non-null function pointer"))
                .expect("non-null function pointer")(_py_tmp);
        }
    }
    (*args).ob_refcnt += 1;
    (*super_0).args = args;
    if (*(args as *mut PyVarObject)).ob_size == 1 as libc::c_int as libc::c_long {
        if !(*self_0).obj.is_null() {
            let mut _py_tmp_0: *mut PyObject = (*self_0).obj;
            (*self_0).obj = 0 as *mut PyObject;
            (*_py_tmp_0).ob_refcnt -= 1;
            if !((*_py_tmp_0).ob_refcnt != 0 as libc::c_int as libc::c_long) {
                Some((*(*_py_tmp_0).ob_type).tp_dealloc.expect("non-null function pointer"))
                    .expect("non-null function pointer")(_py_tmp_0);
            }
        }
        (*self_0).obj =
            *(*(args as *mut PyTupleObject)).ob_item.as_mut_ptr().offset(0 as libc::c_int as isize);
        (*(*self_0).obj).ob_refcnt += 1
    }
    return 0 as libc::c_int;
}
unsafe extern fn exc_traverse(
    mut self_0: *mut zbarException,
    mut visit: visitproc,
    mut arg: *mut libc::c_void,
) -> libc::c_int {
    if !(*self_0).obj.is_null() {
        let mut vret: libc::c_int = visit.expect("non-null function pointer")((*self_0).obj, arg);
        if vret != 0 {
            return vret;
        }
    }
    let mut base: *mut PyTypeObject = PyExc_Exception as *mut PyTypeObject;
    return (*base).tp_traverse.expect("non-null function pointer")(
        self_0 as *mut PyObject,
        visit,
        arg,
    );
}
unsafe extern fn exc_clear(mut self_0: *mut zbarException) -> libc::c_int {
    if !(*self_0).obj.is_null() {
        let mut _py_tmp: *mut PyObject = (*self_0).obj;
        (*self_0).obj = 0 as *mut PyObject;
        (*_py_tmp).ob_refcnt -= 1;
        if !((*_py_tmp).ob_refcnt != 0 as libc::c_int as libc::c_long) {
            Some((*(*_py_tmp).ob_type).tp_dealloc.expect("non-null function pointer"))
                .expect("non-null function pointer")(_py_tmp);
        }
    }
    (*(PyExc_Exception as *mut PyTypeObject)).tp_clear.expect("non-null function pointer")(
        self_0 as *mut PyObject,
    );
    return 0 as libc::c_int;
}
unsafe extern fn exc_dealloc(mut self_0: *mut zbarException) {
    exc_clear(self_0);
    (*(PyExc_Exception as *mut PyTypeObject)).tp_dealloc.expect("non-null function pointer")(
        self_0 as *mut PyObject,
    );
}
unsafe extern fn exc_str(mut self_0: *mut zbarException) -> *mut PyObject {
    return exc_get_message(self_0, 0 as *mut libc::c_void);
}
unsafe extern fn exc_set_message(
    mut self_0: *mut zbarException,
    mut value: *mut PyObject,
    mut closure: *mut libc::c_void,
) -> libc::c_int {
    let mut super_0: *mut PyBaseExceptionObject = self_0 as *mut PyBaseExceptionObject;
    if !(*super_0).message.is_null() {
        let mut _py_tmp: *mut PyObject = (*super_0).message;
        (*super_0).message = 0 as *mut PyObject;
        (*_py_tmp).ob_refcnt -= 1;
        if !((*_py_tmp).ob_refcnt != 0 as libc::c_int as libc::c_long) {
            Some((*(*_py_tmp).ob_type).tp_dealloc.expect("non-null function pointer"))
                .expect("non-null function pointer")(_py_tmp);
        }
    }
    if value.is_null() {
        value = PyString_FromString(b"\x00" as *const u8 as *const libc::c_char)
    } else {
        (*value).ob_refcnt += 1
    }
    (*super_0).message = value;
    return 0 as libc::c_int;
}
static mut exc_getset: [PyGetSetDef; 2] = unsafe {
    [
        {
            let mut init = PyGetSetDef {
                name: b"message\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                get: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarException,
                            _: *mut libc::c_void,
                        ) -> *mut PyObject,
                    >,
                    getter,
                >(Some(
                    exc_get_message
                        as unsafe extern fn(
                            _: *mut zbarException,
                            _: *mut libc::c_void,
                        ) -> *mut PyObject,
                )),
                set: ::std::mem::transmute::<
                    Option<
                        unsafe extern fn(
                            _: *mut zbarException,
                            _: *mut PyObject,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                    >,
                    setter,
                >(Some(
                    exc_set_message
                        as unsafe extern fn(
                            _: *mut zbarException,
                            _: *mut PyObject,
                            _: *mut libc::c_void,
                        ) -> libc::c_int,
                )),
                doc: 0 as *const libc::c_char as *mut libc::c_char,
                closure: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = PyGetSetDef {
                name: 0 as *const libc::c_char as *mut libc::c_char,
                get: None,
                set: None,
                doc: 0 as *const libc::c_char as *mut libc::c_char,
                closure: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
    ]
};
#[no_mangle]
pub static mut zbarException_Type: PyTypeObject = unsafe {
    {
        let mut init = _typeobject {
            ob_refcnt: 1 as libc::c_int as Py_ssize_t,
            ob_type: 0 as *const _typeobject as *mut _typeobject,
            ob_size: 0,
            tp_name: b"zbar.Exception\x00" as *const u8 as *const libc::c_char,
            tp_basicsize: ::std::mem::size_of::<zbarException>() as libc::c_ulong as Py_ssize_t,
            tp_itemsize: 0,
            tp_dealloc: ::std::mem::transmute::<
                Option<unsafe extern fn(_: *mut zbarException) -> ()>,
                destructor,
            >(Some(
                exc_dealloc as unsafe extern fn(_: *mut zbarException) -> (),
            )),
            tp_print: None,
            tp_getattr: None,
            tp_setattr: None,
            tp_compare: None,
            tp_repr: None,
            tp_as_number: 0 as *const PyNumberMethods as *mut PyNumberMethods,
            tp_as_sequence: 0 as *const PySequenceMethods as *mut PySequenceMethods,
            tp_as_mapping: 0 as *const PyMappingMethods as *mut PyMappingMethods,
            tp_hash: None,
            tp_call: None,
            tp_str: ::std::mem::transmute::<
                Option<unsafe extern fn(_: *mut zbarException) -> *mut PyObject>,
                reprfunc,
            >(Some(
                exc_str as unsafe extern fn(_: *mut zbarException) -> *mut PyObject,
            )),
            tp_getattro: None,
            tp_setattro: None,
            tp_as_buffer: 0 as *const PyBufferProcs as *mut PyBufferProcs,
            tp_flags: (1 as libc::c_long) << 0 as libc::c_int
                | (1 as libc::c_long) << 1 as libc::c_int
                | (1 as libc::c_long) << 3 as libc::c_int
                | (1 as libc::c_long) << 5 as libc::c_int
                | (1 as libc::c_long) << 6 as libc::c_int
                | (1 as libc::c_long) << 7 as libc::c_int
                | (1 as libc::c_long) << 8 as libc::c_int
                | 0 as libc::c_int as libc::c_long
                | (1 as libc::c_long) << 17 as libc::c_int
                | 0 as libc::c_int as libc::c_long
                | (1 as libc::c_long) << 10 as libc::c_int
                | (1 as libc::c_long) << 14 as libc::c_int,
            tp_doc: 0 as *const libc::c_char,
            tp_traverse: ::std::mem::transmute::<
                Option<
                    unsafe extern fn(
                        _: *mut zbarException,
                        _: visitproc,
                        _: *mut libc::c_void,
                    ) -> libc::c_int,
                >,
                traverseproc,
            >(Some(
                exc_traverse
                    as unsafe extern fn(
                        _: *mut zbarException,
                        _: visitproc,
                        _: *mut libc::c_void,
                    ) -> libc::c_int,
            )),
            tp_clear: ::std::mem::transmute::<
                Option<unsafe extern fn(_: *mut zbarException) -> libc::c_int>,
                inquiry,
            >(Some(
                exc_clear as unsafe extern fn(_: *mut zbarException) -> libc::c_int,
            )),
            tp_richcompare: None,
            tp_weaklistoffset: 0,
            tp_iter: None,
            tp_iternext: None,
            tp_methods: 0 as *const PyMethodDef as *mut PyMethodDef,
            tp_members: 0 as *const PyMemberDef as *mut PyMemberDef,
            tp_getset: exc_getset.as_ptr() as *mut _,
            tp_base: 0 as *const _typeobject as *mut _typeobject,
            tp_dict: 0 as *const PyObject as *mut PyObject,
            tp_descr_get: None,
            tp_descr_set: None,
            tp_dictoffset: 0,
            tp_init: ::std::mem::transmute::<
                Option<
                    unsafe extern fn(
                        _: *mut zbarException,
                        _: *mut PyObject,
                        _: *mut PyObject,
                    ) -> libc::c_int,
                >,
                initproc,
            >(Some(
                exc_init
                    as unsafe extern fn(
                        _: *mut zbarException,
                        _: *mut PyObject,
                        _: *mut PyObject,
                    ) -> libc::c_int,
            )),
            tp_alloc: None,
            tp_new: None,
            tp_free: None,
            tp_is_gc: None,
            tp_bases: 0 as *const PyObject as *mut PyObject,
            tp_mro: 0 as *const PyObject as *mut PyObject,
            tp_cache: 0 as *const PyObject as *mut PyObject,
            tp_subclasses: 0 as *const PyObject as *mut PyObject,
            tp_weaklist: 0 as *const PyObject as *mut PyObject,
            tp_del: None,
            tp_version_tag: 0,
        };
        init
    }
};
/*------------------------------------------------------------------------
 *  Copyright 2009-2010 (c) Jeff Brown <spadix@users.sourceforge.net>
 *
 *  This file is part of the ZBar Bar Code Reader.
 *
 *  The ZBar Bar Code Reader is free software; you can redistribute it
 *  and/or modify it under the terms of the GNU Lesser Public License as
 *  published by the Free Software Foundation; either version 2.1 of
 *  the License, or (at your option) any later version.
 *
 *  The ZBar Bar Code Reader is distributed in the hope that it will be
 *  useful, but WITHOUT ANY WARRANTY; without even the implied warranty
 *  of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU Lesser Public License for more details.
 *
 *  You should have received a copy of the GNU Lesser Public License
 *  along with the ZBar Bar Code Reader; if not, write to the Free
 *  Software Foundation, Inc., 51 Franklin St, Fifth Floor,
 *  Boston, MA  02110-1301  USA
 *
 *  http://sourceforge.net/projects/zbar
 *------------------------------------------------------------------------ */
#[no_mangle]
pub unsafe extern fn zbarErr_Set(mut self_0: *mut PyObject) -> *mut PyObject {
    let mut zobj: *const libc::c_void =
        (*(self_0 as *mut zbarProcessor)).zproc as *const libc::c_void;
    let mut err: zbar_error_t = _zbar_get_error_code(zobj);
    if err as libc::c_uint == ZBAR_ERR_NOMEM as libc::c_int as libc::c_uint {
        PyErr_NoMemory();
    } else if (err as libc::c_uint) < ZBAR_ERR_NUM as libc::c_int as libc::c_uint {
        let mut type_0: *mut PyObject = zbar_exc[err as usize];
        PyErr_SetObject(type_0, self_0);
    } else {
        PyErr_SetObject(zbar_exc[0 as libc::c_int as usize], self_0);
    }
    return 0 as *mut PyObject;
}
