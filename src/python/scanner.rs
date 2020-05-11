use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type PyMemberDef;
    /*@}*/
    /*------------------------------------------------------------*/
/* * @name Decoder interface
 * @anchor c-decoder
 * low-level bar width stream decoder interface.
 * identifies symbols and extracts encoded data
 */
/*@{*/
    pub type zbar_decoder_s;
    /*@}*/
    /*------------------------------------------------------------*/
/* * @name Scanner interface
 * @anchor c-scanner
 * low-level linear intensity sample stream scanner interface.
 * identifies "bar" edges and measures width between them.
 * optionally passes to bar width decoder
 */
/*@{*/
    pub type zbar_scanner_s;
    /*
_Py_NoneStruct is an object of undefined type which can be used in contexts
where NULL (nil) is not suitable (since NULL often means 'error').

Don't forget to apply Py_INCREF() when returning this value!!!
*/
    #[no_mangle]
    static mut _Py_NoneStruct: PyObject;
    #[no_mangle]
    static mut color_enum: [*mut zbarEnumItem; 2];
    #[no_mangle]
    static mut symbol_NONE: *mut zbarEnumItem;
    #[no_mangle]
    static mut zbarDecoder_Type: PyTypeObject;
    #[no_mangle]
    fn PyInt_FromLong(_: libc::c_long) -> *mut PyObject;
    #[no_mangle]
    fn PyErr_Occurred() -> *mut PyObject;
    #[no_mangle]
    fn PyArg_ParseTupleAndKeywords(_: *mut PyObject, _: *mut PyObject,
                                   _: *const libc::c_char,
                                   _: *mut *mut libc::c_char, _: ...)
     -> libc::c_int;
    /* * constructor.
 * if decoder is non-NULL it will be attached to scanner
 * and called automatically at each new edge
 * current color is initialized to ::ZBAR_SPACE
 * (so an initial BAR->SPACE transition may be discarded)
 */
    #[no_mangle]
    fn zbar_scanner_create(decoder: *mut zbar_decoder_t)
     -> *mut zbar_scanner_t;
    /* * destructor. */
    #[no_mangle]
    fn zbar_scanner_destroy(scanner: *mut zbar_scanner_t);
    /* * clear all scanner state.
 * also resets an associated decoder
 */
    #[no_mangle]
    fn zbar_scanner_reset(scanner: *mut zbar_scanner_t) -> zbar_symbol_type_t;
    /* * mark start of a new scan pass. resets color to ::ZBAR_SPACE.
 * also updates an associated decoder.
 * @returns any decode results flushed from the pipeline
 * @note when not using callback handlers, the return value should
 * be checked the same as zbar_scan_y()
 * @note call zbar_scanner_flush() at least twice before calling this
 * method to ensure no decode results are lost
 */
    #[no_mangle]
    fn zbar_scanner_new_scan(scanner: *mut zbar_scanner_t)
     -> zbar_symbol_type_t;
    /* * process next sample intensity value.
 * intensity (y) is in arbitrary relative units.
 * @returns result of zbar_decode_width() if a decoder is attached,
 * otherwise @returns (::ZBAR_PARTIAL) when new edge is detected
 * or 0 (::ZBAR_NONE) if no new edge is detected
 */
    #[no_mangle]
    fn zbar_scan_y(scanner: *mut zbar_scanner_t, y: libc::c_int)
     -> zbar_symbol_type_t;
    /* * retrieve last scanned width. */
    #[no_mangle]
    fn zbar_scanner_get_width(scanner: *const zbar_scanner_t) -> libc::c_uint;
    /* * retrieve last scanned color. */
    #[no_mangle]
    fn zbar_scanner_get_color(scanner: *const zbar_scanner_t) -> zbar_color_t;
    #[no_mangle]
    fn zbarSymbol_LookupEnum(type_0: zbar_symbol_type_t) -> *mut zbarEnumItem;
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
/*  we used to include an E, backwards compatible alias  */
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
pub type destructor = Option<unsafe extern "C" fn(_: *mut PyObject) -> ()>;
pub type PyObject = _object;
pub type inquiry
    =
    Option<unsafe extern "C" fn(_: *mut PyObject) -> libc::c_int>;
pub type freefunc = Option<unsafe extern "C" fn(_: *mut libc::c_void) -> ()>;
pub type newfunc
    =
    Option<unsafe extern "C" fn(_: *mut _typeobject, _: *mut PyObject,
                                _: *mut PyObject) -> *mut PyObject>;
pub type allocfunc
    =
    Option<unsafe extern "C" fn(_: *mut _typeobject, _: Py_ssize_t)
               -> *mut PyObject>;
pub type initproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: *mut PyObject) -> libc::c_int>;
pub type descrsetfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: *mut PyObject) -> libc::c_int>;
pub type descrgetfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: *mut PyObject) -> *mut PyObject>;
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
pub type setter
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: *mut libc::c_void) -> libc::c_int>;
pub type getter
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut libc::c_void)
               -> *mut PyObject>;
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
pub type PyCFunction
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject)
               -> *mut PyObject>;
pub type iternextfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject) -> *mut PyObject>;
pub type getiterfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject) -> *mut PyObject>;
pub type richcmpfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: libc::c_int) -> *mut PyObject>;
pub type traverseproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: visitproc,
                                _: *mut libc::c_void) -> libc::c_int>;
pub type visitproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut libc::c_void)
               -> libc::c_int>;
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
pub type releasebufferproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut Py_buffer) -> ()>;
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
pub type getbufferproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut Py_buffer,
                                _: libc::c_int) -> libc::c_int>;
pub type charbufferproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t,
                                _: *mut *mut libc::c_char) -> Py_ssize_t>;
pub type segcountproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut Py_ssize_t)
               -> Py_ssize_t>;
pub type writebufferproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t,
                                _: *mut *mut libc::c_void) -> Py_ssize_t>;
pub type readbufferproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t,
                                _: *mut *mut libc::c_void) -> Py_ssize_t>;
pub type setattrofunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: *mut PyObject) -> libc::c_int>;
pub type getattrofunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject)
               -> *mut PyObject>;
pub type reprfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject) -> *mut PyObject>;
pub type ternaryfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: *mut PyObject) -> *mut PyObject>;
pub type hashfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject) -> libc::c_long>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyMappingMethods {
    pub mp_length: lenfunc,
    pub mp_subscript: binaryfunc,
    pub mp_ass_subscript: objobjargproc,
}
pub type objobjargproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject,
                                _: *mut PyObject) -> libc::c_int>;
pub type binaryfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject)
               -> *mut PyObject>;
pub type lenfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject) -> Py_ssize_t>;
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
pub type ssizeargfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t)
               -> *mut PyObject>;
pub type objobjproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject)
               -> libc::c_int>;
pub type ssizessizeobjargproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t,
                                _: Py_ssize_t, _: *mut PyObject)
               -> libc::c_int>;
pub type ssizeobjargproc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t,
                                _: *mut PyObject) -> libc::c_int>;
pub type ssizessizeargfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: Py_ssize_t,
                                _: Py_ssize_t) -> *mut PyObject>;
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
pub type unaryfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject) -> *mut PyObject>;
pub type coercion
    =
    Option<unsafe extern "C" fn(_: *mut *mut PyObject, _: *mut *mut PyObject)
               -> libc::c_int>;
pub type cmpfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut PyObject)
               -> libc::c_int>;
pub type setattrfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut libc::c_char,
                                _: *mut PyObject) -> libc::c_int>;
pub type getattrfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut libc::c_char)
               -> *mut PyObject>;
pub type printfunc
    =
    Option<unsafe extern "C" fn(_: *mut PyObject, _: *mut FILE,
                                _: libc::c_int) -> libc::c_int>;
pub type PyTypeObject = _typeobject;
/* Integer object interface */
/*
PyIntObject represents a (long) integer.  This is an immutable object;
an integer cannot change its value after creation.

There are functions to create new integer objects, to test an object
for integer-ness, and to get the integer value.  The latter functions
returns -1 and sets errno to EBADF if the object is not an PyIntObject.
None of the functions should be applied to nil objects.

The type PyIntObject is (unfortunately) exposed here so we can declare
_Py_TrueStruct and _Py_ZeroStruct in boolobject.h; don't use this.
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PyIntObject {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub ob_ival: libc::c_long,
}
/*------------------------------------------------------------------------
 *  Copyright 2007-2010 (c) Jeff Brown <spadix@users.sourceforge.net>
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
 *------------------------------------------------------------------------*/
/* * @file
 * ZBar Barcode Reader C API definition
 */
/* * @mainpage
 *
 * interface to the barcode reader is available at several levels.
 * most applications will want to use the high-level interfaces:
 *
 * @section high-level High-Level Interfaces
 *
 * these interfaces wrap all library functionality into an easy-to-use
 * package for a specific toolkit:
 * - the "GTK+ 2.x widget" may be used with GTK GUI applications.  a
 *   Python wrapper is included for PyGtk
 * - the @ref zbar::QZBar "Qt4 widget" may be used with Qt GUI
 *   applications
 * - the Processor interface (in @ref c-processor "C" or @ref
 *   zbar::Processor "C++") adds a scanning window to an application
 *   with no GUI.
 *
 * @section mid-level Intermediate Interfaces
 *
 * building blocks used to construct high-level interfaces:
 * - the ImageScanner (in @ref c-imagescanner "C" or @ref
 *   zbar::ImageScanner "C++") looks for barcodes in a library defined
 *   image object
 * - the Window abstraction (in @ref c-window "C" or @ref
 *   zbar::Window "C++") sinks library images, displaying them on the
 *   platform display
 * - the Video abstraction (in @ref c-video "C" or @ref zbar::Video
 *   "C++") sources library images from a video device
 *
 * @section low-level Low-Level Interfaces
 *
 * direct interaction with barcode scanning and decoding:
 * - the Scanner (in @ref c-scanner "C" or @ref zbar::Scanner "C++")
 *   looks for barcodes in a linear intensity sample stream
 * - the Decoder (in @ref c-decoder "C" or @ref zbar::Decoder "C++")
 *   extracts barcodes from a stream of bar and space widths
 */
/* * @name Global library interfaces */
/*@{*/
/* * "color" of element: bar or space. */
pub type zbar_color_e = libc::c_uint;
/* *< dark area or colored bar segment */
/* *< light area or space between bars */
pub const ZBAR_BAR: zbar_color_e = 1;
pub const ZBAR_SPACE: zbar_color_e = 0;
pub type zbar_color_t = zbar_color_e;
/* * decoded symbol type. */
pub type zbar_symbol_type_e = libc::c_uint;
/* * add-on flag mask.
     * @deprecated in 0.11, GS1 add-ons are represented using composite
     * symbols of type ::ZBAR_COMPOSITE; add-on components use ::ZBAR_EAN2
     * or ::ZBAR_EAN5
     */
pub const ZBAR_ADDON: zbar_symbol_type_e = 1792;
/* * 5-digit add-on flag.
     * @deprecated in 0.11, a ::ZBAR_EAN5 component is used for
     * 5-digit GS1 add-ons
     */
pub const ZBAR_ADDON5: zbar_symbol_type_e = 1280;
/* * 2-digit add-on flag.
     * @deprecated in 0.11, a ::ZBAR_EAN2 component is used for
     * 2-digit GS1 add-ons
     */
pub const ZBAR_ADDON2: zbar_symbol_type_e = 512;
/* *< Code 128 */
/* * mask for base symbol type.
     * @deprecated in 0.11, remove this from existing code
     */
pub const ZBAR_SYMBOL: zbar_symbol_type_e = 255;
/* *< Code 93. @since 0.11 */
pub const ZBAR_CODE128: zbar_symbol_type_e = 128;
/* *< QR Code. @since 0.10 */
pub const ZBAR_CODE93: zbar_symbol_type_e = 93;
/* *< PDF417. @since 0.6 */
pub const ZBAR_QRCODE: zbar_symbol_type_e = 64;
/* *< Code 39. @since 0.4 */
pub const ZBAR_PDF417: zbar_symbol_type_e = 57;
/* *< Codabar. @since 0.11 */
pub const ZBAR_CODE39: zbar_symbol_type_e = 39;
/* *< GS1 DataBar Expanded. @since 0.11 */
pub const ZBAR_CODABAR: zbar_symbol_type_e = 38;
/* *< GS1 DataBar (RSS). @since 0.11 */
pub const ZBAR_DATABAR_EXP: zbar_symbol_type_e = 35;
/* *< Interleaved 2 of 5. @since 0.4 */
pub const ZBAR_DATABAR: zbar_symbol_type_e = 34;
/* *< EAN/UPC composite */
pub const ZBAR_I25: zbar_symbol_type_e = 25;
/* *< ISBN-13 (from EAN-13). @since 0.4 */
pub const ZBAR_COMPOSITE: zbar_symbol_type_e = 15;
/* *< EAN-13 */
pub const ZBAR_ISBN13: zbar_symbol_type_e = 14;
/* *< UPC-A */
pub const ZBAR_EAN13: zbar_symbol_type_e = 13;
/* *< ISBN-10 (from EAN-13). @since 0.4 */
pub const ZBAR_UPCA: zbar_symbol_type_e = 12;
/* *< UPC-E */
pub const ZBAR_ISBN10: zbar_symbol_type_e = 10;
/* *< EAN-8 */
pub const ZBAR_UPCE: zbar_symbol_type_e = 9;
/* *< GS1 5-digit add-on */
pub const ZBAR_EAN8: zbar_symbol_type_e = 8;
/* *< GS1 2-digit add-on */
pub const ZBAR_EAN5: zbar_symbol_type_e = 5;
/* *< intermediate status */
pub const ZBAR_EAN2: zbar_symbol_type_e = 2;
/* *< no symbol decoded */
pub const ZBAR_PARTIAL: zbar_symbol_type_e = 1;
pub const ZBAR_NONE: zbar_symbol_type_e = 0;
pub type zbar_symbol_type_t = zbar_symbol_type_e;
/* * opaque decoder object. */
pub type zbar_decoder_t = zbar_decoder_s;
/* * opaque scanner object. */
pub type zbar_scanner_t = zbar_scanner_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbarEnumItem {
    pub val: PyIntObject,
    pub name: *mut PyObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbarDecoder {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub zdcode: *mut zbar_decoder_t,
    pub handler: *mut PyObject,
    pub args: *mut PyObject,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zbarScanner {
    pub ob_refcnt: Py_ssize_t,
    pub ob_type: *mut _typeobject,
    pub zscn: *mut zbar_scanner_t,
    pub decoder: *mut zbarDecoder,
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
 *------------------------------------------------------------------------*/
static mut scanner_doc: [libc::c_char; 107] =
    [108, 111, 119, 32, 108, 101, 118, 101, 108, 32, 105, 110, 116, 101, 110,
     115, 105, 116, 121, 32, 115, 97, 109, 112, 108, 101, 32, 115, 116, 114,
     101, 97, 109, 32, 115, 99, 97, 110, 110, 101, 114, 46, 32, 32, 105, 100,
     101, 110, 116, 105, 102, 105, 101, 115, 32, 34, 98, 97, 114, 34, 32, 101,
     100, 103, 101, 115, 97, 110, 100, 32, 109, 101, 97, 115, 117, 114, 101,
     115, 32, 119, 105, 100, 116, 104, 32, 98, 101, 116, 119, 101, 101, 110,
     32, 116, 104, 101, 109, 46, 10, 10, 70, 73, 88, 77, 69, 46, 0];
unsafe extern "C" fn scanner_new(mut type_0: *mut PyTypeObject,
                                 mut args: *mut PyObject,
                                 mut kwds: *mut PyObject)
 -> *mut zbarScanner {
    let mut decoder: *mut zbarDecoder = 0 as *mut zbarDecoder;
    static mut kwlist: [*mut libc::c_char; 2] =
        [b"decoder\x00" as *const u8 as *const libc::c_char as
             *mut libc::c_char,
         0 as *const libc::c_char as *mut libc::c_char];
    if PyArg_ParseTupleAndKeywords(args, kwds,
                                   b"|O!\x00" as *const u8 as
                                       *const libc::c_char,
                                   kwlist.as_mut_ptr(),
                                   &mut decoder as *mut *mut zbarDecoder,
                                   zbarDecoder_Type) == 0 {
        return 0 as *mut zbarScanner
    }
    let mut self_0: *mut zbarScanner =
        (*type_0).tp_alloc.expect("non-null function pointer")(type_0,
                                                               0 as
                                                                   libc::c_int
                                                                   as
                                                                   Py_ssize_t)
            as *mut zbarScanner;
    if self_0.is_null() { return 0 as *mut zbarScanner }
    let mut zdcode: *mut zbar_decoder_t = 0 as *mut zbar_decoder_t;
    if !decoder.is_null() {
        let ref mut fresh0 = (*(decoder as *mut PyObject)).ob_refcnt;
        *fresh0 += 1;
        (*self_0).decoder = decoder;
        zdcode = (*decoder).zdcode
    }
    (*self_0).zscn = zbar_scanner_create(zdcode);
    if (*self_0).zscn.is_null() {
        let ref mut fresh1 = (*(self_0 as *mut PyObject)).ob_refcnt;
        *fresh1 -= 1;
        if !(*fresh1 != 0 as libc::c_int as libc::c_long) {
            Some((*(*(self_0 as
                          *mut PyObject)).ob_type).tp_dealloc.expect("non-null function pointer")).expect("non-null function pointer")(self_0
                                                                                                                                           as
                                                                                                                                           *mut PyObject);
        }
        return 0 as *mut zbarScanner
    }
    return self_0;
}
unsafe extern "C" fn scanner_traverse(mut self_0: *mut zbarScanner,
                                      mut visit: visitproc,
                                      mut arg: *mut libc::c_void)
 -> libc::c_int {
    if !(*self_0).decoder.is_null() {
        let mut vret: libc::c_int =
            visit.expect("non-null function pointer")((*self_0).decoder as
                                                          *mut PyObject, arg);
        if vret != 0 { return vret }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn scanner_clear(mut self_0: *mut zbarScanner)
 -> libc::c_int {
    if !(*self_0).decoder.is_null() {
        let mut _py_tmp: *mut PyObject = (*self_0).decoder as *mut PyObject;
        (*self_0).decoder = 0 as *mut zbarDecoder;
        (*_py_tmp).ob_refcnt -= 1;
        if !((*_py_tmp).ob_refcnt != 0 as libc::c_int as libc::c_long) {
            Some((*(*_py_tmp).ob_type).tp_dealloc.expect("non-null function pointer")).expect("non-null function pointer")(_py_tmp);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn scanner_dealloc(mut self_0: *mut zbarScanner) {
    scanner_clear(self_0);
    zbar_scanner_destroy((*self_0).zscn);
    (*(*(self_0 as
             *mut PyObject)).ob_type).tp_free.expect("non-null function pointer")(self_0
                                                                                      as
                                                                                      *mut PyObject
                                                                                      as
                                                                                      *mut libc::c_void);
}
unsafe extern "C" fn scanner_get_width(mut self_0: *mut zbarScanner,
                                       mut closure: *mut libc::c_void)
 -> *mut PyObject {
    let mut width: libc::c_uint = zbar_scanner_get_width((*self_0).zscn);
    return PyInt_FromLong(width as libc::c_long);
}
unsafe extern "C" fn scanner_get_color(mut self_0: *mut zbarScanner,
                                       mut closure: *mut libc::c_void)
 -> *mut zbarEnumItem {
    let mut zcol: zbar_color_t = zbar_scanner_get_color((*self_0).zscn);
    let mut color: *mut zbarEnumItem = color_enum[zcol as usize];
    let ref mut fresh2 = (*(color as *mut PyObject)).ob_refcnt;
    *fresh2 += 1;
    return color;
}
static mut scanner_getset: [PyGetSetDef; 3] =
    unsafe {
        [{
             let mut init =
                 PyGetSetDef{name:
                                 b"color\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             get:
                                 ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                         *mut zbarScanner,
                                                                                     _:
                                                                                         *mut libc::c_void)
                                                                    ->
                                                                        *mut zbarEnumItem>,
                                                         getter>(Some(scanner_get_color
                                                                          as
                                                                          unsafe extern "C" fn(_:
                                                                                                   *mut zbarScanner,
                                                                                               _:
                                                                                                   *mut libc::c_void)
                                                                              ->
                                                                                  *mut zbarEnumItem)),
                             set: None,
                             doc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             closure:
                                 0 as *const libc::c_void as
                                     *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 PyGetSetDef{name:
                                 b"width\x00" as *const u8 as
                                     *const libc::c_char as *mut libc::c_char,
                             get:
                                 ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                         *mut zbarScanner,
                                                                                     _:
                                                                                         *mut libc::c_void)
                                                                    ->
                                                                        *mut PyObject>,
                                                         getter>(Some(scanner_get_width
                                                                          as
                                                                          unsafe extern "C" fn(_:
                                                                                                   *mut zbarScanner,
                                                                                               _:
                                                                                                   *mut libc::c_void)
                                                                              ->
                                                                                  *mut PyObject)),
                             set: None,
                             doc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             closure:
                                 0 as *const libc::c_void as
                                     *mut libc::c_void,};
             init
         },
         {
             let mut init =
                 PyGetSetDef{name:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             get: None,
                             set: None,
                             doc:
                                 0 as *const libc::c_char as
                                     *mut libc::c_char,
                             closure:
                                 0 as *const libc::c_void as
                                     *mut libc::c_void,};
             init
         }]
    };
unsafe extern "C" fn scanner_reset(mut self_0: *mut zbarScanner,
                                   mut args: *mut PyObject,
                                   mut kwds: *mut PyObject) -> *mut PyObject {
    static mut kwlist: [*mut libc::c_char; 1] =
        [0 as *const libc::c_char as *mut libc::c_char];
    if PyArg_ParseTupleAndKeywords(args, kwds,
                                   b"\x00" as *const u8 as
                                       *const libc::c_char,
                                   kwlist.as_mut_ptr()) == 0 {
        return 0 as *mut PyObject
    }
    zbar_scanner_reset((*self_0).zscn);
    let ref mut fresh3 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh3 += 1;
    return &mut _Py_NoneStruct;
}
unsafe extern "C" fn scanner_new_scan(mut self_0: *mut zbarScanner,
                                      mut args: *mut PyObject,
                                      mut kwds: *mut PyObject)
 -> *mut PyObject {
    static mut kwlist: [*mut libc::c_char; 1] =
        [0 as *const libc::c_char as *mut libc::c_char];
    if PyArg_ParseTupleAndKeywords(args, kwds,
                                   b"\x00" as *const u8 as
                                       *const libc::c_char,
                                   kwlist.as_mut_ptr()) == 0 {
        return 0 as *mut PyObject
    }
    zbar_scanner_new_scan((*self_0).zscn);
    let ref mut fresh4 = (*(&mut _Py_NoneStruct as *mut PyObject)).ob_refcnt;
    *fresh4 += 1;
    return &mut _Py_NoneStruct;
}
unsafe extern "C" fn scanner_scan_y(mut self_0: *mut zbarScanner,
                                    mut args: *mut PyObject,
                                    mut kwds: *mut PyObject)
 -> *mut zbarEnumItem {
    /* FIXME should accept sequence of values */
    let mut y: libc::c_int = 0 as libc::c_int;
    static mut kwlist: [*mut libc::c_char; 2] =
        [b"y\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
         0 as *const libc::c_char as *mut libc::c_char];
    if PyArg_ParseTupleAndKeywords(args, kwds,
                                   b"i\x00" as *const u8 as
                                       *const libc::c_char,
                                   kwlist.as_mut_ptr(),
                                   &mut y as *mut libc::c_int) == 0 {
        return 0 as *mut zbarEnumItem
    }
    let mut sym: zbar_symbol_type_t = zbar_scan_y((*self_0).zscn, y);
    if !PyErr_Occurred().is_null() {
        /* propagate errors during callback */
        return 0 as *mut zbarEnumItem
    }
    if sym as libc::c_uint == ZBAR_NONE as libc::c_int as libc::c_uint {
        /* hardcode most common case */
        let ref mut fresh5 = (*(symbol_NONE as *mut PyObject)).ob_refcnt;
        *fresh5 += 1;
        return symbol_NONE
    }
    return zbarSymbol_LookupEnum(sym);
}
static mut scanner_methods: [PyMethodDef; 4] =
    unsafe {
        [{
             let mut init =
                 PyMethodDef{ml_name:
                                 b"reset\x00" as *const u8 as
                                     *const libc::c_char,
                             ml_meth:
                                 ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                         *mut zbarScanner,
                                                                                     _:
                                                                                         *mut PyObject,
                                                                                     _:
                                                                                         *mut PyObject)
                                                                    ->
                                                                        *mut PyObject>,
                                                         PyCFunction>(Some(scanner_reset
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut zbarScanner,
                                                                                                    _:
                                                                                                        *mut PyObject,
                                                                                                    _:
                                                                                                        *mut PyObject)
                                                                                   ->
                                                                                       *mut PyObject)),
                             ml_flags:
                                 0x1 as libc::c_int | 0x2 as libc::c_int,
                             ml_doc: 0 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 PyMethodDef{ml_name:
                                 b"new_scan\x00" as *const u8 as
                                     *const libc::c_char,
                             ml_meth:
                                 ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                         *mut zbarScanner,
                                                                                     _:
                                                                                         *mut PyObject,
                                                                                     _:
                                                                                         *mut PyObject)
                                                                    ->
                                                                        *mut PyObject>,
                                                         PyCFunction>(Some(scanner_new_scan
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut zbarScanner,
                                                                                                    _:
                                                                                                        *mut PyObject,
                                                                                                    _:
                                                                                                        *mut PyObject)
                                                                                   ->
                                                                                       *mut PyObject)),
                             ml_flags:
                                 0x1 as libc::c_int | 0x2 as libc::c_int,
                             ml_doc: 0 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 PyMethodDef{ml_name:
                                 b"scan_y\x00" as *const u8 as
                                     *const libc::c_char,
                             ml_meth:
                                 ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                         *mut zbarScanner,
                                                                                     _:
                                                                                         *mut PyObject,
                                                                                     _:
                                                                                         *mut PyObject)
                                                                    ->
                                                                        *mut zbarEnumItem>,
                                                         PyCFunction>(Some(scanner_scan_y
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut zbarScanner,
                                                                                                    _:
                                                                                                        *mut PyObject,
                                                                                                    _:
                                                                                                        *mut PyObject)
                                                                                   ->
                                                                                       *mut zbarEnumItem)),
                             ml_flags:
                                 0x1 as libc::c_int | 0x2 as libc::c_int,
                             ml_doc: 0 as *const libc::c_char,};
             init
         },
         {
             let mut init =
                 PyMethodDef{ml_name: 0 as *const libc::c_char,
                             ml_meth: None,
                             ml_flags: 0,
                             ml_doc: 0 as *const libc::c_char,};
             init
         }]
    };
#[no_mangle]
pub static mut zbarScanner_Type: PyTypeObject =
    unsafe {
        {
            let mut init =
                _typeobject{ob_refcnt: 1 as libc::c_int as Py_ssize_t,
                            ob_type:
                                0 as *const _typeobject as *mut _typeobject,
                            ob_size: 0,
                            tp_name:
                                b"zbar.Scanner\x00" as *const u8 as
                                    *const libc::c_char,
                            tp_basicsize:
                                ::std::mem::size_of::<zbarScanner>() as
                                    libc::c_ulong as Py_ssize_t,
                            tp_itemsize: 0,
                            tp_dealloc:
                                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                        *mut zbarScanner)
                                                                   -> ()>,
                                                        destructor>(Some(scanner_dealloc
                                                                             as
                                                                             unsafe extern "C" fn(_:
                                                                                                      *mut zbarScanner)
                                                                                 ->
                                                                                     ())),
                            tp_print: None,
                            tp_getattr: None,
                            tp_setattr: None,
                            tp_compare: None,
                            tp_repr: None,
                            tp_as_number:
                                0 as *const PyNumberMethods as
                                    *mut PyNumberMethods,
                            tp_as_sequence:
                                0 as *const PySequenceMethods as
                                    *mut PySequenceMethods,
                            tp_as_mapping:
                                0 as *const PyMappingMethods as
                                    *mut PyMappingMethods,
                            tp_hash: None,
                            tp_call: None,
                            tp_str: None,
                            tp_getattro: None,
                            tp_setattro: None,
                            tp_as_buffer:
                                0 as *const PyBufferProcs as
                                    *mut PyBufferProcs,
                            tp_flags:
                                (1 as libc::c_long) << 0 as libc::c_int |
                                    (1 as libc::c_long) << 1 as libc::c_int |
                                    (1 as libc::c_long) << 3 as libc::c_int |
                                    (1 as libc::c_long) << 5 as libc::c_int |
                                    (1 as libc::c_long) << 6 as libc::c_int |
                                    (1 as libc::c_long) << 7 as libc::c_int |
                                    (1 as libc::c_long) << 8 as libc::c_int |
                                    0 as libc::c_int as libc::c_long |
                                    (1 as libc::c_long) << 17 as libc::c_int |
                                    0 as libc::c_int as libc::c_long |
                                    (1 as libc::c_long) << 10 as libc::c_int |
                                    (1 as libc::c_long) << 14 as libc::c_int,
                            tp_doc: scanner_doc.as_ptr() as *mut _,
                            tp_traverse:
                                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                        *mut zbarScanner,
                                                                                    _:
                                                                                        visitproc,
                                                                                    _:
                                                                                        *mut libc::c_void)
                                                                   ->
                                                                       libc::c_int>,
                                                        traverseproc>(Some(scanner_traverse
                                                                               as
                                                                               unsafe extern "C" fn(_:
                                                                                                        *mut zbarScanner,
                                                                                                    _:
                                                                                                        visitproc,
                                                                                                    _:
                                                                                                        *mut libc::c_void)
                                                                                   ->
                                                                                       libc::c_int)),
                            tp_clear:
                                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                        *mut zbarScanner)
                                                                   ->
                                                                       libc::c_int>,
                                                        inquiry>(Some(scanner_clear
                                                                          as
                                                                          unsafe extern "C" fn(_:
                                                                                                   *mut zbarScanner)
                                                                              ->
                                                                                  libc::c_int)),
                            tp_richcompare: None,
                            tp_weaklistoffset: 0,
                            tp_iter: None,
                            tp_iternext: None,
                            tp_methods: scanner_methods.as_ptr() as *mut _,
                            tp_members:
                                0 as *const PyMemberDef as *mut PyMemberDef,
                            tp_getset: scanner_getset.as_ptr() as *mut _,
                            tp_base:
                                0 as *const _typeobject as *mut _typeobject,
                            tp_dict: 0 as *const PyObject as *mut PyObject,
                            tp_descr_get: None,
                            tp_descr_set: None,
                            tp_dictoffset: 0,
                            tp_init: None,
                            tp_alloc: None,
                            tp_new:
                                ::std::mem::transmute::<Option<unsafe extern "C" fn(_:
                                                                                        *mut PyTypeObject,
                                                                                    _:
                                                                                        *mut PyObject,
                                                                                    _:
                                                                                        *mut PyObject)
                                                                   ->
                                                                       *mut zbarScanner>,
                                                        newfunc>(Some(scanner_new
                                                                          as
                                                                          unsafe extern "C" fn(_:
                                                                                                   *mut PyTypeObject,
                                                                                               _:
                                                                                                   *mut PyObject,
                                                                                               _:
                                                                                                   *mut PyObject)
                                                                              ->
                                                                                  *mut zbarScanner)),
                            tp_free: None,
                            tp_is_gc: None,
                            tp_bases: 0 as *const PyObject as *mut PyObject,
                            tp_mro: 0 as *const PyObject as *mut PyObject,
                            tp_cache: 0 as *const PyObject as *mut PyObject,
                            tp_subclasses:
                                0 as *const PyObject as *mut PyObject,
                            tp_weaklist:
                                0 as *const PyObject as *mut PyObject,
                            tp_del: None,
                            tp_version_tag: 0,};
            init
        }
    };
